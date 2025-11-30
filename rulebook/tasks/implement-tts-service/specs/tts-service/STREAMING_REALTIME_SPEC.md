# XTTS Real-Time Cinematic Streaming Specification

## Purpose

This specification defines the technical requirements for implementing real-time cinematic audio streaming using XTTS (Coqui) with zero-gap playback, semantic chunking, and parallel inference pipeline. The goal is to achieve "Critical Role AI" level voice performance with 2.5-4.0s initial latency and continuous voice without breaks.

## ADDED Requirements

### Requirement: Real-Time Audio FIFO Buffer

The TTS Service SHALL implement a thread-safe FIFO (First-In-First-Out) audio buffer that decouples audio generation from audio playback, ensuring continuous playback without gaps or interruptions.

#### Scenario: Audio Buffer Initialization
Given TTS Service initialization
When the service starts
Then the TTS Service SHALL create an AudioBufferThread with FIFO queue
And the TTS Service SHALL initialize queue with capacity for 3-5 seconds of audio
And the TTS Service SHALL use 16-bit integer PCM format (int16) for I/O
And the TTS Service SHALL use Float32 internally for processing (XTTS output)
And the TTS Service SHALL convert Float32 to int16 before audio I/O
And the TTS Service SHALL set frame size to 256-512 samples per frame (low latency)
And the TTS Service SHALL NOT use large buffers (2048/4096) that increase lag
And the TTS Service SHALL implement overfill protection (max buffer size)

#### Scenario: Push Audio Chunk to Buffer
Given a generated audio chunk from XTTS (PCM float32 samples)
When the XTTS worker thread pushes the chunk
Then the AudioBuffer SHALL append the chunk to FIFO queue
And the AudioBuffer SHALL NOT block the XTTS worker thread
And the AudioBuffer SHALL maintain thread safety
And the AudioBuffer SHALL reject chunks if buffer is overfilled (protection)

#### Scenario: Pop Audio Chunk from Buffer
Given an audio output thread consuming audio
When the audio callback requests n_samples
Then the AudioBuffer SHALL return concatenated PCM samples from queue
And the AudioBuffer SHALL block only if buffer is empty (wait for data)
And the AudioBuffer SHALL maintain audio continuity (no gaps between chunks)
And the AudioBuffer SHALL handle underrun gracefully (silence padding if needed)

#### Scenario: Buffer State Management
Given an AudioBuffer with current buffer length
When buffer_length > 2.0 seconds
Then the TTS Service SHALL start playback
And the TTS Service SHALL continue consuming from buffer
When buffer_length <= 2.0 seconds
Then the TTS Service SHALL pause playback (if not already playing)
And the TTS Service SHALL wait for buffer to fill above 2.0 seconds
And the TTS Service SHALL resume playback when buffer is ready

### Requirement: Semantic Text Chunking

The TTS Service SHALL chunk text based on semantic pauses and narrative flow, NOT based on sentence boundaries or fixed token counts.

#### Scenario: Semantic Chunking Rules
Given a long text input from Qwen 14B
When chunking the text for XTTS synthesis
Then the TTS Service SHALL target chunk duration of 3-7 seconds
And the TTS Service SHALL target chunk length of 180-320 characters
And the TTS Service SHALL prefer natural pause points (commas, "and", "as", "while", "when", etc.)
And the TTS Service SHALL avoid cutting mid-phrase or mid-thought
And the TTS Service SHALL respect narrative flow (complete ideas)

#### Scenario: Chunk Boundary Detection
Given text: "In the depths of the ancient dungeon, shadows danced along the stone walls as torchlight flickered. The air was thick with the scent of damp earth."
When chunking semantically
Then the TTS Service SHALL identify pause points: after "dungeon,", after "walls", after "flickered."
And the TTS Service SHALL create chunks that respect these pauses
And the TTS Service SHALL NOT create chunks shorter than 2.4 seconds
And the TTS Service SHALL NOT create chunks longer than 8 seconds

