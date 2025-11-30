use crate::actor::Actor;
use crate::effect::Effect;
use crate::error::{GameError, Result};
use crate::scene::Scene;
use crate::turn::TurnOrder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    pub id: Uuid,
    pub name: String,
    pub current_scene: Option<Uuid>,
    pub scenes: HashMap<Uuid, Scene>,
    pub turn_order: TurnOrder,
    pub effects: Vec<Effect>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl GameSession {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            current_scene: None,
            scenes: HashMap::new(),
            turn_order: TurnOrder::new(),
            effects: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn create_scene(&mut self, name: String) -> Uuid {
        let scene = Scene::new(name);
        let id = scene.id;
        self.scenes.insert(id, scene);
        if self.current_scene.is_none() {
            self.current_scene = Some(id);
        }
        id
    }

    pub fn set_current_scene(&mut self, scene_id: Uuid) -> Result<()> {
        if !self.scenes.contains_key(&scene_id) {
            return Err(GameError::State(format!("Scene not found: {}", scene_id)));
        }
        self.current_scene = Some(scene_id);
        Ok(())
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        self.current_scene.and_then(|id| self.scenes.get(&id))
    }

    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.current_scene.and_then(|id| self.scenes.get_mut(&id))
    }

    pub fn add_actor_to_scene(&mut self, scene_id: Uuid, actor: Actor) -> Result<()> {
        let scene = self
            .scenes
            .get_mut(&scene_id)
            .ok_or_else(|| GameError::State(format!("Scene not found: {}", scene_id)))?;
        scene.add_actor(actor);
        Ok(())
    }

    pub fn start_combat(&mut self) -> Result<()> {
        let scene = self
            .get_current_scene_mut()
            .ok_or_else(|| GameError::State("No current scene".to_string()))?;

        scene.start_combat();

        // Initialize turn order with all actors in scene
        let actor_ids: Vec<Uuid> = scene
            .all_actors()
            .iter()
            .filter(|a| a.active && a.is_alive())
            .map(|a| a.id)
            .collect();

        if actor_ids.is_empty() {
            return Err(GameError::State("No active actors in scene".to_string()));
        }

        self.turn_order.set_initiative_order(actor_ids)?;
        Ok(())
    }

    pub fn next_turn(&mut self) -> Result<Option<Uuid>> {
        // Process expired effects
        self.effects.retain(|e| !e.is_expired());

        // Apply active effects
        if let Some(current_actor_id) = self.turn_order.current_actor() {
            // Collect effects to apply
            let effects_to_apply: Vec<(i32, i32)> = self
                .effects
                .iter()
                .filter(|e| e.target_id == current_actor_id && !e.is_expired())
                .map(|e| (e.apply_damage().unwrap_or(0), e.apply_heal().unwrap_or(0)))
                .collect();

            // Apply effects
            if let Some(scene) = self.get_current_scene_mut() {
                if let Some(actor) = scene.get_actor_mut(current_actor_id) {
                    for (damage, heal) in effects_to_apply {
                        if damage > 0 {
                            actor.take_damage(damage);
                        }
                        if heal > 0 {
                            actor.heal(heal);
                        }
                    }
                }
            }
        }

        Ok(self.turn_order.next_turn())
    }

    pub fn apply_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn get_round(&self) -> u32 {
        self.turn_order.round()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = GameSession::new("Test Session".to_string());
        assert_eq!(session.name, "Test Session");
        assert!(session.current_scene.is_none());
    }

    #[test]
    fn test_session_create_scene() {
        let mut session = GameSession::new("Test".to_string());
        let scene_id = session.create_scene("Scene 1".to_string());

        assert!(session.scenes.contains_key(&scene_id));
        assert_eq!(session.current_scene, Some(scene_id));
    }

    #[test]
    fn test_session_start_combat() {
        let mut session = GameSession::new("Test".to_string());
        let scene_id = session.create_scene("Combat".to_string());

        let actor = Actor::new("Player".to_string(), ActorType::Player);
        session.add_actor_to_scene(scene_id, actor).unwrap();

        session.start_combat().unwrap();

        let scene = session.get_current_scene().unwrap();
        assert!(scene.combat_active);
    }
}
