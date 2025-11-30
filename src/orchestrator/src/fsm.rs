//! Scene State Machine (FSM) for VRPG
//!
//! Manages the four main scene states:
//! - SocialFreeFlow: Dialogue, roleplay, no grid
//! - Exploration: Investigation, free movement
//! - CombatTurnBased: Combat mode, grid active
//! - DowntimePreparation: Between sessions, preparation

use crate::error::{OrchestratorError, Result};
use serde::{Deserialize, Serialize};

/// Scene state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneState {
    /// Social free flow - dialogue, roleplay, no grid
    SocialFreeFlow,
    /// Exploration - investigation, free movement
    Exploration,
    /// Combat turn-based - combat mode, grid active
    CombatTurnBased,
    /// Downtime preparation - between sessions, preparation
    DowntimePreparation,
}

impl SceneState {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            SceneState::SocialFreeFlow => "SocialFreeFlow",
            SceneState::Exploration => "Exploration",
            SceneState::CombatTurnBased => "CombatTurnBased",
            SceneState::DowntimePreparation => "DowntimePreparation",
        }
    }

    /// Check if transition from `from` to `to` is valid
    pub fn can_transition_from(from: SceneState, to: SceneState) -> bool {
        match (from, to) {
            // From SocialFreeFlow
            (SceneState::SocialFreeFlow, SceneState::Exploration) => true,
            (SceneState::SocialFreeFlow, SceneState::CombatTurnBased) => true,
            (SceneState::SocialFreeFlow, SceneState::DowntimePreparation) => true,

            // From Exploration
            (SceneState::Exploration, SceneState::SocialFreeFlow) => true,
            (SceneState::Exploration, SceneState::CombatTurnBased) => true,
            (SceneState::Exploration, SceneState::DowntimePreparation) => true,

            // From CombatTurnBased
            (SceneState::CombatTurnBased, SceneState::SocialFreeFlow) => true,
            (SceneState::CombatTurnBased, SceneState::Exploration) => true,
            (SceneState::CombatTurnBased, SceneState::DowntimePreparation) => true,

            // From DowntimePreparation
            (SceneState::DowntimePreparation, SceneState::SocialFreeFlow) => true,
            (SceneState::DowntimePreparation, SceneState::Exploration) => true,

            // Same state (no-op)
            (a, b) if a == b => true,

            // Invalid transitions
            _ => false,
        }
    }
}

/// Scene State Machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneStateMachine {
    current_state: SceneState,
}

impl SceneStateMachine {
    /// Create a new FSM starting in SocialFreeFlow
    pub fn new() -> Self {
        Self {
            current_state: SceneState::SocialFreeFlow,
        }
    }

    /// Create FSM with initial state
    pub fn with_state(initial_state: SceneState) -> Self {
        Self {
            current_state: initial_state,
        }
    }

    /// Get current state
    pub fn current_state(&self) -> SceneState {
        self.current_state
    }

    /// Transition to new state
    pub fn transition_to(&mut self, new_state: SceneState) -> Result<()> {
        if !SceneState::can_transition_from(self.current_state, new_state) {
            return Err(OrchestratorError::InvalidStateTransition(format!(
                "Cannot transition from {:?} to {:?}",
                self.current_state, new_state
            )));
        }

        self.current_state = new_state;
        Ok(())
    }

    /// Force transition (for recovery/initialization)
    pub fn force_transition(&mut self, new_state: SceneState) {
        self.current_state = new_state;
    }
}

impl Default for SceneStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
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
    }

    #[test]
    fn test_invalid_transitions() {
        // DowntimePreparation -> CombatTurnBased should be invalid
        assert!(!SceneState::can_transition_from(
            SceneState::DowntimePreparation,
            SceneState::CombatTurnBased
        ));
    }

    #[test]
    fn test_same_state_transition() {
        let mut fsm = SceneStateMachine::new();
        let initial = fsm.current_state();

        // Transitioning to same state should be valid (no-op)
        assert!(fsm.transition_to(initial).is_ok());
        assert_eq!(fsm.current_state(), initial);
    }
}