#### Scenario: Chunking Long Narratives
Given a narrative text of 2000+ characters
When chunking for streaming
Then the TTS Service SHALL create multiple chunks of 3-7 seconds each
And the TTS Service SHALL maintain semantic coherence within each chunk
And the TTS Service SHALL ensure smooth transitions between chunks
And the TTS Service SHALL avoid creating chunks that take > 6-8 seconds to generate

### Requirement: Adaptive Pre-Buffer Dynamic Management

The TTS Service SHALL maintain a pre-buffer sized according to hardware capabilities, ensuring continuous playback without overloading modest systems.

#### Scenario: High-End Pre-Buffer Initialization (RTX 5090)
Given High-End GPU classification
When starting synthesis
Then the TTS Service SHALL generate at least 2 chunks before starting playback
And the TTS Service SHALL ensure buffer contains > 2.0 seconds of audio
And the TTS Service SHALL only start playback when pre-buffer is ready

#### Scenario: Modest Pre-Buffer Initialization (RTX 3050)
Given Modest GPU classification
When starting synthesis
Then the TTS Service SHALL generate at least 1 chunk before starting playback
And the TTS Service SHALL ensure buffer contains > 1.0 seconds of audio
And the TTS Service SHALL start playback earlier (lower threshold) to reduce latency

#### Scenario: Continuous Pre-Buffering (High-End)
Given High-End GPU and audio currently playing chunk N
When chunk N is playing
Then the TTS Service SHALL have chunk N+1 already generated and in buffer
And the TTS Service SHALL be generating chunk N+2 (or have it 20-30% ready)
And the TTS Service SHALL maintain buffer ahead of playback at all times
And the TTS Service SHALL never allow buffer to drop below 1.5 seconds

#### Scenario: Continuous Pre-Buffering (Modest)
Given Modest GPU and audio currently playing chunk N
When chunk N is playing
Then the TTS Service SHALL have chunk N+1 ready or generating
And the TTS Service SHALL NOT pre-generate chunk N+2 (save GPU resources)
And the TTS Service SHALL maintain buffer ahead of playback
And the TTS Service SHALL allow buffer to drop to 0.5 seconds before pausing

#### Scenario: Buffer Underrun Prevention
Given buffer is approaching low threshold
When buffer length drops below threshold (tier-dependent)
Then the TTS Service SHALL pause playback gracefully
And the TTS Service SHALL continue generating chunks to fill buffer
And the TTS Service SHALL resume playback when buffer exceeds threshold
And the TTS Service SHALL apply crossfade on resume to avoid clicks
And the TTS Service SHALL use tier-appropriate thresholds (High-End: 2.0s, Modest: 1.0s)

### Requirement: Adaptive Parallel XTTS Inference Pipeline

The TTS Service SHALL parallelize XTTS inference based on hardware capabilities, preventing system overload on modest hardware while maximizing performance on high-end GPUs.

#### Scenario: Hardware-Aware Parallel Inference
Given GPU capability detection completed
When synthesizing audio
Then the TTS Service SHALL use parallel streams ONLY if:
  - GPU tier is High-End (RTX 4090/5090) OR
  - `VRPG_XTTS_GPU_STREAMS` > 1 OR
  - Performance profile is "high_performance"
And the TTS Service SHALL use sequential inference if:
  - GPU tier is Modest/Low-End OR
  - VRAM pressure detected OR
  - System GPU load > 80%

#### Scenario: High-End GPU Parallel Inference (RTX 5090)
Given RTX 5090 GPU classified as High-End
When generating multiple chunks
Then the TTS Service SHALL use 2-3 separate CUDA streams for parallel chunk generation
And the TTS Service SHALL maximize GPU utilization (target 80-95%)
And the TTS Service SHALL NOT serialize chunk generation
And the TTS Service SHALL handle GPU memory management (avoid OOM)
And the TTS Service SHALL NOT limit VRAM usage

