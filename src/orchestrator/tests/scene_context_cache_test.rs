//! Tests for Scene Context Cache - M4.2

use orchestrator::cache::scene_context_cache::{NpcId, SceneContextCache, SceneEvent};
use orchestrator::pipeline::context_14b::ContextEvent;
use std::time::{Duration, SystemTime};

#[test]
fn test_scene_context_cache_storage() {
    let mut cache = SceneContextCache::new();

    let event = SceneEvent::Action {
        actor: "player1".to_string(),
        action: "attacks goblin with sword".to_string(),
        timestamp: SystemTime::now(),
    };

    cache.add_event(event.clone());
    let events = cache.get_recent_events(6);

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, "action");
}

#[test]
fn test_scene_context_cache_limit() {
    let mut cache = SceneContextCache::new();

    // Add more than 6 events
    for i in 0..10 {
        let event = SceneEvent::Action {
            actor: format!("player{}", i),
            action: format!("action {}", i),
            timestamp: SystemTime::now() + Duration::from_secs(i),
        };
        cache.add_event(event);
    }

    let events = cache.get_recent_events(6);
    assert_eq!(events.len(), 6, "Should only keep last 6 events");
}

#[test]
fn test_scene_context_cache_roll_results() {
    let mut cache = SceneContextCache::new();

    let event = SceneEvent::Roll {
        actor: "player1".to_string(),
        roll_type: "attack".to_string(),
        result: 18,
        timestamp: SystemTime::now(),
    };

    cache.add_event(event);
    let events = cache.get_recent_events(6);

    assert_eq!(events.len(), 1);
    if let ContextEvent {
        event_type,
        description,
        ..
    } = &events[0]
    {
        assert_eq!(event_type, "roll");
        assert!(description.contains("18"));
    }
}

#[test]
fn test_scene_context_cache_active_npcs() {
    let mut cache = SceneContextCache::new();

    cache.add_active_npc(NpcId("goblin1".to_string()));
    cache.add_active_npc(NpcId("goblin2".to_string()));

    let npcs = cache.get_active_npcs();
    assert_eq!(npcs.len(), 2);
    assert!(npcs.contains(&NpcId("goblin1".to_string())));
    assert!(npcs.contains(&NpcId("goblin2".to_string())));
}

#[test]
fn test_scene_context_cache_interactions() {
    let mut cache = SceneContextCache::new();

    cache.add_interaction("player1", "goblin1");
    cache.add_interaction("player1", "goblin2");

    let interactions = cache.get_interactions("player1");
    assert_eq!(interactions.len(), 2);
    assert!(interactions.contains(&"goblin1".to_string()));
    assert!(interactions.contains(&"goblin2".to_string()));
}

#[test]
fn test_scene_context_cache_prepare_context_slice() {
    let mut cache = SceneContextCache::new();

    // Add multiple events
    for i in 0..5 {
        let event = SceneEvent::Action {
            actor: format!("player{}", i),
            action: format!("action {}", i),
            timestamp: SystemTime::now() + Duration::from_secs(i as u64),
        };
        cache.add_event(event);
    }

    let context_events = cache.prepare_context_slice();
    assert_eq!(context_events.len(), 5);

    // Events should be sorted by timestamp (most recent first)
    for i in 0..context_events.len() - 1 {
        assert!(
            context_events[i].timestamp >= context_events[i + 1].timestamp,
            "Events should be sorted by timestamp (most recent first)"
        );
    }
}

#[test]
fn test_scene_context_cache_remove_npc() {
    let mut cache = SceneContextCache::new();

    cache.add_active_npc(NpcId("goblin1".to_string()));
    cache.add_active_npc(NpcId("goblin2".to_string()));

    cache.remove_active_npc(&NpcId("goblin1".to_string()));

    let npcs = cache.get_active_npcs();
    assert_eq!(npcs.len(), 1);
    assert!(!npcs.contains(&NpcId("goblin1".to_string())));
    assert!(npcs.contains(&NpcId("goblin2".to_string())));
}

#[test]
fn test_scene_context_cache_clear() {
    let mut cache = SceneContextCache::new();

    cache.add_event(SceneEvent::Action {
        actor: "player1".to_string(),
        action: "test".to_string(),
        timestamp: SystemTime::now(),
    });
    cache.add_active_npc(NpcId("goblin1".to_string()));

    cache.clear();

    assert_eq!(cache.get_recent_events(6).len(), 0);
    assert_eq!(cache.get_active_npcs().len(), 0);
}
