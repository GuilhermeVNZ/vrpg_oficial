# TTS Performance Profiles Specification

## Purpose

This specification defines two distinct TTS performance profiles (FAST and CINEMATIC) to optimize latency for different LLM models. The FAST profile targets sub-1s latency for Qwen 1.5B responses, while the CINEMATIC profile targets 1.5-3s latency for Qwen 14B narratives. This addresses the current issue where streaming functions as batch processing (10s+ latency) instead of real-time streaming.

## ADDED Requirements

### Requirement: TTS Performance Profiles

The TTS Service SHALL implement two distinct performance profiles (FAST and CINEMATIC) with different chunking strategies, sample rates, and pre-buffer configurations to optimize latency for different use cases.

#### Scenario: FAST Profile Configuration (Qwen 1.5B)

Given TTS Service receives text from Qwen 1.5B model
When selecting TTS profile
Then the TTS Service SHALL use FAST profile
And the TTS Service SHALL configure first chunk to maximum 30 characters
And the TTS Service SHALL configure subsequent chunks to maximum 90 characters
And the TTS Service SHALL use 16000 Hz sample rate (mono)
And the TTS Service SHALL use FP16 precision for inference
And the TTS Service SHALL use 50ms audio blocks for FIFO streaming
And the TTS Service SHALL use 240ms initial pre-buffer
And the TTS Service SHALL target time_to_first_audio ≤ 0.8s (ideal 0.5-0.7s)

#### Scenario: CINEMATIC Profile Configuration (Qwen 14B)

Given TTS Service receives text from Qwen 14B model
When selecting TTS profile
Then the TTS Service SHALL use CINEMATIC profile
And the TTS Service SHALL configure first chunk to maximum 100 characters
And the TTS Service SHALL configure subsequent chunks to maximum 150 characters
And the TTS Service SHALL use 24000 Hz sample rate (mono)
And the TTS Service SHALL use FP16 precision for inference
And the TTS Service SHALL use 60-80ms audio blocks for FIFO streaming
And the TTS Service SHALL use 500ms initial pre-buffer
And the TTS Service SHALL target time_to_first_audio in range 1.5-3s

#### Scenario: Profile Auto-Selection from LLM Model

Given TTS Service receives streaming request with LLM model name
When determining TTS profile
Then the TTS Service SHALL select FAST profile if model name contains "1.5" or "1_5"
And the TTS Service SHALL select CINEMATIC profile if model name contains "14" or "14b"
And the TTS Service SHALL default to CINEMATIC profile for unknown models
And the TTS Service SHALL allow explicit profile override in request

### Requirement: Profile-Aware Text Chunking

The TTS Service SHALL chunk text differently based on selected profile, with FAST profile using tiny first chunks (30 chars) and CINEMATIC profile using moderate first chunks (100 chars).

#### Scenario: FAST Profile Chunking

Given FAST profile is selected
When chunking text for TTS synthesis
Then the TTS Service SHALL create first chunk with maximum 30 characters
And the TTS Service SHALL create subsequent chunks with maximum 90 characters
And the TTS Service SHALL allow cutting mid-word or mid-sentence for first chunk (speed priority)
And the TTS Service SHALL NOT wait for punctuation or natural pauses for first chunk
And the TTS Service SHALL prioritize speed over semantic coherence for first chunk only

#### Scenario: CINEMATIC Profile Chunking

Given CINEMATIC profile is selected
When chunking text for TTS synthesis
Then the TTS Service SHALL create first chunk with maximum 100 characters
And the TTS Service SHALL create subsequent chunks with maximum 150 characters
And the TTS Service SHALL prefer cutting at spaces or punctuation when possible
And the TTS Service SHALL respect semantic boundaries when within character limits
And the TTS Service SHALL maintain narrative coherence within chunks

#### Scenario: Chunking Algorithm

Given text input and selected profile
When splitting text into chunks
Then the TTS Service SHALL split by words (not by sentences)
And the TTS Service SHALL use first_chunk_max_chars for first chunk limit
And the TTS Service SHALL switch to next_chunk_max_chars after first chunk
And the TTS Service SHALL create chunks sequentially (first, then subsequent)
And the TTS Service SHALL ensure all text is chunked (no text loss)