#### Scenario: Modest GPU Sequential Inference (RTX 3050)
Given RTX 3050 GPU classified as Modest
When generating multiple chunks
Then the TTS Service SHALL use 1 CUDA stream only (sequential)
And the TTS Service SHALL generate chunks one at a time
And the TTS Service SHALL yield GPU between chunks (prevent saturation)
And the TTS Service SHALL limit VRAM usage to 3GB maximum
And the TTS Service SHALL target GPU utilization 40-60%

#### Scenario: Chunk Generation Priority
Given multiple chunks queued for generation
When prioritizing chunk generation
Then the TTS Service SHALL prioritize chunk N+1 (next to play) as HIGH priority
And the TTS Service SHALL generate chunk N+2 as LOW priority (background, if parallel enabled)
And the TTS Service SHALL cancel low-priority generation if buffer is full
And the TTS Service SHALL ensure high-priority chunks complete first
And the TTS Service SHALL skip parallel generation if GPU tier is Modest/Low-End

### Requirement: Native Audio Output (Bypass Python Audio)

The TTS Service SHALL use native audio frameworks instead of Python audio libraries to avoid gaps, enable real mixing, and handle buffer underrun properly.

#### Scenario: WASAPI Integration (Windows)
Given TTS Service running on Windows
When initializing audio output
Then the TTS Service SHALL use WASAPI (Windows Audio Session API)
And the TTS Service SHALL use exclusive mode or shared mode as configured
And the TTS Service SHALL implement async audio callbacks
And the TTS Service SHALL configure buffer size to 256-512 frames (low latency)
And the TTS Service SHALL NOT use large buffers (2048/4096) that increase lag
And the TTS Service SHALL handle buffer underrun with silence padding
And the TTS Service SHALL use int16 PCM format for audio I/O
And the TTS Service SHALL NOT use sounddevice, pygame, or other Python audio libraries

#### Scenario: ASIO Integration (Windows Pro Audio)
Given TTS Service with ASIO driver available
When ASIO is configured
Then the TTS Service SHALL use ASIO for lowest-latency audio
And the TTS Service SHALL configure buffer sizes 256-512 samples (optimal for voice streaming)
And the TTS Service SHALL NOT use very small buffers (< 256) that cause instability
And the TTS Service SHALL NOT use large buffers (> 512) that increase lag
And the TTS Service SHALL handle ASIO buffer callbacks
And the TTS Service SHALL use int16 PCM format for audio I/O
And the TTS Service SHALL maintain < 5ms audio latency

#### Scenario: CoreAudio Integration (macOS)
Given TTS Service running on macOS
When initializing audio output
Then the TTS Service SHALL use CoreAudio framework
And the TTS Service SHALL use AudioUnit or AVAudioEngine
And the TTS Service SHALL implement audio render callbacks
And the TTS Service SHALL handle sample rate conversion if needed

#### Scenario: WebRTC Opus Integration (Web)
Given TTS Service for web client
When streaming audio to web
Then the TTS Service SHALL encode audio as Opus
And the TTS Service SHALL stream via WebRTC data channels
And the TTS Service SHALL handle network jitter buffering
And the TTS Service SHALL maintain audio continuity across network packets

### Requirement: Continuous Playback Without Gaps

The TTS Service SHALL ensure continuous audio playback without micro-cuts, gaps, or perceptible delays between chunks.

#### Scenario: Seamless Chunk Transition
Given chunk N is playing and chunk N+1 is ready in buffer
When chunk N finishes playing
Then the TTS Service SHALL immediately start playing chunk N+1
And the TTS Service SHALL apply crossfade (10-50ms) between chunks if needed
And the TTS Service SHALL NOT introduce silence gaps
And the TTS Service SHALL maintain sample-accurate timing

