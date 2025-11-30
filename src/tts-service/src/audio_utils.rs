//! Audio Utilities - Volume normalization and audio processing
//!
//! This module provides utilities for audio processing, including volume normalization,
//! peak detection, and audio effects.

use crate::error::{Result, TtsError};

/// Normalize audio samples to a target peak level
///
/// # Arguments
/// * `samples` - Audio samples to normalize (in-place)
/// * `target_peak` - Target peak level (0.0 to 1.0, default 0.95)
///
/// # Returns
/// The gain factor applied
pub fn normalize_volume(samples: &mut [f32], target_peak: f32) -> Result<f32> {
    if samples.is_empty() {
        return Ok(1.0);
    }

    if target_peak <= 0.0 || target_peak > 1.0 {
        return Err(TtsError::Audio(
            "Target peak must be between 0.0 and 1.0".to_string(),
        ));
    }

    // Find current peak
    let current_peak = samples.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

    if current_peak == 0.0 {
        return Ok(1.0); // Silent audio, no normalization needed
    }

    // Calculate gain factor
    let gain = target_peak / current_peak;

    // Apply gain (with safety limit to prevent clipping)
    let safe_gain = gain.min(10.0); // Max 10x amplification
    for sample in samples.iter_mut() {
        *sample *= safe_gain;
        // Soft clipping to prevent harsh distortion
        *sample = sample.tanh();
    }

    Ok(safe_gain)
}

/// Apply volume adjustment to audio samples
///
/// # Arguments
/// * `samples` - Audio samples to adjust (in-place)
/// * `volume` - Volume multiplier (0.0 to 2.0, where 1.0 is original)
pub fn apply_volume(samples: &mut [f32], volume: f32) {
    let clamped_volume = volume.max(0.0).min(2.0);
    for sample in samples.iter_mut() {
        *sample *= clamped_volume;
    }
}

/// Detect peak level in audio samples
///
/// # Returns
/// Peak level (0.0 to 1.0)
pub fn detect_peak(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    samples.iter().map(|&s| s.abs()).fold(0.0f32, f32::max)
}

/// Calculate RMS (Root Mean Square) level
///
/// # Returns
/// RMS level (0.0 to 1.0)
pub fn calculate_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
    let mean_square = sum_squares / samples.len() as f32;
    mean_square.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_volume() {
        let mut samples = vec![0.1, 0.2, 0.3, -0.2, -0.1];
        let gain = normalize_volume(&mut samples, 0.95).unwrap();

        assert!(gain > 1.0); // Should amplify
        assert!(detect_peak(&samples) <= 1.0); // Should not clip
    }

    #[test]
    fn test_apply_volume() {
        let mut samples = vec![0.5, -0.5, 0.3];
        apply_volume(&mut samples, 0.5);

        assert_eq!(samples[0], 0.25);
        assert_eq!(samples[1], -0.25);
    }

    #[test]
    fn test_detect_peak() {
        let samples = vec![0.1, 0.5, -0.3, 0.2];
        let peak = detect_peak(&samples);

        assert_eq!(peak, 0.5);
    }

    #[test]
    fn test_calculate_rms() {
        let samples = vec![0.5, -0.5, 0.5, -0.5];
        let rms = calculate_rms(&samples);

        assert!((rms - 0.5).abs() < 0.01); // Should be approximately 0.5
    }
}
