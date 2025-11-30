//! TTS Profile Configuration
//!
//! Defines performance profiles for TTS synthesis:
//! - FAST: Low latency for Qwen 1.5B (sub-1s target)
//! - CINEMATIC: Higher quality for Qwen 14B (1.5-3s target)

use serde::{Deserialize, Serialize};

/// TTS Performance Profile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TtsProfileType {
    /// Fast profile for Qwen 1.5B - sub-1s latency
    Fast,
    /// Cinematic profile for Qwen 14B - 1.5-3s latency
    Cinematic,
}

/// TTS Profile Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsProfile {
    /// Profile type
    pub profile_type: TtsProfileType,
    /// Maximum characters for first chunk (very small for FAST)
    pub first_chunk_max_chars: usize,
    /// Maximum characters for subsequent chunks
    pub next_chunk_max_chars: usize,
    /// Sample rate (Hz) - 16kHz for FAST, 24kHz for CINEMATIC
    pub sample_rate: u32,
    /// Use FP16 precision (faster inference)
    pub use_fp16: bool,
    /// Audio block size in milliseconds (for FIFO streaming)
    pub audio_block_ms: u32,
    /// Initial pre-buffer size in milliseconds (before starting playback)
    pub initial_prebuffer_ms: u32,
}

impl TtsProfile {
    /// Create FAST profile (for Qwen 1.5B)
    pub fn fast() -> Self {
        Self {
            profile_type: TtsProfileType::Fast,
            first_chunk_max_chars: 30,      // ~0.7-1.0s of speech
            next_chunk_max_chars: 90,        // ~2-3s of speech
            sample_rate: 16000,              // 16kHz for lower latency
            use_fp16: true,                  // FP16 for speed
            audio_block_ms: 50,              // 50ms blocks
            initial_prebuffer_ms: 240,       // ~250ms pre-buffer
        }
    }

    /// Create CINEMATIC profile (for Qwen 14B)
    pub fn cinematic() -> Self {
        Self {
            profile_type: TtsProfileType::Cinematic,
            first_chunk_max_chars: 100,      // ~3s of speech
            next_chunk_max_chars: 150,       // ~4-5s of speech
            sample_rate: 24000,              // 24kHz for quality
            use_fp16: true,                  // FP16 for speed
            audio_block_ms: 60,              // 60-80ms blocks
            initial_prebuffer_ms: 500,       // 400-600ms pre-buffer
        }
    }

    /// Get profile based on LLM model name
    pub fn from_llm_model(llm_model_name: &str) -> Self {
        let model_lower = llm_model_name.to_lowercase();
        
        if model_lower.contains("1.5") || model_lower.contains("1_5") {
            Self::fast()
        } else if model_lower.contains("14") || model_lower.contains("14b") {
            Self::cinematic()
        } else {
            // Default to cinematic for unknown models
            Self::cinematic()
        }
    }

    /// Get audio block size in samples
    pub fn audio_block_samples(&self) -> usize {
        (self.sample_rate as u32 * self.audio_block_ms / 1000) as usize
    }

    /// Get initial pre-buffer size in samples
    pub fn initial_prebuffer_samples(&self) -> usize {
        (self.sample_rate as u32 * self.initial_prebuffer_ms / 1000) as usize
    }
}

impl Default for TtsProfile {
    fn default() -> Self {
        Self::cinematic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_profile() {
        let profile = TtsProfile::fast();
        assert_eq!(profile.first_chunk_max_chars, 30);
        assert_eq!(profile.sample_rate, 16000);
        assert_eq!(profile.audio_block_ms, 50);
        assert_eq!(profile.initial_prebuffer_ms, 240);
    }

    #[test]
    fn test_cinematic_profile() {
        let profile = TtsProfile::cinematic();
        assert_eq!(profile.first_chunk_max_chars, 100);
        assert_eq!(profile.sample_rate, 24000);
        assert_eq!(profile.audio_block_ms, 60);
        assert_eq!(profile.initial_prebuffer_ms, 500);
    }

    #[test]
    fn test_from_llm_model() {
        let fast = TtsProfile::from_llm_model("qwen2.5-1.5b-instruct");
        assert_eq!(fast.profile_type, TtsProfileType::Fast);

        let cinematic = TtsProfile::from_llm_model("qwen2.5-14b-instruct");
        assert_eq!(cinematic.profile_type, TtsProfileType::Cinematic);
    }

    #[test]
    fn test_audio_block_samples() {
        let fast = TtsProfile::fast();
        // 16000 Hz * 50ms / 1000 = 800 samples
        assert_eq!(fast.audio_block_samples(), 800);

        let cinematic = TtsProfile::cinematic();
        // 24000 Hz * 60ms / 1000 = 1440 samples
        assert_eq!(cinematic.audio_block_samples(), 1440);
    }
}



