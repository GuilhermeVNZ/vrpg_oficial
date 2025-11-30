# TTS Service Technical Specification

## Purpose

The TTS Service provides local text-to-speech synthesis using StyleTTS2, supporting multiple voice profiles, Voice INTENTS, and low-latency audio generation. This specification defines the technical requirements for voice synthesis, profile management, and integration with the Orchestrator and Voice INTENTS system.

## ADDED Requirements

### Requirement: StyleTTS2 Integration
The TTS Service SHALL use StyleTTS2 as the primary TTS engine, providing fast, natural speech synthesis with style embeddings.

#### Scenario: Synthesize Speech with StyleTTS2
Given a text input "The corridor is narrow, lit by ancient torches."
And a voice profile "mestre" with style="neutral", emotion="calm"
When the TTS Service synthesizes the speech
Then the TTS Service SHALL load StyleTTS2 model
And the TTS Service SHALL apply style embeddings for neutral/calm
And the TTS Service SHALL generate WAV audio (9600-22050Hz)
And the TTS Service SHALL complete synthesis in < 350ms for short sentences
And the TTS Service SHALL return PCM audio buffer

#### Scenario: Synthesize with Emotion
Given a text input "The ogre advances and the entire room trembles with the impact."
And a voice profile "mestre" with style="intense", emotion="danger"
When the TTS Service synthesizes the speech
Then the TTS Service SHALL apply danger emotion embeddings
And the TTS Service SHALL adjust pitch and tempo for intensity
And the TTS Service SHALL generate audio with appropriate dramatic tone
And the TTS Service SHALL maintain voice profile consistency

### Requirement: Voice Profile Management
The TTS Service SHALL manage multiple voice profiles, loading them at boot and switching between them without reloading models.

#### Scenario: Load Voice Profiles at Boot
Given TTS Service initialization
When the service starts
Then the TTS Service SHALL load all voice profiles from configuration
And the TTS Service SHALL load profiles: mestre, npc_guard, npc_barkeep, npc_mysterious_woman, player_rogue, etc.
And the TTS Service SHALL keep profiles in memory
And the TTS Service SHALL NOT reload models when switching profiles
And the TTS Service SHALL expose available profiles via /voices endpoint

#### Scenario: Switch Voice Profile
Given a current voice profile "mestre"
When switching to voice profile "npc_guard"
Then the TTS Service SHALL switch to the npc_guard profile
And the TTS Service SHALL apply profile settings (pitch, tempo, grain)
And the TTS Service SHALL NOT reload the StyleTTS2 model
And the TTS Service SHALL complete the switch in < 10ms

#### Scenario: Voice Profile Configuration
Given a voice profile configuration:
```json
{
  "npc_guard": {
    "pitch_base": -0.2,
    "tempo_base": 0.9,
    "instability": 0.1,
    "grain": 0.05,
    "style_embedding": "gravel_low"
  }
}
```
When the TTS Service loads the profile
Then the TTS Service SHALL apply all profile parameters
And the TTS Service SHALL store the profile for runtime use
And the TTS Service SHALL validate all parameters are within valid ranges

### Requirement: Voice INTENTS Support
The TTS Service SHALL parse and execute Voice INTENTS, generating speech according to intent type, speaker, style, and emotion.

#### Scenario: Parse Voice INTENT NARRATE
Given a Voice INTENT:
```
[VOICE_INTENT:NARRATE]
{
speaker: "mestre",
style: "neutral",
emotion: "calm",
text: "O corredor é estreito, iluminado por tochas antigas."
}
```
When the TTS Service processes the INTENT
Then the TTS Service SHALL parse the INTENT structure
And the TTS Service SHALL identify speaker="mestre"
And the TTS Service SHALL apply style="neutral" and emotion="calm"
And the TTS Service SHALL synthesize the text with appropriate voice
And the TTS Service SHALL return audio buffer

