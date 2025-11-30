# VRPG Architecture Overview

## System Architecture

VRPG is built as a modular, local-first system with the following core components:

### Core Services

1. **Orchestrator**: Coordinates all services, manages game state, routes INTENTs
2. **LLM Core**: Qwen 1.5B (prelude) + Qwen 14B (narrative)
3. **TTS Service**: XTTS (Coqui) with real-time streaming
4. **ASR Service**: Whisper for voice transcription
5. **Rules5e Service**: D&D 5e rules engine
6. **Memory Service**: Vectorizer for semantic search
7. **Game Engine**: Turn-based combat and state management

## Audio Pipeline Architecture

### Streaming Real-Time Cinematic Pipeline

The audio pipeline uses a **4-thread architecture** with **adaptive TTS performance profiles** for real-time streaming:

```
Thread A: Qwen 1.5B → Prelude text → FAST Profile Chunker (30 chars first)
Thread B: Qwen 14B → Full narrative → CINEMATIC Profile Chunker (100 chars first)
Thread C: XTTS Worker → Generate chunks (FP16, adaptive parallel) → AudioBuffer FIFO (50-80ms blocks)
Thread D: Audio Consumer → AudioBuffer FIFO → Native Audio Output (WASAPI/ASIO/CoreAudio)
```

### TTS Performance Profiles

The system implements two distinct performance profiles to optimize latency for different LLM models:

#### FAST Profile (Qwen 1.5B)

**Purpose**: Ultra-low latency for quick emotional preludes

- **First chunk**: 30 characters maximum (~0.7-1.0s of speech)
- **Subsequent chunks**: 90 characters maximum (~2-3s of speech)
- **Sample rate**: 16 kHz (mono) - reduces computational cost
- **Precision**: FP16 (half precision) - 30-40% faster inference
- **Audio blocks**: 50ms (800 samples @ 16 kHz) for FIFO streaming
- **Pre-buffer**: 240ms (3840 samples) before playback start
- **Target latency**: ≤ 0.8s `time_to_first_audio` (ideal 0.5-0.7s)
- **Chunking strategy**: Speed priority - can cut mid-word/mid-sentence for first chunk

#### CINEMATIC Profile (Qwen 14B)

**Purpose**: Balanced latency and quality for full narratives

- **First chunk**: 100 characters maximum (~3s of speech)
- **Subsequent chunks**: 150 characters maximum (~4-5s of speech)
- **Sample rate**: 24 kHz (mono) - better fidelity
- **Precision**: FP16 (half precision) - same speed optimization
- **Audio blocks**: 60-80ms (1440-1920 samples @ 24 kHz) for FIFO streaming
- **Pre-buffer**: 500ms (12000 samples) before playback start
- **Target latency**: 1.5-3s `time_to_first_audio`
- **Chunking strategy**: Quality priority - prefers punctuation/natural pauses when possible

### Components

1. **TTS Performance Profiles**: Auto-selection based on LLM model name (FAST for 1.5B, CINEMATIC for 14B)
2. **Profile-Aware Chunker**: Adapts chunking strategy based on selected profile (tiny first chunk for FAST, moderate for CINEMATIC)
3. **AudioBuffer FIFO**: Thread-safe queue with Float32 internal format, int16 I/O, streaming with 50-80ms blocks (not batch processing)
4. **Pre-Buffer Manager**: Profile-specific pre-buffering thresholds (240ms FAST, 500ms CINEMATIC)
5. **XTTS Streaming Worker**: FP16 precision, profile-specific sample rates (16 kHz FAST, 24 kHz CINEMATIC), adaptive parallelization, warm-up on initialization
6. **Native Audio Output**: WASAPI/ASIO/CoreAudio, dedicated I/O thread, isolated from model inference

### GPU Adaptive Control

- **Auto-detection**: High-End/Mid-Range/Modest/Low-End based on VRAM and compute capability
- **Adaptive configuration**: 
  - **High-End** (RTX 5090): 2-3 parallel CUDA streams, no VRAM limits, 2.5s pre-buffer
  - **Mid-Range** (RTX 3070): 1-2 parallel streams, moderate VRAM limits, 1.75s pre-buffer
  - **Modest** (RTX 3050): 1 sequential stream, 3GB VRAM limit, 1.25s pre-buffer
  - **Low-End** (< 4GB): 0-1 stream, strict VRAM limits, 0.75s pre-buffer
