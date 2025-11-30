//! 14B Context Preparation - M2.3
//! Prepares complete context for Qwen-14B, including fast_prelude from 1.5B

use crate::error::Result;
use crate::pipeline::PipelineState;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Context event (from scene context slice)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEvent {
    /// Timestamp of the event
    pub timestamp: SystemTime,
    /// Type of event (action, roll, dialogue, etc.)
    pub event_type: String,
    /// Description of the event
    pub description: String,
}

/// Vectorizer search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizerResult {
    /// Original query
    pub query: String,
    /// Search results
    pub results: Vec<String>,
}

/// Prepared context for 14B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context14B {
    /// Full context string (ready for LLM)
    pub full_context: String,
    /// Estimated token count
    pub estimated_tokens: usize,
    /// Fast prelude included
    pub fast_prelude: String,
    /// ASR final included
    pub asr_final: String,
    /// Game state included
    pub game_state: String,
    /// Scene context included
    pub scene_context: String,
    /// Number of events from context slice
    pub events_count: usize,
    /// Vectorizer results included
    pub has_vectorizer_results: bool,
}

/// Maximum tokens for 14B context (8192)
const MAX_TOKENS: usize = 8192;

/// Estimate tokens from text (rough: ~4 characters per token)
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

/// Prepare complete context for 14B
pub fn prepare_14b_context(
    pipeline_state: &PipelineState,
    fast_prelude: &str,
    asr_final: &str,
    context_slice: &[ContextEvent],
    vectorizer_results: Option<&VectorizerResult>,
) -> Result<Context14B> {
    // 1. Always include fast_prelude (highest priority)
    let mut context_parts = vec![format!("[FAST_PRELUDE]\n{}\n[/FAST_PRELUDE]", fast_prelude)];

    // 2. Always include asr_final
    context_parts.push(format!("[ASR_FINAL]\n{}\n[/ASR_FINAL]", asr_final));

    // 3. Include game_state
    let game_state = pipeline_state.game_state();
    if !game_state.is_empty() {
        context_parts.push(format!("[GAME_STATE]\n{}\n[/GAME_STATE]", game_state));
    }

    // 4. Include scene_context
    let scene_context = pipeline_state.scene_context();
    if !scene_context.is_empty() {
        context_parts.push(format!(
            "[SCENE_CONTEXT]\n{}\n[/SCENE_CONTEXT]",
            scene_context
        ));
    }

    // 5. Include context_slice (prioritize recent events)
    // Sort events by timestamp (most recent first)
    let mut sorted_events: Vec<&ContextEvent> = context_slice.iter().collect();
    sorted_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Limit to last 6 events (as per spec: "últimos 3-6 eventos")
    let events_to_include = sorted_events.iter().take(6);
    let mut events_text = Vec::new();
    for event in events_to_include {
        events_text.push(format!("[{}] {}", event.event_type, event.description));
    }
    if !events_text.is_empty() {
        context_parts.push(format!(
            "[CONTEXT_SLICE]\n{}\n[/CONTEXT_SLICE]",
            events_text.join("\n")
        ));
    }

    // 6. Include vectorizer_results if available
    if let Some(vr) = vectorizer_results {
        let results_text = vr.results.join("\n");
        context_parts.push(format!(
            "[VECTORIZER_RESULTS]\nQuery: {}\n{}\n[/VECTORIZER_RESULTS]",
            vr.query, results_text
        ));
    }

    // 7. Combine all parts
    let mut full_context = context_parts.join("\n\n");

    // 8. Check token limit and truncate if necessary
    let estimated_tokens = estimate_tokens(&full_context);
    if estimated_tokens > MAX_TOKENS {
        // Truncate while preserving priority order
        // Keep: fast_prelude, asr_final, then truncate from the end
        let target_chars = MAX_TOKENS * 4; // ~4 chars per token

        // Extract fast_prelude and asr_final (must keep)
        let fast_prelude_section = format!("[FAST_PRELUDE]\n{}\n[/FAST_PRELUDE]", fast_prelude);
        let asr_final_section = format!("[ASR_FINAL]\n{}\n[/ASR_FINAL]", asr_final);

        let mut truncated = fast_prelude_section.clone();
        truncated.push_str("\n\n");
        truncated.push_str(&asr_final_section);

        // Add other parts until we hit the limit
        let mut remaining_chars = target_chars - truncated.len();
        for part in context_parts.iter().skip(2) {
            // Skip fast_prelude and asr_final (already added)
            if remaining_chars > part.len() + 2 {
                truncated.push_str("\n\n");
                truncated.push_str(part);
                remaining_chars -= part.len() + 2;
            } else {
                break;
            }
        }

        full_context = truncated;
    }

    Ok(Context14B {
        full_context: full_context.clone(),
        estimated_tokens: estimate_tokens(&full_context),
        fast_prelude: fast_prelude.to_string(),
        asr_final: asr_final.to_string(),
        game_state: game_state.to_string(),
        scene_context: scene_context.to_string(),
        events_count: sorted_events.len().min(6),
        has_vectorizer_results: vectorizer_results.is_some(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        let text = "Hello world";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0, "Should estimate tokens");
    }

    #[test]
    fn test_prepare_14b_context_basic() {
        let pipeline_state = PipelineState::new();
        let fast_prelude = "A weight settles...";
        let asr_final = "I want to attack";

        let context =
            prepare_14b_context(&pipeline_state, fast_prelude, asr_final, &[], None).unwrap();

        assert!(context.full_context.contains(fast_prelude));
        assert!(context.full_context.contains(asr_final));
    }
}


