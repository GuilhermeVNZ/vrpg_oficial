# Tasks: XTTS Real-Time Cinematic Streaming Implementation

## 1. Implementation Phase

### 1.1 AudioBuffer FIFO
- [ ] Create `audio_buffer.rs` module
- [ ] Implement thread-safe FIFO queue (Arc<Mutex<VecDeque>>)
- [ ] Implement `push()` method (non-blocking)
- [ ] Implement `pop_block()` method (blocking if empty)
- [ ] Implement `buffer_length_seconds()` calculation
- [ ] Implement overfill protection (max buffer size)
- [ ] Implement underrun handling (silence padding)
- [ ] Add unit tests (100% coverage)
- [ ] Add concurrent access tests (thread safety)

### 1.2 Semantic Chunker
- [ ] Create `semantic_chunker.rs` module
- [ ] Implement pause point detection (commas, "and", "as", "while", etc.)
- [ ] Implement chunk duration calculation (3-7s target)
- [ ] Implement character count calculation (180-320 chars target)
- [ ] Implement semantic boundary detection
- [ ] Implement chunk creation with min/max constraints (2.4-8.0s)
- [ ] Add unit tests for chunking rules
- [ ] Add tests for narrative flow preservation

### 1.3 Pre-Buffer Manager
- [ ] Create `prebuffer_manager.rs` module
- [ ] Implement buffer state tracking
- [ ] Implement `should_start_playback()` logic (> 2.0s)
- [ ] Implement `should_pause_playback()` logic (<= 1.5s)
- [ ] Implement playback state machine
- [ ] Add unit tests for state transitions

### 1.4 Native Audio Output
- [ ] Create `audio_output.rs` module
- [ ] Implement WASAPI backend (Windows)
- [ ] Implement ASIO backend (Windows, optional)
- [ ] Implement CoreAudio backend (macOS)
- [ ] Implement WebRTC Opus backend (Web)
- [ ] Implement async audio callbacks
- [ ] Implement buffer underrun handling
- [ ] Add platform detection and selection
- [ ] Add integration tests for each backend

### 1.5 XTTS Streaming Worker
- [ ] Create `xtts_streaming.rs` module
- [ ] Implement parallel CUDA stream support
- [ ] Implement chunk priority queue (N+1 high, N+2 low)
- [ ] Implement non-blocking chunk generation
- [ ] Implement Float32 precision enforcement
- [ ] Implement GPU memory management
- [ ] Add unit tests for parallel generation
- [ ] Add GPU utilization tests

### 1.6 Streaming Pipeline
- [ ] Create `streaming_pipeline.rs` module
- [ ] Implement thread architecture (A, B, C, D)
- [ ] Implement Qwen → Chunker → XTTS flow
- [ ] Implement XTTS → AudioBuffer flow
- [ ] Implement AudioBuffer → Audio Output flow
- [ ] Implement pre-buffer management integration
- [ ] Add end-to-end integration tests

### 1.7 Pipeline Integration
- [ ] Modify `pipeline.rs` to use streaming pipeline
- [ ] Separate Qwen threads (A, B) from XTTS thread (C)
- [ ] Separate Audio thread (D) from generation
- [ ] Implement non-blocking communication
- [ ] Add integration tests for full pipeline

## 2. Testing Phase

### 2.1 Unit Tests
- [ ] AudioBuffer FIFO operations (100% coverage)
- [ ] Semantic chunker boundary detection (100% coverage)
- [ ] Pre-buffer state management (100% coverage)
- [ ] Thread safety tests (concurrent access)

### 2.2 Integration Tests
- [ ] End-to-end streaming (Qwen → XTTS → Audio)
- [ ] Buffer underrun handling
- [ ] Chunk transition continuity
- [ ] Parallel XTTS generation
- [ ] Pre-buffer management

### 2.3 Performance Tests
- [ ] Initial latency measurement (target: 2.5-3.8s)
- [ ] Buffer underrun frequency (target: 0)
- [ ] GPU utilization (target: 80-95%)
- [ ] Real-time factor (target: < 0.5x)
- [ ] Audio gap measurement (target: 0ms)

### 2.4 Quality Tests
- [ ] Audio continuity verification (zero-gap)
- [ ] RAW audio quality preservation
- [ ] Float32 precision verification
- [ ] Chunk semantic coherence

## 3. Documentation Phase

### 3.1 Update Specifications
- [ ] Update `TTS_SERVICE_SPEC.md` with streaming requirements
- [ ] Document semantic chunking rules
- [ ] Document thread architecture
- [ ] Document performance targets

### 3.2 Update Architecture Docs
- [ ] Update `AUDIO_PIPELINE.md` with streaming architecture
- [ ] Document FIFO buffer design
- [ ] Document pre-buffer strategy
- [ ] Document native audio integration

### 3.3 Create Implementation Guide
- [ ] Document AudioBuffer usage
- [ ] Document semantic chunker configuration
- [ ] Document native audio backend selection
- [ ] Document troubleshooting guide

## 4. Optimization Phase

### 4.1 GPU Optimization
- [ ] Tune CUDA stream count for RTX 5090
- [ ] Optimize GPU memory allocation
- [ ] Measure and optimize GPU utilization
- [ ] Test with different chunk sizes

### 4.2 Buffer Optimization
- [ ] Tune pre-buffer size (1.5-3.0s range)
- [ ] Optimize FIFO queue size
- [ ] Test buffer underrun scenarios
- [ ] Optimize chunk size (3-7s range)

### 4.3 Latency Optimization
- [ ] Measure and optimize initial latency
- [ ] Optimize chunk generation order
- [ ] Test time-stretch optimization (optional)
- [ ] Fine-tune semantic chunking rules

## 5. Validation Phase

### 5.1 Performance Validation
- [ ] Verify initial latency < 3.8s
- [ ] Verify zero-gap playback
- [ ] Verify GPU utilization > 80%
- [ ] Verify buffer underrun = 0

### 5.2 Quality Validation
- [ ] Verify RAW audio quality preserved
- [ ] Verify chunk transitions seamless
- [ ] Verify semantic coherence maintained
- [ ] Verify "Critical Role AI" level achieved

### 5.3 User Acceptance
- [ ] Test with real gameplay scenarios
- [ ] Validate immersive experience
- [ ] Verify no perceptible delays
- [ ] Confirm continuous voice performance



