use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorType {
    Player,
    Npc,
    Monster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: Uuid,
    pub name: String,
    pub actor_type: ActorType,
    pub position: (f32, f32, f32), // x, y, z
    pub hp: i32,
    pub max_hp: i32,
    pub ac: i32,
    pub initiative: Option<i32>,
    pub active: bool,
}

impl Default for Actor {
    fn default() -> Self {
        Self::new("Unknown".to_string(), ActorType::Player)
    }
}

impl Actor {
    pub fn new(name: String, actor_type: ActorType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            actor_type,
            position: (0.0, 0.0, 0.0),
            hp: 100,
            max_hp: 100,
            ac: 10,
            initiative: None,
            active: true,
        }
    }

    pub fn with_stats(name: String, actor_type: ActorType, hp: i32, ac: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            actor_type,
            position: (0.0, 0.0, 0.0),
            hp,
            max_hp: hp,
            ac,
            initiative: None,
            active: true,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = (x, y, z);
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp = (self.hp - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn set_initiative(&mut self, initiative: i32) {
        self.initiative = Some(initiative);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_creation() {
        let actor = Actor::new("Test".to_string(), ActorType::Player);
        assert_eq!(actor.name, "Test");
        assert_eq!(actor.actor_type, ActorType::Player);
        assert!(actor.is_alive());
    }

    #[test]
    fn test_actor_damage() {
        let mut actor = Actor::new("Test".to_string(), ActorType::Player);
        actor.take_damage(50);
        assert_eq!(actor.hp, 50);
        assert!(actor.is_alive());

        actor.take_damage(100);
        assert_eq!(actor.hp, 0);
        assert!(!actor.is_alive());
    }

    #[test]
    fn test_actor_heal() {
        let mut actor = Actor::new("Test".to_string(), ActorType::Player);
        actor.take_damage(50);
        actor.heal(30);
        assert_eq!(actor.hp, 80);

        actor.heal(100);
        assert_eq!(actor.hp, actor.max_hp);
    }
}
