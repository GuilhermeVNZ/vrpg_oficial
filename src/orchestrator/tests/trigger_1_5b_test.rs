//! 1.5B Trigger Logic Tests - M2.2
//! Tests for triggering Qwen-1.5B based on time, pause, and action detection

use orchestrator::pipeline::trigger::{
    should_trigger_1_5b, trigger_1_5b, trigger_1_5b_and_send_to_tts, TriggerCriteria,
};
use orchestrator::pipeline::PipelineState;
use std::time::{Duration, Instant};

#[test]
fn test_should_trigger_1_5b_time_based() {
    // Test time-based detection (6-8s)
    let mut criteria = TriggerCriteria::new();

    // Simulate 6s of speech → should trigger
    criteria.speech_duration = Duration::from_secs(6);
    assert!(should_trigger_1_5b(&criteria), "Should trigger after 6s");

    // Simulate 7s of speech → should trigger
    criteria.speech_duration = Duration::from_secs(7);
    assert!(should_trigger_1_5b(&criteria), "Should trigger after 7s");

    // Simulate 8s of speech → should trigger
    criteria.speech_duration = Duration::from_secs(8);
    assert!(should_trigger_1_5b(&criteria), "Should trigger after 8s");

    // Simulate 5s of speech → should not trigger
    criteria.speech_duration = Duration::from_secs(5);
    assert!(
        !should_trigger_1_5b(&criteria),
        "Should not trigger before 6s"
    );

    // Simulate 9s of speech → should trigger (already past threshold)
    criteria.speech_duration = Duration::from_secs(9);
    assert!(should_trigger_1_5b(&criteria), "Should trigger after 9s");
}

#[test]
fn test_should_trigger_1_5b_pause_based() {
    // Test pause-based detection
    let mut criteria = TriggerCriteria::new();

    // Simulate pause > threshold → should trigger
    criteria.pause_duration = Some(Duration::from_millis(1500)); // 1.5s pause
    criteria.pause_threshold = Duration::from_millis(1000); // 1s threshold
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger on long pause"
    );

    // Simulate pause < threshold → should not trigger
    criteria.pause_duration = Some(Duration::from_millis(500)); // 0.5s pause
    assert!(
        !should_trigger_1_5b(&criteria),
        "Should not trigger on short pause"
    );

    // Simulate VAD detecting end → should trigger
    criteria.vad_detected_end = true;
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger when VAD detects end"
    );
}

#[test]
fn test_should_trigger_1_5b_action_based() {
    // Test action-based detection
    let mut criteria = TriggerCriteria::new();

    // Simulate intent parsing detecting action → should trigger
    criteria.has_clear_action = true;
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger on clear action"
    );

    // Simulate intent parsing without clear action → should not trigger
    criteria.has_clear_action = false;
    criteria.speech_duration = Duration::from_secs(3); // Too short
    assert!(
        !should_trigger_1_5b(&criteria),
        "Should not trigger without clear action and short speech"
    );

    // Verify that different action types are detected
    criteria.has_clear_action = true;
    criteria.action_type = Some("attack".to_string());
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger on attack action"
    );

    criteria.action_type = Some("cast".to_string());
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger on cast action"
    );

    criteria.action_type = Some("move".to_string());
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger on move action"
    );
}

#[test]
fn test_should_trigger_1_5b_combined() {
    // Test combination of criteria
    let mut criteria = TriggerCriteria::new();

    // Verify that any criterion can trigger
    // Time-based
    criteria.speech_duration = Duration::from_secs(7);
    assert!(
        should_trigger_1_5b(&criteria),
        "Time criterion should trigger"
    );

    // Pause-based
    criteria.speech_duration = Duration::from_secs(3);
    criteria.pause_duration = Some(Duration::from_millis(1500));
    criteria.pause_threshold = Duration::from_millis(1000);
    assert!(
        should_trigger_1_5b(&criteria),
        "Pause criterion should trigger"
    );

    // Action-based
    criteria.pause_duration = None;
    criteria.has_clear_action = true;
    assert!(
        should_trigger_1_5b(&criteria),
        "Action criterion should trigger"
    );

    // Verify that multiple criteria don't cause multiple triggers
    // (This is handled by the trigger logic, not the detection function)
    criteria.speech_duration = Duration::from_secs(7);
    criteria.pause_duration = Some(Duration::from_millis(1500));
    criteria.has_clear_action = true;
    // Should still trigger only once (tested in integration)
    assert!(
        should_trigger_1_5b(&criteria),
        "Multiple criteria should trigger"
    );
}

