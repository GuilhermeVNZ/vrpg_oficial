use crate::error::{GameError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnOrder {
    actors: VecDeque<Uuid>,
    current_index: usize,
    round: u32,
}

impl Default for TurnOrder {
    fn default() -> Self {
        Self::new()
    }
}

impl TurnOrder {
    pub fn new() -> Self {
        Self {
            actors: VecDeque::new(),
            current_index: 0,
            round: 1,
        }
    }

    pub fn add_actor(&mut self, actor_id: Uuid) {
        if !self.actors.contains(&actor_id) {
            self.actors.push_back(actor_id);
        }
    }

    pub fn remove_actor(&mut self, actor_id: Uuid) {
        self.actors.retain(|&id| id != actor_id);
        if self.current_index >= self.actors.len() && !self.actors.is_empty() {
            self.current_index = 0;
        }
    }

    pub fn set_initiative_order(&mut self, actor_ids: Vec<Uuid>) -> Result<()> {
        if actor_ids.is_empty() {
            return Err(GameError::State(
                "Cannot set empty initiative order".to_string(),
            ));
        }
        self.actors = VecDeque::from(actor_ids);
        self.current_index = 0;
        self.round = 1;
        Ok(())
    }

    pub fn current_actor(&self) -> Option<Uuid> {
        self.actors.get(self.current_index).copied()
    }

    pub fn next_turn(&mut self) -> Option<Uuid> {
        if self.actors.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.actors.len();
        if self.current_index == 0 {
            self.round += 1;
        }

        self.current_actor()
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn all_actors(&self) -> Vec<Uuid> {
        self.actors.iter().copied().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.actors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_order_creation() {
        let order = TurnOrder::new();
        assert_eq!(order.round(), 1);
        assert!(order.is_empty());
    }

    #[test]
    fn test_turn_order_add_actor() {
        let mut order = TurnOrder::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        order.add_actor(id1);
        order.add_actor(id2);

        assert_eq!(order.all_actors().len(), 2);
    }

    #[test]
    fn test_turn_order_next_turn() {
        let mut order = TurnOrder::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        order.set_initiative_order(vec![id1, id2]).unwrap();

        assert_eq!(order.current_actor(), Some(id1));
        assert_eq!(order.next_turn(), Some(id2));
        assert_eq!(order.next_turn(), Some(id1));
        assert_eq!(order.round(), 2);
    }
}
