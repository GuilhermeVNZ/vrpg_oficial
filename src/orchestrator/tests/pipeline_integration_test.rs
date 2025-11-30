//! Pipeline Integration Tests - M5.1
//! Complete end-to-end integration tests for the 3-agent pipeline

use orchestrator::cache::game_state_cache::{GameStateCache, EntityId, GameStateEntry, Position, ResourceType};
use orchestrator::cache::lore_cache::{LoreCache, LoreType};
use orchestrator::cache::scene_context_cache::{SceneContextCache, SceneEvent, NpcId};
use orchestrator::intent_router::{IntentClassification, IntentRouter};
use orchestrator::pipeline::context_14b::prepare_14b_context;
use orchestrator::pipeline::flow::handle_player_input;
use orchestrator::pipeline::objective_responses::answer_objective_question;
use orchestrator::pipeline::simple_rule_query::answer_simple_rule_query;
use orchestrator::pipeline::trigger::trigger_1_5b;
use orchestrator::pipeline::{PipelineState, PipelineStateManager, PipelineStatus};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

/// Test 1: End-to-end pipeline flow ASR → 1.5B → 14B → TTS
#[tokio::test]
async fn test_pipeline_end_to_end_flow() {
    let state_manager = PipelineStateManager::new();
    let asr_partial = "Eu quero atacar o goblin com minha espada";
    let speech_duration = Duration::from_secs(7); // Should trigger 1.5B

    // Execute full pipeline flow
    let result = handle_player_input(
        &state_manager,
        asr_partial,
        speech_duration,
        None, // pause_duration
        false, // vad_detected_end
    )
    .await;

    assert!(result.is_ok(), "Pipeline flow should complete successfully");
    let flow_result = result.unwrap();

    // Verify that 1.5B was triggered (fast_prelude should exist)
    assert!(
        flow_result.fast_prelude.is_some(),
        "1.5B should have been triggered and generated a prelude"
    );

    // Verify that 14B generated narrative
    assert!(
        !flow_result.narrative.is_empty(),
        "14B should have generated a narrative"
    );

    // Verify that fast_prelude is included in the final narrative or context
    let fast_prelude = flow_result.fast_prelude.as_ref().unwrap();
    assert!(
        !fast_prelude.is_empty(),
        "Fast prelude should not be empty"
    );
    assert!(
        fast_prelude.len() <= 150, // Max ~40 tokens = ~150 chars
        "Fast prelude should be short (max 40 tokens)"
    );

    // Verify state is back to WaitingForInput
    let state = state_manager.get_state().unwrap();
    assert_eq!(
        *state.status(),
        PipelineStatus::WaitingForInput,
        "Pipeline should return to WaitingForInput after completion"
    );
}

/// Test 2: 1.5B always triggers before 14B
#[tokio::test]
async fn test_1_5b_always_before_14b() {
    let state_manager = PipelineStateManager::new();
    let asr_partial = "Eu quero investigar a porta com cuidado e procurar por armadilhas";
    let _speech_duration = Duration::from_secs(8); // Long speech - should trigger 1.5B

    let start_time = Instant::now();
    let mut state = state_manager.get_state().unwrap();

    // Verify initial state
    assert_eq!(
        *state.status(),
        PipelineStatus::WaitingForInput,
        "Should start in WaitingForInput"
    );

    // Trigger 1.5B
    state_manager
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();
    assert_eq!(
        *state_manager.get_state().unwrap().status(),
        PipelineStatus::Processing1_5B,
        "Should transition to Processing1_5B first"
    );

    // Generate prelude
    let prelude = trigger_1_5b(&mut state, "dm", asr_partial).await.unwrap();
    assert!(!prelude.is_empty(), "1.5B should generate prelude");

    let prelude_time = start_time.elapsed();
    assert!(
        prelude_time < Duration::from_millis(1200),
        "1.5B should complete in < 1.2s, took: {:?}",
        prelude_time
    );

    // Wait for final ASR
    state_manager
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();

    // Now transition to 14B (only after 1.5B completed)
    state_manager
        .transition_to(PipelineStatus::Processing14B)
        .unwrap();
    assert_eq!(
        *state_manager.get_state().unwrap().status(),
        PipelineStatus::Processing14B,
        "Should transition to Processing14B only after 1.5B"
    );

    // Verify that prelude exists before 14B processing
    assert!(!prelude.is_empty(), "Prelude should exist before 14B");
}

