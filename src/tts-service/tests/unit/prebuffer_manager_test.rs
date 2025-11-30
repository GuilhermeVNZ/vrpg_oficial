//! Unit tests for pre-buffer manager

use tts_service::audio_buffer::{AudioBuffer, AudioChunk};
use tts_service::gpu_config::{GpuConfig, PerformanceProfile};
use tts_service::gpu_detector::{GpuCapability, GpuTier};
use tts_service::prebuffer_manager::{PreBufferManager, PreBufferState};

#[test]
fn test_should_start_playback() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let manager = PreBufferManager::new(gpu_config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Empty buffer - should not start
    assert!(!manager.should_start_playback(&buffer).unwrap());

    // Add enough audio
    let chunk = AudioChunk {
        samples: vec![0.1; (24000.0 * 1.5) as usize],
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Should start now
    assert!(manager.should_start_playback(&buffer).unwrap());
}

#[test]
fn test_should_pause_playback() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config.clone());
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Set state to Playing
    manager.set_state(PreBufferState::Playing);

    // Buffer too low - should pause
    let chunk = AudioChunk {
        samples: vec![0.1; 2400], // 0.1 seconds (below 50% of 1.2s threshold)
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    assert!(manager.should_pause_playback(&buffer).unwrap());
}

#[test]
fn test_update_state_filling_to_ready() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Initially Filling
    assert_eq!(manager.state(), PreBufferState::Filling);

    // Add enough audio
    let chunk = AudioChunk {
        samples: vec![0.1; (24000.0 * 1.5) as usize],
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Update state
    manager.update_state(&buffer).unwrap();
    assert_eq!(manager.state(), PreBufferState::Ready);
}

#[test]
fn test_update_state_playing_to_paused() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Set to Playing
    manager.set_state(PreBufferState::Playing);

    // Buffer too low
    let chunk = AudioChunk {
        samples: vec![0.1; 2400], // 0.1 seconds
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Update state
    manager.update_state(&buffer).unwrap();
    assert_eq!(manager.state(), PreBufferState::Paused);
}

#[test]
fn test_update_state_paused_to_ready() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Set to Paused
    manager.set_state(PreBufferState::Paused);

    // Add enough audio
    let chunk = AudioChunk {
        samples: vec![0.1; (24000.0 * 1.5) as usize],
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Update state
    manager.update_state(&buffer).unwrap();
    assert_eq!(manager.state(), PreBufferState::Ready);
}

#[test]
fn test_update_state_ready_to_filling() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Set to Ready
    manager.set_state(PreBufferState::Ready);

    // Buffer too low
    let chunk = AudioChunk {
        samples: vec![0.1; 2400], // 0.1 seconds
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Update state
    manager.update_state(&buffer).unwrap();
    assert_eq!(manager.state(), PreBufferState::Filling);
}

#[test]
fn test_update_state_playing_stays_playing() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config);
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    // Set to Playing
    manager.set_state(PreBufferState::Playing);

    // Buffer sufficient
    let chunk = AudioChunk {
        samples: vec![0.1; (24000.0 * 1.5) as usize],
        sample_rate: 24000,
        channels: 1,
    };
    buffer.push(chunk).unwrap();

    // Update state
    manager.update_state(&buffer).unwrap();
    assert_eq!(manager.state(), PreBufferState::Playing);
}

#[test]
fn test_state_getter_setter() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };
    let gpu_config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    let mut manager = PreBufferManager::new(gpu_config);

    assert_eq!(manager.state(), PreBufferState::Filling);
    manager.set_state(PreBufferState::Playing);
    assert_eq!(manager.state(), PreBufferState::Playing);
    manager.set_state(PreBufferState::Ready);
    assert_eq!(manager.state(), PreBufferState::Ready);
}

