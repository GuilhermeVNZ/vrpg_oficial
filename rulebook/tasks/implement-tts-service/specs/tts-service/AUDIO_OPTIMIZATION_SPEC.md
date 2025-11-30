# Audio I/O Optimization Specification

## Purpose

This specification defines audio format and buffer optimizations for real-time voice streaming, ensuring low latency, efficient bandwidth usage, and system responsiveness without unnecessary processing overhead.

## ADDED Requirements

### Requirement: Optimal Sample Rate for Voice

The TTS Service SHALL use 16 kHz or 24 kHz sample rate for voice audio, NOT 48 kHz which wastes bandwidth and processing.

#### Scenario: Sample Rate Configuration
Given TTS Service initialization
When configuring audio output
Then the TTS Service SHALL use 16000 Hz or 24000 Hz sample rate
And the TTS Service SHALL NOT use 48000 Hz (wasteful for voice)
And the TTS Service SHALL use XTTS native 24000 Hz if available
And the TTS Service SHALL support 16000 Hz for lower bandwidth if needed
And the TTS Service SHALL maintain voice quality at 16-24 kHz

#### Scenario: Sample Rate Selection
Given XTTS generates audio at 24000 Hz
When configuring audio output
Then the TTS Service SHALL use 24000 Hz (XTTS native, no resampling)
And the TTS Service SHALL NOT resample to 48000 Hz (unnecessary)
And the TTS Service SHALL maintain 24000 Hz throughout pipeline
And the TTS Service SHALL only resample if target is 16000 Hz (downsample)

### Requirement: Mono Channel Configuration

The TTS Service SHALL use mono (1 channel) audio, NOT stereo which doubles bandwidth and processing.

#### Scenario: Mono Channel Configuration
Given TTS Service initialization
When configuring audio output
Then the TTS Service SHALL use mono (1 channel) audio
And the TTS Service SHALL NOT use stereo (2 channels)
And the TTS Service SHALL convert stereo to mono if input is stereo
And the TTS Service SHALL maintain mono throughout pipeline
And the TTS Service SHALL reduce bandwidth by 50% vs stereo

#### Scenario: Stereo Input Handling
Given XTTS generates mono audio (1 channel)
When processing audio
Then the TTS Service SHALL maintain mono format
And the TTS Service SHALL NOT duplicate channels to stereo
And the TTS Service SHALL verify channel count is 1

### Requirement: Low-Latency Buffer Size

The TTS Service SHALL use small buffer sizes (256-512 frames) for real-time voice streaming, NOT large buffers (2048/4096) that increase lag.

#### Scenario: Buffer Size Configuration
Given TTS Service initialization
When configuring audio buffer
Then the TTS Service SHALL use 256-512 frames per buffer
And the TTS Service SHALL NOT use large buffers (2048/4096) that increase lag
And the TTS Service SHALL test 256, 384, 512 frames for optimal latency
And the TTS Service SHALL balance stability vs latency
And the TTS Service SHALL prefer smaller buffers for lower latency

#### Scenario: Buffer Size Selection
Given audio framework (WASAPI/ASIO/CoreAudio)
When selecting buffer size
Then the TTS Service SHALL start with 512 frames (stable)
And the TTS Service SHALL reduce to 384 or 256 if system supports
And the TTS Service SHALL NOT use < 256 frames (unstable)
And the TTS Service SHALL NOT use > 512 frames (increases lag)
And the TTS Service SHALL measure actual latency and adjust

### Requirement: Efficient Audio Format (int16 for I/O)

The TTS Service SHALL use int16 PCM format for audio I/O, NOT float64 or other heavy formats that waste bandwidth.

#### Scenario: Internal vs I/O Format
Given audio processing pipeline
When handling audio data
Then the TTS Service SHALL use Float32 internally (XTTS inference)
And the TTS Service SHALL convert Float32 to int16 for audio I/O
And the TTS Service SHALL NOT use float64 for I/O (wasteful)
And the TTS Service SHALL NOT use Float32 for I/O (unnecessary precision)
And the TTS Service SHALL maintain int16 compatibility with Opus and codecs

#### Scenario: Format Conversion
Given Float32 audio samples from XTTS
When sending to audio output
Then the TTS Service SHALL convert Float32 [-1.0, 1.0] to int16 [-32768, 32767]
And the TTS Service SHALL perform efficient conversion (no quality loss for voice)
And the TTS Service SHALL maintain audio quality (int16 sufficient for voice)
And the TTS Service SHALL reduce bandwidth vs Float32

