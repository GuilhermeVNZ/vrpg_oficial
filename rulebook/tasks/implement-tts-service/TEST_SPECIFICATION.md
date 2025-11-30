# Test Specification: XTTS Real-Time Streaming with Adaptive GPU Control

## Purpose

This document defines comprehensive test specifications for production deployment of XTTS real-time streaming with adaptive GPU control and audio optimizations. All tests MUST pass before production deployment.

## Test Requirements

### Coverage Requirements
- **Unit Tests**: 100% coverage for all new modules
- **Integration Tests**: All critical paths covered
- **Performance Tests**: All targets validated
- **Quality Tests**: All quality metrics verified
- **Production Tests**: All production scenarios tested

### Test Execution
- All tests MUST pass (100% pass rate)
- Tests MUST be deterministic (no flaky tests)
- Tests MUST be fast (< 5s for unit tests, < 30s for integration tests)
- Tests MUST be isolated (no shared state)

## Unit Test Specifications

### GPU Detection Tests

#### Test: GPU Name Detection
```rust
#[test]
fn test_gpu_name_detection() {
    // Given: CUDA is available
    // When: Detecting GPU name
    // Then: Should return GPU name string
    // And: Should handle no GPU gracefully
}
```

#### Test: VRAM Detection
```rust
#[test]
fn test_vram_detection() {
    // Given: CUDA is available
    // When: Detecting VRAM
    // Then: Should return VRAM in GB
    // And: Should handle invalid GPU gracefully
}
```

#### Test: Tier Classification
```rust
#[test]
fn test_tier_classification_high_end() {
    // Given: RTX 5090 (32GB VRAM, compute 8.0+)
    // When: Classifying GPU tier
    // Then: Should return HighEnd tier
}

#[test]
fn test_tier_classification_modest() {
    // Given: RTX 3050 (4-8GB VRAM, compute 6.0+)
    // When: Classifying GPU tier
    // Then: Should return Modest tier
}

#[test]
fn test_tier_classification_cpu_fallback() {
    // Given: No GPU available
    // When: Classifying GPU tier
    // Then: Should return CPU fallback
}
```

### GPU Configuration Tests

#### Test: Config Generation Per Tier
```rust
#[test]
fn test_config_high_end() {
    // Given: HighEnd tier
    // When: Generating config
    // Then: Should have parallel_streams = 2-3
    // And: Should have vram_limit = 0 (unlimited)
    // And: Should have utilization_target = 0.85
    // And: Should have prebuffer_seconds = 2.5
}

#[test]
fn test_config_modest() {
    // Given: Modest tier
    // When: Generating config
    // Then: Should have parallel_streams = 1
    // And: Should have vram_limit = 3072 MB
    // And: Should have utilization_target = 0.50
    // And: Should have prebuffer_seconds = 1.25
    // And: Should have yield_between_chunks = true
}
```

### AudioBuffer Tests

#### Test: FIFO Push/Pop
```rust
#[test]
fn test_audio_buffer_push_pop() {
    // Given: Empty AudioBuffer
    // When: Pushing Float32 chunk
    // Then: Should append to queue
    // And: Should not block
    // When: Popping chunk
    // Then: Should return int16 PCM
    // And: Should block if empty
}

#[test]
fn test_audio_buffer_concurrent_access() {
    // Given: AudioBuffer with multiple threads
    // When: Concurrent push/pop operations
    // Then: Should maintain thread safety
    // And: Should not have race conditions
    // And: Should maintain FIFO order
}
```

#### Test: Format Conversion
```rust
#[test]
fn test_float32_to_int16_conversion() {
    // Given: Float32 samples [-1.0, 1.0]
    // When: Converting to int16
    // Then: Should return int16 samples [-32768, 32767]
    // And: Should preserve audio quality
    // And: Should handle clipping correctly
}
```

### Semantic Chunker Tests

#### Test: Chunking Rules
```rust
#[test]
fn test_semantic_chunking() {
    // Given: Long text with semantic pauses
    // When: Chunking text
    // Then: Should create chunks of 3-7 seconds
    // And: Should respect semantic boundaries
    // And: Should not cut mid-phrase
    // And: Should maintain narrative flow
}

#[test]
fn test_chunk_duration_constraints() {
    // Given: Text chunks
    // When: Validating chunk duration
    // Then: Should enforce min 2.4s
    // And: Should enforce max 8.0s
    // And: Should target 3-7s
}
```

