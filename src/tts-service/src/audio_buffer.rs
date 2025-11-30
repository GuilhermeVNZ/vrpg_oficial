//! Audio Buffer FIFO
//!
//! Thread-safe FIFO buffer for streaming audio chunks
//! Stores Float32 internally (XTTS output) and converts to int16 for I/O

use crate::error::{Result, TtsError};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Audio sample format
#[derive(Debug, Clone, Copy)]
pub enum AudioFormat {
    /// 32-bit floating point (internal XTTS format)
    Float32,
    /// 16-bit integer (I/O format)
    Int16,
}

/// Audio chunk with metadata
#[derive(Debug, Clone)]
pub struct AudioChunk {
    /// Audio samples (Float32 format)
    pub samples: Vec<f32>,
    /// Sample rate (Hz)
    pub sample_rate: u32,
    /// Channels (1 = mono, 2 = stereo)
    pub channels: u16,
}

impl AudioChunk {
    /// Convert Float32 samples to int16
    pub fn to_int16(&self) -> Vec<i16> {
        self.samples
            .iter()
            .map(|&s| {
                // Clamp to [-1.0, 1.0] and convert to i16
                let clamped = s.clamp(-1.0, 1.0);
                // Convert: -1.0 -> -32768, 0.0 -> 0, 1.0 -> 32767
                // Use 32768.0 for negative values to get -32768, 32767.0 for positive
                if clamped < 0.0 {
                    (clamped * 32768.0) as i16
                } else {
                    (clamped * 32767.0) as i16
                }
            })
            .collect()
    }

    /// Get duration in seconds
    pub fn duration_seconds(&self) -> f32 {
        self.samples.len() as f32 / self.sample_rate as f32 / self.channels as f32
    }
}

/// Thread-safe FIFO audio buffer
pub struct AudioBuffer {
    /// Internal queue (Float32 samples)
    queue: Arc<Mutex<VecDeque<AudioChunk>>>,
    /// Sample rate (assumed constant)
    sample_rate: u32,
    /// Channels (assumed constant, 1 = mono)
    channels: u16,
    /// Maximum buffer size in seconds
    max_buffer_seconds: f32,
}

impl AudioBuffer {
    /// Create new audio buffer
    pub fn new(sample_rate: u32, channels: u16, max_buffer_seconds: f32) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            sample_rate,
            channels,
            max_buffer_seconds,
        }
    }

    /// Push audio chunk (non-blocking)
    /// Returns error if buffer would exceed max size
    pub fn push(&self, chunk: AudioChunk) -> Result<()> {
        let mut queue = self.queue.lock().map_err(|e| {
            TtsError::ModelLoad(format!("AudioBuffer lock poisoned: {}", e))
        })?;

        // Check if adding this chunk would exceed max buffer
        let current_duration = self.buffer_length_seconds_internal(&queue);
        let chunk_duration = chunk.duration_seconds();

        if current_duration + chunk_duration > self.max_buffer_seconds {
            return Err(TtsError::ModelLoad(format!(
                "Buffer would exceed max size: {:.2}s + {:.2}s > {:.2}s",
                current_duration, chunk_duration, self.max_buffer_seconds
            )));
        }

        queue.push_back(chunk);
        Ok(())
    }

    /// Pop audio chunk (blocking if empty)
    /// Returns int16 samples for I/O
    pub fn pop_block(&self, n_samples: usize) -> Result<Vec<i16>> {
        let mut queue = self.queue.lock().map_err(|e| {
            TtsError::ModelLoad(format!("AudioBuffer lock poisoned: {}", e))
        })?;

        let mut result = Vec::with_capacity(n_samples);
        let mut remaining = n_samples;

        while remaining > 0 {
            if queue.is_empty() {
                // Buffer underrun - pad with silence
                result.extend(vec![0i16; remaining]);
                break;
            }

            let chunk = queue.front_mut().unwrap();
            let chunk_samples = chunk.to_int16();
            let chunk_len = chunk_samples.len();

            if chunk_len <= remaining {
                // Use entire chunk
                result.extend(chunk_samples);
                remaining -= chunk_len;
                queue.pop_front();
            } else {
                // Use partial chunk
                result.extend(chunk_samples.iter().take(remaining));
                // Remove used samples from chunk
                chunk.samples.drain(0..remaining);
                remaining = 0;
            }
        }

        Ok(result)
    }

    /// Get current buffer length in seconds
    pub fn buffer_length_seconds(&self) -> Result<f32> {
        let queue = self.queue.lock().map_err(|e| {
            TtsError::ModelLoad(format!("AudioBuffer lock poisoned: {}", e))
        })?;
        Ok(self.buffer_length_seconds_internal(&queue))
    }

    /// Internal buffer length calculation
    fn buffer_length_seconds_internal(&self, queue: &VecDeque<AudioChunk>) -> f32 {
        queue
            .iter()
            .map(|chunk| chunk.duration_seconds())
            .sum()
    }

    /// Clear buffer
    pub fn clear(&self) -> Result<()> {
        let mut queue = self.queue.lock().map_err(|e| {
            TtsError::ModelLoad(format!("AudioBuffer lock poisoned: {}", e))
        })?;
        queue.clear();
        Ok(())
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> Result<bool> {
        let queue = self.queue.lock().map_err(|e| {
            TtsError::ModelLoad(format!("AudioBuffer lock poisoned: {}", e))
        })?;
        Ok(queue.is_empty())
    }

    /// Get number of chunks in buffer
    pub fn chunk_count(&self) -> Result<usize> {
        let queue = self.queue.lock().map_err(|e| {
            TtsError::ModelLoad(format!("AudioBuffer lock poisoned: {}", e))
        })?;
        Ok(queue.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_chunk_to_int16() {
        let chunk = AudioChunk {
            samples: vec![0.0, 0.5, 1.0, -1.0],
            sample_rate: 24000,
            channels: 1,
        };

        let int16_samples = chunk.to_int16();
        assert_eq!(int16_samples[0], 0);
        assert_eq!(int16_samples[1], 16383); // ~0.5 * 32767
        assert_eq!(int16_samples[2], 32767);
        assert_eq!(int16_samples[3], -32768);
    }

    #[test]
    fn test_buffer_push_pop() {
        let buffer = AudioBuffer::new(24000, 1, 10.0);

        let chunk = AudioChunk {
            samples: vec![0.1; 2400], // 0.1 seconds at 24kHz
            sample_rate: 24000,
            channels: 1,
        };

        buffer.push(chunk.clone()).unwrap();
        assert_eq!(buffer.chunk_count().unwrap(), 1);

        let samples = buffer.pop_block(2400).unwrap();
        assert_eq!(samples.len(), 2400);
        assert!(buffer.is_empty().unwrap());
    }

    #[test]
    fn test_buffer_underrun() {
        let buffer = AudioBuffer::new(24000, 1, 10.0);

        // Try to pop from empty buffer
        let samples = buffer.pop_block(2400).unwrap();
        assert_eq!(samples.len(), 2400);
        // Should be silence (zeros)
        assert!(samples.iter().all(|&s| s == 0));
    }

    #[test]
    fn test_buffer_max_size() {
        let buffer = AudioBuffer::new(24000, 1, 1.0); // 1 second max

        let chunk = AudioChunk {
            samples: vec![0.1; 24000 * 2], // 2 seconds
            sample_rate: 24000,
            channels: 1,
        };

        // Should fail - exceeds max buffer
        assert!(buffer.push(chunk).is_err());
    }
}

