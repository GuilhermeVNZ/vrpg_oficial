# Tasks: XTTS Real-Time Streaming with Adaptive GPU Control (Complete)

## 1. Implementation Phase

### 1.1 GPU Capability Detection
- [ ] Create `gpu_detector.rs` module
- [ ] Implement GPU name detection (CUDA)
- [ ] Implement VRAM detection
- [ ] Implement compute capability detection
- [ ] Implement tier classification (High-End/Mid-Range/Modest/Low-End)
- [ ] Add unit tests for tier classification (100% coverage)
- [ ] Add tests for CPU fallback detection

### 1.2 Adaptive GPU Configuration
- [ ] Create `gpu_config.rs` module
- [ ] Implement `GpuConfig` struct with tier-based settings
- [ ] Implement parallel streams configuration (0-3 based on tier)
- [ ] Implement VRAM limit enforcement
- [ ] Implement GPU utilization target (30-95% based on tier)
- [ ] Implement pre-buffer size adaptation (0.5-3.0s based on tier)
- [ ] Add environment variable override support
- [ ] Add performance profile selection (high_performance/balanced/modest/auto)
- [ ] Add unit tests for config generation (100% coverage)

### 1.3 GPU Memory Management
- [ ] Create `gpu_memory.rs` module
- [ ] Implement VRAM usage tracking
- [ ] Implement VRAM limit enforcement
- [ ] Implement CUDA cache clearing
- [ ] Implement memory cleanup triggers
- [ ] Implement memory pressure detection
- [ ] Add unit tests for memory management (100% coverage)

### 1.4 AudioBuffer FIFO
- [ ] Create `audio_buffer.rs` module
- [ ] Implement thread-safe FIFO queue (Arc<Mutex<VecDeque>>)
- [ ] Implement Float32 internal storage (XTTS output)
- [ ] Implement int16 conversion for I/O
- [ ] Implement `push()` method (non-blocking)
- [ ] Implement `pop_block()` method (blocking if empty, returns int16)
- [ ] Implement `buffer_length_seconds()` calculation
- [ ] Implement overfill protection (max buffer size)
- [ ] Implement underrun handling (silence padding)
- [ ] Add unit tests (100% coverage)
- [ ] Add concurrent access tests (thread safety)
- [ ] Add format conversion tests (Float32 to int16)

### 1.5 Semantic Chunker
- [ ] Create `semantic_chunker.rs` module
- [ ] Implement pause point detection (commas, "and", "as", "while", "when", etc.)
- [ ] Implement chunk duration calculation (3-7s target)
- [ ] Implement character count calculation (180-320 chars target)
- [ ] Implement semantic boundary detection
- [ ] Implement chunk creation with min/max constraints (2.4-8.0s)
- [ ] Implement narrative flow preservation
- [ ] Add unit tests for chunking rules (100% coverage)
- [ ] Add tests for narrative flow preservation
- [ ] Add tests for edge cases (very short/long text)

### 1.6 Pre-Buffer Manager
- [ ] Create `prebuffer_manager.rs` module
- [ ] Implement buffer state tracking
- [ ] Implement tier-based threshold configuration
- [ ] Implement `should_start_playback()` logic (tier-dependent)
- [ ] Implement `should_pause_playback()` logic (tier-dependent)
- [ ] Implement playback state machine
- [ ] Add unit tests for state transitions (100% coverage)
- [ ] Add tests for tier-based thresholds

### 1.7 Native Audio Output
- [ ] Create `audio_output.rs` module
- [ ] Implement platform detection (Windows/macOS/Web)
- [ ] Implement WASAPI backend (Windows)
  - [ ] Configure buffer size 256-512 frames
  - [ ] Implement int16 PCM format
  - [ ] Implement async audio callbacks
  - [ ] Implement buffer underrun handling
- [ ] Implement ASIO backend (Windows, optional)
  - [ ] Configure buffer size 256-512 frames
  - [ ] Implement int16 PCM format
  - [ ] Implement ASIO buffer callbacks
- [ ] Implement CoreAudio backend (macOS)
  - [ ] Configure buffer size 256-512 frames
  - [ ] Implement int16 PCM format
  - [ ] Implement audio render callbacks
- [ ] Implement WebRTC Opus backend (Web)
  - [ ] Implement Opus encoding
  - [ ] Implement network streaming
- [ ] Implement dedicated audio I/O thread
- [ ] Add integration tests for each backend
- [ ] Add platform-specific tests

