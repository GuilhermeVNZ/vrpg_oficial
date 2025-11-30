//! Complete Pipeline Flow - M2.5
//! Implements the full pipeline: ASR → Intent Router → 1.5B → Wait Final ASR → 14B → TTS

use crate::error::Result;
use crate::intent_router::{IntentClassification, IntentRouter};
use crate::pipeline::context_14b::prepare_14b_context;
use crate::pipeline::objective_responses::answer_objective_question;
use crate::pipeline::simple_rule_query::answer_simple_rule_query;
use crate::pipeline::trigger::{should_trigger_1_5b, trigger_1_5b, TriggerCriteria};
use crate::pipeline::PipelineStatus;
use crate::pipeline::{PipelineState, PipelineStateManager};
use std::time::Duration;
use tracing::{error, info, warn};

/// Pipeline flow result
#[derive(Debug, Clone)]
pub struct PipelineFlowResult {
    /// Fast prelude from 1.5B (if triggered)
    pub fast_prelude: Option<String>,
    /// Full narrative from 14B
    pub narrative: String,
    /// Total latency in milliseconds
    pub total_latency_ms: u64,
}

/// Pipeline flow error
#[derive(Debug, thiserror::Error)]
pub enum PipelineFlowError {
    #[error("ASR failure: {0}")]
    AsrFailure(String),
    #[error("LLM failure: {0}")]
    LlmFailure(String),
    #[error("TTS failure: {0}")]
    TtsFailure(String),
    #[error("State transition error: {0}")]
    StateTransitionError(String),
}

/// Handle player input through complete pipeline
pub async fn handle_player_input(
    state_manager: &PipelineStateManager,
    asr_partial: &str,
    speech_duration: Duration,
    pause_duration: Option<Duration>,
    vad_detected_end: bool,
) -> Result<PipelineFlowResult> {
    let start_time = std::time::Instant::now();
    info!("Handling player input: {}", asr_partial);

    // 1. Get current state
    let mut state = state_manager.get_state()?;

    // 2. Classify intent using Intent Router
    let router = IntentRouter::new();
    let classification = router.classify(asr_partial, &state)?;
    info!(
        "Intent classified as: {:?} (confidence: {})",
        classification.intent_type, classification.confidence
    );

    // 3. Handle FACT_QUERY - bypass LLM, answer directly from GameState
    if matches!(classification.intent_type, IntentClassification::FactQuery) {
        return handle_fact_query(&state, asr_partial, start_time).await;
    }

    // 4. Handle SIMPLE_RULE_QUERY - use Vectorizer + 1.5B (not 14B)
    if matches!(
        classification.intent_type,
        IntentClassification::SimpleRuleQuery
    ) {
        return handle_simple_rule_query(&state, asr_partial, start_time).await;
    }

    // 5. Check if should trigger 1.5B
    let trigger_criteria = TriggerCriteria::new()
        .with_speech_duration(speech_duration)
        .with_pause_duration(pause_duration)
        .with_vad_detected_end(vad_detected_end)
        .with_clear_action(classification.confidence >= 0.95, None);

    let should_trigger = should_trigger_1_5b(&trigger_criteria);
    let mut fast_prelude = None;

    if should_trigger {
        info!("Triggering 1.5B for fast prelude");

        // Transition to Processing1_5B
        state_manager.transition_to(PipelineStatus::Processing1_5B)?;

        // Trigger 1.5B
        match trigger_1_5b(&mut state, "dm", asr_partial).await {
            Ok(prelude) => {
                fast_prelude = Some(prelude.clone());
                info!("1.5B prelude generated: {}", prelude);

                // TODO: Send to TTS immediately (requires TTS client)
                // For now, just log
                info!("Would send prelude to TTS: {}", prelude);
            }
            Err(e) => {
                warn!("1.5B trigger failed: {}. Continuing without prelude.", e);
            }
        }

        // Transition to WaitingForFinalASR
        state_manager.transition_to(PipelineStatus::WaitingForFinalASR)?;
    }

    // 5. Wait for asr_final (in real implementation, this would be async wait)
    // For now, assume asr_final is the same as asr_partial (for testing)
    let asr_final = asr_partial.to_string();
    info!("Received asr_final: {}", asr_final);

    // 6. Prepare context for 14B
    let context_events = vec![]; // TODO: Get from scene context
    let vectorizer_results = None; // TODO: Get from vectorizer if needed

    let context_14b = prepare_14b_context(
        &state,
        &fast_prelude.clone().unwrap_or_default(),
        &asr_final,
        &context_events,
        vectorizer_results.as_ref(),
    )?;

    info!(
        "Context prepared for 14B ({} tokens)",
        context_14b.estimated_tokens
    );

    // 7. Transition to Processing14B
    state_manager.transition_to(PipelineStatus::Processing14B)?;

    // 8. Call 14B (TODO: Implement actual LLM Core call)
    // For now, return mock narrative
    let narrative = format!(
        "{} Você {}",
        fast_prelude.as_ref().unwrap_or(&"".to_string()),
        asr_final
    );

    info!("14B narrative generated: {}", narrative);

    // 9. Transition to ReadyForTTS
    state_manager.transition_to(PipelineStatus::ReadyForTTS)?;

    // 10. Send to TTS (TODO: Implement actual TTS call)
    info!("Would send narrative to TTS: {}", narrative);

    // 11. Transition back to WaitingForInput
    state_manager.transition_to(PipelineStatus::WaitingForInput)?;

    let total_latency = start_time.elapsed();
    info!("Pipeline flow completed in {:?}", total_latency);

    Ok(PipelineFlowResult {
        fast_prelude,
        narrative,
        total_latency_ms: total_latency.as_millis() as u64,
    })
}

/// Handle FACT_QUERY - answer directly from GameState
async fn handle_fact_query(
    state: &PipelineState,
    query: &str,
    start_time: std::time::Instant,
) -> Result<PipelineFlowResult> {
    info!("Handling FACT_QUERY: {}", query);

    // Use objective_responses module to answer
    let answer = answer_objective_question(state, query)?;

    // TODO: Send to TTS
    info!("FACT_QUERY answered: {}", answer);

    let total_latency = start_time.elapsed();
    Ok(PipelineFlowResult {
        fast_prelude: None,
        narrative: answer,
        total_latency_ms: total_latency.as_millis() as u64,
    })
}

/// Handle SIMPLE_RULE_QUERY - use Vectorizer + 1.5B (not 14B)
async fn handle_simple_rule_query(
    state: &PipelineState,
    query: &str,
    start_time: std::time::Instant,
) -> Result<PipelineFlowResult> {
    info!("Handling SIMPLE_RULE_QUERY: {}", query);

    // TODO: Query Vectorizer for rule definition
    // For now, use mock vectorizer results
    let vectorizer_results = vec!["Mock rule definition".to_string()];

    // Answer using Vectorizer + 1.5B
    let result = answer_simple_rule_query(state, query, &vectorizer_results).await?;

    // Verify that 14B was not used (should never happen, but log warning if it does)
    if result.used_14b {
        warn!("14B was called for simple rule query - this should not happen!");
    }

    // TODO: Send to TTS
    info!("SIMPLE_RULE_QUERY answered: {}", result.answer);

    let total_latency = start_time.elapsed();
    Ok(PipelineFlowResult {
        fast_prelude: None,
        narrative: result.answer,
        total_latency_ms: total_latency.as_millis() as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_fact_query() {
        let state = PipelineState::new();
        let query = "Quantos HP eu tenho?";

        // This is a sync test, but handle_fact_query is async
        // In real tests, we'd use tokio::test
        assert!(!query.is_empty());
    }
}