#### Scenario: Parse Voice INTENT NPC_DIALOGUE
Given a Voice INTENT:
```
[VOICE_INTENT:NPC_DIALOGUE]
{
speaker: "npc_guard",
style: "gravel_low",
emotion: "mild_irritation",
text: "Não tenho tempo pra vocês. Sigam andando."
}
```
When the TTS Service processes the INTENT
Then the TTS Service SHALL load npc_guard voice profile
And the TTS Service SHALL apply gravel_low style
And the TTS Service SHALL apply mild_irritation emotion
And the TTS Service SHALL synthesize with NPC voice characteristics
And the TTS Service SHALL return audio buffer

#### Scenario: Parse Voice INTENT EVENT
Given a Voice INTENT:
```
[VOICE_INTENT:EVENT]
{
speaker: "mestre",
style: "intense",
emotion: "danger",
text: "O ogro avança e a sala inteira treme com o impacto."
}
```
When the TTS Service processes the INTENT
Then the TTS Service SHALL apply intense style
And the TTS Service SHALL apply danger emotion with high intensity
And the TTS Service SHALL adjust pitch and tempo for dramatic effect
And the TTS Service SHALL generate audio with appropriate impact

#### Scenario: Parse Voice INTENT CONDITION_EXPIRE
Given a Voice INTENT:
```
[VOICE_INTENT:CONDITION_EXPIRE]
{
speaker: "mestre",
style: "neutral",
emotion: "solemn",
text: "A energia rubra abandona seus músculos. A dor retorna."
}
```
When the TTS Service processes the INTENT
Then the TTS Service SHALL apply solemn emotion
And the TTS Service SHALL synthesize with appropriate gravity
And the TTS Service SHALL maintain narrative tone

### Requirement: Multi-Voice Support
The TTS Service SHALL support multiple voice types: Master (DM), NPCs, AI Players, and Monsters, each with distinct characteristics.

#### Scenario: Master Voice Synthesis
Given text to be spoken by the Master
When synthesizing Master voice
Then the TTS Service SHALL use mestre voice profile
And the TTS Service SHALL apply neutral to dramatic style range
And the TTS Service SHALL maintain consistent narrator voice
And the TTS Service SHALL support various emotions (calm, tense, danger, solemn)

#### Scenario: NPC Voice Synthesis
Given text to be spoken by an NPC with profile "npc_barkeep"
When synthesizing NPC voice
Then the TTS Service SHALL use npc_barkeep voice profile
And the TTS Service SHALL apply NPC-specific characteristics
And the TTS Service SHALL maintain voice consistency across sessions
And the TTS Service SHALL support emotion variations within character limits

#### Scenario: AI Player Voice Synthesis
Given text to be spoken by an AI Player with profile "player_rogue"
When synthesizing AI Player voice
Then the TTS Service SHALL use player_rogue voice profile
And the TTS Service SHALL apply casual, natural style
And the TTS Service SHALL include slight hesitations and natural pauses
And the TTS Service SHALL NOT sound robotic or NPC-like

#### Scenario: Monster Voice Synthesis
Given text to be spoken by a monster
When synthesizing monster voice
Then the TTS Service SHALL apply monster-specific effects (reverb, pitch shift, filters)
And the TTS Service SHALL maintain monstrous characteristics
And the TTS Service SHALL support various monster types (undead, beast, demonic)

### Requirement: Audio Effects and Post-Processing
The TTS Service SHALL apply audio effects (pitch shifting, reverb, filters) based on voice profile and context.

#### Scenario: Apply Pitch Shifting
Given a voice profile with pitch_base = -0.2 (lower pitch)
When synthesizing speech
Then the TTS Service SHALL shift pitch down by 0.2 semitones
And the TTS Service SHALL maintain natural voice quality
And the TTS Service SHALL NOT introduce artifacts

#### Scenario: Apply Reverb Effect
Given a voice profile with reverb effect for "monster_shadow"
When synthesizing monster speech
Then the TTS Service SHALL apply reverb effect
And the TTS Service SHALL create atmospheric echo
And the TTS Service SHALL maintain speech clarity

