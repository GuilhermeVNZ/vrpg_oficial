//! Unit tests for GPU configuration

use tts_service::gpu_config::{GpuConfig, PerformanceProfile};
use tts_service::gpu_detector::{GpuCapability, GpuTier};

#[test]
fn test_high_end_config() {
    let capability = GpuCapability {
        gpu_name: "RTX 4090".to_string(),
        vram_total_gb: 24.0,
        compute_capability: Some((8, 9)),
        tier: GpuTier::HighEnd,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.max_parallel_streams, 2);
    assert_eq!(config.vram_limit_mb, 0);
    assert!((config.utilization_target - 0.90).abs() < 0.01);
    assert!((config.prebuffer_seconds - 2.5).abs() < 0.01);
    assert!(!config.yield_between_chunks);
}

#[test]
fn test_modest_config() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.max_parallel_streams, 1);
    assert_eq!(config.vram_limit_mb, 3072);
    assert!((config.utilization_target - 0.50).abs() < 0.01);
    assert!(config.yield_between_chunks);
}

#[test]
fn test_mid_range_config() {
    let capability = GpuCapability {
        gpu_name: "RTX 3080".to_string(),
        vram_total_gb: 10.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::MidRange,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.max_parallel_streams, 1);
    assert_eq!(config.vram_limit_mb, 6144);
    assert!((config.utilization_target - 0.70).abs() < 0.01);
}

#[test]
fn test_low_end_config() {
    let capability = GpuCapability {
        gpu_name: "GTX 1050".to_string(),
        vram_total_gb: 2.0,
        compute_capability: Some((6, 1)),
        tier: GpuTier::LowEnd,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.max_parallel_streams, 0);
    assert_eq!(config.vram_limit_mb, 2048);
    assert!((config.utilization_target - 0.40).abs() < 0.01);
}

#[test]
fn test_cpu_only_config() {
    let capability = GpuCapability {
        gpu_name: "CPU".to_string(),
        vram_total_gb: 0.0,
        compute_capability: None,
        tier: GpuTier::CpuOnly,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.max_parallel_streams, 0);
    assert_eq!(config.vram_limit_mb, 0);
    assert!(config.cpu_fallback_enabled);
}

#[test]
fn test_environment_override() {
    std::env::set_var("VRPG_XTTS_GPU_STREAMS", "3");
    std::env::set_var("VRPG_XTTS_GPU_VRAM_LIMIT_MB", "4096");
    std::env::set_var("VRPG_XTTS_GPU_UTILIZATION_TARGET", "0.85");
    std::env::set_var("VRPG_XTTS_PREBUFFER_SECONDS", "2.0");

    let capability = GpuCapability {
        gpu_name: "RTX 4090".to_string(),
        vram_total_gb: 24.0,
        compute_capability: Some((8, 9)),
        tier: GpuTier::HighEnd,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.max_parallel_streams, 3);
    assert_eq!(config.vram_limit_mb, 4096);
    assert!((config.utilization_target - 0.85).abs() < 0.01);
    assert!((config.prebuffer_seconds - 2.0).abs() < 0.01);

    // Cleanup
    std::env::remove_var("VRPG_XTTS_GPU_STREAMS");
    std::env::remove_var("VRPG_XTTS_GPU_VRAM_LIMIT_MB");
    std::env::remove_var("VRPG_XTTS_GPU_UTILIZATION_TARGET");
    std::env::remove_var("VRPG_XTTS_PREBUFFER_SECONDS");
}

#[test]
fn test_performance_profile_high_performance() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::HighPerformance).unwrap();
    // Should use HighEnd config even though GPU is Modest
    assert_eq!(config.max_parallel_streams, 2);
}

#[test]
fn test_performance_profile_balanced() {
    let capability = GpuCapability {
        gpu_name: "RTX 3050".to_string(),
        vram_total_gb: 4.0,
        compute_capability: Some((8, 6)),
        tier: GpuTier::Modest,
    };

    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Balanced).unwrap();
    // Should use MidRange config
    assert_eq!(config.max_parallel_streams, 1);
    assert_eq!(config.vram_limit_mb, 6144);
}

#[test]
fn test_apply_overrides_vram_limit() {
    std::env::set_var("VRPG_XTTS_GPU_VRAM_LIMIT_MB", "8192");
    let capability = GpuCapability {
        gpu_name: "RTX 4090".to_string(),
        vram_total_gb: 24.0,
        compute_capability: Some((8, 9)),
        tier: GpuTier::HighEnd,
    };
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert_eq!(config.vram_limit_mb, 8192);
    std::env::remove_var("VRPG_XTTS_GPU_VRAM_LIMIT_MB");
}

#[test]
fn test_apply_overrides_utilization_target() {
    std::env::set_var("VRPG_XTTS_GPU_UTILIZATION_TARGET", "0.75");
    let capability = GpuCapability {
        gpu_name: "RTX 4090".to_string(),
        vram_total_gb: 24.0,
        compute_capability: Some((8, 9)),
        tier: GpuTier::HighEnd,
    };
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert!((config.utilization_target - 0.75).abs() < 0.01);
    std::env::remove_var("VRPG_XTTS_GPU_UTILIZATION_TARGET");
}

#[test]
fn test_apply_overrides_prebuffer_seconds() {
    std::env::set_var("VRPG_XTTS_PREBUFFER_SECONDS", "1.5");
    let capability = GpuCapability {
        gpu_name: "RTX 4090".to_string(),
        vram_total_gb: 24.0,
        compute_capability: Some((8, 9)),
        tier: GpuTier::HighEnd,
    };
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert!((config.prebuffer_seconds - 1.5).abs() < 0.01);
    std::env::remove_var("VRPG_XTTS_PREBUFFER_SECONDS");
}

#[test]
fn test_apply_overrides_clamp_values() {
    // Test that values are clamped correctly
    std::env::set_var("VRPG_XTTS_GPU_UTILIZATION_TARGET", "1.5"); // Should clamp to 0.95
    std::env::set_var("VRPG_XTTS_PREBUFFER_SECONDS", "5.0"); // Should clamp to 3.0
    let capability = GpuCapability {
        gpu_name: "RTX 4090".to_string(),
        vram_total_gb: 24.0,
        compute_capability: Some((8, 9)),
        tier: GpuTier::HighEnd,
    };
    let config = GpuConfig::from_capability(&capability, PerformanceProfile::Auto).unwrap();
    assert!((config.utilization_target - 0.95).abs() < 0.01);
    assert!((config.prebuffer_seconds - 3.0).abs() < 0.01);
    std::env::remove_var("VRPG_XTTS_GPU_UTILIZATION_TARGET");
    std::env::remove_var("VRPG_XTTS_PREBUFFER_SECONDS");
}

