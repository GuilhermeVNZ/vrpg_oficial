//! Pre-Buffer Manager
//!
//! Manages pre-buffering state and playback control based on buffer levels

use crate::audio_buffer::AudioBuffer;
use crate::error::Result;
use crate::gpu_config::GpuConfig;

/// Pre-buffer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreBufferState {
    /// Filling buffer (not ready for playback)
    Filling,
    /// Ready for playback (buffer sufficient)
    Ready,
    /// Playing (buffer maintained)
    Playing,
    /// Paused (buffer low, need to refill)
    Paused,
}

/// Pre-Buffer Manager
pub struct PreBufferManager {
    config: GpuConfig,
    state: PreBufferState,
}

impl PreBufferManager {
    /// Create new pre-buffer manager
    pub fn new(config: GpuConfig) -> Self {
        Self {
            config,
            state: PreBufferState::Filling,
        }
    }

    /// Check if playback should start
    pub fn should_start_playback(&self, buffer: &AudioBuffer) -> Result<bool> {
        let buffer_length = buffer.buffer_length_seconds()?;
        let threshold = self.config.prebuffer_seconds;

        Ok(buffer_length >= threshold && self.state == PreBufferState::Filling)
    }

    /// Check if playback should pause (buffer too low)
    pub fn should_pause_playback(&self, buffer: &AudioBuffer) -> Result<bool> {
        let buffer_length = buffer.buffer_length_seconds()?;
        let threshold = self.config.prebuffer_seconds * 0.5; // Pause at 50% of target

        Ok(buffer_length < threshold && self.state == PreBufferState::Playing)
    }

    /// Update state based on buffer level
    pub fn update_state(&mut self, buffer: &AudioBuffer) -> Result<PreBufferState> {
        let buffer_length = buffer.buffer_length_seconds()?;
        let threshold = self.config.prebuffer_seconds;

        self.state = match self.state {
            PreBufferState::Filling => {
                if buffer_length >= threshold {
                    PreBufferState::Ready
                } else {
                    PreBufferState::Filling
                }
            }
            PreBufferState::Ready => {
                if buffer_length >= threshold {
                    PreBufferState::Ready
                } else {
                    PreBufferState::Filling
                }
            }
            PreBufferState::Playing => {
                if buffer_length < threshold * 0.5 {
                    PreBufferState::Paused
                } else {
                    PreBufferState::Playing
                }
            }
            PreBufferState::Paused => {
                if buffer_length >= threshold {
                    PreBufferState::Ready
                } else {
                    PreBufferState::Paused
                }
            }
        };

        Ok(self.state)
    }

    /// Get current state
    pub fn state(&self) -> PreBufferState {
        self.state
    }

    /// Set state (for external control)
    pub fn set_state(&mut self, state: PreBufferState) {
        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio_buffer::{AudioBuffer, AudioChunk};

    #[test]
    fn test_should_start_playback() {
        let config = GpuConfig {
            max_parallel_streams: 1,
            vram_limit_mb: 3072,
            utilization_target: 0.5,
            prebuffer_seconds: 1.5,
            yield_between_chunks: true,
            cpu_fallback_enabled: true,
        };

        let manager = PreBufferManager::new(config);
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
}



