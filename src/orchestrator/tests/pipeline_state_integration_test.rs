//! Pipeline State Integration Tests - M2.1
//! Integration tests for pipeline state with 1.5B, 14B, and ASR

use orchestrator::pipeline::{PipelineStateManager, PipelineStatus};

#[tokio::test]
async fn test_pipeline_state_with_1_5b() {
    // Test integration with 1.5B
    let manager = PipelineStateManager::new();

    // Verify state is updated when 1.5B starts
    assert!(manager
        .transition_to(PipelineStatus::Processing1_5B)
        .is_ok());
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::Processing1_5B);

    // Verify state is updated when 1.5B finishes
    assert!(manager
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .is_ok());
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::WaitingForFinalASR);

    // Verify transitions are correct
    assert!(manager.transition_to(PipelineStatus::Processing14B).is_ok());
}

#[tokio::test]
async fn test_pipeline_state_with_14b() {
    // Test integration with 14B
    let manager = PipelineStateManager::new();

    // Setup: go through pipeline to reach 14B
    manager
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();
    manager
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();

    // Verify state is updated when 14B starts
    assert!(manager.transition_to(PipelineStatus::Processing14B).is_ok());
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::Processing14B);

    // Verify state is updated when 14B finishes
    assert!(manager.transition_to(PipelineStatus::ReadyForTTS).is_ok());
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::ReadyForTTS);

    // Verify transitions are correct
    assert!(manager
        .transition_to(PipelineStatus::WaitingForInput)
        .is_ok());
}

#[tokio::test]
async fn test_pipeline_state_with_asr() {
    // Test integration with ASR
    let manager = PipelineStateManager::new();

    // Start with 1.5B processing
    manager
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();

    // Verify state is updated when ASR partial arrives (still processing 1.5B)
    // This is handled by the pipeline logic, not state transitions

    // Verify state is updated when ASR final arrives
    assert!(manager
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .is_ok());
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::WaitingForFinalASR);

    // Verify transitions are correct
    assert!(manager.transition_to(PipelineStatus::Processing14B).is_ok());
}

#[tokio::test]
async fn test_pipeline_state_full_cycle() {
    // Test full pipeline cycle
    let manager = PipelineStateManager::new();

    // Start: WaitingForInput
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::WaitingForInput);

    // Step 1: Trigger 1.5B
    manager
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::Processing1_5B);

    // Step 2: 1.5B finishes, wait for final ASR
    manager
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::WaitingForFinalASR);

    // Step 3: Final ASR arrives, process with 14B
    manager
        .transition_to(PipelineStatus::Processing14B)
        .unwrap();
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::Processing14B);

    // Step 4: 14B finishes, ready for TTS
    manager.transition_to(PipelineStatus::ReadyForTTS).unwrap();
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::ReadyForTTS);

    // Step 5: TTS completes, back to waiting
    manager
        .transition_to(PipelineStatus::WaitingForInput)
        .unwrap();
    let state = manager.get_state().unwrap();
    assert_eq!(state.status(), &PipelineStatus::WaitingForInput);
}