- **Performance maintained**: Profile-specific latency targets (≤ 0.8s FAST, 1.5-3s CINEMATIC), zero-gap playback

### Interjection System

**Purpose**: Mask TTS latency for long responses with pre-recorded interjection clips that create a natural "thinking" delay.

**Behavior**:
- **Detection**: Uses heuristic `expected_duration = text_length_chars / 25.0`
- **Threshold**: 3.0s for CINEMATIC, 4.0s for FAST (more conservative)
- **Delay**: Natural 1.5s delay from user speech end to interjection start
- **Selection**: Avoids repeating last 5 interjections used
- **Playback**: Interjection → Gap (50ms) → TTS Principal (sequential)

**Assets**:
- **Location**: `assets-and-models/voices/interjections/`
- **Total**: 53 interjection clips (WAV, Float32, 24kHz mono)
- **Categories**: Short interjections (Hmm, Ah, etc.), short phrases ("Got it", "I understand"), non-verbal sounds

**Integration**:
- Interjections play automatically for long responses (>3s expected duration)
- TTS generation runs in parallel (doesn't wait for interjection)
- Creates seamless experience: no "cognitive silence" during processing

### Audio Optimizations

#### Format and Quality
- **Sample rate**: 16 kHz (FAST profile) or 24 kHz (CINEMATIC profile), NOT 48 kHz (wasteful for voice)
- **Channels**: Mono (1 channel, NOT stereo - 50% bandwidth reduction)
- **Internal format**: Float32 (preserves XTTS quality, no quantization artifacts)
- **I/O format**: int16 PCM (efficient, compatible with Opus, 50% bandwidth reduction vs Float32)
- **Precision**: FP16 (half precision) for XTTS inference - 30-40% faster, no quality loss for voice

#### Streaming Configuration
- **Audio blocks**: 50ms (FAST, 800 samples @ 16 kHz) or 60-80ms (CINEMATIC, 1440-1920 samples @ 24 kHz)
- **Buffer size**: 256-512 frames per buffer (low latency, NOT 2048/4096)
- **Pre-buffer**: 240ms (FAST, 3840 samples) or 500ms (CINEMATIC, 12000 samples) before playback start
- **Streaming mode**: Real-time FIFO (blocks pushed immediately), NOT batch processing

#### XTTS Optimizations
- **Model warm-up**: One short inference on service initialization to "compile" CUDA kernels
- **FP16 inference**: `model.half().to("cuda")` with `torch.cuda.amp.autocast(device_type="cuda")`
- **Inference mode**: `torch.inference_mode()` for maximum performance
- **Sample rate per profile**: Generate directly at target rate (16 kHz FAST, 24 kHz CINEMATIC), no resampling

## Communication Architecture

### IPC Protocol

Services communicate via:
- **HTTP REST**: For service-to-service calls
- **WebSocket**: For real-time UI updates
- **Message Queue**: For async operations

### Data Flow

```
Player Voice → Whisper (ASR) → Orchestrator
    ↓
Orchestrator → Qwen 1.5B (prelude) → FAST Profile TTS (≤ 0.8s latency)
    ↓
    Chunker FAST (30 chars first) → XTTS (16 kHz, FP16) → FIFO (50ms blocks)
    ↓
    AudioBuffer FIFO → Native Audio Output → Player hears prelude
    ↓
Orchestrator → Qwen 14B (narrative) → CINEMATIC Profile TTS (1.5-3s latency)
    ↓
    Chunker CINEMATIC (100 chars first) → XTTS (24 kHz, FP16) → FIFO (60-80ms blocks)
    ↓
    AudioBuffer FIFO → Native Audio Output → Player hears narrative
    ↓
Player hears continuous voice (zero gaps after initial start)
```

**Key Characteristics**:
- **Real-time streaming**: Blocks pushed immediately to FIFO (not batch processing)
- **Profile auto-selection**: Based on LLM model name in request
- **Zero-gap playback**: Continuous audio after pre-buffer threshold met
- **Adaptive optimization**: FP16, sample rate, and chunking strategy per profile

## State Management

### Game State

- **Combat State**: Turn order, HP, conditions, positions
- **Scene State**: SocialFreeFlow, Exploration, CombatTurnBased, DowntimePreparation
- **Player State**: Inventory, spells, resources, stats

### Memory

- **Episodic Memory**: Vectorizer for semantic search
- **Lore Memory**: D&D 5e books indexed
- **Session Memory**: Recent events, context

## Performance Targets

### Latency Metrics

The system measures and targets specific latency metrics for each profile:

#### FAST Profile (Qwen 1.5B)
- **time_to_first_audio**: ≤ 0.8s (ideal 0.5-0.7s)
  - Time from text ready to first audio block in device
- **first_chunk_audio_duration**: ~0.7-1.0s (30 chars → audio)
- **xtts_first_chunk_infer_time**: < 0.8s (RTF < 1.0x)
- **Streaming**: Continuous, zero-gap playback after initial start

#### CINEMATIC Profile (Qwen 14B)
- **time_to_first_audio**: 1.5-3s
  - Time from text ready to first audio block in device
- **first_chunk_audio_duration**: ~3s (100 chars → audio)
- **xtts_first_chunk_infer_time**: < 3s (RTF < 1.0x)
- **Streaming**: Continuous, zero-gap playback after initial start

### Performance Characteristics

- **Real-time factor**: < 1.0x (all profiles, all tiers) - XTTS generates faster than real-time
- **First chunk size**: 30 chars (FAST) or 100 chars (CINEMATIC) - optimized for latency
- **Chunking strategy**: Profile-aware (speed priority FAST, quality priority CINEMATIC)
- **Streaming mode**: Real-time FIFO with small blocks (50-80ms), NOT batch processing
- **Zero gaps**: Continuous playback after pre-buffer threshold met

### GPU Utilization

- **High-End**: 80-95% (2-3 parallel streams)
- **Mid-Range**: 60-80% (1-2 streams)
- **Modest**: 40-60% (1 sequential stream)
- **Low-End**: 30-50% (0-1 stream)

## Implementation Details

### Profile Auto-Selection

The system automatically selects the appropriate TTS profile based on the LLM model name:

```rust
// Auto-selection logic
if llm_model_name.contains("1.5") || llm_model_name.contains("1_5") {
    TtsProfile::fast()  // ≤ 0.8s latency target
} else if llm_model_name.contains("14") || llm_model_name.contains("14b") {
    TtsProfile::cinematic()  // 1.5-3s latency target
} else {
    TtsProfile::cinematic()  // Default to cinematic
}
```

### Chunking Algorithm

The chunker adapts its strategy based on the selected profile:

- **FAST Profile**: First chunk limited to 30 chars (can cut mid-word), subsequent chunks 90 chars
- **CINEMATIC Profile**: First chunk limited to 100 chars (prefers punctuation), subsequent chunks 150 chars
- **Word-based splitting**: Splits by words, not sentences, for fine-grained control
- **Dynamic limits**: First chunk uses `first_chunk_max_chars`, subsequent chunks use `next_chunk_max_chars`

### FIFO Streaming Architecture

The streaming pipeline uses a producer-consumer pattern with FIFO queue:

1. **Producer (TTS Generation)**: Generates audio chunks, splits into 50-80ms blocks, pushes immediately to FIFO
2. **Consumer (Audio Playback)**: Waits for pre-buffer threshold, then consumes blocks continuously
3. **Non-blocking**: Producer never waits for playback, consumer never blocks generation
4. **Real-time**: Blocks pushed as soon as generated (not batch processing)

## References

- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Detailed pipeline architecture
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Orchestrator specification
- [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md) - Audio pipeline details
- [TTS_PROFILES_STRATEGY.md](TTS_PROFILES_STRATEGY.md) - TTS performance profiles strategy and implementation
- [STREAMING_REALTIME.md](STREAMING_REALTIME.md) - Streaming real-time documentation