### 1.8 Audio Format Optimization
- [ ] Create `audio_format.rs` module
- [ ] Implement Float32 to int16 conversion
- [ ] Implement sample rate validation (16-24 kHz)
- [ ] Implement mono channel enforcement
- [ ] Implement format validation
- [ ] Implement resampling (if 16kHz target, downsample from 24kHz)
- [ ] Add unit tests for format conversion (100% coverage)
- [ ] Add tests for sample rate validation
- [ ] Add tests for channel count validation
- [ ] Add tests for resampling quality

### 1.9 XTTS Streaming Worker (Adaptive)
- [ ] Create `xtts_streaming.rs` module
- [ ] Implement GPU tier detection integration
- [ ] Implement adaptive parallel CUDA stream support (0-3 based on tier)
- [ ] Implement sequential inference for Modest/Low-End
- [ ] Implement chunk priority queue (N+1 high, N+2 low)
- [ ] Implement non-blocking chunk generation
- [ ] Implement Float32 precision enforcement (inference)
- [ ] Implement GPU memory management integration
- [ ] Implement GPU yield between chunks (Modest hardware)
- [ ] Implement chunk cancellation (for interrupt handling)
- [ ] Add unit tests for adaptive parallel generation
- [ ] Add GPU utilization tests
- [ ] Add tests for sequential vs parallel modes
- [ ] Add tests for chunk cancellation

### 1.10 Streaming Pipeline
- [ ] Create `streaming_pipeline.rs` module
- [ ] Implement thread architecture (A, B, C, D)
  - [ ] Thread A: Qwen 1.5B → Prelude
  - [ ] Thread B: Qwen 14B → Narrative
  - [ ] Thread C: XTTS Worker (adaptive)
  - [ ] Thread D: Audio Consumer (dedicated I/O)
- [ ] Implement Qwen → Chunker → XTTS flow
- [ ] Implement XTTS → AudioBuffer flow
- [ ] Implement AudioBuffer → Audio Output flow
- [ ] Implement pre-buffer management integration
- [ ] Implement GPU adaptive control integration
- [ ] Implement audio format optimization integration
- [ ] Implement streaming cancellation (hard cancel)
- [ ] Implement crossfade between chunks (10-50ms if needed)
- [ ] Add end-to-end integration tests

### 1.11 Pipeline Integration & Migration
- [ ] Remove SoVITS code from `pipeline.rs`
- [ ] Remove SoVITS module (`sovits.rs`) or mark as deprecated
- [ ] Update `pipeline.rs` to use streaming pipeline
- [ ] Separate Qwen threads (A, B) from XTTS thread (C)
- [ ] Separate Audio thread (D) from generation
- [ ] Implement non-blocking communication
- [ ] Integrate GPU adaptive control
- [ ] Integrate audio format optimization
- [ ] Update `voice_profiles.rs` to use XTTS embeddings (not SoVITS models)
- [ ] Update `server.rs` to remove SoVITS endpoints
- [ ] Update `lib.rs` to export new modules
- [ ] Update `metrics.rs` to track streaming metrics (not SoVITS)
- [ ] Add integration tests for full pipeline
- [ ] Add migration tests (verify old code removed)

### 1.12 Orchestrator Integration
- [ ] Create `orchestrator_integration.rs` module
- [ ] Implement non-blocking text input from Orchestrator
- [ ] Implement streaming cancellation on interrupt
- [ ] Implement status reporting to Orchestrator
- [ ] Implement error reporting to Orchestrator
- [ ] Add integration tests with Orchestrator mock

### 1.13 Configuration & Environment
- [ ] Create `config.rs` module
- [ ] Implement environment variable parsing
- [ ] Implement configuration file support (optional)
- [ ] Implement default configuration
- [ ] Implement configuration validation
- [ ] Add unit tests for configuration (100% coverage)

### 1.14 Logging & Monitoring
- [ ] Implement structured logging for streaming events
- [ ] Implement metrics collection (latency, RTF, GPU usage, buffer state)
- [ ] Implement performance monitoring
- [ ] Implement error tracking
- [ ] Add logging tests

### 1.15 Error Handling
- [ ] Implement streaming-specific error types
- [ ] Implement graceful degradation (GPU unavailable → CPU)
- [ ] Implement buffer underrun recovery
- [ ] Implement chunk generation failure handling
- [ ] Implement audio output failure handling
- [ ] Add error handling tests