#### Scenario: Audio Continuity Check
Given two consecutive audio chunks
When transitioning between chunks
Then the TTS Service SHALL verify audio continuity (no DC offset jumps)
And the TTS Service SHALL apply fade-in/fade-out if needed (minimal, 10ms)
And the TTS Service SHALL NOT apply aggressive processing (preserve RAW quality)
And the TTS Service SHALL maintain consistent sample rate and format

#### Scenario: Buffer Underrun Handling
Given buffer underrun occurs (buffer empty while playing)
When audio callback requests samples but buffer is empty
Then the TTS Service SHALL pad with silence (minimal, < 100ms)
And the TTS Service SHALL log underrun event
And the TTS Service SHALL continue generating chunks to refill buffer
And the TTS Service SHALL resume playback as soon as buffer has data

### Requirement: XTTS Window Advantage Utilization

The TTS Service SHALL leverage XTTS real-time factor advantage (0.4x) to always stay 1-2 chunks ahead of playback.

#### Scenario: Real-Time Factor Calculation
Given XTTS generates 27 seconds of audio in 10 seconds
When calculating real-time factor
Then the TTS Service SHALL recognize RTF = 0.4x (2.5x faster than real-time)
And the TTS Service SHALL use this advantage to pre-buffer chunks
And the TTS Service SHALL ensure generation is always ahead of playback

#### Scenario: Chunk Generation Scheduling
Given current playback at chunk N (10 seconds into audio)
When scheduling next chunks
Then the TTS Service SHALL have chunk N+1 ready (already generated)
And the TTS Service SHALL be generating chunk N+2 (or have it ready)
And the TTS Service SHALL maintain 1-2 chunks ahead at all times
And the TTS Service SHALL never fall behind playback

### Requirement: Thread Architecture Separation

The TTS Service SHALL separate Qwen text generation, XTTS audio generation, and audio playback into independent threads that never block each other.

#### Scenario: Thread A - Qwen Text Generation
Given a voice input or game event
When generating narrative text
Then Thread A (Qwen 1.5B) SHALL generate prelude text
And Thread A SHALL NOT block audio playback
And Thread A SHALL NOT block XTTS generation
And Thread A SHALL send text to XTTS worker when ready

#### Scenario: Thread B - Qwen 14B Narrative
Given prelude text from Qwen 1.5B
When generating complete narrative
Then Thread B (Qwen 14B) SHALL generate full narrative text
And Thread B SHALL chunk text semantically
And Thread B SHALL send chunks to XTTS worker as ready
And Thread B SHALL NOT block other threads

#### Scenario: Thread C - XTTS Generation Worker
Given text chunks from Qwen threads
When generating audio
Then Thread C SHALL generate audio chunks in parallel (if GPU allows)
And Thread C SHALL push chunks to AudioBuffer FIFO
And Thread C SHALL NOT block on audio playback
And Thread C SHALL prioritize chunks based on playback position

#### Scenario: Thread D - Audio Consumer (Dedicated I/O Thread)
Given AudioBuffer FIFO with audio chunks
When playing audio
Then Thread D SHALL be a dedicated thread for audio I/O only
And Thread D SHALL NOT share thread with UI, model inference, or other operations
And Thread D SHALL consume chunks from FIFO continuously
And the TTS Service SHALL use native audio framework (WASAPI/ASIO/CoreAudio)
And Thread D SHALL handle audio callbacks asynchronously
And Thread D SHALL NOT block XTTS generation
And Thread D SHALL use int16 PCM format for audio I/O (convert from Float32)

### Requirement: Audio Format and Precision

The TTS Service SHALL use Float32 precision for XTTS inference (internal processing) and int16 PCM for audio I/O to optimize bandwidth and compatibility.

#### Scenario: Internal Processing Precision
Given XTTS model loading
When configuring inference precision
Then the TTS Service SHALL use Float32 (full precision) for XTTS inference
And the TTS Service SHALL NOT use FP16 (half precision)
And the TTS Service SHALL NOT use BF16 (bfloat16)
And the TTS Service SHALL NOT use INT8 quantization
And the TTS Service SHALL maintain Float32 for internal processing pipeline

