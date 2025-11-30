//! Streaming Pipeline
//!
//! Orchestrates real-time XTTS streaming with semantic chunking,
//! pre-buffering, and adaptive GPU control

use crate::audio_buffer::{AudioBuffer, AudioChunk};
use crate::error::Result;
use crate::gpu_config::GpuConfig;
use crate::gpu_detector::{GpuCapability, GpuDetector};
use crate::interjections::InterjectionManager;
use crate::prebuffer_manager::{PreBufferManager, PreBufferState};
use crate::semantic_chunker::{SemanticChunker, TextChunk};
use crate::tts_profile::TtsProfile;
use crate::xtts::{AudioOutput, SharedXttsModel};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

/// Streaming request
#[derive(Debug, Clone)]
pub struct StreamingRequest {
    /// Text to synthesize
    pub text: String,
    /// Character ID for voice embedding
    pub character_id: String,
    /// Language code
    pub language: String,
    /// TTS Profile (FAST or CINEMATIC)
    pub profile: Option<TtsProfile>,
    /// LLM model name (used to auto-select profile if profile is None)
    pub llm_model_name: Option<String>,
    /// Timestamp when user finished speaking (for interjection delay calculation)
    pub user_speech_end_ts: Option<std::time::Instant>,
}

/// Streaming status
#[derive(Debug, Clone)]
pub enum StreamingStatus {
    /// Initializing
    Initializing,
    /// Chunking text
    Chunking,
    /// Generating audio
    Generating,
    /// Playing
    Playing,
    /// Completed
    Completed,
    /// Error
    Error(String),
    /// Cancelled
    Cancelled,
}

/// Streaming pipeline
pub struct StreamingPipeline {
    xtts_model: SharedXttsModel,
    gpu_config: GpuConfig,
    chunker: SemanticChunker,
    buffer: Arc<AudioBuffer>,
    prebuffer_manager: Arc<RwLock<PreBufferManager>>,
    status_tx: Option<mpsc::Sender<StreamingStatus>>,
    interjection_manager: Option<Arc<InterjectionManager>>,
    base_path: std::path::PathBuf,
}

impl StreamingPipeline {
    /// Create new streaming pipeline
    pub async fn new(
        xtts_model: SharedXttsModel,
        gpu_config: GpuConfig,
        base_path: std::path::PathBuf,
    ) -> Result<Self> {
        // Detect GPU capabilities
        let capability = GpuDetector::detect()?;
        info!("GPU detected: {} ({})", capability.gpu_name, capability.tier);

        // Create audio buffer (24kHz, mono, 10s max)
        let buffer = Arc::new(AudioBuffer::new(24000, 1, 10.0));

        // Create pre-buffer manager
        let prebuffer_manager = Arc::new(RwLock::new(PreBufferManager::new(gpu_config.clone())));

        // Create semantic chunker
        let chunker = SemanticChunker::new(Default::default());

        // Try to load interjection manager
        let interjection_manager = {
            let config_path = base_path.join("config").join("interjections.yaml");
            if !config_path.exists() {
                warn!("Interjection config not found: {:?}", config_path);
                None
            } else {
                match InterjectionManager::from_config_file(&config_path, base_path.clone()) {
                    Ok(manager) => {
                        info!("Interjection manager loaded successfully for streaming");
                        Some(Arc::new(manager))
                    }
                    Err(e) => {
                        warn!("Failed to load interjection manager: {}", e);
                        None
                    }
                }
            }
        };

        Ok(Self {
            xtts_model,
            gpu_config,
            chunker,
            buffer,
            prebuffer_manager,
            status_tx: None,
            interjection_manager,
            base_path,
        })
    }

