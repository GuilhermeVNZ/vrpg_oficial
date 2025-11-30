//! Session Persistence Tests - M4.4
//! Tests for session save/load functionality

use orchestrator::cache::game_state_cache::GameStateCache;
use orchestrator::cache::lore_cache::LoreCache;
use orchestrator::cache::scene_context_cache::SceneContextCache;
use orchestrator::pipeline::PipelineState;
use orchestrator::session::persistence::SessionPersistence;
use orchestrator::session::{GameSession, SessionManager};
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

/// Test 1: Save complete session
#[test]
fn test_save_session_complete() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec!["Action 1".to_string(), "Action 2".to_string()];
    let session_settings = HashMap::new();

    let result = persistence.save_session(
        &session,
        &pipeline_state,
        &game_state_cache,
        &scene_context_cache,
        &lore_cache,
        action_history,
        session_settings,
    );

    assert!(result.is_ok(), "Should save session successfully");
    
    // Verify file was created
    let save_path = temp_dir.path().join(format!("{}.json", session.session_id));
    assert!(save_path.exists(), "Session file should be created");
}

/// Test 2: Load complete session
#[test]
fn test_load_session_complete() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    // Create and save a session first
    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec!["Action 1".to_string()];
    let session_settings = HashMap::new();

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history,
            session_settings,
        )
        .unwrap();

    // Now load it
    let loaded = persistence.load_session(&session.session_id).unwrap();

    assert_eq!(loaded.metadata.session_id, session.session_id);
    assert_eq!(loaded.metadata.format_version, 1);
    assert!(!loaded.action_history.is_empty());
}

/// Test 3: Load restores state correctly
#[test]
fn test_load_session_restores_state() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    // Create session with modified state
    let session = GameSession::new();
    let mut pipeline_state = PipelineState::new();
    pipeline_state.update_game_state("HP: 50/100".to_string());
    pipeline_state.update_scene_context("Tavern, 3 NPCs".to_string());

    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec!["Action 1".to_string()];
    let session_settings = HashMap::new();

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history,
            session_settings,
        )
        .unwrap();

    // Load and restore
    let loaded = persistence.load_session(&session.session_id).unwrap();
    let (scene_state, restored_pipeline_state) = SessionPersistence::restore_session_state(&loaded).unwrap();

    assert_eq!(restored_pipeline_state.game_state(), "HP: 50/100");
    assert_eq!(restored_pipeline_state.scene_context(), "Tavern, 3 NPCs");
    assert_eq!(scene_state, session.current_state());
}

/// Test 4: Version validation
#[test]
fn test_session_version_validation() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    // Create and save a session
    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec![];
    let session_settings = HashMap::new();

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history,
            session_settings,
        )
        .unwrap();

    // Load and verify version
    let loaded = persistence.load_session(&session.session_id).unwrap();
    assert_eq!(loaded.metadata.format_version, 1);
}

/// Test 5: Integrity check (session ID validation)
#[test]
fn test_session_integrity_check() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    // Create and save a session
    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec![];
    let session_settings = HashMap::new();

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history,
            session_settings,
        )
        .unwrap();

    // Try to load with wrong session ID - should fail
    let wrong_id = "wrong-id";
    let result = persistence.load_session(wrong_id);
    assert!(result.is_err(), "Should fail when session ID doesn't match");
}

/// Test 6: List all sessions
#[test]
fn test_list_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    // Create and save multiple sessions
    for i in 0..3 {
        let session = GameSession::new();
        let pipeline_state = PipelineState::new();
        let game_state_cache = GameStateCache::new();
        let scene_context_cache = SceneContextCache::new();
        let lore_cache = LoreCache::new();
        let action_history = vec![format!("Action {}", i)];
        let session_settings = HashMap::new();

        persistence
            .save_session(
                &session,
                &pipeline_state,
                &game_state_cache,
                &scene_context_cache,
                &lore_cache,
                action_history,
                session_settings,
            )
            .unwrap();
    }

    // List sessions
    let sessions = persistence.list_sessions().unwrap();
    assert_eq!(sessions.len(), 3, "Should list all 3 sessions");
}

/// Test 7: Delete session
#[test]
fn test_delete_session() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    // Create and save a session
    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec![];
    let session_settings = HashMap::new();

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history,
            session_settings,
        )
        .unwrap();

    // Verify file exists
    let save_path = temp_dir.path().join(format!("{}.json", session.session_id));
    assert!(save_path.exists());

    // Delete session
    let result = persistence.delete_session(&session.session_id);
    assert!(result.is_ok(), "Should delete session successfully");

    // Verify file was deleted
    assert!(!save_path.exists(), "Session file should be deleted");

    // Try to load deleted session - should fail
    let result = persistence.load_session(&session.session_id);
    assert!(result.is_err(), "Should fail to load deleted session");
}

/// Test 8: Action history persistence
#[test]
fn test_action_history_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec![
        "Action 1".to_string(),
        "Action 2".to_string(),
        "Action 3".to_string(),
    ];
    let session_settings = HashMap::new();

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history.clone(),
            session_settings,
        )
        .unwrap();

    // Load and verify action history
    let loaded = persistence.load_session(&session.session_id).unwrap();
    assert_eq!(loaded.action_history.len(), 3);
    assert_eq!(loaded.action_history[0], "Action 1");
    assert_eq!(loaded.action_history[1], "Action 2");
    assert_eq!(loaded.action_history[2], "Action 3");
}

/// Test 9: Session settings persistence
#[test]
fn test_session_settings_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    let session = GameSession::new();
    let pipeline_state = PipelineState::new();
    let game_state_cache = GameStateCache::new();
    let scene_context_cache = SceneContextCache::new();
    let lore_cache = LoreCache::new();
    let action_history = vec![];
    let mut session_settings = HashMap::new();
    session_settings.insert("difficulty".to_string(), "normal".to_string());
    session_settings.insert("language".to_string(), "pt-BR".to_string());

    persistence
        .save_session(
            &session,
            &pipeline_state,
            &game_state_cache,
            &scene_context_cache,
            &lore_cache,
            action_history,
            session_settings.clone(),
        )
        .unwrap();

    // Load and verify settings
    let loaded = persistence.load_session(&session.session_id).unwrap();
    assert_eq!(loaded.session_settings.len(), 2);
    assert_eq!(loaded.session_settings.get("difficulty"), Some(&"normal".to_string()));
    assert_eq!(loaded.session_settings.get("language"), Some(&"pt-BR".to_string()));
}

/// Test 10: Error handling - file not found
#[test]
fn test_load_nonexistent_session() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    let result = persistence.load_session("nonexistent-session-id");
    assert!(result.is_err(), "Should fail when session doesn't exist");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("not found"), "Error should mention file not found");
}

/// Test 11: Error handling - delete nonexistent session
#[test]
fn test_delete_nonexistent_session() {
    let temp_dir = TempDir::new().unwrap();
    let persistence = SessionPersistence::new(temp_dir.path()).unwrap();

    let result = persistence.delete_session("nonexistent-session-id");
    assert!(result.is_err(), "Should fail when trying to delete nonexistent session");
}