### Requirement: XTTS Optimization for Profiles

The TTS Service SHALL optimize XTTS inference based on selected profile, using FP16 precision, profile-specific sample rates, and warm-up to minimize latency.

#### Scenario: FP16 Precision Configuration

Given TTS Service initializes XTTS model
When configuring inference precision
Then the TTS Service SHALL load model in FP16 (half precision) format
And the TTS Service SHALL use `model.half().to("cuda")` for GPU inference
And the TTS Service SHALL use `torch.cuda.amp.autocast(device_type="cuda")` for inference
And the TTS Service SHALL use `torch.inference_mode()` for inference path
And the TTS Service SHALL reduce inference time by 30-40% vs Float32

#### Scenario: Profile-Specific Sample Rate

Given TTS Service receives synthesis request with profile
When configuring XTTS synthesis
Then the TTS Service SHALL use 16000 Hz sample rate for FAST profile
And the TTS Service SHALL use 24000 Hz sample rate for CINEMATIC profile
And the TTS Service SHALL pass sample_rate parameter to XTTS synthesis
And the TTS Service SHALL NOT resample after generation (generate at target rate)
And the TTS Service SHALL reduce computational cost for FAST profile (16 kHz)

#### Scenario: Model Warm-Up

Given TTS Service initializes
When service starts
Then the TTS Service SHALL perform one warm-up inference with short text
And the TTS Service SHALL use FAST profile for warm-up (minimal text)
And the TTS Service SHALL complete warm-up before accepting requests
And the TTS Service SHALL ensure first real request does not pay warm-up cost
And the TTS Service SHALL log warm-up completion time

### Requirement: Real-Time Streaming with FIFO

The TTS Service SHALL implement real-time streaming using FIFO (First-In-First-Out) queue with small audio blocks, NOT batch processing that waits for entire chunks before playback.

#### Scenario: FIFO Producer (TTS Generation)

Given text chunks ready for synthesis
When generating audio with XTTS
Then the TTS Service SHALL generate audio for each chunk sequentially
And the TTS Service SHALL split generated audio into blocks of audio_block_ms size
And the TTS Service SHALL push each block immediately to FIFO queue
And the TTS Service SHALL NOT wait for entire chunk to complete before pushing blocks
And the TTS Service SHALL NOT wait for playback to finish before generating next chunk

#### Scenario: FIFO Consumer (Audio Playback)

Given FIFO queue with audio blocks
When starting audio playback
Then the TTS Service SHALL wait until queue contains initial_prebuffer_ms of audio
And the TTS Service SHALL start playback as soon as pre-buffer threshold is met
And the TTS Service SHALL consume blocks from FIFO continuously
And the TTS Service SHALL send blocks to native audio output (WASAPI/ASIO/CoreAudio)
And the TTS Service SHALL maintain continuous playback without gaps

#### Scenario: FAST Profile Streaming

Given FAST profile is selected
When streaming audio
Then the TTS Service SHALL use 50ms audio blocks (800 samples @ 16 kHz)
And the TTS Service SHALL start playback after 240ms of audio is buffered
And the TTS Service SHALL achieve time_to_first_audio ≤ 0.8s
And the TTS Service SHALL maintain continuous playback after start
And the TTS Service SHALL NOT wait for large chunks before starting playback

#### Scenario: CINEMATIC Profile Streaming

Given CINEMATIC profile is selected
When streaming audio
Then the TTS Service SHALL use 60-80ms audio blocks (1440-1920 samples @ 24 kHz)
And the TTS Service SHALL start playback after 500ms of audio is buffered
And the TTS Service SHALL achieve time_to_first_audio in range 1.5-3s
And the TTS Service SHALL maintain continuous playback after start
And the TTS Service SHALL use larger blocks for efficiency (vs FAST)

### Requirement: Latency Metrics

The TTS Service SHALL measure and report latency metrics for each profile, including time_to_first_audio, first_chunk_audio_duration, and RTF per chunk.

#### Scenario: Latency Measurement

