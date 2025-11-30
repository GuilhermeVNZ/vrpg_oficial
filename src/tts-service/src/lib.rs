// TTS Service - Text-to-Speech synthesis using XTTS with voice embeddings
// This module provides multi-voice text-to-speech capabilities with real-time streaming

pub mod audio_buffer;
pub mod audio_utils;
pub mod error;
pub mod gpu_config;
pub mod gpu_detector;
pub mod interjections;
pub mod metrics;
pub mod phonemizer;
pub mod pipeline;
pub mod prebuffer_manager;
pub mod semantic_chunker;
pub mod server;
pub mod streaming;
pub mod streaming_server;
pub mod tts_profile;
pub mod voice;
pub mod voice_intent;
pub mod voice_profiles;
pub mod xtts;

pub use audio_utils::{apply_volume, calculate_rms, detect_peak, normalize_volume};
pub use error::{Result, TtsError};
pub use metrics::{
    LatencyTimer, MetricsCollector, MetricsStats, PipelineMetrics, SharedMetricsCollector,
};
pub use pipeline::{PipelineRequest, PipelineResponse, TtsPipeline};
pub use server::TtsServer;
pub use voice_intent::{ParsedVoiceIntent, VoiceIntent, VoiceIntentParser, VoiceIntentType};
pub use voice_profiles::{
    CharacterType, SharedVoiceProfileManager, VoiceProfile, VoiceProfileManager,
};
pub use xtts::{AudioOutput, SharedXttsModel, SynthesisRequest, XttsModel};
pub use audio_buffer::{AudioBuffer, AudioChunk};
pub use gpu_config::{GpuConfig, PerformanceProfile};
pub use gpu_detector::{GpuCapability, GpuDetector, GpuTier};
pub use prebuffer_manager::{PreBufferManager, PreBufferState};
pub use semantic_chunker::{ChunkerConfig, SemanticChunker, TextChunk};
pub use streaming::{StreamingPipeline, StreamingRequest, StreamingStatus};
pub use streaming_server::{
    cancel_streaming, get_streaming_status, handle_sse_stream, handle_websocket_stream,
    AudioChunkMessage, StreamingRequestPayload, StreamingResponse,
};
pub use tts_profile::{TtsProfile, TtsProfileType};
pub use interjections::{
    InterjectionClip, InterjectionConfig, InterjectionManager, InterjectionState,
};