#### Scenario: Audio I/O Format
Given audio ready for playback
When sending audio to native audio framework
Then the TTS Service SHALL convert Float32 to int16 PCM format
And the TTS Service SHALL use int16 for audio I/O (bandwidth efficient)
And the TTS Service SHALL NOT use float64 or other heavy formats for I/O
And the TTS Service SHALL maintain compatibility with Opus and other codecs

#### Scenario: Quality Preservation
Given Float32 precision for inference and int16 for I/O
When generating and playing audio
Then the TTS Service SHALL preserve natural voice quality
And the TTS Service SHALL avoid metallic artifacts
And the TTS Service SHALL maintain audio fidelity (int16 sufficient for voice)
And the TTS Service SHALL output RAW audio (no post-processing)

### Requirement: Time-Stretch Optimization (Optional)

The TTS Service MAY apply subtle time-stretch (1.05-1.15x) to audio playback to create buffer time for XTTS generation without player perception.

#### Scenario: Time-Stretch Application
Given audio chunk ready for playback
When time-stretch is enabled
Then the TTS Service SHALL apply 1.05-1.15x speed increase
And the TTS Service SHALL use high-quality time-stretch algorithm (SoX, Audacity, or Unity DSP)
And the TTS Service SHALL maintain pitch (time-stretch only, no pitch shift)
And the TTS Service SHALL ensure player does not perceive speed change

#### Scenario: Time-Stretch Buffer Advantage
Given time-stretch of 1.1x applied
When calculating buffer advantage
Then the TTS Service SHALL gain 10% additional time for XTTS generation
And the TTS Service SHALL use this time to pre-buffer more chunks
And the TTS Service SHALL reduce risk of buffer underrun

### Requirement: Latency Targets

The TTS Service SHALL meet strict latency targets for cinematic real-time voice interaction.

#### Scenario: Initial Response Latency
Given a voice input or game event triggering narrative
When starting audio playback
Then the TTS Service SHALL achieve initial audio start in 2.5-3.8 seconds
And the TTS Service SHALL have pre-buffer of 1.5-3.0 seconds ready
And the TTS Service SHALL maintain continuous playback after start
And the TTS Service SHALL NOT have perceptible delays

#### Scenario: Streaming Continuity
Given audio is playing continuously
When streaming long narratives
Then the TTS Service SHALL maintain zero-gap playback
And the TTS Service SHALL NOT interrupt audio flow
And the TTS Service SHALL maintain buffer ahead of playback
And the TTS Service SHALL achieve "Critical Role AI" level performance

#### Scenario: Chunk Generation Latency
Given a text chunk of 3-7 seconds duration
When generating audio with XTTS
Then the TTS Service SHALL complete generation in 1.2-2.8 seconds (RTF 0.4x)
And the TTS Service SHALL push chunk to buffer immediately
And the TTS Service SHALL NOT wait for playback to finish

## MODIFIED Requirements

### Requirement: Audio Streaming (Updated)

The TTS Service SHALL support real-time cinematic streaming with semantic chunking, pre-buffering, and parallel inference, NOT simple sentence-based chunking.

#### Scenario: Real-Time Cinematic Streaming
Given a long narrative text from Qwen 14B
When synthesizing and playing audio
Then the TTS Service SHALL chunk text semantically (3-7s chunks, 180-320 chars)
And the TTS Service SHALL pre-buffer 2 chunks before starting playback
And the TTS Service SHALL generate chunks in parallel (staggered pipeline)
And the TTS Service SHALL maintain 1-2 chunks ahead of playback
And the TTS Service SHALL use FIFO buffer for continuous playback
And the TTS Service SHALL achieve zero-gap playback
And the TTS Service SHALL NOT chunk by sentences or fixed intervals

## Technical Constraints