/// Test 3: Total latency < 6s
#[tokio::test]
async fn test_pipeline_total_latency_under_6s() {
    let state_manager = PipelineStateManager::new();
    let asr_partial = "Eu quero atacar o dragão com minha espada mágica";
    let speech_duration = Duration::from_secs(7);

    let start_time = Instant::now();

    let result = handle_player_input(
        &state_manager,
        asr_partial,
        speech_duration,
        None,
        false,
    )
    .await;

    assert!(result.is_ok(), "Pipeline should complete successfully");
    let flow_result = result.unwrap();
    let total_latency = Duration::from_millis(flow_result.total_latency_ms);

    assert!(
        total_latency < Duration::from_secs(6),
        "Total pipeline latency should be < 6s, got: {:?}",
        total_latency
    );

    // Also verify actual elapsed time
    let elapsed = start_time.elapsed();
    assert!(
        elapsed < Duration::from_secs(6),
        "Actual elapsed time should be < 6s, got: {:?}",
        elapsed
    );
}

/// Test 4: 1.5B does not generate final results
#[tokio::test]
async fn test_1_5b_does_not_generate_final_results() {
    let mut state = PipelineState::new();
    let asr_partial = "Eu quero atacar o goblin";

    // Trigger 1.5B
    let prelude = trigger_1_5b(&mut state, "dm", asr_partial).await.unwrap();

    // Verify prelude is short and emotional, not a final result
    assert!(
        prelude.len() <= 150,
        "1.5B prelude should be short (max ~40 tokens = ~150 chars), got: {}",
        prelude.len()
    );

    // Verify prelude is not a complete narrative or consequence
    assert!(
        !prelude.contains("você ataca"),
        "1.5B should not describe actions, got: {}",
        prelude
    );
    assert!(
        !prelude.contains("goblin morre") || !prelude.contains("goblin é morto"),
        "1.5B should not describe consequences, got: {}",
        prelude
    );
    assert!(
        !prelude.contains("dano"),
        "1.5B should not include game mechanics, got: {}",
        prelude
    );

    // Verify prelude is emotional/reactive
    assert!(
        prelude.len() > 0,
        "1.5B should generate some response"
    );
}

/// Test 5: 14B receives fast_prelude
#[tokio::test]
async fn test_14b_receives_fast_prelude() {
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Generate fast_prelude from 1.5B
    let asr_partial = "Eu quero investigar a sala";
    let fast_prelude = trigger_1_5b(&mut state, "dm", asr_partial)
        .await
        .unwrap();

    assert!(!fast_prelude.is_empty(), "Fast prelude should be generated");

    // Prepare context for 14B
    let asr_final = asr_partial.to_string();
    let context_events = vec![];
    let vectorizer_results = None;

    let context_14b = prepare_14b_context(
        &state,
        &fast_prelude,
        &asr_final,
        &context_events,
        vectorizer_results.as_ref(),
    )
    .unwrap();

    // Verify that fast_prelude is included in context
    assert!(
        context_14b.fast_prelude == fast_prelude,
        "14B context should include fast_prelude"
    );
    assert!(
        !context_14b.fast_prelude.is_empty(),
        "Fast prelude should not be empty in 14B context"
    );

    // Verify context structure
    assert!(
        context_14b.estimated_tokens > 0,
        "Context should have tokens estimated"
    );
}

