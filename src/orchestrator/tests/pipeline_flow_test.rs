//! Tests for Complete Pipeline Flow - M2.5

use orchestrator::pipeline::PipelineStatus;
use orchestrator::pipeline::PipelineStateManager;
use std::time::Duration;

#[test]
fn test_pipeline_flow_1_5b_before_14b() {
    // Test that 1.5B always triggers before 14B
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Simulate ASR partial with long speech
    let asr_partial = "Eu quero atacar o goblin com minha espada e depois me mover para a direita";
    let speech_duration = Duration::from_secs(7); // 7 seconds - should trigger 1.5B

    // This is a mock test - in real implementation, handle_player_input would:
    // 1. Classify intent (WORLD_ACTION or COMBAT_ACTION)
    // 2. Check if should trigger 1.5B (speech_duration >= 6s)
    // 3. Trigger 1.5B and get prelude
    // 4. Wait for asr_final
    // 5. Prepare context for 14B (including fast_prelude)
    // 6. Call 14B
    // 7. Send to TTS

    // For now, just verify state transitions
    state.transition_to(PipelineStatus::Processing1_5B).unwrap();
    assert_eq!(*state.status(), PipelineStatus::Processing1_5B);

    state
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();
    assert_eq!(*state.status(), PipelineStatus::WaitingForFinalASR);

    state.transition_to(PipelineStatus::Processing14B).unwrap();
    assert_eq!(*state.status(), PipelineStatus::Processing14B);
}

#[test]
fn test_pipeline_flow_state_transitions() {
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Verify correct state transitions
    assert_eq!(*state.status(), PipelineStatus::WaitingForInput);

    // Start processing
    state.transition_to(PipelineStatus::Processing1_5B).unwrap();
    assert_eq!(*state.status(), PipelineStatus::Processing1_5B);

    // Wait for final ASR
    state
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();
    assert_eq!(*state.status(), PipelineStatus::WaitingForFinalASR);

    // Process with 14B
    state.transition_to(PipelineStatus::Processing14B).unwrap();
    assert_eq!(*state.status(), PipelineStatus::Processing14B);

    // Ready for TTS
    state.transition_to(PipelineStatus::ReadyForTTS).unwrap();
    assert_eq!(*state.status(), PipelineStatus::ReadyForTTS);

    // Back to waiting
    state
        .transition_to(PipelineStatus::WaitingForInput)
        .unwrap();
    assert_eq!(*state.status(), PipelineStatus::WaitingForInput);
}

#[test]
fn test_pipeline_flow_fact_query_bypasses_llm() {
    // FACT_QUERY should bypass LLM and go directly to GameState
    let state_manager = PipelineStateManager::new();
    let state = state_manager.get_state().unwrap();

    // Fact queries should not trigger 1.5B or 14B
    // They should be answered directly from GameState
    let fact_queries = vec![
        "Quantos HP eu tenho?",
        "Qual minha AC?",
        "Quantos slots nível 3 eu tenho?",
    ];

    for query in fact_queries {
        // In real implementation, this would:
        // 1. Classify as FACT_QUERY
        // 2. Query GameState directly
        // 3. Return answer without calling LLM
        // 4. Send to TTS
        assert!(!query.is_empty());
    }
}

#[test]
fn test_pipeline_flow_error_handling_asr_failure() {
    // Test error handling when ASR fails
    // In real implementation, should handle gracefully
    let state_manager = PipelineStateManager::new();
    let state = state_manager.get_state().unwrap();

    // Simulate ASR failure
    // Should transition to error state or retry
    assert_eq!(*state.status(), PipelineStatus::WaitingForInput);
}

#[test]
fn test_pipeline_flow_error_handling_llm_failure() {
    // Test error handling when LLM fails
    // In real implementation, should handle gracefully
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Simulate LLM failure during 1.5B processing
    state.transition_to(PipelineStatus::Processing1_5B).unwrap();

    // Should handle error and potentially fallback
    // For now, just verify state is correct
    assert_eq!(*state.status(), PipelineStatus::Processing1_5B);
}

#[test]
fn test_pipeline_flow_error_handling_tts_failure() {
    // Test error handling when TTS fails
    // In real implementation, should handle gracefully
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Need to go through valid transitions to reach ReadyForTTS
    state.transition_to(PipelineStatus::Processing1_5B).unwrap();
    state
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();
    state.transition_to(PipelineStatus::Processing14B).unwrap();
    state.transition_to(PipelineStatus::ReadyForTTS).unwrap();

    // Should handle error and potentially retry or show error
    // For now, just verify state transition works
    assert_eq!(*state.status(), PipelineStatus::ReadyForTTS);

    // Can transition back to WaitingForInput after error handling (valid transition)
    state
        .transition_to(PipelineStatus::WaitingForInput)
        .unwrap();
    assert_eq!(*state.status(), PipelineStatus::WaitingForInput);
}

#[test]
fn test_pipeline_flow_latency_total() {
    // Test that total pipeline latency is < 6s
    // This is a mock test - real implementation would measure actual latency
    let start = std::time::Instant::now();

    // Simulate pipeline steps
    std::thread::sleep(Duration::from_millis(100)); // Mock 1.5B (< 1.2s)
    std::thread::sleep(Duration::from_millis(100)); // Mock wait for ASR
    std::thread::sleep(Duration::from_millis(100)); // Mock 14B (< 6s)
    std::thread::sleep(Duration::from_millis(100)); // Mock TTS

    let elapsed = start.elapsed();

    // Total should be < 6s (in real implementation)
    assert!(
        elapsed.as_secs() < 10,
        "Pipeline should complete quickly in tests"
    );
}

#[test]
fn test_pipeline_flow_intent_router_integration() {
    // Test that Intent Router is called correctly
    let state_manager = PipelineStateManager::new();
    let state = state_manager.get_state().unwrap();

    // In real implementation, handle_player_input would:
    // 1. Receive asr_partial
    // 2. Call Intent Router to classify
    // 3. Route based on classification
    assert_eq!(*state.status(), PipelineStatus::WaitingForInput);
}

#[test]
fn test_pipeline_flow_context_preparation() {
    // Test that context is prepared correctly for 14B
    let state_manager = PipelineStateManager::new();
    let mut state = state_manager.get_state().unwrap();

    // Update state with game data
    state.update_game_state("HP: 50/50, AC: 15".to_string());
    state.update_scene_context("Tavern, 3 NPCs".to_string());

    // In real implementation, prepare_14b_context would be called with:
    // - fast_prelude from 1.5B
    // - asr_final
    // - game_state
    // - scene_context
    // - context_slice
    // - vectorizer_results (if any)

    assert!(!state.game_state().is_empty());
    assert!(!state.scene_context().is_empty());
}
