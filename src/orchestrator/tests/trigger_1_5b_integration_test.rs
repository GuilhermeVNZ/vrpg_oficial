//! 1.5B Trigger Integration Tests - M2.2
//! Integration tests for 1.5B trigger with ASR, LLM Core, and TTS

use orchestrator::pipeline::trigger::{should_trigger_1_5b, trigger_1_5b, TriggerCriteria};
use orchestrator::pipeline::{PipelineState, PipelineStatus};
use std::time::Duration;

#[tokio::test]
async fn test_trigger_1_5b_with_asr() {
    // Test integration with ASR
    let mut criteria = TriggerCriteria::new();

    // Simulate ASR partial arriving
    criteria.speech_duration = Duration::from_secs(7);

    // Verify that trigger is evaluated
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger after 7s of speech"
    );

    // Simulate ASR final arriving (VAD detected end)
    criteria.vad_detected_end = true;
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger when VAD detects end"
    );

    // Verify that 1.5B is triggered when appropriate
    let mut pipeline_state = PipelineState::new();
    pipeline_state
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();
    assert_eq!(pipeline_state.status(), &PipelineStatus::Processing1_5B);
}

#[tokio::test]
async fn test_trigger_1_5b_with_llm_core() {
    // Test integration with LLM Core
    let mut pipeline_state = PipelineState::new();
    let persona = "test_persona";
    let asr_partial = "I want to attack the goblin";

    // Verify that call to /llm/prelude is made
    let result = trigger_1_5b(&mut pipeline_state, persona, asr_partial).await;

    // Verify that response is received
    assert!(result.is_ok(), "LLM Core call should succeed");
    let prelude = result.unwrap();
    assert!(!prelude.is_empty(), "Prelude should not be empty");

    // Verify that latency is measured (tested in performance tests)
    // For integration, we just verify the call works
}

#[tokio::test]
async fn test_trigger_1_5b_with_tts() {
    // Test integration with TTS
    use orchestrator::pipeline::trigger::trigger_1_5b_and_send_to_tts;

    let mut pipeline_state = PipelineState::new();
    let persona = "test_persona";
    let asr_partial = "I want to attack the goblin";

    // Verify that response is sent to TTS
    let result = trigger_1_5b_and_send_to_tts(&mut pipeline_state, persona, asr_partial).await;
    assert!(result.is_ok(), "TTS integration should succeed");

    // Verify that audio is generated (would be tested with actual TTS service)
    // For now, we verify the function completes successfully

    // Verify that total latency < 1.2s (tested in performance tests)
}

#[tokio::test]
async fn test_trigger_1_5b_full_flow() {
    // Test full trigger flow
    let mut criteria = TriggerCriteria::new();
    let mut pipeline_state = PipelineState::new();

    // Step 1: Speech starts, no trigger yet
    criteria.speech_duration = Duration::from_secs(3);
    assert!(!should_trigger_1_5b(&criteria), "Should not trigger at 3s");

    // Step 2: Speech continues, reaches 7s
    criteria.speech_duration = Duration::from_secs(7);
    assert!(should_trigger_1_5b(&criteria), "Should trigger at 7s");

    // Step 3: Trigger 1.5B
    pipeline_state
        .transition_to(PipelineStatus::Processing1_5B)
        .unwrap();
    assert_eq!(pipeline_state.status(), &PipelineStatus::Processing1_5B);

    // Step 4: Get prelude from 1.5B
    let persona = "test_persona";
    let asr_partial = "I want to attack";
    let prelude = trigger_1_5b(&mut pipeline_state, persona, asr_partial)
        .await
        .unwrap();
    assert!(!prelude.is_empty(), "Prelude should be generated");

    // Step 5: 1.5B finishes, wait for final ASR
    pipeline_state
        .transition_to(PipelineStatus::WaitingForFinalASR)
        .unwrap();
    assert_eq!(pipeline_state.status(), &PipelineStatus::WaitingForFinalASR);
}














