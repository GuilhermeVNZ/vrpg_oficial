//! Integration tests for Orchestrator

use orchestrator::{
    communication::{ActionKind, PlayerAction},
    CommunicationState, Orchestrator, SessionManager,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_player_action_voice_flow() {
    // Setup
    let session_manager = Arc::new(RwLock::new(SessionManager::new()));
    let communication = Arc::new(CommunicationState::new(session_manager.clone()));

    // Create session
    let session_id = {
        let mut sm = session_manager.write().await;
        sm.create_session()
    };

    // Create orchestrator
    let orchestrator = Arc::new(Orchestrator::new(
        session_manager.clone(),
        communication.clone(),
    ));

    // Note: In a real implementation, we'd update communication with orchestrator
    // For now, the orchestrator processes actions directly

    // Create voice action
    let action = PlayerAction {
        session_id: session_id.clone(),
        player_id: "player_1".to_string(),
        kind: ActionKind::Voice,
        text: Some("I want to check for traps".to_string()),
        ui_intent: None,
        target_id: None,
        metadata: None,
    };

    // Process action
    // Note: This may fail if broadcast channel is closed (no clients connected)
    // In production, clients would be connected via WebSocket
    let result = orchestrator.process_player_action(action).await;
    // We accept either success or broadcast error (expected in test environment)
    if let Err(e) = &result {
        // Only fail if it's not a broadcast error
        if !e.to_string().contains("Broadcast failed") {
            panic!("Unexpected error: {:?}", e);
        }
    }

    // Verify session still exists
    let sm = session_manager.read().await;
    assert!(sm.get_session(&session_id).is_some());
}

#[tokio::test]
async fn test_player_action_ui_flow() {
    // Setup
    let session_manager = Arc::new(RwLock::new(SessionManager::new()));
    let communication = Arc::new(CommunicationState::new(session_manager.clone()));

    // Create session
    let session_id = {
        let mut sm = session_manager.write().await;
        sm.create_session()
    };

    // Create orchestrator
    let orchestrator = Arc::new(Orchestrator::new(
        session_manager.clone(),
        communication.clone(),
    ));

    // Create UI action
    let action = PlayerAction {
        session_id: session_id.clone(),
        player_id: "player_1".to_string(),
        kind: ActionKind::Ui,
        text: None,
        ui_intent: Some("end_turn".to_string()),
        target_id: None,
        metadata: None,
    };

    // Process action
    // Note: This may fail if broadcast channel is closed (no clients connected)
    // In production, clients would be connected via WebSocket
    let result = orchestrator.process_player_action(action).await;
    // We accept either success or broadcast error (expected in test environment)
    if let Err(e) = &result {
        // Only fail if it's not a broadcast error
        if !e.to_string().contains("Broadcast failed") {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_roll_result_processing() {
    // Setup
    let session_manager = Arc::new(RwLock::new(SessionManager::new()));
    let communication = Arc::new(CommunicationState::new(session_manager.clone()));

    // Create session
    let session_id = {
        let mut sm = session_manager.write().await;
        sm.create_session()
    };

    // Create orchestrator
    let orchestrator = Arc::new(Orchestrator::new(
        session_manager.clone(),
        communication.clone(),
    ));

    // Create roll result (without matching request - should fail gracefully)
    let roll_result = orchestrator::communication::RollResult {
        session_id: session_id.clone(),
        request_id: "nonexistent".to_string(),
        actor_id: "player_1".to_string(),
        total: 15,
        natural: 12,
        breakdown: serde_json::json!({}),
        client_seed: None,
        timestamp: chrono::Utc::now().timestamp(),
    };

    // Process roll result (should handle missing request gracefully)
    let result = orchestrator.process_roll_result(roll_result).await;
    // This should fail because request doesn't exist, but shouldn't panic
    assert!(result.is_err());
}
