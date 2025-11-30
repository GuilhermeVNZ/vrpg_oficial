use crate::actor::Actor;
use crate::error::{GameError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub actors: HashMap<Uuid, Actor>,
    pub combat_active: bool,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new("Default Scene".to_string())
    }
}

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            actors: HashMap::new(),
            combat_active: false,
        }
    }

    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.insert(actor.id, actor);
    }

    pub fn remove_actor(&mut self, actor_id: Uuid) -> Result<()> {
        self.actors
            .remove(&actor_id)
            .ok_or_else(|| GameError::State(format!("Actor not found: {}", actor_id)))?;
        Ok(())
    }

    pub fn get_actor(&self, actor_id: Uuid) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    pub fn get_actor_mut(&mut self, actor_id: Uuid) -> Option<&mut Actor> {
        self.actors.get_mut(&actor_id)
    }

    pub fn all_actors(&self) -> Vec<&Actor> {
        self.actors.values().collect()
    }

    pub fn start_combat(&mut self) {
        self.combat_active = true;
    }

    pub fn end_combat(&mut self) {
        self.combat_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorType;

    #[test]
    fn test_scene_creation() {
        let scene = Scene::new("Test Scene".to_string());
        assert_eq!(scene.name, "Test Scene");
        assert!(!scene.combat_active);
    }

    #[test]
    fn test_scene_add_actor() {
        let mut scene = Scene::new("Test".to_string());
        let actor = Actor::new("Player".to_string(), ActorType::Player);
        scene.add_actor(actor);

        assert_eq!(scene.all_actors().len(), 1);
    }

    #[test]
    fn test_scene_combat() {
        let mut scene = Scene::new("Test".to_string());
        assert!(!scene.combat_active);

        scene.start_combat();
        assert!(scene.combat_active);

        scene.end_combat();
        assert!(!scene.combat_active);
    }
}
