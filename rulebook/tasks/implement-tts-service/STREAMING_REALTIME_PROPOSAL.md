# Proposal: XTTS Real-Time Cinematic Streaming

## Why

The current XTTS streaming implementation uses a naive "generate → play → wait" approach that creates "popcorn" audio with 2-5 second gaps between chunks. This breaks immersion and does not achieve the "Critical Role AI" level performance required for VRPG.

The current implementation:
- Chunks by sentences (wrong approach)
- Generates sequentially (wastes GPU power)
- Uses Python audio libraries (introduces gaps)
- Has no pre-buffering (causes interruptions)
- Blocks generation on playback (inefficient)

We need real-time cinematic streaming that:
- Starts audio in 2.5-3.8 seconds
- Maintains continuous playback without gaps
- Uses semantic chunking (not sentence-based)
- Parallelizes XTTS inference (maximize GPU)
- Pre-buffers 1-2 chunks ahead
- Uses native audio frameworks (WASAPI/ASIO/CoreAudio)

## What Changes

### New Components

1. **AudioBuffer FIFO** (`audio_buffer.rs`)
   - Thread-safe FIFO queue for PCM float32 chunks
   - Decouples generation from playback
   - Overfill protection
   - Underrun handling

2. **Semantic Chunker** (`semantic_chunker.rs`)
   - Chunks text by semantic pauses (commas, "and", "as", etc.)
   - Target: 3-7 seconds per chunk (180-320 chars)
   - Respects narrative flow
   - Avoids mid-phrase cuts

3. **Pre-Buffer Manager** (`prebuffer_manager.rs`)
   - Manages buffer state (start/pause playback)
   - Ensures 1-2 chunks ahead of playback
   - Handles buffer underrun

4. **XTTS Streaming Worker** (`xtts_streaming.rs`)
   - Parallel chunk generation (multiple CUDA streams)
   - Priority-based scheduling (N+1 high, N+2 low)
   - Non-blocking push to AudioBuffer

5. **Native Audio Output** (`audio_output.rs`)
   - WASAPI (Windows)
   - ASIO (Windows Pro Audio)
   - CoreAudio (macOS)
   - WebRTC Opus (Web)
   - Replaces Python audio libraries

6. **Streaming Pipeline** (`streaming_pipeline.rs`)
   - Orchestrates all components
   - Manages thread architecture
   - Coordinates Qwen → XTTS → Audio flow

### Modified Components

1. **Pipeline Architecture** (`pipeline.rs`)
   - Separate threads: Qwen (A), Qwen 14B (B), XTTS (C), Audio (D)
   - Non-blocking communication
   - Parallel inference support

2. **XTTS Integration** (`xtts.rs`)
   - Float32 precision (no FP16/BF16/INT8)
   - Parallel CUDA stream support
   - Chunk-based generation API

### Removed Components

1. **Python Audio Libraries**
   - Remove sounddevice dependency
   - Remove pygame.audio usage
   - Replace with native frameworks

2. **Sentence-Based Chunking**
   - Remove simple sentence splitting
   - Replace with semantic chunking

## Impact

- **Affected specs**: `TTS_SERVICE_SPEC.md` (Audio Streaming requirement)
- **Affected code**: 
  - `src/tts-service/src/pipeline.rs` (thread architecture)
  - `src/tts-service/src/xtts.rs` (parallel inference)
  - New files: `audio_buffer.rs`, `semantic_chunker.rs`, `prebuffer_manager.rs`, `xtts_streaming.rs`, `audio_output.rs`, `streaming_pipeline.rs`
- **Breaking change**: YES (streaming API changes)
- **User benefit**: 
  - 2.5-3.8s initial latency (vs 10-20s current)
  - Continuous voice without gaps
  - "Critical Role AI" level performance
  - Better GPU utilization (80-95% vs 5% current)

## Technical Decisions

### Why FIFO Buffer?
Decouples generation from playback, enabling parallel inference and pre-buffering.

### Why Semantic Chunking?
Sentence-based chunking creates unnatural pauses. Semantic chunking respects narrative flow and creates cinematic pauses.

### Why Native Audio?
Python audio libraries introduce gaps and latency. Native frameworks (WASAPI/ASIO) provide zero-gap playback and proper buffer management.

### Why Parallel Inference?
RTX 5090 can handle multiple CUDA streams. Serial generation wastes 95% of GPU power. Parallel generation maximizes throughput.

### Why Float32?
Half precision (FP16/BF16) causes metallic artifacts. INT8 is unplayable. Float32 preserves RAW quality (infinitely better).

### Why Pre-Buffer 2 Chunks?
Ensures continuous playback. If generation slows, buffer provides safety margin. Never start playback with < 2s buffer.

## Performance Targets

- **Initial latency**: 2.5-3.8 seconds
- **Streaming continuity**: Zero gaps
- **Buffer stability**: +1.5-3.0 seconds ahead
- **GPU utilization**: 80-95%
- **Real-time factor**: < 0.5x (generation faster than playback)

## Success Criteria

✅ Audio starts in 2.5-3.8 seconds  
✅ Continuous playback without gaps  
✅ No buffer underruns  
✅ GPU utilization > 80%  
✅ "Critical Role AI" level performance  
✅ Player does not perceive delays or breaks



