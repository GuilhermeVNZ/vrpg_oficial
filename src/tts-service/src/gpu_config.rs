//! Adaptive GPU Configuration
//!
//! Provides tier-based GPU configuration for XTTS streaming

use crate::error::Result;
use crate::gpu_detector::{GpuCapability, GpuTier};

/// Performance profile selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceProfile {
    /// High performance: Maximum GPU usage
    HighPerformance,
    /// Balanced: Moderate GPU usage
    Balanced,
    /// Modest: Conservative GPU usage
    Modest,
    /// Auto: Automatically select based on GPU tier
    Auto,
}

impl Default for PerformanceProfile {
    fn default() -> Self {
        PerformanceProfile::Auto
    }
}

/// GPU configuration for XTTS streaming
#[derive(Debug, Clone)]
pub struct GpuConfig {
    /// Maximum parallel CUDA streams (0-3)
    pub max_parallel_streams: usize,
    /// VRAM limit in MB (0 = unlimited)
    pub vram_limit_mb: usize,
    /// Target GPU utilization (0.3-0.95)
    pub utilization_target: f32,
    /// Pre-buffer size in seconds (0.5-3.0)
    pub prebuffer_seconds: f32,
    /// Yield GPU between chunks (for modest hardware)
    pub yield_between_chunks: bool,
    /// Allow CPU fallback for some operations
    pub cpu_fallback_enabled: bool,
}

impl GpuConfig {
    /// Create configuration from GPU capability and profile
    pub fn from_capability(
        capability: &GpuCapability,
        profile: PerformanceProfile,
    ) -> Result<Self> {
        let tier = capability.tier;
        let _profile = match profile {
            PerformanceProfile::Auto => tier,
            PerformanceProfile::HighPerformance => GpuTier::HighEnd,
            PerformanceProfile::Balanced => GpuTier::MidRange,
            PerformanceProfile::Modest => GpuTier::Modest,
        };

        let config = match tier {
            GpuTier::HighEnd => GpuConfig {
                max_parallel_streams: 2,
                vram_limit_mb: 0, // Unlimited
                utilization_target: 0.90,
                prebuffer_seconds: 2.5,
                yield_between_chunks: false,
                cpu_fallback_enabled: false,
            },
            GpuTier::MidRange => GpuConfig {
                max_parallel_streams: 1,
                vram_limit_mb: 6144, // 6GB
                utilization_target: 0.70,
                prebuffer_seconds: 1.8,
                yield_between_chunks: false,
                cpu_fallback_enabled: true,
            },
            GpuTier::Modest => GpuConfig {
                max_parallel_streams: 1,
                vram_limit_mb: 3072, // 3GB
                utilization_target: 0.50,
                prebuffer_seconds: 1.2,
                yield_between_chunks: true,
                cpu_fallback_enabled: true,
            },
            GpuTier::LowEnd => GpuConfig {
                max_parallel_streams: 0,
                vram_limit_mb: 2048, // 2GB
                utilization_target: 0.40,
                prebuffer_seconds: 0.8,
                yield_between_chunks: true,
                cpu_fallback_enabled: true,
            },
            GpuTier::CpuOnly => GpuConfig {
                max_parallel_streams: 0,
                vram_limit_mb: 0,
                utilization_target: 0.0,
                prebuffer_seconds: 0.5,
                yield_between_chunks: true,
                cpu_fallback_enabled: true,
            },
        };

        // Apply environment variable overrides
        Ok(config.apply_overrides())
    }

    /// Apply environment variable overrides
    fn apply_overrides(mut self) -> Self {
        if let Ok(streams) = std::env::var("VRPG_XTTS_GPU_STREAMS") {
            if let Ok(val) = streams.parse::<usize>() {
                self.max_parallel_streams = val.min(3);
            }
        }

        if let Ok(vram) = std::env::var("VRPG_XTTS_GPU_VRAM_LIMIT_MB") {
            if let Ok(val) = vram.parse::<usize>() {
                self.vram_limit_mb = val;
            }
        }

        if let Ok(util) = std::env::var("VRPG_XTTS_GPU_UTILIZATION_TARGET") {
            if let Ok(val) = util.parse::<f32>() {
                self.utilization_target = val.clamp(0.3, 0.95);
            }
        }

        if let Ok(prebuf) = std::env::var("VRPG_XTTS_PREBUFFER_SECONDS") {
            if let Ok(val) = prebuf.parse::<f32>() {
                self.prebuffer_seconds = val.clamp(0.5, 3.0);
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(config.yield_between_chunks);
    }
}