### Pre-Buffer Manager Tests

#### Test: Playback State Machine
```rust
#[test]
fn test_should_start_playback() {
    // Given: Buffer with 2.5s audio (High-End)
    // When: Checking should_start_playback
    // Then: Should return true
}

#[test]
fn test_should_pause_playback() {
    // Given: Buffer with 1.0s audio (Modest)
    // When: Checking should_pause_playback
    // Then: Should return true
}
```

## Integration Test Specifications

### End-to-End Streaming Test

#### Test: Full Pipeline High-End
```rust
#[tokio::test]
async fn test_full_pipeline_high_end() {
    // Given: High-End GPU, long narrative text
    // When: Running full pipeline
    // Then: Should generate audio in parallel
    // And: Should maintain 2-3 chunks ahead
    // And: Should achieve < 3.8s initial latency
    // And: Should have zero-gap playback
    // And: Should maintain GPU utilization 80-95%
}
```

#### Test: Full Pipeline Modest
```rust
#[tokio::test]
async fn test_full_pipeline_modest() {
    // Given: Modest GPU, long narrative text
    // When: Running full pipeline
    // Then: Should generate audio sequentially
    // And: Should maintain 1 chunk ahead
    // And: Should achieve < 4.5s initial latency
    // And: Should have zero-gap playback
    // And: Should maintain GPU utilization 40-60%
    // And: Should not overload system
}
```

### Buffer Management Tests

#### Test: Buffer Underrun Recovery
```rust
#[tokio::test]
async fn test_buffer_underrun_recovery() {
    // Given: Buffer approaching empty
    // When: Buffer underrun occurs
    // Then: Should pause playback gracefully
    // And: Should continue generating chunks
    // And: Should resume when buffer > threshold
    // And: Should apply crossfade on resume
}
```

### GPU Adaptive Control Tests

#### Test: GPU Tier Auto-Detection
```rust
#[tokio::test]
async fn test_gpu_tier_auto_detection() {
    // Given: System with GPU
    // When: Initializing TTS Service
    // Then: Should detect GPU tier automatically
    // And: Should apply appropriate configuration
    // And: Should respect environment variables if set
}
```

## Performance Test Specifications

### Latency Tests

#### Test: Initial Latency Measurement
```rust
#[tokio::test]
async fn test_initial_latency_high_end() {
    // Given: High-End GPU configuration
    // When: Starting audio playback
    // Then: Initial latency SHOULD be < 3.8s
    // And: SHOULD be measured and logged
}

#[tokio::test]
async fn test_initial_latency_modest() {
    // Given: Modest GPU configuration
    // When: Starting audio playback
    // Then: Initial latency SHOULD be < 4.5s
    // And: SHOULD be measured and logged
}
```

### GPU Performance Tests

#### Test: GPU Utilization Measurement
```rust
#[tokio::test]
async fn test_gpu_utilization_high_end() {
    // Given: High-End GPU configuration
    // When: Generating audio chunks
    // Then: GPU utilization SHOULD be 80-95%
    // And: SHOULD be measured and logged
}

#[tokio::test]
async fn test_gpu_utilization_modest() {
    // Given: Modest GPU configuration
    // When: Generating audio chunks
    // Then: GPU utilization SHOULD be 40-60%
    // And: SHOULD not overload system
    // And: SHOULD be measured and logged
}
```

### Real-Time Factor Tests

#### Test: RTF Measurement
```rust
#[tokio::test]
async fn test_real_time_factor() {
    // Given: Audio generation
    // When: Measuring RTF
    // Then: High-End SHOULD have RTF < 0.5x
    // And: Modest SHOULD have RTF < 0.8x
    // And: Low-End SHOULD have RTF < 1.0x
}
```

## Quality Test Specifications

### Audio Quality Tests

