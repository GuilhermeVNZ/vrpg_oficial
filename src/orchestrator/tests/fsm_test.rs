//! Comprehensive FSM tests

use orchestrator::fsm::{SceneState, SceneStateMachine};

#[test]
fn test_fsm_initial_state() {
    let fsm = SceneStateMachine::new();
    assert_eq!(fsm.current_state(), SceneState::SocialFreeFlow);
}

#[test]
fn test_fsm_with_initial_state() {
    let fsm = SceneStateMachine::with_state(SceneState::CombatTurnBased);
    assert_eq!(fsm.current_state(), SceneState::CombatTurnBased);
}

#[test]
fn test_fsm_all_valid_transitions() {
    let mut fsm = SceneStateMachine::new();

    // SocialFreeFlow -> Exploration
    assert!(fsm.transition_to(SceneState::Exploration).is_ok());
    assert_eq!(fsm.current_state(), SceneState::Exploration);

    // Exploration -> CombatTurnBased
    assert!(fsm.transition_to(SceneState::CombatTurnBased).is_ok());
    assert_eq!(fsm.current_state(), SceneState::CombatTurnBased);

    // CombatTurnBased -> SocialFreeFlow
    assert!(fsm.transition_to(SceneState::SocialFreeFlow).is_ok());
    assert_eq!(fsm.current_state(), SceneState::SocialFreeFlow);

    // SocialFreeFlow -> CombatTurnBased
    assert!(fsm.transition_to(SceneState::CombatTurnBased).is_ok());
    assert_eq!(fsm.current_state(), SceneState::CombatTurnBased);

    // CombatTurnBased -> Exploration
    assert!(fsm.transition_to(SceneState::Exploration).is_ok());
    assert_eq!(fsm.current_state(), SceneState::Exploration);

    // Exploration -> SocialFreeFlow
    assert!(fsm.transition_to(SceneState::SocialFreeFlow).is_ok());
    assert_eq!(fsm.current_state(), SceneState::SocialFreeFlow);

    // Any state -> DowntimePreparation
    assert!(fsm.transition_to(SceneState::DowntimePreparation).is_ok());
    assert_eq!(fsm.current_state(), SceneState::DowntimePreparation);
}

#[test]
fn test_fsm_invalid_transitions() {
    let mut fsm = SceneStateMachine::with_state(SceneState::DowntimePreparation);

    // DowntimePreparation -> CombatTurnBased should be invalid
    assert!(fsm.transition_to(SceneState::CombatTurnBased).is_err());

    // Should remain in DowntimePreparation
    assert_eq!(fsm.current_state(), SceneState::DowntimePreparation);
}

#[test]
fn test_fsm_same_state_transition() {
    let mut fsm = SceneStateMachine::new();
    let initial = fsm.current_state();

    // Transitioning to same state should be valid (no-op)
    assert!(fsm.transition_to(initial).is_ok());
    assert_eq!(fsm.current_state(), initial);
}

#[test]
fn test_fsm_force_transition() {
    let mut fsm = SceneStateMachine::new();

    // Force transition bypasses validation
    fsm.force_transition(SceneState::CombatTurnBased);
    assert_eq!(fsm.current_state(), SceneState::CombatTurnBased);

    // Even invalid transitions work with force
    fsm.force_transition(SceneState::DowntimePreparation);
    assert_eq!(fsm.current_state(), SceneState::DowntimePreparation);
    fsm.force_transition(SceneState::CombatTurnBased);
    assert_eq!(fsm.current_state(), SceneState::CombatTurnBased);
}

#[test]
fn test_fsm_state_names() {
    assert_eq!(SceneState::SocialFreeFlow.name(), "SocialFreeFlow");
    assert_eq!(SceneState::Exploration.name(), "Exploration");
    assert_eq!(SceneState::CombatTurnBased.name(), "CombatTurnBased");
    assert_eq!(
        SceneState::DowntimePreparation.name(),
        "DowntimePreparation"
    );
}

#[test]
fn test_fsm_serialization() {
    use serde_json;

    let fsm = SceneStateMachine::with_state(SceneState::CombatTurnBased);
    let serialized = serde_json::to_string(&fsm).unwrap();
    let deserialized: SceneStateMachine = serde_json::from_str(&serialized).unwrap();

    assert_eq!(fsm.current_state(), deserialized.current_state());
}