#[test]
fn test_should_trigger_1_5b_no_premature() {
    // Test that it doesn't trigger prematurely
    let mut criteria = TriggerCriteria::new();

    // Simulate 1s of speech → should not trigger
    criteria.speech_duration = Duration::from_secs(1);
    assert!(!should_trigger_1_5b(&criteria), "Should not trigger at 1s");

    // Simulate 2s of speech → should not trigger
    criteria.speech_duration = Duration::from_secs(2);
    assert!(!should_trigger_1_5b(&criteria), "Should not trigger at 2s");

    // Simulate 3s of speech → should not trigger
    criteria.speech_duration = Duration::from_secs(3);
    assert!(!should_trigger_1_5b(&criteria), "Should not trigger at 3s");

    // Simulate 4s of speech → should not trigger
    criteria.speech_duration = Duration::from_secs(4);
    assert!(!should_trigger_1_5b(&criteria), "Should not trigger at 4s");

    // Simulate 5s of speech → should not trigger
    criteria.speech_duration = Duration::from_secs(5);
    assert!(!should_trigger_1_5b(&criteria), "Should not trigger at 5s");

    // But with clear action, should trigger even at 3s
    criteria.has_clear_action = true;
    criteria.speech_duration = Duration::from_secs(3);
    assert!(
        should_trigger_1_5b(&criteria),
        "Should trigger at 3s with clear action"
    );
}

#[tokio::test]
async fn test_trigger_1_5b_function() {
    // Test trigger_1_5b() function
    let mut pipeline_state = PipelineState::new();
    let persona = "test_persona".to_string();
    let asr_partial = "I want to attack".to_string();

    // Verify that emotional prompt is prepared
    // Verify that call to LLM Core /llm/prelude is made
    // Verify that prelude text is returned
    // Verify that latency < 1.2s

    let start = Instant::now();
    let result = trigger_1_5b(&mut pipeline_state, &persona, &asr_partial).await;
    let duration = start.elapsed();

    // Result should be Ok with prelude text
    assert!(result.is_ok(), "trigger_1_5b should succeed");
    let prelude = result.unwrap();
    assert!(!prelude.is_empty(), "Prelude should not be empty");
    assert!(
        prelude.len() <= 200,
        "Prelude should be short (max 200 chars)"
    );

    // Verify latency < 1.2s
    assert!(
        duration < Duration::from_millis(1200),
        "Latency should be < 1.2s, got {:?}",
        duration
    );
}

#[tokio::test]
async fn test_trigger_1_5b_immediate_tts() {
    // Test immediate TTS sending
    let mut pipeline_state = PipelineState::new();
    let persona = "test_persona".to_string();
    let asr_partial = "I want to attack".to_string();

    // Verify that 1.5B response is sent to TTS immediately
    // Verify that it doesn't wait for 14B
    // Verify that total latency < 1.2s

    let start = Instant::now();
    let result = trigger_1_5b_and_send_to_tts(&mut pipeline_state, &persona, &asr_partial).await;
    let duration = start.elapsed();

    // Result should be Ok
    assert!(
        result.is_ok(),
        "trigger_1_5b_and_send_to_tts should succeed"
    );

    // Verify latency < 1.2s
    assert!(
        duration < Duration::from_millis(1200),
        "Total latency should be < 1.2s, got {:?}",
        duration
    );

    // Verify that TTS was called (this would be verified in integration tests)
}
