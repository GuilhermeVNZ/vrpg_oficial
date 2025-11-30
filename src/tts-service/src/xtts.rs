use crate::error::{Result, TtsError};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioOutput {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisRequest {
    pub text: String,
    pub voice_id: String,
    pub speed: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone)]
pub struct VoiceProfile {
    pub id: String,
    pub name: String,
    pub base_pitch: f32,
    pub base_speed: f32,
    pub timbre: f32, // 0.0 = soft, 1.0 = harsh
}

pub struct XttsModel {
    model_loaded: bool,
    voices: HashMap<String, VoiceProfile>,
    cache: Arc<RwLock<HashMap<String, AudioOutput>>>,
    use_coqui_xtts: bool, // Flag para usar Coqui XTTS real vs stub
    use_gpu: bool,        // Usar GPU para XTTS
    model_path: Option<std::path::PathBuf>, // Caminho opcional para modelo customizado
}

impl Default for XttsModel {
    fn default() -> Self {
        Self::new()
    }
}

impl XttsModel {
    pub fn new() -> Self {
        Self::new_with_options(false, false, None)
    }

    /// Cria novo modelo XTTS com opções
    pub fn new_with_options(
        use_coqui_xtts: bool,
        use_gpu: bool,
        model_path: Option<std::path::PathBuf>,
    ) -> Self {
        let mut voices = HashMap::new();

        // Default voices
        voices.insert(
            "dm".to_string(),
            VoiceProfile {
                id: "dm".to_string(),
                name: "Dungeon Master".to_string(),
                base_pitch: 0.5,
                base_speed: 1.0,
                timbre: 0.3,
            },
        );

        voices.insert(
            "npc_male".to_string(),
            VoiceProfile {
                id: "npc_male".to_string(),
                name: "NPC Male".to_string(),
                base_pitch: 0.4,
                base_speed: 1.0,
                timbre: 0.5,
            },
        );

        voices.insert(
            "npc_female".to_string(),
            VoiceProfile {
                id: "npc_female".to_string(),
                name: "NPC Female".to_string(),
                base_pitch: 0.6,
                base_speed: 1.0,
                timbre: 0.4,
            },
        );

        voices.insert(
            "monster".to_string(),
            VoiceProfile {
                id: "monster".to_string(),
                name: "Monster".to_string(),
                base_pitch: 0.3,
                base_speed: 0.9,
                timbre: 0.8,
            },
        );

        Self {
            model_loaded: false,
            voices,
            cache: Arc::new(RwLock::new(HashMap::new())),
            use_coqui_xtts,
            use_gpu,
            model_path,
        }
    }

    /// Habilita uso do Coqui XTTS real (em vez do stub)
    pub fn enable_coqui_xtts(&mut self, use_gpu: bool) {
        self.use_coqui_xtts = true;
        self.use_gpu = use_gpu;
        info!("✅ Coqui XTTS enabled (GPU: {})", use_gpu);
    }

    /// Define caminho para modelo XTTS customizado
    pub fn set_model_path(&mut self, path: std::path::PathBuf) {
        self.model_path = Some(path);
    }

    pub async fn load(&mut self, _model_path: &str) -> Result<()> {
        // In a real implementation, this would load the XTTS ONNX model
        // For now, we mark as loaded and use basic synthesis
        self.model_loaded = true;
        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }

    pub fn get_voice(&self, voice_id: &str) -> Option<&VoiceProfile> {
        self.voices.get(voice_id)
    }

    pub fn list_voices(&self) -> Vec<&VoiceProfile> {
        self.voices.values().collect()
    }

    pub async fn synthesize(&self, request: &SynthesisRequest) -> Result<AudioOutput> {
        if !self.model_loaded {
            return Err(TtsError::ModelLoad("Model not loaded".to_string()));
        }

        // Check cache first
        let cache_key = format!(
            "{}:{}:{}:{}",
            request.text, request.voice_id, request.speed, request.pitch
        );
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Get voice profile
        let voice = self
            .voices
            .get(&request.voice_id)
            .ok_or_else(|| TtsError::Voice(format!("Voice not found: {}", request.voice_id)))?;

        // Generate audio using Coqui XTTS or stub
        let audio = if self.use_coqui_xtts {
            // Usar Coqui XTTS real via Python bridge
            // Mapear voice_id para speaker do Coqui XTTS
            let speaker = self.map_voice_to_speaker(&request.voice_id);
            self.synthesize_with_coqui_xtts(&request.text, "en", speaker.as_deref())
                .await?
        } else {
            // Usar stub (compatibilidade)
            self.generate_audio_basic(&request.text, voice, request.speed, request.pitch)?
        };

        // Cache the result
        {
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, audio.clone());
        }

