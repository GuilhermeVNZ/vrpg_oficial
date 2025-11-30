use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceActivity {
    Silence,
    Speech,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VadResult {
    pub activity: VoiceActivity,
    pub confidence: f32,
    pub energy: f32,
}

pub struct VoiceActivityDetector {
    #[allow(dead_code)]
    energy_threshold: f32,
    speech_threshold: f32,
    silence_threshold: f32,
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceActivityDetector {
    pub fn new() -> Self {
        Self {
            energy_threshold: 0.01,
            speech_threshold: 0.05,
            silence_threshold: 0.01,
        }
    }

    pub fn with_thresholds(
        energy_threshold: f32,
        speech_threshold: f32,
        silence_threshold: f32,
    ) -> Self {
        Self {
            energy_threshold,
            speech_threshold,
            silence_threshold,
        }
    }

    pub fn detect(&self, audio_data: &[f32]) -> Result<VadResult> {
        if audio_data.is_empty() {
            return Ok(VadResult {
                activity: VoiceActivity::Silence,
                confidence: 1.0,
                energy: 0.0,
            });
        }

        // Calculate RMS energy
        let energy =
            (audio_data.iter().map(|&x| x * x).sum::<f32>() / audio_data.len() as f32).sqrt();

        let (activity, confidence) = if energy >= self.speech_threshold {
            (
                VoiceActivity::Speech,
                (energy / self.speech_threshold).min(1.0),
            )
        } else if energy <= self.silence_threshold {
            (
                VoiceActivity::Silence,
                1.0 - (energy / self.silence_threshold),
            )
        } else {
            (VoiceActivity::Unknown, 0.5)
        };

        Ok(VadResult {
            activity,
            confidence,
            energy,
        })
    }

    pub fn detect_start(&self, audio_data: &[f32]) -> bool {
        match self.detect(audio_data) {
            Ok(result) => result.activity == VoiceActivity::Speech,
            Err(_) => false,
        }
    }

    pub fn detect_end(&self, audio_data: &[f32], consecutive_silence_chunks: usize) -> bool {
        match self.detect(audio_data) {
            Ok(result) => {
                result.activity == VoiceActivity::Silence && consecutive_silence_chunks >= 3
            }
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_detection_speech() {
        let vad = VoiceActivityDetector::new();
        // High energy audio (simulating speech)
        let audio = vec![0.5; 1600]; // 100ms at 16kHz
        let result = vad.detect(&audio).unwrap();
        assert_eq!(result.activity, VoiceActivity::Speech);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_vad_detection_silence() {
        let vad = VoiceActivityDetector::new();
        // Low energy audio (simulating silence)
        let audio = vec![0.001; 1600];
        let result = vad.detect(&audio).unwrap();
        assert_eq!(result.activity, VoiceActivity::Silence);
    }

    #[test]
    fn test_vad_detection_start() {
        let vad = VoiceActivityDetector::new();
        let audio = vec![0.5; 1600];
        assert!(vad.detect_start(&audio));
    }

    #[test]
    fn test_vad_detection_end() {
        let vad = VoiceActivityDetector::new();
        let audio = vec![0.001; 1600];
        assert!(vad.detect_end(&audio, 3));
        assert!(!vad.detect_end(&audio, 2));
    }

    #[test]
    fn test_vad_empty_audio() {
        let vad = VoiceActivityDetector::new();
        let audio = vec![];
        let result = vad.detect(&audio).unwrap();
        assert_eq!(result.activity, VoiceActivity::Silence);
    }
}
