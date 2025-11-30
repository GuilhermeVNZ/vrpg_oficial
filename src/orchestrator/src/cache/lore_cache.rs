//! Lore Cache - M4.3
//! Cache for lore queries using Vectorizer (races, locations, NPCs, history, factions)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tracing::debug;

/// Lore types for categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoreType {
    Race,
    Location,
    Npc,
    History,
    Faction,
    Other,
}

/// Cached lore query result
#[derive(Debug, Clone)]
pub struct LoreQueryResult {
    /// Original query
    pub query: String,
    /// Search results from Vectorizer
    pub results: Vec<String>,
    /// Type of lore
    pub lore_type: LoreType,
    /// Timestamp when cached
    pub cached_at: SystemTime,
    /// TTL (Time To Live)
    pub ttl: Duration,
}

impl LoreQueryResult {
    /// Check if result is still valid (not expired)
    pub fn is_valid(&self) -> bool {
        if let Ok(elapsed) = self.cached_at.elapsed() {
            elapsed < self.ttl
        } else {
            // SystemTime went backwards (shouldn't happen, but handle gracefully)
            false
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct LoreCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub stores: u64,
    pub evictions: u64,
}

/// Lore cache with TTL support
pub struct LoreCache {
    /// Cached results
    data: Arc<Mutex<HashMap<String, LoreQueryResult>>>,
    /// Statistics
    stats: Arc<Mutex<LoreCacheStats>>,
    /// Default TTL (5 minutes)
    default_ttl: Duration,
}

impl LoreCache {
    /// Create new lore cache with default TTL of 5 minutes
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(LoreCacheStats::default())),
            default_ttl: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Store query result with default TTL (5 minutes)
    pub fn store_query_result(
        &mut self,
        query: &str,
        results: &[String],
        lore_type: LoreType,
    ) {
        self.store_query_result_with_ttl(query, results, lore_type, self.default_ttl);
    }

    /// Store query result with custom TTL
    pub fn store_query_result_with_ttl(
        &mut self,
        query: &str,
        results: &[String],
        lore_type: LoreType,
        ttl: Duration,
    ) {
        let result = LoreQueryResult {
            query: query.to_string(),
            results: results.to_vec(),
            lore_type,
            cached_at: SystemTime::now(),
            ttl,
        };

        let mut data = self.data.lock().unwrap();
        data.insert(query.to_string(), result);
        drop(data);

        let mut stats = self.stats.lock().unwrap();
        stats.stores += 1;
        debug!("Stored lore query result: {} (TTL: {:?})", query, ttl);
    }

    /// Get query result (returns None if not cached or expired)
    pub fn get_query_result(&self, query: &str) -> Option<LoreQueryResult> {
        let mut data = self.data.lock().unwrap();
        let result = data.get(query).cloned();

        let mut stats = self.stats.lock().unwrap();

        match result {
            Some(ref res) if res.is_valid() => {
                stats.hits += 1;
                Some(res.clone())
            }
            Some(_) => {
                // Expired - remove from cache
                data.remove(query);
                stats.evictions += 1;
                stats.misses += 1;
                None
            }
            None => {
                stats.misses += 1;
                None
            }
        }
    }

    /// Prepare lore context string for 14B from multiple queries
    pub fn prepare_lore_context(&self, queries: &[&str]) -> String {
        let mut context_parts = Vec::new();

        for query in queries {
            if let Some(result) = self.get_query_result(query) {
                let results_text = result.results.join("\n");
                context_parts.push(format!(
                    "[{}] {}\n{}",
                    format!("{:?}", result.lore_type),
                    result.query,
                    results_text
                ));
            }
        }

        if context_parts.is_empty() {
            String::new()
        } else {
            format!("[LORE_CONTEXT]\n{}\n[/LORE_CONTEXT]", context_parts.join("\n\n"))
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> LoreCacheStats {
        self.stats.lock().unwrap().clone()
    }

    /// Clear all cache
    pub fn clear(&mut self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
        drop(data);

        let mut stats = self.stats.lock().unwrap();
        *stats = LoreCacheStats::default();
        debug!("Cleared lore cache");
    }

    /// Clean expired entries (call periodically)
    pub fn clean_expired(&mut self) {
        let mut data = self.data.lock().unwrap();
        let mut expired = Vec::new();

        for (query, result) in data.iter() {
            if !result.is_valid() {
                expired.push(query.clone());
            }
        }

        let expired_count = expired.len();
        for query in expired {
            data.remove(&query);
        }

        if expired_count > 0 {
            let mut stats = self.stats.lock().unwrap();
            stats.evictions += expired_count as u64;
            debug!("Cleaned {} expired lore cache entries", expired_count);
        }
    }
}

impl Default for LoreCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = LoreCache::new();
        cache.store_query_result(
            "test",
            &vec!["result".to_string()],
            LoreType::Location,
        );

        let result = cache.get_query_result("test");
        assert!(result.is_some());
    }
}