## 2. Testing Phase

### 2.1 Unit Tests

#### 2.1.1 GPU Detection Tests
- [ ] Test GPU name detection
- [ ] Test VRAM detection
- [ ] Test compute capability detection
- [ ] Test tier classification (all tiers)
- [ ] Test CPU fallback detection
- [ ] Test edge cases (no GPU, invalid GPU)

#### 2.1.2 GPU Configuration Tests
- [ ] Test config generation for High-End tier
- [ ] Test config generation for Mid-Range tier
- [ ] Test config generation for Modest tier
- [ ] Test config generation for Low-End tier
- [ ] Test environment variable override
- [ ] Test performance profile selection
- [ ] Test invalid configuration handling

#### 2.1.3 GPU Memory Tests
- [ ] Test VRAM usage tracking
- [ ] Test VRAM limit enforcement
- [ ] Test CUDA cache clearing
- [ ] Test memory cleanup triggers
- [ ] Test memory pressure detection

#### 2.1.4 AudioBuffer Tests
- [ ] Test FIFO push operation (100% coverage)
- [ ] Test FIFO pop operation (100% coverage)
- [ ] Test buffer length calculation
- [ ] Test overfill protection
- [ ] Test underrun handling
- [ ] Test Float32 to int16 conversion
- [ ] Test concurrent access (thread safety)
- [ ] Test empty buffer handling
- [ ] Test full buffer handling

#### 2.1.5 Semantic Chunker Tests
- [ ] Test pause point detection (100% coverage)
- [ ] Test chunk duration calculation
- [ ] Test character count calculation
- [ ] Test semantic boundary detection
- [ ] Test min/max constraints (2.4-8.0s)
- [ ] Test narrative flow preservation
- [ ] Test edge cases (very short text, very long text)
- [ ] Test multiple pause points

#### 2.1.6 Pre-Buffer Manager Tests
- [ ] Test buffer state tracking (100% coverage)
- [ ] Test tier-based threshold configuration
- [ ] Test `should_start_playback()` for all tiers
- [ ] Test `should_pause_playback()` for all tiers
- [ ] Test playback state machine transitions
- [ ] Test edge cases (buffer exactly at threshold)

#### 2.1.7 Audio Format Tests
- [ ] Test Float32 to int16 conversion (100% coverage)
- [ ] Test sample rate validation (16-24 kHz)
- [ ] Test invalid sample rate rejection (48 kHz)
- [ ] Test mono channel enforcement
- [ ] Test stereo to mono conversion
- [ ] Test format validation
- [ ] Test resampling quality (24kHz → 16kHz)

#### 2.1.8 XTTS Streaming Worker Tests
- [ ] Test adaptive parallel generation (High-End)
- [ ] Test sequential generation (Modest)
- [ ] Test chunk priority queue
- [ ] Test non-blocking generation
- [ ] Test Float32 precision enforcement
- [ ] Test GPU yield between chunks
- [ ] Test GPU memory management integration
- [ ] Test chunk cancellation

### 2.2 Integration Tests

#### 2.2.1 End-to-End Streaming
- [ ] Test full pipeline: Qwen → Chunker → XTTS → AudioBuffer → Audio Output
- [ ] Test with High-End GPU configuration
- [ ] Test with Modest GPU configuration
- [ ] Test with CPU fallback
- [ ] Test long narrative streaming (10+ chunks)
- [ ] Test short text streaming (1-2 chunks)

#### 2.2.2 Buffer Management
- [ ] Test buffer underrun handling
- [ ] Test buffer overfill protection
- [ ] Test pre-buffer initialization
- [ ] Test continuous pre-buffering
- [ ] Test buffer state transitions

#### 2.2.3 Chunk Continuity
- [ ] Test seamless chunk transitions
- [ ] Test zero-gap playback
- [ ] Test audio continuity between chunks
- [ ] Test crossfade application (if needed)
- [ ] Test DC offset handling

#### 2.2.4 GPU Adaptive Control
- [ ] Test High-End GPU parallel inference
- [ ] Test Modest GPU sequential inference
- [ ] Test GPU memory limit enforcement
- [ ] Test GPU yield between chunks
- [ ] Test GPU utilization monitoring
- [ ] Test automatic tier detection and configuration