#### Scenario: Apply Filter Effects
Given a voice profile requiring highpass filter at 200Hz
When synthesizing speech
Then the TTS Service SHALL apply highpass filter
And the TTS Service SHALL remove frequencies below 200Hz
And the TTS Service SHALL maintain voice intelligibility

### Requirement: Real-Time Cinematic Audio Streaming
The TTS Service SHALL support real-time cinematic audio streaming with semantic chunking, adaptive pre-buffering, and zero-gap playback.

#### Scenario: Real-Time Cinematic Streaming
Given a long text input from Qwen 14B
When synthesizing and playing audio
Then the TTS Service SHALL chunk text semantically (3-7s chunks, 180-320 chars)
And the TTS Service SHALL pre-buffer 1-2 chunks before starting playback (tier-dependent)
And the TTS Service SHALL generate chunks in parallel if High-End GPU (2-3 CUDA streams)
And the TTS Service SHALL generate chunks sequentially if Modest GPU (1 CUDA stream)
And the TTS Service SHALL maintain 1-2 chunks ahead of playback at all times
And the TTS Service SHALL use FIFO buffer for continuous playback
And the TTS Service SHALL achieve zero-gap playback
And the TTS Service SHALL use dedicated audio I/O thread (not shared with UI/model)
And the TTS Service SHALL achieve initial latency 2.5-4.0s (all tiers)

#### Scenario: Adaptive GPU Control
Given TTS Service initialization
When detecting GPU capabilities
Then the TTS Service SHALL detect GPU tier (High-End/Mid-Range/Modest/Low-End)
And the TTS Service SHALL apply tier-appropriate configuration
And the TTS Service SHALL limit VRAM usage per tier (unlimited/6GB/3GB/2GB)
And the TTS Service SHALL control parallel streams (2-3/1-2/1/0-1)
And the TTS Service SHALL adapt pre-buffer size (2.5s/1.75s/1.25s/0.75s)
And the TTS Service SHALL yield GPU between chunks if Modest/Low-End
And the TTS Service SHALL maintain system responsiveness (no lag)

#### Scenario: Audio Format Optimization
Given audio ready for playback
When configuring audio output
Then the TTS Service SHALL use 16-24 kHz sample rate (NOT 48 kHz)
And the TTS Service SHALL use mono channel (1 channel, NOT stereo)
And the TTS Service SHALL use 256-512 frame buffer (NOT 2048/4096)
And the TTS Service SHALL use int16 PCM for I/O (NOT float64)
And the TTS Service SHALL use Float32 internally for XTTS inference
And the TTS Service SHALL convert Float32 to int16 before audio I/O

### Requirement: Audio Caching
The TTS Service SHALL cache frequently used phrases to reduce latency.

#### Scenario: Cache Common Phrase
Given a phrase "Roll initiative!" that is frequently used
When the phrase is first synthesized
Then the TTS Service SHALL store the audio in cache
And the TTS Service SHALL associate cache with voice profile and emotion
And subsequent requests for the same phrase SHALL return cached audio immediately
And cache hits SHALL complete in < 10ms

#### Scenario: Cache Invalidation
Given cached audio for a phrase
When the voice profile or emotion changes
Then the TTS Service SHALL generate new audio (cache miss)
And the TTS Service SHALL store new audio in cache with new key
And the TTS Service SHALL NOT use old cached audio

### Requirement: Latency Targets
The TTS Service SHALL meet strict latency targets for real-time voice interaction.

#### Scenario: Short Sentence Latency
Given a short sentence (< 10 words)
When synthesizing with StyleTTS2
Then the TTS Service SHALL complete synthesis in < 350ms
And on GPU (RTX 4090/5090), synthesis SHALL complete in < 120ms
And the TTS Service SHALL return audio buffer ready for playback