Given TTS Service receives synthesis request
When measuring latency
Then the TTS Service SHALL record tts_text_length_chars (total text length)
And the TTS Service SHALL record first_chunk_text_length_chars (first chunk only)
And the TTS Service SHALL record first_chunk_audio_duration_sec (based on samples generated)
And the TTS Service SHALL record xtts_first_chunk_infer_time_sec (inference time for chunk 1)
And the TTS Service SHALL record time_to_first_audio_sec (from request start to first audio block in device)
And the TTS Service SHALL calculate RTF per chunk (inference_time / audio_duration)

#### Scenario: FAST Profile Latency Targets

Given FAST profile is used
When measuring latency
Then the TTS Service SHALL achieve time_to_first_audio_sec ≤ 0.8s
And the TTS Service SHALL target time_to_first_audio_sec in range 0.5-0.7s
And the TTS Service SHALL report first_chunk_audio_duration_sec (typically 0.7-1.0s)
And the TTS Service SHALL report xtts_first_chunk_infer_time_sec
And the TTS Service SHALL verify RTF < 1.0x for first chunk

#### Scenario: CINEMATIC Profile Latency Targets

Given CINEMATIC profile is used
When measuring latency
Then the TTS Service SHALL achieve time_to_first_audio_sec in range 1.5-3s
And the TTS Service SHALL report first_chunk_audio_duration_sec (typically 3s)
And the TTS Service SHALL report xtts_first_chunk_infer_time_sec
And the TTS Service SHALL verify RTF < 1.0x for first chunk
And the TTS Service SHALL maintain continuous streaming after first chunk

## MODIFIED Requirements

### Requirement: Semantic Text Chunking (Updated)

The TTS Service SHALL chunk text based on selected profile, with FAST profile using tiny first chunks (30 chars) and CINEMATIC profile using moderate first chunks (100 chars), NOT fixed 3-7s chunks for all profiles.

#### Scenario: Profile-Aware Chunking

Given text input and selected profile
When chunking text
Then the TTS Service SHALL use profile-specific limits (first_chunk_max_chars, next_chunk_max_chars)
And the TTS Service SHALL create first chunk using first_chunk_max_chars limit
And the TTS Service SHALL create subsequent chunks using next_chunk_max_chars limit
And the TTS Service SHALL NOT use fixed 3-7s duration for all chunks
And the TTS Service SHALL adapt chunking strategy to profile requirements

### Requirement: Audio Streaming (Updated)

The TTS Service SHALL implement real-time streaming with FIFO queue and small audio blocks, NOT batch processing that waits for entire chunks before playback.

#### Scenario: Real-Time FIFO Streaming

Given text ready for synthesis
When streaming audio
Then the TTS Service SHALL generate audio in small blocks (50-80ms)
And the TTS Service SHALL push blocks to FIFO queue immediately
And the TTS Service SHALL start playback after pre-buffer threshold
And the TTS Service SHALL NOT wait for entire chunk before starting playback
And the TTS Service SHALL maintain continuous playback without gaps

## Technical Constraints

### FAST Profile Configuration
- **first_chunk_max_chars**: 30 (target: ~0.7-1.0s of speech)
- **next_chunk_max_chars**: 90 (target: ~2-3s of speech)
- **sample_rate**: 16000 Hz (mono)
- **dtype**: FP16 (half precision)
- **audio_block_ms**: 50ms (800 samples @ 16 kHz)
- **initial_prebuffer_ms**: 240ms (3840 samples @ 16 kHz)
- **time_to_first_audio target**: ≤ 0.8s (ideal 0.5-0.7s)

### CINEMATIC Profile Configuration
- **first_chunk_max_chars**: 100 (target: ~3s of speech)
- **next_chunk_max_chars**: 150 (target: ~4-5s of speech)
- **sample_rate**: 24000 Hz (mono)
- **dtype**: FP16 (half precision)
- **audio_block_ms**: 60-80ms (1440-1920 samples @ 24 kHz)
- **initial_prebuffer_ms**: 500ms (12000 samples @ 24 kHz)
- **time_to_first_audio target**: 1.5-3s

### Performance Targets
- **FAST profile**: time_to_first_audio ≤ 0.8s (ideal 0.5-0.7s)
- **CINEMATIC profile**: time_to_first_audio in range 1.5-3s
- **RTF per chunk**: < 1.0x (all profiles)
- **Continuous playback**: Zero gaps after initial start