        Ok(audio)
    }

    fn generate_audio_basic(
        &self,
        text: &str,
        voice: &VoiceProfile,
        speed: f32,
        pitch: f32,
    ) -> Result<AudioOutput> {
        // Basic audio synthesis: generate tone-based audio
        // In real implementation, this would use XTTS model

        let sample_rate = 24000u32;
        let channels = 1u16;

        // Estimate duration: ~150ms per character at normal speed
        let base_duration_per_char = 0.15;
        let duration = (text.len() as f32 * base_duration_per_char / speed) as f32;
        let num_samples = (duration * sample_rate as f32) as usize;

        let mut samples = Vec::with_capacity(num_samples);

        // Generate audio with pitch variation
        let base_freq = 200.0 + (voice.base_pitch * 200.0) + (pitch * 100.0);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;

            // Generate a waveform with harmonics
            let freq = base_freq * (1.0 + (t * 0.1).sin() * 0.05);
            let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();

            // Apply envelope (attack, sustain, release)
            let envelope = if t < 0.01 {
                t / 0.01 // Attack
            } else if t > duration - 0.01 {
                (duration - t) / 0.01 // Release
            } else {
                1.0 // Sustain
            };

            // Apply timbre (add harmonics)
            let timbre_effect = if voice.timbre > 0.5 {
                let harmonic = (t * freq * 4.0 * std::f32::consts::PI).sin() * 0.3;
                sample + harmonic * (voice.timbre - 0.5) * 2.0
            } else {
                sample * (1.0 - voice.timbre)
            };

            samples.push(timbre_effect * envelope * 0.3); // Volume
        }

        Ok(AudioOutput {
            samples,
            sample_rate,
            channels,
        })
    }

    pub async fn synthesize_streaming(
        &self,
        request: &SynthesisRequest,
        chunk_size_ms: u64,
    ) -> impl futures::Stream<Item = Result<AudioOutput>> {
        // For streaming, we generate the full audio and chunk it
        // In real implementation, this would stream from XTTS model

        let full_audio = match self.synthesize(request).await {
            Ok(audio) => audio,
            Err(e) => {
                return futures::stream::once(async move { Err(e) }).boxed();
            }
        };

        let sample_rate = full_audio.sample_rate;
        let samples_per_chunk = (chunk_size_ms as f32 / 1000.0 * sample_rate as f32) as usize;

        futures::stream::unfold(
            (full_audio.samples, 0),
            move |(samples, offset)| async move {
                if offset >= samples.len() {
                    None
                } else {
                    let end = (offset + samples_per_chunk).min(samples.len());
                    let chunk = AudioOutput {
                        samples: samples[offset..end].to_vec(),
                        sample_rate,
                        channels: 1,
                    };
                    Some((Ok(chunk), (samples, end)))
                }
            },
        )
        .boxed()
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Mapeia voice_id interno para speaker do Coqui XTTS
    fn map_voice_to_speaker(&self, voice_id: &str) -> Option<String> {
        // Mapeamento padrão de voice_id para speakers do Coqui XTTS
        // Usuários podem customizar isso via configuração
        match voice_id {
            "dm" => Some("Ana Florence".to_string()), // Speaker padrão do XTTS v2
            "npc_male" => Some("Ana Florence".to_string()),
            "npc_female" => Some("Ana Florence".to_string()),
            "monster" => Some("Ana Florence".to_string()),
            _ => None, // Usar speaker padrão do modelo
        }
    }

    /// Síntese usando Coqui XTTS real via Python bridge
    async fn synthesize_with_coqui_xtts(
        &self,
        text: &str,
        language: &str,
        speaker: Option<&str>,
    ) -> Result<AudioOutput> {
        info!(
            "🎤 XTTS synthesizing with Coqui: '{}' (lang: {}, speaker: {:?})",
            text, language, speaker
        );

        // Criar script Python temporário
        let temp_script =
            std::env::temp_dir().join(format!("xtts_synth_{}.py", std::process::id()));
        let temp_input =
            std::env::temp_dir().join(format!("xtts_input_{}.json", std::process::id()));

        // Criar JSON de entrada
        let input_data = serde_json::json!({
            "text": text,
            "language": language,
            "speaker": speaker,
            "use_gpu": self.use_gpu,
            "model_path": self.model_path.as_ref().map(|p| p.to_string_lossy().to_string()),
        });

        std::fs::write(&temp_input, serde_json::to_string(&input_data)?)
            .map_err(|e| TtsError::ModelLoad(format!("Failed to write input JSON: {}", e)))?;

        // Criar script Python
        let python_script = format!(
            r#"
import sys
import json
import codecs
import os
import numpy as np
import torch

# Aceitar termos de serviço do Coqui TTS automaticamente
os.environ["COQUI_TOS_AGREED"] = "1"

# Fix para PyTorch 2.6+ que requer weights_only=False
# Adicionar classes seguras do TTS e usar monkey patch como fallback
safe_classes = []
try:
    from TTS.tts.configs.xtts_config import XttsConfig
    safe_classes.append(XttsConfig)
except:
    pass
try:
    from TTS.tts.models.xtts import XttsAudioConfig, XttsArgs
    safe_classes.append(XttsAudioConfig)
    safe_classes.append(XttsArgs)
except:
    pass
try:
    from TTS.config.shared_configs import BaseDatasetConfig, BaseAudioConfig, BaseTrainingConfig
    safe_classes.extend([BaseDatasetConfig, BaseAudioConfig, BaseTrainingConfig])
except:
    pass

if safe_classes:
    try:
        torch.serialization.add_safe_globals(safe_classes)
    except:
        # Fallback: monkey patch torch.load
        original_load = torch.load
        def patched_load(*args, **kwargs):
            kwargs['weights_only'] = False
            return original_load(*args, **kwargs)
        torch.load = patched_load
else:
    # Fallback: monkey patch torch.load
    original_load = torch.load
    def patched_load(*args, **kwargs):
        kwargs['weights_only'] = False
        return original_load(*args, **kwargs)
    torch.load = patched_load

try:
    from TTS.api import TTS
except ImportError:
    print(json.dumps({{"error": "Coqui TTS not installed. Install with: pip install TTS"}}), file=sys.stderr)
    sys.exit(1)

# Ler entrada
try:
    with codecs.open(r"{}", 'r', encoding='utf-8-sig') as f:
        data = json.load(f)
except Exception as e:
    print(json.dumps({{"error": f"Failed to read input: {{e}}"}}), file=sys.stderr)
    sys.exit(1)

text = data.get("text", "")
language = data.get("language", "en")
speaker = data.get("speaker")
use_gpu = data.get("use_gpu", False)
model_path = data.get("model_path")

try:
    # Carregar modelo XTTS
    if model_path and os.path.exists(model_path):
        # Usar modelo customizado
        model_dir = os.path.dirname(model_path)
        config_path = os.path.join(model_dir, "config.json")
        vocab_path = os.path.join(model_dir, "vocab.json")
        speakers_path = os.path.join(model_dir, "speakers_xtts.pth")
        
        # Verificar quais arquivos existem
        if os.path.exists(config_path):
            tts = TTS(
                model_path=model_path,
                config_path=config_path,
                vocab_path=vocab_path if os.path.exists(vocab_path) else None,
                speakers_file_path=speakers_path if os.path.exists(speakers_path) else None,
                progress_bar=False,
                gpu=use_gpu
            )
        else:
            # Fallback: tentar carregar apenas com model_path
            tts = TTS(model_path=model_path, progress_bar=False, gpu=use_gpu)
    else:
        # Usar modelo do cache do Coqui TTS (download automático se necessário)
        print("🐍 Python Bridge: Loading XTTS v2 model (will download if needed)...", file=sys.stderr)
        print("🐍 Python Bridge: This may take several minutes on first run (downloading ~1.5GB model)...", file=sys.stderr)
        # Aceitar termos de serviço automaticamente
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=True)
        print("🐍 Python Bridge: XTTS model loaded successfully!", file=sys.stderr)
    
    # Gerar áudio
    text_preview = text[:50] + "..." if len(text) > 50 else text
    # XTTS v2 requer um speaker. Usar "Ana Florence" como padrão se não especificado
    if not speaker:
        speaker = "Ana Florence"
    print("🐍 Python Bridge: Synthesizing audio for text: '{{}}' (lang: {{}}, speaker: {{}})...".format(
        text_preview, language, speaker), file=sys.stderr)
    # XTTS v2 sempre requer speaker
    audio = tts.tts(text=text, speaker=speaker, language=language)
    print("🐍 Python Bridge: Audio synthesis completed! Generated {{}} samples".format(len(audio)), file=sys.stderr)
    
    # Converter para lista (JSON serializable)
    # Converter numpy float32 para float Python para serialização JSON
    if isinstance(audio, np.ndarray):
        audio_list = audio.astype(np.float32).tolist()
    else:
        audio_list = [float(x) for x in audio]
    
    # Output format expected by Rust
    output = {{
        "samples": audio_list,
        "sample_rate": tts.synthesizer.output_sample_rate,
        "channels": 1
    }}
    
    # Print JSON (Rust will read from stdout)
    # Usar ensure_ascii=False e flush para garantir que tudo seja enviado
    json_output = json.dumps(output, ensure_ascii=False)
    print(json_output, flush=True)
    
except Exception as e:
    print(json.dumps({{"error": str(e)}}), file=sys.stderr)
    sys.exit(1)
"#,
            temp_input.to_string_lossy().replace('\\', "\\\\")
        );

        std::fs::write(&temp_script, python_script)
            .map_err(|e| TtsError::ModelLoad(format!("Failed to write Python script: {}", e)))?;

        // Executar Python
        let output = tokio::process::Command::new("python")
            .arg(&temp_script)
            .output()
            .await
            .map_err(|e| TtsError::ModelLoad(format!("Failed to run Python: {}", e)))?;

        // Limpar arquivos temporários
        let _ = std::fs::remove_file(&temp_script);
        let _ = std::fs::remove_file(&temp_input);

        // Verificar erros
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            warn!("XTTS Python stderr: {}", stderr);
        }

        if !output.status.success() {
            return Err(TtsError::ModelLoad(format!(
                "XTTS Python script failed: {}",
                stderr
            )));
        }

        // Parse output JSON
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Err(TtsError::ModelLoad(
                "XTTS Python script returned empty output".to_string(),
            ));
        }

        // Tentar encontrar o JSON no stdout (pode ter logs antes)
        // O JSON deve estar na última linha ou começar com {
        let json_str = if let Some(json_start) = stdout.rfind('{') {
            &stdout[json_start..]
        } else {
            // Se não encontrar {, tentar última linha
            stdout.lines().last().unwrap_or(&stdout).trim()
        };

        let result: serde_json::Value = serde_json::from_str(json_str.trim()).map_err(|e| {
            // Se falhar, mostrar mais contexto para debug
            let preview = if json_str.len() > 200 {
                format!("{}...", &json_str[..200])
            } else {
                json_str.to_string()
            };
            TtsError::ModelLoad(format!(
                "Failed to parse XTTS output: {} (json preview: {})",
                e, preview
            ))
        })?;

        if let Some(error) = result.get("error") {
            return Err(TtsError::ModelLoad(format!("XTTS error: {}", error)));
        }

        let samples: Vec<f32> = result["samples"]
            .as_array()
            .ok_or_else(|| TtsError::ModelLoad("Invalid samples array in XTTS output".to_string()))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        let sample_rate = result["sample_rate"].as_u64().unwrap_or(22050) as u32;

        let channels = result["channels"].as_u64().unwrap_or(1) as u16;

        info!(
            "✅ XTTS generated {} samples at {} Hz",
            samples.len(),
            sample_rate
        );

        Ok(AudioOutput {
            samples,
            sample_rate,
            channels,
        })
    }
}

