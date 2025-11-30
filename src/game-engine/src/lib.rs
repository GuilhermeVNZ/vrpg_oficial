// Game Engine - Session management, combat, and turn tracking
// This module provides the core game state management

pub mod actor;
pub mod effect;
pub mod error;
pub mod scene;
pub mod session;
pub mod turn;

pub use actor::{Actor, ActorType};
pub use effect::{Effect, EffectType};
pub use error::{GameError, Result};
pub use scene::Scene;
pub use session::GameSession;
pub use turn::TurnOrder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_compiles() {
        assert!(true);
    }
}