## Implementation Architecture

### Profile Structure

```rust
pub struct TtsProfile {
    pub profile_type: TtsProfileType,  // FAST or CINEMATIC
    pub first_chunk_max_chars: usize,  // 30 (FAST) or 100 (CINEMATIC)
    pub next_chunk_max_chars: usize,   // 90 (FAST) or 150 (CINEMATIC)
    pub sample_rate: u32,               // 16000 (FAST) or 24000 (CINEMATIC)
    pub use_fp16: bool,                 // true (both profiles)
    pub audio_block_ms: u32,            // 50 (FAST) or 60-80 (CINEMATIC)
    pub initial_prebuffer_ms: u32,      // 240 (FAST) or 500 (CINEMATIC)
}
```

### Chunking Algorithm

```rust
fn split_text_for_tts(text: &str, profile: &TtsProfile) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_limit = profile.first_chunk_max_chars;
    
    for word in words {
        let word_with_space = if current.is_empty() {
            word.to_string()
        } else {
            format!(" {}", word)
        };
        
        if current.len() + word_with_space.len() > current_limit && !current.is_empty() {
            chunks.push(current.trim().to_string());
            current.clear();
            current_limit = profile.next_chunk_max_chars; // Switch to next chunk limit
        }
        
        current.push_str(&word_with_space);
    }
    
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }
    
    chunks
}
```

### FIFO Streaming Pseudo-Code

```rust
// Producer: TTS Generation
async fn tts_producer(text: String, profile: TtsProfile) {
    let chunks = split_text_for_tts(&text, &profile);
    
    for chunk in chunks {
        let audio = xtts.synthesize(&chunk, sample_rate: profile.sample_rate).await;
        
        // Split into blocks
        let block_samples = profile.audio_block_samples();
        for block_start in (0..audio.len()).step_by(block_samples) {
            let block = audio[block_start..block_start + block_samples];
            audio_fifo.put(block);  // Push immediately
        }
    }
}

// Consumer: Audio Playback
async fn audio_consumer(profile: TtsProfile) {
    // Wait for pre-buffer
    while fifo_duration_ms() < profile.initial_prebuffer_ms {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Start playback
    start_audio_output();
    
    // Consume blocks continuously
    while let Some(block) = audio_fifo.get().await {
        play_block(block);  // Send to native audio output
    }
}
```

## Testing Requirements

### Unit Tests
- Profile creation and configuration (100% coverage)
- Profile auto-selection from LLM model name (100% coverage)
- Text chunking with FAST profile (first chunk ≤ 30 chars)
- Text chunking with CINEMATIC profile (first chunk ≤ 100 chars)
- Audio block size calculation per profile
- Pre-buffer size calculation per profile

### Integration Tests
- End-to-end FAST profile streaming (time_to_first_audio ≤ 0.8s)
- End-to-end CINEMATIC profile streaming (time_to_first_audio 1.5-3s)
- FIFO producer/consumer synchronization
- Pre-buffer threshold behavior
- Continuous playback without gaps

### Performance Tests
- FAST profile latency measurement (target ≤ 0.8s)
- CINEMATIC profile latency measurement (target 1.5-3s)
- RTF per chunk verification (< 1.0x)
- First chunk audio duration validation
- Time to first audio measurement accuracy

## Migration Notes

### Breaking Changes
- Chunking strategy changes (profile-aware vs fixed)
- Sample rate configuration (profile-specific)
- Pre-buffer configuration (profile-specific)
- Streaming API changes (FIFO blocks vs chunks)

### Migration Path
1. Implement TtsProfile structure
2. Implement profile-aware chunking
3. Implement FIFO streaming with blocks
4. Add FP16 support to XTTS
5. Add warm-up on initialization
6. Update streaming pipeline to use profiles
7. Test and validate latency targets
8. Update documentation

## References

- Current latency issue: 10.16s (batch processing, 13s first chunk)
- Target FAST latency: ≤ 0.8s (sub-1s for Qwen 1.5B)
- Target CINEMATIC latency: 1.5-3s (for Qwen 14B)
- RTF advantage: 0.67x (XTTS faster than real-time)
- Problem diagnosis: Large chunks + no real streaming = high latency



