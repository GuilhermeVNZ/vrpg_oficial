//! Integration tests for 14B Context Preparation - M2.3

use orchestrator::pipeline::context_14b::{prepare_14b_context, ContextEvent, VectorizerResult};
use orchestrator::pipeline::PipelineStatus;
use orchestrator::pipeline::{PipelineState, PipelineStateManager};
use std::time::{Duration, SystemTime};

#[test]
fn test_context_14b_with_pipeline_state_transitions() {
    // Create pipeline state manager
    let manager = PipelineStateManager::new();

    // Transition to Processing1_5B
    manager
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();

    // Get state and update with game state
    let mut state = manager.get_state().unwrap();
    state.update_game_state("HP: 50/50, AC: 15".to_string());
    state.update_scene_context("Tavern, 3 NPCs".to_string());

    // Prepare context
    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack the goblin".to_string();

    let context = prepare_14b_context(&state, &fast_prelude, &asr_final, &[], None).unwrap();

    // Context should include game state and scene context
    assert!(context.full_context.contains("HP: 50/50"));
    assert!(context.full_context.contains("Tavern"));
}

#[test]
fn test_context_14b_with_full_pipeline_flow() {
    let mut state = PipelineState::new();

    // Simulate pipeline flow:
    // 1. 1.5B generates fast_prelude
    let fast_prelude = "A weight settles...".to_string();

    // 2. ASR final arrives
    let asr_final = "I want to attack the goblin with my sword".to_string();

    // 3. Update game state
    state.update_game_state("HP: 50/50, AC: 15, Weapon: Longsword".to_string());

    // 4. Update scene context
    state.update_scene_context("Combat: Goblin at (5, 3), Player at (3, 3)".to_string());

    // 5. Add context events (recent actions)
    let events = vec![
        ContextEvent {
            timestamp: SystemTime::now() - Duration::from_secs(10),
            event_type: "combat".to_string(),
            description: "Goblin attacked player, missed".to_string(),
        },
        ContextEvent {
            timestamp: SystemTime::now() - Duration::from_secs(5),
            event_type: "movement".to_string(),
            description: "Player moved to (3, 3)".to_string(),
        },
    ];

    // 6. Add vectorizer results (lore/rule query)
    let vectorizer_results = Some(VectorizerResult {
        query: "goblin stats".to_string(),
        results: vec![
            "Goblin: AC 15, HP 7, Speed 30ft".to_string(),
            "Goblin attacks with scimitar (+4 to hit, 1d6+2 damage)".to_string(),
        ],
    });

    // 7. Prepare context for 14B
    let context = prepare_14b_context(
        &state,
        &fast_prelude,
        &asr_final,
        &events,
        vectorizer_results.as_ref(),
    )
    .unwrap();

    // Verify all components are included
    assert!(context.full_context.contains(&fast_prelude));
    assert!(context.full_context.contains(&asr_final));
    assert!(context.full_context.contains("HP: 50/50"));
    assert!(context.full_context.contains("Combat: Goblin"));
    assert!(context.full_context.contains("Goblin attacked player"));
    assert!(context.full_context.contains("Goblin: AC 15"));

    // Verify structure
    assert!(context.estimated_tokens > 0);
    assert!(context.estimated_tokens <= 8192);
    assert_eq!(context.events_count, 2);
    assert!(context.has_vectorizer_results);
}

#[test]
fn test_context_14b_with_large_context_truncation() {
    let mut state = PipelineState::new();

    // Create very large game state
    let large_game_state = format!(
        "HP: 50/50, AC: 15, Position: (5, 3), Inventory: {}",
        "Item, ".repeat(500)
    );
    state.update_game_state(large_game_state);

    // Create many events
    let mut events = Vec::new();
    for i in 0..50 {
        events.push(ContextEvent {
            timestamp: SystemTime::now() - Duration::from_secs(i as u64),
            event_type: "action".to_string(),
            description: format!("Event {}: Player did something very detailed and long", i),
        });
    }

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    let context = prepare_14b_context(&state, &fast_prelude, &asr_final, &events, None).unwrap();

    // Fast prelude should always be included
    assert!(context.full_context.contains(&fast_prelude));

    // ASR final should always be included
    assert!(context.full_context.contains(&asr_final));

    // Context should not exceed token limit
    assert!(context.estimated_tokens <= 8192);
}

#[test]
fn test_context_14b_with_vectorizer_only() {
    let state = PipelineState::new();

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "What are the rules for stealth?".to_string();

    let vectorizer_results = Some(VectorizerResult {
        query: "stealth rules".to_string(),
        results: vec![
            "Stealth is a Dexterity (Stealth) check".to_string(),
            "You can hide when you are heavily obscured or behind cover".to_string(),
            "When you make an attack, you reveal your position".to_string(),
        ],
    });

    let context = prepare_14b_context(
        &state,
        &fast_prelude,
        &asr_final,
        &[],
        vectorizer_results.as_ref(),
    )
    .unwrap();

    // Should include vectorizer results
    assert!(context.full_context.contains("Stealth is a Dexterity"));
    assert!(context.has_vectorizer_results);
}

#[test]
fn test_context_14b_event_prioritization() {
    let state = PipelineState::new();

    let fast_prelude = "A weight settles...".to_string();
    let asr_final = "I want to attack".to_string();

    // Create events with different timestamps
    let now = SystemTime::now();
    let events = vec![
        ContextEvent {
            timestamp: now - Duration::from_secs(60),
            event_type: "action".to_string(),
            description: "OLDEST: Player entered the room".to_string(),
        },
        ContextEvent {
            timestamp: now - Duration::from_secs(30),
            event_type: "action".to_string(),
            description: "MIDDLE: Player moved to position (3, 3)".to_string(),
        },
        ContextEvent {
            timestamp: now - Duration::from_secs(10),
            event_type: "action".to_string(),
            description: "RECENT: Player drew weapon".to_string(),
        },
        ContextEvent {
            timestamp: now,
            event_type: "action".to_string(),
            description: "MOST_RECENT: Player is ready to attack".to_string(),
        },
    ];

    let context = prepare_14b_context(&state, &fast_prelude, &asr_final, &events, None).unwrap();

    // Most recent events should appear first
    let context_str = context.full_context;
    let most_recent_pos = context_str.find("MOST_RECENT").unwrap_or(0);
    let recent_pos = context_str.find("RECENT").unwrap_or(0);
    let middle_pos = context_str.find("MIDDLE").unwrap_or(0);
    let oldest_pos = context_str.find("OLDEST").unwrap_or(0);

    assert!(
        most_recent_pos < recent_pos && recent_pos < middle_pos && middle_pos < oldest_pos,
        "Events should be ordered by recency (most recent first)"
    );
}