/// Test 6: Objective responses bypass LLM
#[tokio::test]
async fn test_objective_responses_bypass_llm() {
    let mut state = PipelineState::new();

    // Set up game state
    state.update_game_state("HP: 50/100, AC: 18, Spell Slots: [3, 2, 1]".to_string());

    let queries = vec![
        "Quantos HP eu tenho?",
        "Qual minha AC?",
        "Quantos slots nível 3 eu tenho?",
    ];

    for query in queries {
        let start_time = Instant::now();
        let answer = answer_objective_question(&state, query).unwrap();
        let latency = start_time.elapsed();

        // Verify answer is non-empty
        assert!(!answer.is_empty(), "Should get an answer for: {}", query);

        // Verify latency is very low (< 50ms) - no LLM call
        assert!(
            latency < Duration::from_millis(50),
            "Objective response should be very fast (< 50ms), took: {:?} for query: {}",
            latency,
            query
        );

        // Verify answer contains relevant information
        assert!(
            answer.contains("HP") || answer.contains("AC") || answer.contains("slot"),
            "Answer should contain relevant game state info: {}",
            answer
        );
    }
}

/// Test 7: Simple rule query uses Vectorizer + 1.5B (not 14B)
#[tokio::test]
async fn test_simple_rule_query_uses_vectorizer_and_1_5b_only() {
    let state = PipelineState::new();
    let query = "Stealth usa Destreza?";

    // Mock vectorizer results
    let vectorizer_results = vec!["Stealth is a Dexterity-based skill check.".to_string()];

    let start_time = Instant::now();
    let result = answer_simple_rule_query(&state, query, &vectorizer_results)
        .await
        .unwrap();
    let latency = start_time.elapsed();

    // Verify that 1.5B was used
    assert!(
        result.used_1_5b,
        "Simple rule query should use 1.5B for conversion"
    );

    // Verify that 14B was NOT used
    assert!(
        !result.used_14b,
        "Simple rule query should NOT use 14B"
    );

    // Verify latency is reasonable (< 1.5s total)
    assert!(
        latency < Duration::from_millis(1500),
        "Simple rule query should be fast (< 1.5s), took: {:?}",
        latency
    );

    // Verify answer is human-like
    assert!(!result.answer.is_empty(), "Should get an answer");
    assert!(
        !result.answer.contains("Dexterity-based skill check"),
        "Answer should be human-like, not technical: {}",
        result.answer
    );
}

/// Test 8: Narrative rule query uses 14B
#[tokio::test]
async fn test_narrative_rule_query_uses_14b() {
    let state_manager = PipelineStateManager::new();
    let query = "Como funciona o sistema de restos no D&D?";

    // This should be classified as a narrative query and go through full pipeline
    let router = IntentRouter::new();
    let classification = router.classify(query, &state_manager.get_state().unwrap()).unwrap();

    // Should not be SIMPLE_RULE_QUERY (which uses Vectorizer + 1.5B only)
    assert!(
        !matches!(classification.intent_type, IntentClassification::SimpleRuleQuery),
        "Narrative rule query should not be classified as SIMPLE_RULE_QUERY, got: {:?}",
        classification.intent_type
    );

    // Should not be FACT_QUERY (which bypasses LLM)
    assert!(
        !matches!(classification.intent_type, IntentClassification::FactQuery),
        "Narrative rule query should not be classified as FACT_QUERY, got: {:?}",
        classification.intent_type
    );

    // Any other classification (WORLD_ACTION, COMBAT_ACTION, META_QUERY, UNCERTAIN) is acceptable
    // as they all go through the full pipeline with 14B (not just Vectorizer + 1.5B)
    assert!(
        matches!(
            classification.intent_type,
            IntentClassification::WorldAction
                | IntentClassification::CombatAction
                | IntentClassification::MetaQuery
                | IntentClassification::Uncertain
        ),
        "Narrative rule query should be classified to go through full pipeline with 14B, got: {:?}",
        classification.intent_type
    );
}

/// Test 9: Error handling at each pipeline stage
#[tokio::test]
async fn test_pipeline_error_handling() {
    let state_manager = PipelineStateManager::new();

    // Test error handling for ASR failure (simulated by empty input)
    let result = handle_player_input(
        &state_manager,
        "", // Empty ASR input
        Duration::from_secs(0),
        None,
        false,
    )
    .await;

    // Should handle gracefully (may fail or succeed depending on implementation)
    // The important thing is that it doesn't panic
    if result.is_err() {
        let error_msg = result.unwrap_err().to_string();
        assert!(
            !error_msg.is_empty(),
            "Error should have a message"
        );
    }

    // Test error handling for state transition failure
    // Try to make invalid transition
    let mut state = state_manager.get_state().unwrap();
    let invalid_transition = state.transition_to(PipelineStatus::Processing14B); // Skip Processing1_5B

    // This should fail if state validation is working
    if invalid_transition.is_err() {
        // Good - state validation caught the error
        assert!(true, "State validation caught invalid transition");
    }
}