### Requirement: Dedicated Audio I/O Thread

The TTS Service SHALL use a dedicated thread for audio I/O (capture and playback), NOT sharing thread with UI, model inference, or other operations.

#### Scenario: Dedicated Audio Thread
Given TTS Service initialization
When setting up audio I/O
Then the TTS Service SHALL create dedicated thread for audio I/O
And the TTS Service SHALL NOT share thread with UI operations
And the TTS Service SHALL NOT share thread with model inference
And the TTS Service SHALL NOT share thread with other I/O operations
And the TTS Service SHALL isolate audio I/O from other processing

#### Scenario: Thread Isolation
Given dedicated audio I/O thread
When processing audio
Then the TTS Service SHALL handle audio callbacks in dedicated thread
And the TTS Service SHALL NOT block on UI or model operations
And the TTS Service SHALL maintain real-time audio scheduling
And the TTS Service SHALL prevent audio glitches from other operations

## Technical Constraints

### Audio Format Requirements
- **Sample rate**: 16000-24000 Hz (16 kHz or 24 kHz, NOT 48 kHz)
- **Channels**: Mono (1 channel, NOT stereo)
- **Internal format**: Float32 (for XTTS inference)
- **I/O format**: int16 PCM (for audio output)
- **Buffer size**: 256-512 frames (NOT 2048/4096)

### Latency Targets
- **Buffer latency**: < 20ms (at 24 kHz, 512 frames = ~21ms, acceptable)
- **Total I/O latency**: < 30ms
- **System responsiveness**: No lag from audio processing

### Bandwidth Optimization
- **Mono vs Stereo**: 50% bandwidth reduction
- **int16 vs Float32**: 50% bandwidth reduction for I/O
- **16 kHz vs 24 kHz**: 33% bandwidth reduction (if 16 kHz acceptable)

## Implementation Notes

### Format Conversion

```rust
fn convert_float32_to_int16(float_samples: Vec<f32>) -> Vec<i16> {
    float_samples
        .iter()
        .map(|&sample| {
            // Clamp to [-1.0, 1.0] and convert to int16
            let clamped = sample.max(-1.0).min(1.0);
            (clamped * 32767.0) as i16
        })
        .collect()
}
```

### Buffer Size Selection

```rust
fn select_optimal_buffer_size(audio_framework: AudioFramework) -> usize {
    match audio_framework {
        AudioFramework::ASIO => 256,  // Lowest latency
        AudioFramework::WASAPI => 384,  // Balanced
        AudioFramework::CoreAudio => 512, // Stable
    }
}
```

### Sample Rate Handling

```rust
fn configure_sample_rate(xtts_output_rate: u32) -> u32 {
    // Use XTTS native rate (24 kHz) or 16 kHz if needed
    match xtts_output_rate {
        24000 => 24000,  // Native, no resampling
        16000 => 16000,  // Lower bandwidth
        _ => 24000,      // Default to 24 kHz
    }
}
```

## Testing Requirements

### Unit Tests
- Format conversion (Float32 to int16) - 100% coverage
- Sample rate validation (16/24 kHz only)
- Channel count validation (mono only)
- Buffer size validation (256-512 range)

### Integration Tests
- Audio I/O with int16 format
- Mono channel verification
- Low-latency buffer performance
- Dedicated thread isolation

### Performance Tests
- Latency measurement (target < 30ms)
- Bandwidth usage (verify 50% reduction vs stereo/Float32)
- CPU usage (verify no overhead from format conversion)

## Migration Notes

### Breaking Changes
- Audio I/O format changes (int16 instead of Float32)
- Buffer size changes (256-512 instead of 2048/4096)
- Sample rate constraints (16/24 kHz only)

### Migration Path
1. Implement Float32 to int16 conversion
2. Configure mono channel output
3. Reduce buffer sizes to 256-512
4. Verify sample rate (16/24 kHz)
5. Test latency and quality
6. Update audio framework integration

## References

- Voice sample rate: 16-24 kHz sufficient (NOT 48 kHz)
- Mono vs Stereo: 50% bandwidth reduction
- int16 PCM: Standard, efficient, compatible with Opus
- Buffer size: 256-512 frames optimal for real-time voice
- Dedicated thread: Essential for real-time audio I/O