#### Scenario: Medium Sentence Latency
Given a medium sentence (10-20 words)
When synthesizing with StyleTTS2
Then the TTS Service SHALL complete synthesis in < 600ms
And the TTS Service SHALL support streaming for earlier playback start

### Requirement: Volume Normalization
The TTS Service SHALL normalize volume across different voice profiles to maintain consistent playback levels.

#### Scenario: Normalize Volume
Given audio from different voice profiles with varying volumes
When preparing audio for playback
Then the TTS Service SHALL normalize volume to target level
And the TTS Service SHALL maintain dynamic range
And the TTS Service SHALL prevent clipping
And the TTS Service SHALL ensure consistent loudness across profiles

### Requirement: HTTP API
The TTS Service SHALL expose HTTP endpoints for voice synthesis and management.

#### Scenario: Health Check Endpoint
Given TTS Service is running
When GET /health is called
Then the TTS Service SHALL return status 200
And the response SHALL include service status
And the response SHALL include loaded voice profiles count
And the response SHALL include model status

#### Scenario: Speak Endpoint
Given a POST /speak request with Voice INTENT:
```json
{
  "intent": "[VOICE_INTENT:NARRATE]{...}",
  "session_id": "sess_123"
}
```
When the endpoint is called
Then the TTS Service SHALL parse the Voice INTENT
And the TTS Service SHALL synthesize speech
And the TTS Service SHALL return audio as WAV or PCM
And the TTS Service SHALL include metadata (duration, sample_rate)

#### Scenario: List Voices Endpoint
Given a GET /voices request
When the endpoint is called
Then the TTS Service SHALL return list of all available voice profiles
And each profile SHALL include name, style, and characteristics
And the response SHALL be JSON formatted

## Technical Constraints

### Performance Requirements
- Initial latency (all tiers): < 4.0s (High-End: < 3.8s, Modest: < 4.5s)
- Real-time factor: < 1.0x (High-End: < 0.5x, Modest: < 0.8x)
- GPU utilization: Tier-dependent (High-End: 80-95%, Modest: 40-60%)
- Buffer underrun: 0 (mandatory)
- Audio gaps: 0ms (mandatory)
- Chunk generation: 1.2-2.8s per chunk (RTF 0.4x advantage)
- Voice profile switching: < 10ms
- Cache hit response: < 10ms

### Audio Quality Requirements
- Sample rate: 16000-24000 Hz (16 kHz or 24 kHz, NOT 48 kHz)
- Channels: Mono (1 channel, NOT stereo)
- Bit depth: 
  - Internal: 32-bit float (Float32) for XTTS inference
  - I/O: 16-bit integer (int16) for audio output
- Buffer size: 256-512 frames (low latency, NOT 2048/4096)
- Format: RAW PCM (no post-processing)
- Quality: RAW XTTS output (infinitely better than processed)
- Mean Opinion Score (MOS): > 3.5 for naturalness

### Resource Requirements
- Memory per voice profile: < 50MB
- GPU memory for StyleTTS2: < 2GB
- CPU usage during synthesis: < 30% (single core)
- Disk space for cached audio: configurable (default 100MB)

## Implementation Notes

### Rust Module Structure
```
src/tts-service/
├── lib.rs              # Public API
├── error.rs            # Error types
├── server.rs           # HTTP server
├── voice.rs            # Voice profile management
├── styletts2.rs        # StyleTTS2 integration
├── voice_intents.rs    # Voice INTENT parser
└── cache.rs            # Audio caching
```

### Dependencies
- StyleTTS2 bindings (Rust or Python wrapper)
- `serde` for serialization
- `tokio` for async runtime
- `tracing` for logging
- Audio processing library (miniaudio, cpal, or similar)

### Testing Requirements
- Unit tests for Voice INTENT parsing (100% coverage)
- Unit tests for voice profile management (100% coverage)
- Integration tests for StyleTTS2 synthesis
- Latency benchmarks for various sentence lengths
- Quality tests (MOS evaluation)
- Cache hit/miss tests