#### 2.2.5 Audio Format Integration
- [ ] Test Float32 internal processing
- [ ] Test int16 I/O format
- [ ] Test sample rate handling (24 kHz)
- [ ] Test mono channel output
- [ ] Test format conversion pipeline
- [ ] Test resampling (if 16kHz target)

#### 2.2.6 Thread Architecture
- [ ] Test thread isolation (A, B, C, D)
- [ ] Test non-blocking communication
- [ ] Test dedicated audio I/O thread
- [ ] Test thread synchronization
- [ ] Test thread safety under load

#### 2.2.7 Streaming Cancellation
- [ ] Test hard cancel during generation
- [ ] Test cancel during playback
- [ ] Test AudioBuffer FIFO clearing on cancel
- [ ] Test chunk cancellation in XTTS Worker
- [ ] Test graceful stop of audio output

#### 2.2.8 Orchestrator Integration
- [ ] Test non-blocking text input
- [ ] Test streaming cancellation on interrupt
- [ ] Test status reporting
- [ ] Test error reporting

### 2.3 Performance Tests

#### 2.3.1 Latency Tests
- [ ] Measure initial latency (target: 2.5-4.0s all tiers)
- [ ] Measure chunk generation latency
- [ ] Measure audio I/O latency
- [ ] Measure total pipeline latency
- [ ] Test latency consistency (multiple runs)
- [ ] Test latency under load

#### 2.3.2 Buffer Performance
- [ ] Measure buffer underrun frequency (target: 0)
- [ ] Measure buffer fill rate
- [ ] Measure buffer consumption rate
- [ ] Test buffer stability under load
- [ ] Test buffer recovery from underrun

#### 2.3.3 GPU Performance
- [ ] Measure GPU utilization (target: tier-dependent)
  - [ ] High-End: 80-95%
  - [ ] Mid-Range: 60-80%
  - [ ] Modest: 40-60%
  - [ ] Low-End: 30-50%
- [ ] Measure VRAM usage
- [ ] Measure real-time factor (target: < 0.5x High-End, < 1.0x Low-End)
- [ ] Test GPU performance under load
- [ ] Test GPU memory pressure handling

#### 2.3.4 Audio Performance
- [ ] Measure audio gap frequency (target: 0)
- [ ] Measure audio continuity
- [ ] Measure format conversion overhead
- [ ] Test audio performance under load
- [ ] Test audio quality preservation

#### 2.3.5 System Performance
- [ ] Measure CPU usage (should not spike)
- [ ] Measure memory usage
- [ ] Test system responsiveness (no lag)
- [ ] Test concurrent operations (UI, other services)
- [ ] Test resource cleanup

### 2.4 Quality Tests

#### 2.4.1 Audio Quality
- [ ] Verify RAW audio quality preserved
- [ ] Verify no metallic artifacts
- [ ] Verify no distortion
- [ ] Verify natural voice quality
- [ ] Compare with baseline (RAW XTTS output)

#### 2.4.2 Chunk Quality
- [ ] Verify chunk transitions seamless
- [ ] Verify semantic coherence maintained
- [ ] Verify narrative flow preserved
- [ ] Verify no incomplete words
- [ ] Verify no duplicate audio

#### 2.4.3 Format Quality
- [ ] Verify int16 conversion quality (no artifacts)
- [ ] Verify sample rate consistency (24 kHz)
- [ ] Verify mono channel output
- [ ] Verify format compatibility (Opus, etc.)

#### 2.4.4 User Experience
- [ ] Verify "Critical Role AI" level performance
- [ ] Verify immersive experience
- [ ] Verify no perceptible delays
- [ ] Verify continuous voice (no breaks)
- [ ] Verify system responsiveness (no lag)

### 2.5 Benchmark Tests

#### 2.5.1 Pipeline Configuration Benchmark
- [ ] Test High-End configurations (3 streams, 2 streams, 1 stream)
- [ ] Test Mid-Range configurations
- [ ] Test Modest configurations
- [ ] Test different buffer sizes (256, 384, 512 frames)
- [ ] Test different pre-buffer sizes (0.5s - 3.0s)
- [ ] Test different sample rates (16kHz, 24kHz)
- [ ] Test time-stretch optimization
- [ ] Compare all configurations
- [ ] Identify best configuration for lowest latency
- [ ] Generate detailed benchmark report