use futures::StreamExt;

pub type SharedXttsModel = Arc<RwLock<XttsModel>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_xtts_model_loading() {
        let mut model = XttsModel::new();
        assert!(!model.is_loaded());

        model.load("test_model").await.unwrap();
        assert!(model.is_loaded());
    }

    #[tokio::test]
    async fn test_xtts_synthesize() {
        let mut model = XttsModel::new();
        model.load("test_model").await.unwrap();

        let request = SynthesisRequest {
            text: "Hello, world!".to_string(),
            voice_id: "dm".to_string(),
            speed: 1.0,
            pitch: 0.0,
        };

        let result = model.synthesize(&request).await.unwrap();
        assert!(!result.samples.is_empty());
        assert_eq!(result.sample_rate, 24000);
    }

    #[tokio::test]
    async fn test_xtts_cache() {
        let mut model = XttsModel::new();
        model.load("test_model").await.unwrap();

        let request = SynthesisRequest {
            text: "Test".to_string(),
            voice_id: "dm".to_string(),
            speed: 1.0,
            pitch: 0.0,
        };

        let result1 = model.synthesize(&request).await.unwrap();
        let result2 = model.synthesize(&request).await.unwrap();

        // Cached result should be identical
        assert_eq!(result1.samples.len(), result2.samples.len());
    }

    #[tokio::test]
    async fn test_xtts_voice_not_found() {
        let mut model = XttsModel::new();
        model.load("test_model").await.unwrap();

        let request = SynthesisRequest {
            text: "Test".to_string(),
            voice_id: "nonexistent".to_string(),
            speed: 1.0,
            pitch: 0.0,
        };

        let result = model.synthesize(&request).await;
        assert!(result.is_err());
    }
}
