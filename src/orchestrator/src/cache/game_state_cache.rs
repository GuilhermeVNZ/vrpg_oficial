//! Game State Cache - M4.1
//! In-memory cache for game state (HP, AC, resources, status, position, initiative)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;

/// Entity identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityId {
    Player(String),
    Npc(String),
    Monster(String),
}

/// Position in 2D/3D grid
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Resource types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Rage,
    Ki,
    SorceryPoints,
    SpellSlot(u8), // Level
    Smite,
    ChannelDivinity,
}

/// Status types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatusType {
    Poisoned,
    Stealth,
    Prone,
    Stunned,
    Charmed,
    Frightened,
    Invisible,
    // Add more as needed
}

/// Game state entry for an entity
#[derive(Debug, Clone)]
pub struct GameStateEntry {
    pub hp: i32,
    pub max_hp: i32,
    pub ac: i32,
    pub resources: HashMap<ResourceType, i32>,
    pub statuses: Vec<StatusType>,
    pub position: Position,
    pub initiative: Option<i32>,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub updates: u64,
    pub invalidations: u64,
}

/// Game state cache
pub struct GameStateCache {
    data: Arc<Mutex<HashMap<EntityId, GameStateEntry>>>,
    stats: Arc<Mutex<CacheStats>>,
}

impl GameStateCache {
    /// Create new game state cache
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    /// Update entity state in cache
    pub fn update_entity(&mut self, entity_id: &EntityId, entry: GameStateEntry) {
        let mut data = self.data.lock().unwrap();
        data.insert(entity_id.clone(), entry);
        drop(data);

        let mut stats = self.stats.lock().unwrap();
        stats.updates += 1;
        debug!("Updated entity {:?} in cache", entity_id);
    }

    /// Get entity state from cache
    pub fn get_entity(&self, entity_id: &EntityId) -> Option<GameStateEntry> {
        let data = self.data.lock().unwrap();
        let entry = data.get(entity_id).cloned();
        drop(data);

        let mut stats = self.stats.lock().unwrap();
        if entry.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }

        entry
    }

    /// Invalidate entity from cache
    pub fn invalidate_entity(&mut self, entity_id: &EntityId) {
        let mut data = self.data.lock().unwrap();
        data.remove(entity_id);
        drop(data);

        let mut stats = self.stats.lock().unwrap();
        stats.invalidations += 1;
        debug!("Invalidated entity {:?} from cache", entity_id);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        self.stats.lock().unwrap().clone()
    }

    /// Clear all cache
    pub fn clear(&mut self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
        drop(data);

        let mut stats = self.stats.lock().unwrap();
        *stats = CacheStats::default();
        debug!("Cleared game state cache");
    }
}

impl Default for GameStateCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = GameStateCache::new();
        let entity_id = EntityId::Player("player1".to_string());
        let entry = GameStateEntry {
            hp: 50,
            max_hp: 50,
            ac: 15,
            resources: HashMap::new(),
            statuses: vec![],
            position: Position { x: 5, y: 3, z: 0 },
            initiative: None,
        };

        cache.update_entity(&entity_id, entry);
        let retrieved = cache.get_entity(&entity_id).unwrap();
        assert_eq!(retrieved.hp, 50);
    }
}