### Performance Requirements
- Initial audio start: 2.5-3.8 seconds
- Pre-buffer size: 1.5-3.0 seconds minimum
- Chunk duration: 3-7 seconds (target), 2.4-8.0 seconds (range)
- Chunk generation: 1.2-2.8 seconds per chunk (RTF 0.4x)
- Buffer underrun: < 100ms silence padding maximum
- Audio gap between chunks: 0ms (zero-gap)
- GPU utilization: 80-95% target

### Audio Quality Requirements
- Sample rate: 16000-24000 Hz (16 kHz or 24 kHz sufficient for voice, NOT 48 kHz)
- Bit depth: 
  - Internal processing: 32-bit float (Float32)
  - Audio I/O: 16-bit integer (int16)
- Channels: Mono (1 channel) - NOT stereo (doubles bandwidth and processing)
- Format: RAW PCM (no post-processing)
- Precision: Float32 for inference (no FP16/BF16/INT8)
- Buffer size: 256-512 frames per buffer (low latency, NOT 2048/4096)
- Chunk transitions: Crossfade 10-50ms if needed
- Quality: RAW XTTS output (infinitely better than processed)

### Architecture Requirements
- Thread separation: Qwen (A), Qwen 14B (B), XTTS (C), Audio (D) - independent
- FIFO buffer: Thread-safe, lock-free preferred
- Native audio: WASAPI/ASIO/CoreAudio (no Python audio libraries)
- Parallel inference: Multiple CUDA streams for chunk generation
- Pre-buffering: Always 1-2 chunks ahead of playback

### Chunking Requirements
- Semantic boundaries: Commas, "and", "as", "while", "when", etc.
- Duration target: 3-7 seconds per chunk
- Character target: 180-320 characters per chunk
- Minimum duration: 2.4 seconds (avoid too short)
- Maximum duration: 8 seconds (avoid generation delay > 6-8s)
- Avoid: Mid-phrase cuts, mid-thought breaks, sentence-only boundaries

## Implementation Architecture

### Component Structure

```
src/tts-service/
├── audio_buffer.rs          # FIFO buffer with thread safety
├── audio_output.rs          # Native audio (WASAPI/ASIO/CoreAudio)
├── semantic_chunker.rs      # Semantic text chunking
├── xtts_streaming.rs        # XTTS parallel inference pipeline
├── prebuffer_manager.rs     # Pre-buffer state management
└── streaming_pipeline.rs    # Orchestrates all components
```

### Thread Architecture

```
Thread A: Qwen 1.5B → Prelude text → Chunker
Thread B: Qwen 14B → Full narrative → Chunker
Thread C: XTTS Worker → Generate chunks → AudioBuffer.push()
Thread D: Audio Consumer → AudioBuffer.pop() → Native Audio Output
```

### Data Flow

```
Qwen Text → Semantic Chunker → Chunk Queue
                                    ↓
                            XTTS Worker (parallel)
                                    ↓
                            AudioBuffer FIFO
                                    ↓
                            Audio Output Thread
                                    ↓
                            Native Audio (WASAPI/ASIO)
```

### Pseudo-Code: AudioBuffer FIFO

```rust
struct AudioBuffer {
    queue: Arc<Mutex<VecDeque<Vec<f32>>>>,  // PCM float32 chunks (internal)
    sample_rate: u32,                        // 16000 or 24000 Hz (voice sufficient)
    channels: u16,                          // 1 (mono)
    max_buffer_seconds: f32,
    frames_per_buffer: usize,               // 256-512 (low latency)
}

impl AudioBuffer {
    fn push(&self, pcm_chunk: Vec<f32>) -> Result<()> {
        // Append chunk to queue (Float32 internal)
        // Check overfill protection
        // Non-blocking
    }
    
    fn pop_block(&self, n_samples: usize) -> Vec<i16> {
        // Concatenate chunks until n_samples (Float32)
        // Convert Float32 to int16 for I/O
        // Block if buffer empty (wait for data)
        // Maintain continuity
        // Return int16 PCM
    }
    
    fn buffer_length_seconds(&self) -> f32 {
        // Calculate total audio duration in queue
    }
    
    fn convert_float32_to_int16(&self, float_samples: Vec<f32>) -> Vec<i16> {
        // Convert Float32 [-1.0, 1.0] to int16 [-32768, 32767]
        // Efficient conversion for audio I/O
    }
}
```