#### Test: RAW Quality Preservation
```rust
#[tokio::test]
async fn test_raw_quality_preservation() {
    // Given: XTTS generated audio (RAW)
    // When: Processing through pipeline
    // Then: Audio quality SHOULD be preserved
    // And: SHOULD not have metallic artifacts
    // And: SHOULD not have distortion
    // And: SHOULD maintain natural voice quality
}
```

#### Test: Format Conversion Quality
```rust
#[tokio::test]
async fn test_format_conversion_quality() {
    // Given: Float32 audio from XTTS
    // When: Converting to int16 for I/O
    // Then: Audio quality SHOULD be preserved
    // And: SHOULD not introduce artifacts
    // And: SHOULD be sufficient for voice
}
```

### Chunk Continuity Tests

#### Test: Zero-Gap Playback
```rust
#[tokio::test]
async fn test_zero_gap_playback() {
    // Given: Multiple audio chunks
    // When: Playing chunks sequentially
    // Then: SHOULD have zero gaps between chunks
    // And: SHOULD maintain audio continuity
    // And: SHOULD not have clicks or pops
}
```

## Production Readiness Tests

### Stress Tests

#### Test: Long Narrative Stress
```rust
#[tokio::test]
async fn test_long_narrative_stress() {
    // Given: Very long narrative (100+ chunks)
    // When: Streaming audio
    // Then: SHOULD maintain continuous playback
    // And: SHOULD not have buffer underruns
    // And: SHOULD maintain quality
    // And: SHOULD not exhaust resources
}
```

### Error Handling Tests

#### Test: GPU Unavailable
```rust
#[tokio::test]
async fn test_gpu_unavailable() {
    // Given: No GPU available
    // When: Initializing TTS Service
    // Then: SHOULD fallback to CPU
    // And: SHOULD still generate audio
    // And: SHOULD adjust latency expectations
}
```

#### Test: GPU OOM Recovery
```rust
#[tokio::test]
async fn test_gpu_oom_recovery() {
    // Given: GPU memory pressure
    // When: Approaching VRAM limit
    // Then: SHOULD trigger cleanup
    // And: SHOULD free unused memory
    // And: SHOULD continue operation
}
```

### Compatibility Tests

#### Test: Platform Compatibility
```rust
#[tokio::test]
#[cfg(target_os = "windows")]
async fn test_windows_wasapi() {
    // Given: Windows platform
    // When: Initializing audio output
    // Then: SHOULD use WASAPI backend
    // And: SHOULD configure 256-512 frame buffer
    // And: SHOULD use int16 PCM format
}

#[tokio::test]
#[cfg(target_os = "macos")]
async fn test_macos_coreaudio() {
    // Given: macOS platform
    // When: Initializing audio output
    // Then: SHOULD use CoreAudio backend
    // And: SHOULD configure 256-512 frame buffer
    // And: SHOULD use int16 PCM format
}
```

## Test Execution Plan

### Pre-Commit Tests
- Run unit tests (fast, < 5s)
- Run format/lint checks
- Run basic integration tests

### Pre-Push Tests
- Run all unit tests
- Run integration tests
- Run performance tests (quick)

### Pre-Merge Tests
- Run all tests (unit, integration, performance, quality)
- Run production readiness tests
- Run compatibility tests

### Pre-Production Tests
- Run full test suite
- Run stress tests
- Run production scenarios
- Verify all metrics meet targets

## Test Metrics and Targets

### Coverage Targets
- Unit test coverage: 100%
- Integration test coverage: All critical paths
- Performance test coverage: All targets validated

### Performance Targets
- Initial latency: < 4.0s (all tiers)
- GPU utilization: Tier-dependent (30-95%)
- Real-time factor: < 1.0x (all tiers)
- Buffer underrun: 0
- Audio gaps: 0ms

### Quality Targets
- Audio quality: RAW preserved, no artifacts
- Chunk continuity: Zero gaps, seamless
- Format conversion: No quality loss
- User experience: "Critical Role AI" level

## Test Reporting

### Test Results Format
- Test name and status (PASS/FAIL)
- Execution time
- Coverage percentage
- Performance metrics
- Quality metrics

### Failure Reporting
- Detailed error messages
- Stack traces
- Performance deviations
- Quality issues

### Continuous Monitoring
- Test execution history
- Performance trends
- Quality trends
- Failure patterns



