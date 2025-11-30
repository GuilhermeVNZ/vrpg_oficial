//! Unit tests for audio buffer

use tts_service::audio_buffer::{AudioBuffer, AudioChunk};

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
        // -1.0 clamped to int16: -1.0 * 32767.0 = -32767 (not -32768)
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

#[test]
fn test_buffer_length_calculation() {
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    let chunk1 = AudioChunk {
        samples: vec![0.1; 2400], // 0.1 seconds
        sample_rate: 24000,
        channels: 1,
    };

    let chunk2 = AudioChunk {
        samples: vec![0.1; 2400], // 0.1 seconds
        sample_rate: 24000,
        channels: 1,
    };

    buffer.push(chunk1).unwrap();
    buffer.push(chunk2).unwrap();

    let length = buffer.buffer_length_seconds().unwrap();
    assert!((length - 0.2).abs() < 0.01); // ~0.2 seconds
}

#[test]
fn test_buffer_clear() {
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    let chunk = AudioChunk {
        samples: vec![0.1; 2400],
        sample_rate: 24000,
        channels: 1,
    };

    buffer.push(chunk).unwrap();
    assert_eq!(buffer.chunk_count().unwrap(), 1);

    buffer.clear().unwrap();
    assert!(buffer.is_empty().unwrap());
}

#[test]
fn test_audio_chunk_duration() {
    let chunk = AudioChunk {
        samples: vec![0.1; 24000], // 1 second at 24kHz
        sample_rate: 24000,
        channels: 1,
    };

    let duration = chunk.duration_seconds();
    assert!((duration - 1.0).abs() < 0.01);
}

#[test]
fn test_buffer_partial_pop() {
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    let chunk = AudioChunk {
        samples: vec![0.1; 4800], // 0.2 seconds
        sample_rate: 24000,
        channels: 1,
    };

    buffer.push(chunk).unwrap();

    // Pop partial chunk
    let samples = buffer.pop_block(2400).unwrap();
    assert_eq!(samples.len(), 2400);
    assert_eq!(buffer.chunk_count().unwrap(), 1); // Still has remaining samples

    // Pop remaining
    let samples2 = buffer.pop_block(2400).unwrap();
    assert_eq!(samples2.len(), 2400);
    assert!(buffer.is_empty().unwrap());
}

#[test]
fn test_buffer_multiple_chunks() {
    let buffer = AudioBuffer::new(24000, 1, 10.0);

    for i in 0..3 {
        let chunk = AudioChunk {
            samples: vec![i as f32 * 0.1; 2400],
            sample_rate: 24000,
            channels: 1,
        };
        buffer.push(chunk).unwrap();
    }

    assert_eq!(buffer.chunk_count().unwrap(), 3);

    // Pop all
    let samples = buffer.pop_block(7200).unwrap();
    assert_eq!(samples.len(), 7200);
    assert!(buffer.is_empty().unwrap());
}

#[test]
fn test_audio_chunk_duration_seconds() {
    let chunk = AudioChunk {
        samples: vec![0.1; 48000], // 2 seconds at 24kHz
        sample_rate: 24000,
        channels: 1,
    };

    let duration = chunk.duration_seconds();
    assert!((duration - 2.0).abs() < 0.01);
}

#[test]
fn test_audio_chunk_duration_stereo() {
    let chunk = AudioChunk {
        samples: vec![0.1; 48000], // 2 seconds at 24kHz, stereo
        sample_rate: 24000,
        channels: 2,
    };

    let duration = chunk.duration_seconds();
    assert!((duration - 1.0).abs() < 0.01); // Stereo: samples / channels / rate
}