#### 2.5.2 Latency Optimization Benchmark
- [ ] Measure initial latency for each configuration
- [ ] Measure chunk generation latency
- [ ] Measure total pipeline latency
- [ ] Identify configuration with lowest latency
- [ ] Verify latency targets are met
- [ ] Generate latency comparison report

#### 2.5.3 GPU Utilization Benchmark
- [ ] Measure GPU utilization for each configuration
- [ ] Measure VRAM usage
- [ ] Measure real-time factor
- [ ] Identify optimal GPU configuration
- [ ] Verify utilization targets are met
- [ ] Generate GPU utilization report

### 2.6 Production Readiness Tests

#### 2.6.1 Stress Tests
- [ ] Test with very long narratives (100+ chunks)
- [ ] Test with rapid successive requests
- [ ] Test with concurrent users (if applicable)
- [ ] Test with low system resources
- [ ] Test with network interruptions (Web backend)

#### 2.6.2 Error Handling Tests
- [ ] Test GPU unavailable scenario
- [ ] Test GPU OOM scenario
- [ ] Test buffer underrun recovery
- [ ] Test invalid input handling
- [ ] Test service restart/recovery
- [ ] Test streaming cancellation under various conditions

#### 2.6.3 Compatibility Tests
- [ ] Test on Windows (WASAPI)
- [ ] Test on Windows with ASIO
- [ ] Test on macOS (CoreAudio)
- [ ] Test on Web (WebRTC Opus)
- [ ] Test with different GPU tiers
- [ ] Test with CPU-only mode

#### 2.6.4 Regression Tests
- [ ] Test existing functionality still works
- [ ] Test backward compatibility (if any)
- [ ] Test migration from old pipeline
- [ ] Test configuration migration

## 3. Documentation Phase

### 3.1 Update Specifications
- [ ] Update `TTS_SERVICE_SPEC.md` with streaming requirements
- [ ] Document semantic chunking rules
- [ ] Document thread architecture
- [ ] Document performance targets
- [ ] Document GPU adaptive control
- [ ] Document audio format optimization

### 3.2 Update Architecture Docs
- [ ] Update `AUDIO_PIPELINE.md` with streaming architecture
- [ ] Document FIFO buffer design
- [ ] Document pre-buffer strategy
- [ ] Document native audio integration
- [ ] Document GPU adaptive control
- [ ] Document audio format optimization

### 3.3 Create Implementation Guides
- [ ] Document AudioBuffer usage
- [ ] Document semantic chunker configuration
- [ ] Document native audio backend selection
- [ ] Document GPU configuration (tiers, profiles)
- [ ] Document environment variables
- [ ] Document troubleshooting guide
- [ ] Document performance tuning guide

### 3.4 Create User Documentation
- [ ] Document streaming features
- [ ] Document GPU requirements
- [ ] Document performance profiles
- [ ] Document configuration options
- [ ] Document troubleshooting common issues

### 3.5 Migration Documentation
- [ ] Document SoVITS removal
- [ ] Document XTTS embedding migration
- [ ] Document API changes
- [ ] Document breaking changes
- [ ] Create migration guide

## 4. Code Cleanup Phase

### 4.1 Remove SoVITS Code
- [ ] Remove SoVITS module (`sovits.rs`) or mark as deprecated
- [ ] Remove SoVITS references from `pipeline.rs`
- [ ] Remove SoVITS references from `voice_profiles.rs`
- [ ] Remove SoVITS references from `server.rs`
- [ ] Remove SoVITS references from `lib.rs`
- [ ] Remove SoVITS references from `metrics.rs`
- [ ] Remove SoVITS tests
- [ ] Remove SoVITS documentation files
- [ ] Verify no SoVITS code remains

### 4.2 Update Dependencies
- [ ] Remove SoVITS Python dependencies (if any)
- [ ] Update Cargo.toml dependencies
- [ ] Add native audio dependencies (cpal, rodio, or similar)
- [ ] Update Python bridge (if still needed for XTTS)
- [ ] Verify all dependencies are correct

### 4.3 Update Build Configuration
- [ ] Update build scripts
- [ ] Update CI/CD configuration
- [ ] Update Docker configuration (if applicable)
- [ ] Update environment variable documentation

## 5. Optimization Phase

### 5.1 GPU Optimization
- [ ] Tune CUDA stream count for each tier
- [ ] Optimize GPU memory allocation
- [ ] Measure and optimize GPU utilization
- [ ] Test with different chunk sizes
- [ ] Optimize GPU yield timing (Modest hardware)

