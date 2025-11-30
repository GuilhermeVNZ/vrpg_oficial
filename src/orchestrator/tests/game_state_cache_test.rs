//! Tests for Game State Cache - M4.1

use orchestrator::cache::game_state_cache::{
    EntityId, GameStateCache, GameStateEntry, Position, ResourceType, StatusType,
};
use std::collections::HashMap;

#[test]
fn test_game_state_cache_storage_and_retrieval() {
    let mut cache = GameStateCache::new();

    let entity_id = EntityId::Player("player1".to_string());
    let entry = GameStateEntry {
        hp: 50,
        max_hp: 50,
        ac: 15,
        resources: HashMap::new(),
        statuses: vec![],
        position: Position { x: 5, y: 3, z: 0 },
        initiative: Some(15),
    };

    cache.update_entity(&entity_id, entry.clone());

    let retrieved = cache.get_entity(&entity_id).unwrap();
    assert_eq!(retrieved.hp, 50);
    assert_eq!(retrieved.ac, 15);
    assert_eq!(retrieved.position.x, 5);
}

#[test]
fn test_game_state_cache_update() {
    let mut cache = GameStateCache::new();

    let entity_id = EntityId::Player("player1".to_string());
    let mut entry = GameStateEntry {
        hp: 50,
        max_hp: 50,
        ac: 15,
        resources: HashMap::new(),
        statuses: vec![],
        position: Position { x: 5, y: 3, z: 0 },
        initiative: Some(15),
    };

    cache.update_entity(&entity_id, entry.clone());

    // Update HP
    entry.hp = 30;
    cache.update_entity(&entity_id, entry.clone());

    let retrieved = cache.get_entity(&entity_id).unwrap();
    assert_eq!(retrieved.hp, 30);
    assert_eq!(retrieved.max_hp, 50);
}

#[test]
fn test_game_state_cache_resources() {
    let mut cache = GameStateCache::new();

    let entity_id = EntityId::Player("player1".to_string());
    let mut entry = GameStateEntry {
        hp: 50,
        max_hp: 50,
        ac: 15,
        resources: HashMap::new(),
        statuses: vec![],
        position: Position { x: 5, y: 3, z: 0 },
        initiative: None,
    };

    entry.resources.insert(ResourceType::Rage, 3);
    entry.resources.insert(ResourceType::SpellSlot(3), 2);
    entry.resources.insert(ResourceType::Ki, 5);

    cache.update_entity(&entity_id, entry);

    let retrieved = cache.get_entity(&entity_id).unwrap();
    assert_eq!(retrieved.resources.get(&ResourceType::Rage), Some(&3));
    assert_eq!(
        retrieved.resources.get(&ResourceType::SpellSlot(3)),
        Some(&2)
    );
    assert_eq!(retrieved.resources.get(&ResourceType::Ki), Some(&5));
}

#[test]
fn test_game_state_cache_statuses() {
    let mut cache = GameStateCache::new();

    let entity_id = EntityId::Npc("goblin1".to_string());
    let mut entry = GameStateEntry {
        hp: 20,
        max_hp: 20,
        ac: 12,
        resources: HashMap::new(),
        statuses: vec![StatusType::Poisoned, StatusType::Prone],
        position: Position { x: 10, y: 5, z: 0 },
        initiative: Some(12),
    };

    cache.update_entity(&entity_id, entry);

    let retrieved = cache.get_entity(&entity_id).unwrap();
    assert!(retrieved.statuses.contains(&StatusType::Poisoned));
    assert!(retrieved.statuses.contains(&StatusType::Prone));
}

#[test]
fn test_game_state_cache_invalidation() {
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
    assert!(cache.get_entity(&entity_id).is_some());

    cache.invalidate_entity(&entity_id);
    assert!(cache.get_entity(&entity_id).is_none());
}

#[test]
fn test_game_state_cache_latency() {
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

    let start = std::time::Instant::now();
    let _retrieved = cache.get_entity(&entity_id).unwrap();
    let elapsed = start.elapsed();

    // Should be < 10ms for cache queries
    assert!(
        elapsed.as_millis() < 10,
        "Cache query should be fast (< 10ms), took: {:?}",
        elapsed
    );
}

#[test]
fn test_game_state_cache_multiple_entities() {
    let mut cache = GameStateCache::new();

    let player_id = EntityId::Player("player1".to_string());
    let npc_id = EntityId::Npc("goblin1".to_string());

    let player_entry = GameStateEntry {
        hp: 50,
        max_hp: 50,
        ac: 15,
        resources: HashMap::new(),
        statuses: vec![],
        position: Position { x: 5, y: 3, z: 0 },
        initiative: Some(15),
    };

    let npc_entry = GameStateEntry {
        hp: 20,
        max_hp: 20,
        ac: 12,
        resources: HashMap::new(),
        statuses: vec![StatusType::Poisoned],
        position: Position { x: 10, y: 5, z: 0 },
        initiative: Some(12),
    };

    cache.update_entity(&player_id, player_entry);
    cache.update_entity(&npc_id, npc_entry);

    let player = cache.get_entity(&player_id).unwrap();
    let npc = cache.get_entity(&npc_id).unwrap();

    assert_eq!(player.hp, 50);
    assert_eq!(npc.hp, 20);
    assert!(npc.statuses.contains(&StatusType::Poisoned));
}

#[test]
fn test_game_state_cache_metrics() {
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

    // First access after update - hit (entity is in cache)
    let _ = cache.get_entity(&entity_id);
    let stats = cache.get_stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 0);

    // Second access - hit
    let _ = cache.get_entity(&entity_id);
    let stats = cache.get_stats();
    assert_eq!(stats.hits, 2);
    assert_eq!(stats.misses, 0);

    // Access non-existent entity - miss
    let non_existent = EntityId::Player("player2".to_string());
    let _ = cache.get_entity(&non_existent);
    let stats = cache.get_stats();
    assert_eq!(stats.hits, 2);
    assert_eq!(stats.misses, 1);
}
