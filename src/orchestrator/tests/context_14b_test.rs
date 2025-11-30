//! Tests for 14B Context Preparation - M2.3

use orchestrator::pipeline::context_14b::{prepare_14b_context, ContextEvent, VectorizerResult};
use orchestrator::pipeline::PipelineState;
use std::time::{Duration, SystemTime};

#[test]
fn test_prepare_14b_context_includes_fast_prelude() {
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/50, AC: 15".to_string());
    pipeline_state.update_scene_context("Tavern, 3 NPCs present".to_string());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack the goblin".to_string();

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &[], None).unwrap();

    // Fast prelude should always be included
    assert!(
        context.full_context.contains(&fast_prelude),
        "Fast prelude should be included in context"
    );
}

#[test]
fn test_prepare_14b_context_includes_asr_final() {
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/50".to_string());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack the goblin".to_string();

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &[], None).unwrap();

    // ASR final should always be included
    assert!(
        context.full_context.contains(&asr_final),
        "ASR final should be included in context"
    );
}

#[test]
fn test_prepare_14b_context_includes_game_state() {
    let mut pipeline_state = PipelineState::new();
    let game_state = "HP: 50/50, AC: 15, Position: (5, 3)".to_string();
    pipeline_state.update_game_state(game_state.clone());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &[], None).unwrap();

    // Game state should be included
    assert!(
        context.full_context.contains(&game_state),
        "Game state should be included in context"
    );
}

#[test]
fn test_prepare_14b_context_includes_scene_context() {
    let mut pipeline_state = PipelineState::new();
    let scene_context = "Tavern, 3 NPCs present, dim lighting".to_string();
    pipeline_state.update_scene_context(scene_context.clone());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &[], None).unwrap();

    // Scene context should be included
    assert!(
        context.full_context.contains(&scene_context),
        "Scene context should be included in context"
    );
}

#[test]
fn test_prepare_14b_context_includes_context_slice() {
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/50".to_string());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    let events = vec![
        ContextEvent {
            timestamp: SystemTime::now(),
            event_type: "action".to_string(),
            description: "Player moved to position (5, 3)".to_string(),
        },
        ContextEvent {
            timestamp: SystemTime::now(),
            event_type: "roll".to_string(),
            description: "Rolled 15 for Perception".to_string(),
        },
        ContextEvent {
            timestamp: SystemTime::now(),
            event_type: "dialogue".to_string(),
            description: "NPC said: 'Welcome to the tavern'".to_string(),
        },
    ];

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &events, None).unwrap();

    // Context slice events should be included
    for event in &events {
        assert!(
            context.full_context.contains(&event.description),
            "Event should be included in context: {}",
            event.description
        );
    }
}

#[test]
fn test_prepare_14b_context_includes_vectorizer_results() {
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/50".to_string());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    let vectorizer_results = Some(VectorizerResult {
        query: "goblin stats".to_string(),
        results: vec![
            "Goblin: AC 15, HP 7, Speed 30ft".to_string(),
            "Goblin attacks with scimitar".to_string(),
        ],
    });

    let context = prepare_14b_context(
        &pipeline_state,
        &fast_prelude,
        &asr_final,
        &[],
        vectorizer_results.as_ref(),
    )
    .unwrap();

    // Vectorizer results should be included
    assert!(
        context.full_context.contains("Goblin: AC 15"),
        "Vectorizer results should be included in context"
    );
}

#[test]
fn test_prepare_14b_context_prioritizes_recent_events() {
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/50".to_string());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    // Create events with different timestamps (oldest first)
    let now = SystemTime::now();
    let events = vec![
        ContextEvent {
            timestamp: now - Duration::from_secs(60), // 1 minute ago
            event_type: "action".to_string(),
            description: "OLD: Player moved to position (1, 1)".to_string(),
        },
        ContextEvent {
            timestamp: now - Duration::from_secs(30), // 30 seconds ago
            event_type: "action".to_string(),
            description: "MIDDLE: Player moved to position (3, 3)".to_string(),
        },
        ContextEvent {
            timestamp: now, // Now
            event_type: "action".to_string(),
            description: "RECENT: Player moved to position (5, 5)".to_string(),
        },
    ];

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &events, None).unwrap();

    // Recent events should appear first in context
    let context_str = context.full_context;
    let recent_pos = context_str.find("RECENT").unwrap_or(0);
    let middle_pos = context_str.find("MIDDLE").unwrap_or(0);
    let old_pos = context_str.find("OLD").unwrap_or(0);

    assert!(
        recent_pos < middle_pos && middle_pos < old_pos,
        "Recent events should appear before older events"
    );
}

#[test]
fn test_prepare_14b_context_limits_tokens() {
    let mut pipeline_state = PipelineState::new();
    // Create very large game state
    let large_game_state = "HP: 50/50, ".repeat(1000);
    pipeline_state.update_game_state(large_game_state);
    pipeline_state.update_scene_context("Scene context ".repeat(1000));

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    // Create many events
    let mut events = Vec::new();
    for i in 0..100 {
        events.push(ContextEvent {
            timestamp: SystemTime::now(),
            event_type: "action".to_string(),
            description: format!("Event {}: Player did something", i),
        });
    }

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &events, None).unwrap();

    // Context should not exceed token limit (8192 tokens ≈ 6000 words)
    // Estimate: ~4 characters per token, so 8192 tokens ≈ 32768 characters
    let estimated_tokens = context.full_context.len() / 4;
    assert!(
        estimated_tokens <= 8192,
        "Context should not exceed 8192 tokens. Estimated: {}",
        estimated_tokens
    );
}

#[test]
fn test_prepare_14b_context_always_includes_fast_prelude_even_when_large() {
    let mut pipeline_state = PipelineState::new();
    // Create very large game state
    let large_game_state = "HP: 50/50, ".repeat(1000);
    pipeline_state.update_game_state(large_game_state);

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    // Create many events
    let mut events = Vec::new();
    for i in 0..100 {
        events.push(ContextEvent {
            timestamp: SystemTime::now(),
            event_type: "action".to_string(),
            description: format!("Event {}: Player did something", i),
        });
    }

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &events, None).unwrap();

    // Fast prelude should always be included, even when context is large
    assert!(
        context.full_context.contains(&fast_prelude),
        "Fast prelude should always be included, even when context is large"
    );
}

#[test]
fn test_prepare_14b_context_structure() {
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/50".to_string());

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &[], None).unwrap();

    // Context should have all required fields
    assert!(
        !context.full_context.is_empty(),
        "Context should not be empty"
    );
    assert!(
        context.estimated_tokens > 0,
        "Estimated tokens should be > 0"
    );
    assert!(
        context.estimated_tokens <= 8192,
        "Estimated tokens should not exceed 8192"
    );
}

#[test]
fn test_prepare_14b_context_with_empty_inputs() {
    let pipeline_state = PipelineState::new();

    let fast_prelude = "".to_string();
    let asr_final = "".to_string();

    let context =
        prepare_14b_context(&pipeline_state, &fast_prelude, &asr_final, &[], None).unwrap();

    // Should still create valid context even with empty inputs
    assert!(!context.full_context.is_empty() || context.estimated_tokens == 0);
}
