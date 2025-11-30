//! GPU Capability Detector
//!
//! Detects GPU capabilities and classifies hardware into performance tiers
//! for adaptive XTTS streaming configuration.

use crate::error::{Result, TtsError};
use std::fmt;

/// Check if CUDA is available
fn is_cuda_available() -> bool {
    // In production, check actual CUDA availability
    // For now, check environment variable
    std::env::var("CUDA_VISIBLE_DEVICES").is_ok()
        || std::env::var("VRPG_USE_GPU").is_ok()
}

/// GPU performance tier classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuTier {
    /// High-End: RTX 4090/5090, A100, H100 (8GB+ VRAM, high compute)
    HighEnd,
    /// Mid-Range: RTX 3060-3080, RTX 4060-4070 (6-8GB VRAM)
    MidRange,
    /// Modest: RTX 3050, GTX 1660 (4-6GB VRAM)
    Modest,
    /// Low-End: < 4GB VRAM or integrated graphics
    LowEnd,
    /// CPU-only: No GPU available
    CpuOnly,
}

impl fmt::Display for GpuTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuTier::HighEnd => write!(f, "High-End"),
            GpuTier::MidRange => write!(f, "Mid-Range"),
            GpuTier::Modest => write!(f, "Modest"),
            GpuTier::LowEnd => write!(f, "Low-End"),
            GpuTier::CpuOnly => write!(f, "CPU-Only"),
        }
    }
}

/// GPU capability information
#[derive(Debug, Clone)]
pub struct GpuCapability {
    pub gpu_name: String,
    pub vram_total_gb: f32,
    pub compute_capability: Option<(u32, u32)>,
    pub tier: GpuTier,
}

/// GPU Capability Detector
pub struct GpuDetector;

impl GpuDetector {
    /// Detect GPU capabilities and classify tier
    pub fn detect() -> Result<GpuCapability> {
        // Check if CUDA is available
        if !is_cuda_available() {
            return Ok(GpuCapability {
                gpu_name: "CPU".to_string(),
                vram_total_gb: 0.0,
                compute_capability: None,
                tier: GpuTier::CpuOnly,
            });
        }

        // Try to get GPU info via CUDA
        // For now, we'll use a simplified detection
        // In production, use nvml or similar
        let gpu_name = Self::detect_gpu_name()?;
        let vram_gb = Self::detect_vram_gb()?;
        let compute_cap = Self::detect_compute_capability()?;
        let tier = Self::classify_tier(&gpu_name, vram_gb, compute_cap);

        Ok(GpuCapability {
            gpu_name,
            vram_total_gb: vram_gb,
            compute_capability: compute_cap,
            tier,
        })
    }

    /// Detect GPU name
    fn detect_gpu_name() -> Result<String> {
        // Try environment variable first (for testing/override)
        if let Ok(name) = std::env::var("VRPG_GPU_NAME") {
            return Ok(name);
        }

        // In production, query CUDA device name
        // For now, return a placeholder
        Ok("Unknown GPU".to_string())
    }

    /// Detect VRAM in GB
    fn detect_vram_gb() -> Result<f32> {
        // Try environment variable first (for testing/override)
        if let Ok(vram) = std::env::var("VRPG_GPU_VRAM_GB") {
            return vram
                .parse::<f32>()
                .map_err(|_| TtsError::ModelLoad("Invalid VRAM value".to_string()));
        }

        // In production, query CUDA device memory
        // For now, return a default
        Ok(8.0)
    }

    /// Detect compute capability
    fn detect_compute_capability() -> Result<Option<(u32, u32)>> {
        // Try environment variable first (for testing/override)
        if let Ok(cc) = std::env::var("VRPG_GPU_COMPUTE_CAPABILITY") {
            let parts: Vec<&str> = cc.split('.').collect();
            if parts.len() == 2 {
                if let (Ok(major), Ok(minor)) = (parts[0].parse(), parts[1].parse()) {
                    return Ok(Some((major, minor)));
                }
            }
        }

        // In production, query CUDA device compute capability
        // For now, return None
        Ok(None)
    }

    /// Classify GPU into performance tier
    fn classify_tier(gpu_name: &str, vram_gb: f32, compute_cap: Option<(u32, u32)>) -> GpuTier {
        let name_lower = gpu_name.to_lowercase();

        // High-End: RTX 4090/5090, A100, H100
        if name_lower.contains("rtx 4090")
            || name_lower.contains("rtx 5090")
            || name_lower.contains("a100")
            || name_lower.contains("h100")
            || (vram_gb >= 16.0 && compute_cap.map_or(false, |(m, _)| m >= 8))
        {
            return GpuTier::HighEnd;
        }

        // Mid-Range: RTX 3060-3080, RTX 4060-4070
        if name_lower.contains("rtx 3080")
            || name_lower.contains("rtx 4070")
            || name_lower.contains("rtx 4060")
            || (vram_gb >= 6.0 && vram_gb < 16.0)
        {
            return GpuTier::MidRange;
        }

        // Modest: RTX 3050, GTX 1660
        if name_lower.contains("rtx 3050")
            || name_lower.contains("gtx 1660")
            || (vram_gb >= 4.0 && vram_gb < 6.0)
        {
            return GpuTier::Modest;
        }

        // Low-End: < 4GB VRAM
        if vram_gb > 0.0 && vram_gb < 4.0 {
            return GpuTier::LowEnd;
        }

        // Default to CPU-only if no GPU detected
        GpuTier::CpuOnly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_classification_high_end() {
        let tier = GpuDetector::classify_tier("RTX 4090", 24.0, Some((8, 9)));
        assert_eq!(tier, GpuTier::HighEnd);
    }

    #[test]
    fn test_tier_classification_mid_range() {
        let tier = GpuDetector::classify_tier("RTX 3080", 10.0, Some((8, 6)));
        assert_eq!(tier, GpuTier::MidRange);
    }

    #[test]
    fn test_tier_classification_modest() {
        let tier = GpuDetector::classify_tier("RTX 3050", 4.0, Some((8, 6)));
        assert_eq!(tier, GpuTier::Modest);
    }

    #[test]
    fn test_tier_classification_low_end() {
        let tier = GpuDetector::classify_tier("GTX 1050", 2.0, Some((6, 1)));
        assert_eq!(tier, GpuTier::LowEnd);
    }

    #[test]
    fn test_tier_classification_cpu_only() {
        let tier = GpuDetector::classify_tier("CPU", 0.0, None);
        assert_eq!(tier, GpuTier::CpuOnly);
    }
}

