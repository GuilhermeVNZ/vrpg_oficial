//! Unit tests for GPU detector

use tts_service::gpu_detector::{GpuDetector, GpuTier};

#[test]
fn test_tier_classification_high_end() {
    let tier = tts_service::gpu_detector::GpuDetector::classify_tier("RTX 4090", 24.0, Some((8, 9)));
    assert_eq!(tier, GpuTier::HighEnd);
}

#[test]
fn test_tier_classification_mid_range() {
    let tier = tts_service::gpu_detector::GpuDetector::classify_tier("RTX 3080", 10.0, Some((8, 6)));
    assert_eq!(tier, GpuTier::MidRange);
}

#[test]
fn test_tier_classification_modest() {
    let tier = tts_service::gpu_detector::GpuDetector::classify_tier("RTX 3050", 4.0, Some((8, 6)));
    assert_eq!(tier, GpuTier::Modest);
}

#[test]
fn test_tier_classification_low_end() {
    let tier = tts_service::gpu_detector::GpuDetector::classify_tier("GTX 1050", 2.0, Some((6, 1)));
    assert_eq!(tier, GpuTier::LowEnd);
}

#[test]
fn test_tier_classification_cpu_only() {
    let tier = tts_service::gpu_detector::GpuDetector::classify_tier("CPU", 0.0, None);
    assert_eq!(tier, GpuTier::CpuOnly);
}

#[test]
fn test_gpu_detection() {
    // Test with environment variable override
    std::env::set_var("VRPG_GPU_NAME", "RTX 4090");
    std::env::set_var("VRPG_GPU_VRAM_GB", "24");
    
    let capability = GpuDetector::detect().unwrap();
    assert_eq!(capability.gpu_name, "RTX 4090");
    assert_eq!(capability.vram_total_gb, 24.0);
    
    // Cleanup
    std::env::remove_var("VRPG_GPU_NAME");
    std::env::remove_var("VRPG_GPU_VRAM_GB");
}

#[test]
fn test_gpu_detection_compute_capability() {
    std::env::set_var("VRPG_GPU_NAME", "RTX 4090");
    std::env::set_var("VRPG_GPU_VRAM_GB", "24");
    std::env::set_var("VRPG_GPU_COMPUTE_CAPABILITY", "8.9");
    
    let capability = GpuDetector::detect().unwrap();
    assert_eq!(capability.compute_capability, Some((8, 9)));
    
    // Cleanup
    std::env::remove_var("VRPG_GPU_NAME");
    std::env::remove_var("VRPG_GPU_VRAM_GB");
    std::env::remove_var("VRPG_GPU_COMPUTE_CAPABILITY");
}

#[test]
fn test_gpu_detection_cpu_only() {
    // Without GPU environment variables, should detect CPU-only
    std::env::remove_var("VRPG_USE_GPU");
    std::env::remove_var("CUDA_VISIBLE_DEVICES");
    
    let capability = GpuDetector::detect().unwrap();
    assert_eq!(capability.tier, GpuTier::CpuOnly);
}