    /// Start streaming synthesis
    pub async fn stream(
        &self,
        request: StreamingRequest,
    ) -> Result<mpsc::Receiver<Vec<i16>>> {
        // Create status channel (not used yet, but reserved for future status updates)
        let _status_tx = mpsc::channel::<StreamingStatus>(10).0;

        // Create audio output channel
        let (audio_tx, audio_rx) = mpsc::channel(100);

        // Determine TTS profile
        let profile = if let Some(profile) = &request.profile {
            profile.clone()
        } else if let Some(llm_model) = &request.llm_model_name {
            TtsProfile::from_llm_model(llm_model)
        } else {
            TtsProfile::cinematic() // Default to cinematic
        };
        
        info!("Using TTS profile: {:?}", profile.profile_type);
        
        // Chunk text with profile-specific configuration
        let text_chunks = self.chunker.chunk_with_profile(&request.text, Some(&profile))?;
        info!("Chunked text into {} chunks (first: {} chars, next: {} chars)", 
              text_chunks.len(), 
              text_chunks.first().map(|c| c.char_count).unwrap_or(0),
              profile.next_chunk_max_chars);

        // Start generation task
        let xtts_model = self.xtts_model.clone();
        let buffer = self.buffer.clone();
        let prebuffer_manager = self.prebuffer_manager.clone();
        let gpu_config = self.gpu_config.clone();
        let character_id = request.character_id.clone();
        let language = request.language.clone();
        let profile_clone = profile.clone();

        tokio::spawn(async move {
            // Generate chunks with adaptive parallelism based on GPU tier
            let use_parallel = gpu_config.max_parallel_streams > 1;
            
            if use_parallel {
                // Parallel generation for High-End GPUs
                self::generate_chunks_parallel(
                    text_chunks,
                    xtts_model,
                    buffer,
                    prebuffer_manager,
                    audio_tx,
                    character_id,
                    gpu_config,
                ).await;
            } else {
                // Sequential generation for Modest/Low-End GPUs
                for (i, text_chunk) in text_chunks.iter().enumerate() {
                    info!("Generating chunk {}/{}", i + 1, text_chunks.len());

                    // Generate audio with XTTS
                    let synthesis_request = crate::xtts::SynthesisRequest {
                        text: text_chunk.text.clone(),
                        voice_id: character_id.clone(),
                        speed: 1.0,
                        pitch: 0.5,
                    };

                    let xtts_guard = xtts_model.read().await;
                    match xtts_guard.synthesize(&synthesis_request).await {
                        Ok(audio_output) => {
                            drop(xtts_guard); // Release lock early
                            
                            // Create audio chunk (convert to mono if needed)
                            let samples = if audio_output.channels == 1 {
                                audio_output.samples
                            } else {
                                // Convert stereo to mono (average channels)
                                audio_output.samples
                                    .chunks(audio_output.channels as usize)
                                    .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
                                    .collect()
                            };

                            let chunk = AudioChunk {
                                samples,
                                sample_rate: audio_output.sample_rate,
                                channels: 1, // Always mono for streaming
                            };

                            // Push to buffer
                            if let Err(e) = buffer.push(chunk) {
                                warn!("Failed to push chunk to buffer: {}", e);
                                break;
                            }

                            // Update pre-buffer manager
                            let mut manager = prebuffer_manager.write().await;
                            manager.update_state(&buffer).unwrap();
                            drop(manager);

                            // Send audio to output channel
                            let int16_samples = buffer.pop_block(2400).unwrap_or_default();
                            if let Err(_) = audio_tx.send(int16_samples).await {
                                // Receiver dropped
                                break;
                            }

                            // Yield GPU if configured
                            if gpu_config.yield_between_chunks {
                                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to generate chunk: {}", e);
                            break;
                        }
                    }
                }
            }
        });

        Ok(audio_rx)
    }

    /// Cancel streaming
    pub async fn cancel(&self) -> Result<()> {
        self.buffer.clear()?;
        let mut manager = self.prebuffer_manager.write().await;
        manager.set_state(PreBufferState::Paused);
        Ok(())
    }

    /// Get buffer length
    pub fn buffer_length_seconds(&self) -> Result<f32> {
        self.buffer.buffer_length_seconds()
    }

    /// Get pre-buffer state
    pub async fn prebuffer_state(&self) -> PreBufferState {
        self.prebuffer_manager.read().await.state()
    }
}

/// Generate chunks in parallel (for High-End GPUs)
async fn generate_chunks_parallel(
    text_chunks: Vec<TextChunk>,
    xtts_model: SharedXttsModel,
    buffer: Arc<AudioBuffer>,
    prebuffer_manager: Arc<RwLock<PreBufferManager>>,
    audio_tx: mpsc::Sender<Vec<i16>>,
    character_id: String,
    gpu_config: GpuConfig,
) {
    use futures::future::join_all;
    
    // Pre-generate first 2 chunks before starting playback
    let prebuffer_count = (gpu_config.prebuffer_seconds / 3.0).ceil() as usize;
    let prebuffer_count = prebuffer_count.min(text_chunks.len()).max(1);
    
    info!("Pre-generating {} chunks before playback", prebuffer_count);
    
    // Generate pre-buffer chunks in parallel
    let mut prebuffer_tasks = Vec::new();
    for i in 0..prebuffer_count.min(text_chunks.len()) {
        let chunk = text_chunks[i].clone();
        let xtts_model = xtts_model.clone();
        let character_id = character_id.clone();
        
        prebuffer_tasks.push(tokio::spawn(async move {
            let synthesis_request = crate::xtts::SynthesisRequest {
                text: chunk.text,
                voice_id: character_id,
                speed: 1.0,
                pitch: 0.5,
            };
            
            let xtts_guard = xtts_model.read().await;
            xtts_guard.synthesize(&synthesis_request).await
        }));
    }
    
    // Wait for pre-buffer chunks
    let prebuffer_results: Vec<_> = join_all(prebuffer_tasks).await;
    
    // Push pre-buffer chunks to buffer
    for (i, result) in prebuffer_results.iter().enumerate() {
        if let Ok(Ok(audio_output)) = result {
            let samples = if audio_output.channels == 1 {
                audio_output.samples.clone()
            } else {
                audio_output.samples
                    .chunks(audio_output.channels as usize)
                    .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
                    .collect()
            };
            
            let chunk = AudioChunk {
                samples,
                sample_rate: audio_output.sample_rate,
                channels: 1,
            };
            
            if let Err(e) = buffer.push(chunk) {
                warn!("Failed to push pre-buffer chunk {}: {}", i, e);
            }
        }
    }
    
    // Generate remaining chunks with staggered parallelism
    for i in prebuffer_count..text_chunks.len() {
        info!("Generating chunk {}/{}", i + 1, text_chunks.len());
        
        // Generate current chunk
        let synthesis_request = crate::xtts::SynthesisRequest {
            text: text_chunks[i].text.clone(),
            voice_id: character_id.clone(),
            speed: 1.0,
            pitch: 0.5,
        };
        
        let xtts_guard = xtts_model.read().await;
        match xtts_guard.synthesize(&synthesis_request).await {
            Ok(audio_output) => {
                drop(xtts_guard);
                
                let samples = if audio_output.channels == 1 {
                    audio_output.samples
                } else {
                    audio_output.samples
                        .chunks(audio_output.channels as usize)
                        .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
                        .collect()
                };
                
                let chunk = AudioChunk {
                    samples,
                    sample_rate: audio_output.sample_rate,
                    channels: 1,
                };
                
                if let Err(e) = buffer.push(chunk) {
                    warn!("Failed to push chunk to buffer: {}", e);
                    break;
                }
                
                // Update pre-buffer manager
                let mut manager = prebuffer_manager.write().await;
                manager.update_state(&buffer).unwrap();
                drop(manager);
                
                // Send audio to output channel
                let int16_samples = buffer.pop_block(2400).unwrap_or_default();
                if let Err(_) = audio_tx.send(int16_samples).await {
                    break;
                }
            }
            Err(e) => {
                warn!("Failed to generate chunk: {}", e);
                break;
            }
        }
        
        // Start generating next chunk in parallel if available
        if i + 1 < text_chunks.len() && gpu_config.max_parallel_streams > 1 {
            let next_chunk = text_chunks[i + 1].clone();
            let xtts_model_clone = xtts_model.clone();
            let character_id_clone = character_id.clone();
            
            tokio::spawn(async move {
                let synthesis_request = crate::xtts::SynthesisRequest {
                    text: next_chunk.text,
                    voice_id: character_id_clone,
                    speed: 1.0,
                    pitch: 0.5,
                };
                
                let xtts_guard = xtts_model_clone.read().await;
                let _ = xtts_guard.synthesize(&synthesis_request).await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full integration tests require XTTS model
    // These are placeholder tests
    #[tokio::test]
    async fn test_streaming_pipeline_creation() {
        // This would require a real XTTS model
        // For now, just test that the structure compiles
    }
}