### 5.2 Buffer Optimization
- [ ] Tune pre-buffer size per tier (0.5-3.0s range)
- [ ] Optimize FIFO queue size
- [ ] Test buffer underrun scenarios
- [ ] Optimize chunk size (3-7s range)
- [ ] Optimize buffer thresholds

### 5.3 Latency Optimization
- [ ] Measure and optimize initial latency
- [ ] Optimize chunk generation order
- [ ] Test time-stretch optimization (optional)
- [ ] Fine-tune semantic chunking rules
- [ ] Optimize audio I/O latency

### 5.4 Format Optimization
- [ ] Optimize Float32 to int16 conversion
- [ ] Test different buffer sizes (256/384/512)
- [ ] Optimize sample rate handling
- [ ] Optimize channel processing
- [ ] Optimize resampling (if needed)

## 6. Validation Phase

### 6.1 Performance Validation
- [ ] Verify initial latency < 4.0s (all tiers)
- [ ] Verify zero-gap playback
- [ ] Verify GPU utilization targets met (per tier)
- [ ] Verify buffer underrun = 0
- [ ] Verify real-time factor targets met

### 6.2 Quality Validation
- [ ] Verify RAW audio quality preserved
- [ ] Verify chunk transitions seamless
- [ ] Verify semantic coherence maintained
- [ ] Verify format conversion quality
- [ ] Verify "Critical Role AI" level achieved

### 6.3 System Validation
- [ ] Verify system responsiveness (no lag)
- [ ] Verify resource usage acceptable
- [ ] Verify error handling robust
- [ ] Verify compatibility across platforms
- [ ] Verify production readiness

### 6.4 Migration Validation
- [ ] Verify SoVITS code completely removed
- [ ] Verify XTTS embeddings working
- [ ] Verify no breaking changes for users
- [ ] Verify backward compatibility (if applicable)

### 6.5 User Acceptance
- [ ] Test with real gameplay scenarios
- [ ] Validate immersive experience
- [ ] Verify no perceptible delays
- [ ] Confirm continuous voice performance
- [ ] Gather user feedback

## 7. Production Deployment

### 7.1 Pre-Deployment
- [ ] Code review completed
- [ ] All tests passing (100% pass rate)
- [ ] Coverage check (95%+ required)
- [ ] Performance benchmarks met
- [ ] Documentation complete
- [ ] Migration plan ready
- [ ] SoVITS code removal verified

### 7.2 Deployment
- [ ] Deploy to staging environment
- [ ] Run smoke tests
- [ ] Monitor performance metrics
- [ ] Verify GPU adaptive control working
- [ ] Verify audio streaming working
- [ ] Verify SoVITS removal complete
- [ ] Deploy to production

### 7.3 Post-Deployment
- [ ] Monitor production metrics
- [ ] Monitor error rates
- [ ] Monitor performance
- [ ] Gather user feedback
- [ ] Plan improvements

## 8. Additional Critical Tasks

### 8.1 Crossfade Implementation
- [ ] Implement crossfade between chunks (10-50ms)
- [ ] Test crossfade quality (no artifacts)
- [ ] Make crossfade optional/configurable
- [ ] Add unit tests for crossfade

### 8.2 Resampling Support
- [ ] Implement resampling from 24kHz to 16kHz (if needed)
- [ ] Use high-quality resampling algorithm
- [ ] Test resampling quality (no artifacts)
- [ ] Add unit tests for resampling

### 8.3 Chunk Cancellation
- [ ] Implement chunk cancellation in XTTS Worker
- [ ] Implement AudioBuffer clearing on cancel
- [ ] Implement graceful audio stop
- [ ] Test cancellation under various conditions
- [ ] Add unit tests for cancellation

### 8.4 Metrics & Monitoring
- [ ] Implement streaming-specific metrics
- [ ] Track latency per tier
- [ ] Track GPU utilization
- [ ] Track buffer state
- [ ] Track chunk generation times
- [ ] Implement metrics export

### 8.5 Configuration Management
- [ ] Implement configuration file support
- [ ] Implement runtime configuration updates
- [ ] Implement configuration validation
- [ ] Add configuration tests

### 8.6 Error Recovery
- [ ] Implement automatic GPU fallback
- [ ] Implement buffer underrun recovery
- [ ] Implement chunk generation retry
- [ ] Implement graceful degradation
- [ ] Add error recovery tests



