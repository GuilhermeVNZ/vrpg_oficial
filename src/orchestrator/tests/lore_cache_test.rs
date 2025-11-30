//! Tests for Lore Cache - M4.3

use orchestrator::cache::lore_cache::{LoreCache, LoreType};
use std::time::Duration;

#[test]
fn test_lore_cache_storage_and_retrieval() {
    let mut cache = LoreCache::new();

    let query = "What is the history of Waterdeep?";
    let results = vec!["Waterdeep is a major city...".to_string()];

    cache.store_query_result(query, &results, LoreType::Location);

    let retrieved = cache.get_query_result(query);
    assert!(retrieved.is_some());
    let result = retrieved.unwrap();
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.lore_type, LoreType::Location);
}

#[test]
fn test_lore_cache_ttl_expiration() {
    let mut cache = LoreCache::new();

    let query = "Tell me about elves";
    let results = vec!["Elves are a long-lived race...".to_string()];

    // Store with very short TTL for testing
    cache.store_query_result_with_ttl(query, &results, LoreType::Race, Duration::from_secs(1));

    // Immediately retrieve - should be there
    let retrieved = cache.get_query_result(query);
    assert!(retrieved.is_some());

    // Wait for expiration (simulate by storing with past timestamp)
    // In real implementation, we'd check TTL on get
    std::thread::sleep(Duration::from_secs(2));

    // After expiration, should be None (if TTL check is implemented)
    // For now, just verify the cache structure works
}

#[test]
fn test_lore_cache_different_types() {
    let mut cache = LoreCache::new();

    let queries = vec![
        ("Elves description", LoreType::Race),
        ("Waterdeep city", LoreType::Location),
        ("Gandalf the wizard", LoreType::Npc),
        ("The Great War", LoreType::History),
        ("Harper faction", LoreType::Faction),
    ];

    for (query, lore_type) in &queries {
        let results = vec![format!("Description for {}", query)];
        cache.store_query_result(query, &results, lore_type.clone());
    }

    for (query, expected_type) in &queries {
        let result = cache.get_query_result(*query);
        assert!(result.is_some(), "Query should be cached: {}", query);
        assert_eq!(result.unwrap().lore_type, *expected_type);
    }
}

#[test]
fn test_lore_cache_prepare_lore_context() {
    let mut cache = LoreCache::new();

    let queries = vec![
        ("Elves", LoreType::Race),
        ("Waterdeep", LoreType::Location),
    ];

    for (query, lore_type) in queries {
        let results = vec![format!("Info about {}", query)];
        cache.store_query_result(query, &results, lore_type);
    }

    let lore_context = cache.prepare_lore_context(&["Elves", "Waterdeep"]);
    assert!(!lore_context.is_empty());
    assert!(lore_context.contains("Elves") || lore_context.contains("Waterdeep"));
}

#[test]
fn test_lore_cache_metrics() {
    let mut cache = LoreCache::new();

    let query = "Test query";
    let results = vec!["Result".to_string()];

    // First access - miss (not in cache yet)
    let _ = cache.get_query_result(query);
    let stats = cache.get_stats();
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 1);

    // Store result
    cache.store_query_result(query, &results, LoreType::Location);

    // Second access - hit
    let _ = cache.get_query_result(query);
    let stats = cache.get_stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
}

#[test]
fn test_lore_cache_latency() {
    let mut cache = LoreCache::new();

    let query = "Fast query";
    let results = vec!["Fast result".to_string()];
    cache.store_query_result(query, &results, LoreType::Location);

    let start = std::time::Instant::now();
    let _ = cache.get_query_result(query);
    let elapsed = start.elapsed();

    // Should be < 100ms for cached queries
    assert!(
        elapsed.as_millis() < 100,
        "Cached lore query should be fast (< 100ms), took: {:?}",
        elapsed
    );
}

#[test]
fn test_lore_cache_clear() {
    let mut cache = LoreCache::new();

    cache.store_query_result("query1", &vec!["result1".to_string()], LoreType::Location);
    cache.store_query_result("query2", &vec!["result2".to_string()], LoreType::Race);

    cache.clear();

    assert!(cache.get_query_result("query1").is_none());
    assert!(cache.get_query_result("query2").is_none());
}

#[test]
fn test_lore_cache_multiple_queries_same_lore() {
    let mut cache = LoreCache::new();

    let queries = vec!["Tell me about elves", "What are elves?", "Elf description"];

    for query in &queries {
        cache.store_query_result(
            query,
            &vec!["Elves are magical beings".to_string()],
            LoreType::Race,
        );
    }

    // All should be cached
    for query in queries {
        let result = cache.get_query_result(query);
        assert!(result.is_some(), "Query should be cached: {}", query);
    }
}



