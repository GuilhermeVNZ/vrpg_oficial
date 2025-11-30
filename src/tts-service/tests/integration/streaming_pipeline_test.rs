//! Integration tests for streaming pipeline

use tts_service::audio_buffer::AudioBuffer;
use tts_service::gpu_config::{GpuConfig, PerformanceProfile};
use tts_service::gpu_detector::GpuDetector;
use tts_service::prebuffer_manager::PreBufferManager;
use tts_service::semantic_chunker::SemanticChunker;

#[tokio::test]
async fn test_gpu_config_from_capability() {
    let capability = GpuDetector::detect().unwrap();
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();

    // Config should be valid
    assert!(config.max_parallel_streams <= 3);
    assert!(config.utilization_target >= 0.3 && config.utilization_target <= 0.95);
    assert!(config.prebuffer_seconds >= 0.5 && config.prebuffer_seconds <= 3.0);
}

#[tokio::test]
async fn test_prebuffer_manager() {
    let capability = GpuDetector::detect().unwrap();
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let manager = PreBufferManager::new(config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Initially should not start playback
    assert!(!manager.should_start_playback(&buffer).unwrap());

    // Add enough audio
    let chunk = tts_service::audio_buffer::AudioChunk {
        samples: vec![0.1; (24000.0 * 1.5) as usize],
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Should start now
    assert!(manager.should_start_playback(&buffer).unwrap());
}

#[tokio::test]
async fn test_semantic_chunker_integration() {
    let chunker = SemanticChunker::new(Default::default());
    let long_text = "This is a very long narrative text that should be split into multiple semantic chunks. ".repeat(20);
    
    let chunks = chunker.chunk(&long_text).unwrap();
    
    // Should create multiple chunks
    assert!(chunks.len() > 1);
    
    // Each chunk should meet requirements
    for chunk in &chunks {
        assert!(chunk.char_count >= 180);
        assert!(chunk.char_count <= 320);
        assert!(chunk.estimated_duration >= 2.4);
        assert!(chunk.estimated_duration <= 8.0);
    }
}

#[tokio::test]
async fn test_buffer_prebuffer_integration() {
    let capability = GpuDetector::detect().unwrap();
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(config.clone());
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Add chunks gradually
    for i in 0..5 {
        let chunk = tts_service::audio_buffer::AudioChunk {
            samples: vec![0.1; 2400], // 0.1 seconds each
            sample_rate: 24000,
            channels: 1,
        };
        buffer.push(chunk).unwrap();

        // Update state
        manager.update_state(&buffer).unwrap();

        if i < 2 {
            // First chunks - should be filling
            assert!(manager.state() == tts_service::prebuffer_manager::PreBufferState::Filling);
        } else {
            // After threshold - should be ready
            assert!(manager.state() == tts_service::prebuffer_manager::PreBufferState::Ready);
        }
    }
}
