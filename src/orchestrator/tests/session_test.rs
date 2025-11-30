//! Comprehensive Session Management tests

use orchestrator::fsm::SceneState;
use orchestrator::session::{GameSession, SessionManager};

#[test]
fn test_create_session() {
    let session = GameSession::new();
    assert!(!session.session_id.is_empty());
    assert_eq!(session.current_state(), SceneState::SocialFreeFlow);
}

#[test]
fn test_session_transition() {
    let mut session = GameSession::new();

    // Transition to Exploration
    assert!(session.transition_to(SceneState::Exploration).is_ok());
    assert_eq!(session.current_state(), SceneState::Exploration);

    // Transition to CombatTurnBased
    assert!(session.transition_to(SceneState::CombatTurnBased).is_ok());
    assert_eq!(session.current_state(), SceneState::CombatTurnBased);
}

#[test]
fn test_session_manager() {
    let mut manager = SessionManager::new();

    // Create session
    let session_id = manager.create_session();
    assert!(!session_id.is_empty());

    // Get session
    let session = manager.get_session(&session_id);
    assert!(session.is_some());
    assert_eq!(session.unwrap().session_id, session_id);

    // Get mutable session
    let session_mut = manager.get_session_mut(&session_id);
    assert!(session_mut.is_some());

    // Remove session
    assert!(manager.remove_session(&session_id));
    assert!(manager.get_session(&session_id).is_none());
}

#[test]
fn test_session_manager_multiple_sessions() {
    let mut manager = SessionManager::new();

    let id1 = manager.create_session();
    let id2 = manager.create_session();
    let id3 = manager.create_session();

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);

    assert!(manager.get_session(&id1).is_some());
    assert!(manager.get_session(&id2).is_some());
    assert!(manager.get_session(&id3).is_some());
}

#[test]
fn test_session_default() {
    let session = GameSession::default();
    assert!(!session.session_id.is_empty());
}

#[test]
fn test_session_manager_default() {
    let manager = SessionManager::default();
    // Just verify it doesn't panic
    assert!(manager.get_session("nonexistent").is_none());
}
