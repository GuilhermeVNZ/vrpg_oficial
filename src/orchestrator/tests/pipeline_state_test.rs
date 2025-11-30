//! Pipeline State Tests - M2.1
//! Tests for pipeline state management (waiting, processing_1_5b, waiting_final_asr, processing_14b)

use orchestrator::pipeline::{PipelineState, PipelineStatus};

#[test]
fn test_pipeline_status_enum() {
    // Test enum PipelineStatus
    // Verify all states exist
    let _waiting = PipelineStatus::WaitingForInput;
    let _processing_1_5b = PipelineStatus::Processing1_5B;
    let _waiting_final_asr = PipelineStatus::WaitingForFinalASR;
    let _processing_14b = PipelineStatus::Processing14B;
    let _ready_for_tts = PipelineStatus::ReadyForTTS;

    // Verify enum is serializable (if serde is enabled)
    // This will be tested when PipelineStatus is implemented
}

#[test]
fn test_pipeline_state_creation() {
    // Test PipelineState creation
    let state = PipelineState::new();

    // Verify structure is created correctly
    assert_eq!(state.status(), &PipelineStatus::WaitingForInput);

    // Verify fields are initialized
    assert!(state.game_state().is_empty());
    assert!(state.scene_context().is_empty());
    assert!(state.lore_cache().is_empty());
}

#[test]
fn test_pipeline_state_transitions_valid() {
    let mut state = PipelineState::new();

    // Test all valid transitions:
    // WaitingForInput → Processing1_5B
    assert!(state.transition_to(PipelineStatus::Processing1_5B).is_ok());
    assert_eq!(state.status(), &PipelineStatus::Processing1_5B);

    // Processing1_5B → WaitingForFinalASR
    assert!(state
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .is_ok());
    assert_eq!(state.status(), &PipelineStatus::WaitingForFinalASR);

    // WaitingForFinalASR → Processing14B
    assert!(state.transition_to(PipelineStatus::Processing14B).is_ok());
    assert_eq!(state.status(), &PipelineStatus::Processing14B);

    // Processing14B → ReadyForTTS
    assert!(state.transition_to(PipelineStatus::ReadyForTTS).is_ok());
    assert_eq!(state.status(), &PipelineStatus::ReadyForTTS);

    // ReadyForTTS → WaitingForInput
    assert!(state.transition_to(PipelineStatus::WaitingForInput).is_ok());
    assert_eq!(state.status(), &PipelineStatus::WaitingForInput);
}

#[test]
fn test_pipeline_state_transitions_invalid() {
    let mut state = PipelineState::new();

    // Test rejection of invalid transitions:
    // WaitingForInput → Processing14B (should be blocked)
    assert!(state.transition_to(PipelineStatus::Processing14B).is_err());
    assert_eq!(state.status(), &PipelineStatus::WaitingForInput);

    // Processing1_5B → ReadyForTTS (should be blocked)
    state.transition_to(PipelineStatus::Processing1_5B).unwrap();
    assert!(state.transition_to(PipelineStatus::ReadyForTTS).is_err());
    assert_eq!(state.status(), &PipelineStatus::Processing1_5B);

    // Processing14B → Processing1_5B (should be blocked)
    state
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();
    state.transition_to(PipelineStatus::Processing14B).unwrap();
    assert!(state.transition_to(PipelineStatus::Processing1_5B).is_err());
    assert_eq!(state.status(), &PipelineStatus::Processing14B);
}

#[test]
fn test_pipeline_state_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    // Test thread-safety
    let state = Arc::new(std::sync::Mutex::new(PipelineState::new()));
    let mut handles = vec![];

    // Create multiple threads accessing state
    for i in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let mut state = state_clone.lock().unwrap();
            // Try valid transition
            if i % 2 == 0 {
                let _ = state.transition_to(PipelineStatus::Processing1_5B);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify state is consistent (no panic, no corruption)
    let final_state = state.lock().unwrap();
    assert!(matches!(
        final_state.status(),
        PipelineStatus::WaitingForInput | PipelineStatus::Processing1_5B
    ));
}

#[test]
fn test_pipeline_state_persistence() {
    // Test state persistence
    let mut state = PipelineState::new();
    state.transition_to(PipelineStatus::Processing1_5B).unwrap();

    // Save state (serialize)
    let serialized = serde_json::to_string(&state).expect("Should serialize");

    // Load state (deserialize)
    let deserialized: PipelineState =
        serde_json::from_str(&serialized).expect("Should deserialize");

    // Verify state is restored correctly
    assert_eq!(deserialized.status(), state.status());
    assert_eq!(deserialized.game_state(), state.game_state());
    assert_eq!(deserialized.scene_context(), state.scene_context());
    assert_eq!(deserialized.lore_cache(), state.lore_cache());
}

#[test]
fn test_pipeline_state_game_state() {
    // Test game_state field
    let mut state = PipelineState::new();

    // Verify game_state can be updated
    state.update_game_state("test_state".to_string());
    assert_eq!(state.game_state(), "test_state");

    // Verify game_state is consulted correctly
    assert!(!state.game_state().is_empty());

    // Verify game_state is maintained in RAM (no persistence needed for this test)
}

#[test]
fn test_pipeline_state_scene_context() {
    // Test scene_context field
    let mut state = PipelineState::new();

    // Verify scene_context can be updated
    state.update_scene_context("test_context".to_string());
    assert_eq!(state.scene_context(), "test_context");

    // Verify scene_context is consulted correctly
    assert!(!state.scene_context().is_empty());

    // Verify integration with Vectorizer (will be tested in integration tests)
}

#[test]
fn test_pipeline_state_lore_cache() {
    // Test lore_cache field
    let mut state = PipelineState::new();

    // Verify lore_cache can be updated
    state.update_lore_cache("test_lore".to_string());
    assert_eq!(state.lore_cache(), "test_lore");

    // Verify lore_cache is consulted correctly
    assert!(!state.lore_cache().is_empty());

    // Verify integration with Vectorizer (will be tested in integration tests)
}














