//! Scene Context Cache - M4.2
//! Cache of scene context (last 3-6 actions, roll results, active NPCs, interactions)

use crate::pipeline::context_14b::ContextEvent;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tracing::debug;

/// NPC identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NpcId(pub String);

/// Scene event types
#[derive(Debug, Clone)]
pub enum SceneEvent {
    Action {
        actor: String,
        action: String,
        timestamp: SystemTime,
    },
    Roll {
        actor: String,
        roll_type: String,
        result: i32,
        timestamp: SystemTime,
    },
    Dialogue {
        speaker: String,
        message: String,
        timestamp: SystemTime,
    },
    Interaction {
        from: String,
        to: String,
        interaction_type: String,
        timestamp: SystemTime,
    },
}

/// Scene context cache
pub struct SceneContextCache {
    /// Recent events (max 6)
    events: Arc<Mutex<VecDeque<ContextEvent>>>,
    /// Active NPCs
    active_npcs: Arc<Mutex<HashSet<NpcId>>>,
    /// Interactions: who interacted with whom
    interactions: Arc<Mutex<HashMap<String, HashSet<String>>>>,
}

impl SceneContextCache {
    /// Create new scene context cache
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::new())),
            active_npcs: Arc::new(Mutex::new(HashSet::new())),
            interactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add event to cache (keeps max 6 events)
    pub fn add_event(&mut self, event: SceneEvent) {
        let context_event = match event {
            SceneEvent::Action {
                actor,
                action,
                timestamp,
            } => ContextEvent {
                timestamp,
                event_type: "action".to_string(),
                description: format!("{}: {}", actor, action),
            },
            SceneEvent::Roll {
                actor,
                roll_type,
                result,
                timestamp,
            } => ContextEvent {
                timestamp,
                event_type: "roll".to_string(),
                description: format!("{} rolled {} for {}: {}", actor, result, roll_type, result),
            },
            SceneEvent::Dialogue {
                speaker,
                message,
                timestamp,
            } => ContextEvent {
                timestamp,
                event_type: "dialogue".to_string(),
                description: format!("{}: {}", speaker, message),
            },
            SceneEvent::Interaction {
                from,
                to,
                interaction_type,
                timestamp,
            } => ContextEvent {
                timestamp,
                event_type: "interaction".to_string(),
                description: format!("{} {} with {}", from, interaction_type, to),
            },
        };

        let mut events = self.events.lock().unwrap();
        events.push_back(context_event);

        // Keep only last 6 events
        while events.len() > 6 {
            events.pop_front();
        }

        debug!(
            "Added event to scene context cache (total: {})",
            events.len()
        );
    }

    /// Get recent events (up to limit)
    pub fn get_recent_events(&self, limit: usize) -> Vec<ContextEvent> {
        let events = self.events.lock().unwrap();
        events.iter().rev().take(limit).cloned().collect()
    }

    /// Add active NPC
    pub fn add_active_npc(&mut self, npc_id: NpcId) {
        let mut npcs = self.active_npcs.lock().unwrap();
        npcs.insert(npc_id);
    }

    /// Remove active NPC
    pub fn remove_active_npc(&mut self, npc_id: &NpcId) {
        let mut npcs = self.active_npcs.lock().unwrap();
        npcs.remove(npc_id);
    }

    /// Get active NPCs
    pub fn get_active_npcs(&self) -> HashSet<NpcId> {
        let npcs = self.active_npcs.lock().unwrap();
        npcs.clone()
    }

    /// Add interaction
    pub fn add_interaction(&mut self, from: &str, to: &str) {
        let mut interactions = self.interactions.lock().unwrap();
        interactions
            .entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());
    }

    /// Get interactions for an entity
    pub fn get_interactions(&self, entity: &str) -> HashSet<String> {
        let interactions = self.interactions.lock().unwrap();
        interactions.get(entity).cloned().unwrap_or_default()
    }

    /// Prepare context slice for 14B (sorted by timestamp, most recent first)
    pub fn prepare_context_slice(&self) -> Vec<ContextEvent> {
        let events = self.events.lock().unwrap();
        let mut sorted: Vec<ContextEvent> = events.iter().cloned().collect();
        sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        sorted
    }

    /// Clear all cache
    pub fn clear(&mut self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
        drop(events);

        let mut npcs = self.active_npcs.lock().unwrap();
        npcs.clear();
        drop(npcs);

        let mut interactions = self.interactions.lock().unwrap();
        interactions.clear();
        debug!("Cleared scene context cache");
    }
}

impl Default for SceneContextCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = SceneContextCache::new();
        let event = SceneEvent::Action {
            actor: "player1".to_string(),
            action: "test".to_string(),
            timestamp: SystemTime::now(),
        };

        cache.add_event(event);
        assert_eq!(cache.get_recent_events(6).len(), 1);
    }
}