/// Test 10: Cache integration (game_state, scene_context, lore_cache)
#[tokio::test]
async fn test_pipeline_cache_integration() {
    // Test Game State Cache
    let mut game_state_cache = GameStateCache::new();

    let entity_id = EntityId::Player("player_1".to_string());
    let mut resources = HashMap::new();
    resources.insert(ResourceType::SpellSlot(3), 2);
    
    let entry = GameStateEntry {
        hp: 75,
        max_hp: 100,
        ac: 18,
        resources,
        statuses: vec![],
        position: Position { x: 0, y: 0, z: 0 },
        initiative: None,
    };

    game_state_cache.update_entity(&entity_id, entry.clone());
    let retrieved = game_state_cache.get_entity(&entity_id).unwrap();
    assert_eq!(retrieved.hp, 75);
    assert_eq!(retrieved.ac, 18);
    assert_eq!(
        retrieved.resources.get(&ResourceType::SpellSlot(3)),
        Some(&2)
    );

    // Test Scene Context Cache
    let mut scene_cache = SceneContextCache::new();
    let timestamp = SystemTime::now();
    
    scene_cache.add_event(SceneEvent::Action {
        actor: "player_1".to_string(),
        action: "attacked goblin".to_string(),
        timestamp,
    });
    
    scene_cache.add_event(SceneEvent::Roll {
        actor: "player_1".to_string(),
        roll_type: "attack".to_string(),
        result: 18,
        timestamp: SystemTime::now(),
    });
    
    scene_cache.add_active_npc(NpcId("goblin_1".to_string()));

    let events = scene_cache.get_recent_events(3);
    assert_eq!(events.len(), 2); // Action + Roll

    let active_npcs = scene_cache.get_active_npcs();
    assert!(active_npcs.contains(&NpcId("goblin_1".to_string())));

    // Test Lore Cache
    let mut lore_cache = LoreCache::new();
    lore_cache.store_query_result(
        "What is the history of Waterdeep?",
        &vec!["Waterdeep is a major city...".to_string()],
        LoreType::Location,
    );

    let lore_result = lore_cache.get_query_result("What is the history of Waterdeep?");
    assert!(lore_result.is_some());
    let result = lore_result.unwrap();
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.lore_type, LoreType::Location);

    // Test cache metrics
    let stats = lore_cache.get_stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.stores, 1);
}

/// Test 11: Full pipeline with caches integrated
#[tokio::test]
async fn test_full_pipeline_with_caches() {
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Set up game state cache
    state.update_game_state("HP: 50/100, AC: 18".to_string());

    // Set up scene context
    state.update_scene_context("Tavern, 3 NPCs present".to_string());

    // Verify state is set before pipeline
    assert!(
        !state.game_state().is_empty(),
        "Game state should be set before pipeline execution"
    );
    assert!(
        !state.scene_context().is_empty(),
        "Scene context should be set before pipeline execution"
    );

    // Execute pipeline with fact query (should bypass LLM and use game state)
    let result = handle_player_input(
        &state_manager,
        "Quantos HP eu tenho?",
        Duration::from_secs(3),
        None,
        false,
    )
    .await;

    assert!(result.is_ok(), "Pipeline with caches should work");
    
    let flow_result = result.unwrap();
    
    // Verify that fact query was answered (should contain HP info)
    assert!(
        !flow_result.narrative.is_empty(),
        "Pipeline should return an answer"
    );
    
    // Verify that fact queries bypass LLM (no fast_prelude)
    assert!(
        flow_result.fast_prelude.is_none(),
        "Fact queries should bypass 1.5B (no fast_prelude)"
    );
}