### Pseudo-Code: Pre-Buffer Manager

```rust
struct PreBufferManager {
    target_buffer_seconds: f32,  // 2.0s
    min_buffer_seconds: f32,     // 1.5s
    playback_state: PlaybackState,
}

impl PreBufferManager {
    fn should_start_playback(&self, buffer: &AudioBuffer) -> bool {
        buffer.buffer_length_seconds() > self.target_buffer_seconds
    }
    
    fn should_pause_playback(&self, buffer: &AudioBuffer) -> bool {
        buffer.buffer_length_seconds() <= self.min_buffer_seconds
    }
}
```

### Pseudo-Code: Semantic Chunker

```rust
struct SemanticChunker {
    min_duration_seconds: f32,  // 2.4s
    max_duration_seconds: f32,  // 8.0s
    target_duration_seconds: f32,  // 3-7s
    target_chars: usize,  // 180-320
}

impl SemanticChunker {
    fn chunk(&self, text: &str) -> Vec<String> {
        // Find semantic pause points
        // Create chunks of 3-7 seconds target
        // Respect narrative flow
        // Avoid mid-phrase cuts
    }
    
    fn find_pause_points(&self, text: &str) -> Vec<usize> {
        // Commas, "and", "as", "while", "when", etc.
        // Sentence boundaries (secondary)
    }
}
```

### Pseudo-Code: XTTS Streaming Worker

```rust
struct XTTSStreamingWorker {
    chunk_queue: Arc<Mutex<VecDeque<String>>>,  // Text chunks
    audio_buffer: Arc<AudioBuffer>,
    gpu_streams: Vec<CudaStream>,  // Parallel CUDA streams
}

impl XTTSStreamingWorker {
    async fn generate_parallel(&self) {
        // Thread C: Generate chunks in parallel
        // Priority: N+1 (high), N+2 (low)
        // Push to AudioBuffer immediately
        // Never block on playback
    }
}
```

## Testing Requirements

### Unit Tests
- AudioBuffer FIFO push/pop operations (100% coverage)
- Semantic chunker boundary detection (100% coverage)
- Pre-buffer state management (100% coverage)
- Thread safety of AudioBuffer (concurrent access tests)

### Integration Tests
- End-to-end streaming pipeline (Qwen → XTTS → Audio)
- Buffer underrun handling
- Chunk transition continuity
- Parallel XTTS generation

### Performance Tests
- Initial latency measurement (target: 2.5-3.8s)
- Buffer underrun frequency (target: 0 underruns)
- GPU utilization (target: 80-95%)
- Real-time factor verification (target: < 0.5x)

### Quality Tests
- Audio continuity between chunks (zero-gap verification)
- RAW audio quality preservation (no artifacts)
- Float32 precision verification
- Chunk semantic coherence

## Migration Notes

### Breaking Changes
- Audio streaming API changes (now uses FIFO buffer)
- Chunking strategy changes (semantic vs sentence-based)
- Audio output framework changes (native vs Python)

### Migration Path
1. Implement AudioBuffer FIFO
2. Implement semantic chunker
3. Implement native audio output
4. Implement parallel XTTS worker
5. Integrate pre-buffer manager
6. Test and validate performance
7. Deprecate old streaming API

## References

- XTTS Real-Time Factor: 0.4x (27s audio in 10s generation)
- RAW Audio Quality: Infinitely better than processed (see DESCOBERTA_RAW.md)
- GPU: RTX 5090 with CUDA streams for parallel inference
- Target Performance: "Critical Role AI" level (2.5-4.0s initial, continuous voice)

