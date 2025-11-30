// ASR Service - Automatic Speech Recognition using Whisper
// This module provides real-time speech recognition capabilities

pub mod error;
pub mod server;
pub mod vad;
pub mod whisper;

pub use error::{AsrError, Result};
pub use server::AsrServer;
pub use vad::{VadResult, VoiceActivity, VoiceActivityDetector};
pub use whisper::{AudioChunk, SharedWhisperModel, TranscriptionResult, WhisperModel};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        assert!(true);
    }
}
