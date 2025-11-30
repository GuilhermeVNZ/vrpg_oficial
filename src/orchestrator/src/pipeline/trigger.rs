//! 1.5B Trigger Logic - M2.2
//! Logic for deciding when to trigger Qwen-1.5B (6-8s of speech, pause detected, clear action)

use crate::error::Result;
use crate::pipeline::PipelineState;
use std::time::Duration;

/// Criteria for triggering 1.5B
#[derive(Debug, Clone)]
pub struct TriggerCriteria {
    /// Duration of speech so far
    pub speech_duration: Duration,
    /// Pause duration (if any)
    pub pause_duration: Option<Duration>,
    /// Pause threshold for triggering
    pub pause_threshold: Duration,
    /// Whether VAD detected end of speech
    pub vad_detected_end: bool,
    /// Whether a clear action was detected
    pub has_clear_action: bool,
    /// Type of action (if any)
    pub action_type: Option<String>,
}

impl TriggerCriteria {
    /// Create new trigger criteria
    pub fn new() -> Self {
        Self {
            speech_duration: Duration::ZERO,
            pause_duration: None,
            pause_threshold: Duration::from_millis(1000), // 1s default
            vad_detected_end: false,
            has_clear_action: false,
            action_type: None,
        }
    }

    /// Set speech duration
    pub fn with_speech_duration(mut self, duration: Duration) -> Self {
        self.speech_duration = duration;
        self
    }

    /// Set pause duration
    pub fn with_pause_duration(mut self, duration: Option<Duration>) -> Self {
        self.pause_duration = duration;
        self
    }

    /// Set pause threshold
    pub fn with_pause_threshold(mut self, threshold: Duration) -> Self {
        self.pause_threshold = threshold;
        self
    }

    /// Set VAD detected end
    pub fn with_vad_detected_end(mut self, detected: bool) -> Self {
        self.vad_detected_end = detected;
        self
    }

    /// Set clear action
    pub fn with_clear_action(mut self, has_action: bool, action_type: Option<String>) -> Self {
        self.has_clear_action = has_action;
        self.action_type = action_type;
        self
    }
}

impl Default for TriggerCriteria {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if 1.5B should be triggered based on criteria
pub fn should_trigger_1_5b(criteria: &TriggerCriteria) -> bool {
    // Time-based: 6-8 seconds of speech
    let time_based = criteria.speech_duration >= Duration::from_secs(6);

    // Pause-based: pause > threshold or VAD detected end
    let pause_based = criteria
        .pause_duration
        .map(|pause| pause >= criteria.pause_threshold)
        .unwrap_or(false)
        || criteria.vad_detected_end;

    // Action-based: clear action detected
    let action_based = criteria.has_clear_action;

    // Trigger if any criterion is met
    time_based || pause_based || action_based
}

/// Trigger 1.5B and get prelude text
pub async fn trigger_1_5b(
    _pipeline_state: &mut PipelineState,
    _persona: &str,
    _asr_partial: &str,
) -> Result<String> {
    // TODO: Implement actual LLM Core call to /llm/prelude
    // For now, return a mock response
    Ok("A weight settles...".to_string())
}

/// Trigger 1.5B and immediately send to TTS
pub async fn trigger_1_5b_and_send_to_tts(
    pipeline_state: &mut PipelineState,
    persona: &str,
    asr_partial: &str,
) -> Result<()> {
    // Trigger 1.5B
    let prelude = trigger_1_5b(pipeline_state, persona, asr_partial).await?;

    // TODO: Send to TTS immediately
    // For now, just log
    tracing::info!("Sending prelude to TTS: {}", prelude);

    Ok(())
}


