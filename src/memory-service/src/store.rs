use crate::error::{MemoryError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub vector_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub query: String,
    pub limit: Option<usize>,
    pub filters: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub memory: Memory,
    pub score: f32,
}

pub struct MemoryStore {
    memories: Arc<RwLock<HashMap<String, Memory>>>,
    index: Arc<RwLock<HashMap<String, Vec<String>>>>, // metadata -> memory_ids
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            memories: Arc::new(RwLock::new(HashMap::new())),
            index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn store(
        &self,
        content: String,
        metadata: HashMap<String, String>,
    ) -> Result<Memory> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let memory = Memory {
            id: id.clone(),
            content,
            metadata: metadata.clone(),
            created_at: now,
            updated_at: now,
            vector_id: None,
        };

        // Store memory
        {
            let mut memories = self.memories.write().await;
            memories.insert(id.clone(), memory.clone());
        }

        // Update index
        {
            let mut index = self.index.write().await;
            for (key, value) in &metadata {
                let index_key = format!("{}:{}", key, value);
                index
                    .entry(index_key)
                    .or_insert_with(Vec::new)
                    .push(id.clone());
            }
        }

        Ok(memory)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Memory>> {
        let memories = self.memories.read().await;
        Ok(memories.get(id).cloned())
    }

    pub async fn update(
        &self,
        id: &str,
        content: Option<String>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Memory> {
        let mut memories = self.memories.write().await;

        let memory = memories
            .get_mut(id)
            .ok_or_else(|| MemoryError::Storage(format!("Memory not found: {}", id)))?;

        if let Some(new_content) = content {
            memory.content = new_content;
        }

        if let Some(new_metadata) = metadata {
            // Update index
            {
                let mut index = self.index.write().await;
                // Remove old index entries
                for (key, value) in &memory.metadata {
                    let index_key = format!("{}:{}", key, value);
                    if let Some(ids) = index.get_mut(&index_key) {
                        ids.retain(|x| x != id);
                    }
                }
                // Add new index entries
                for (key, value) in &new_metadata {
                    let index_key = format!("{}:{}", key, value);
                    index
                        .entry(index_key)
                        .or_insert_with(Vec::new)
                        .push(id.to_string());
                }
            }
            memory.metadata = new_metadata;
        }

        memory.updated_at = Utc::now();

        Ok(memory.clone())
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let memory = {
            let memories = self.memories.read().await;
            memories.get(id).cloned()
        };

        if let Some(memory) = memory {
            // Remove from index
            {
                let mut index = self.index.write().await;
                for (key, value) in &memory.metadata {
                    let index_key = format!("{}:{}", key, value);
                    if let Some(ids) = index.get_mut(&index_key) {
                        ids.retain(|x| x != id);
                    }
                }
            }

            // Remove from store
            {
                let mut memories = self.memories.write().await;
                memories.remove(id);
            }
        }

        Ok(())
    }

    pub async fn search(&self, query: &MemoryQuery) -> Result<Vec<SearchResult>> {
        let memories = self.memories.read().await;
        let mut results = Vec::new();

        // Simple text search (in real implementation, this would use vector search)
        let query_lower = query.query.to_lowercase();

        for memory in memories.values() {
            let content_lower = memory.content.to_lowercase();

            // Check if query matches content
            let score = if content_lower.contains(&query_lower) {
                // Simple scoring: count matches
                let matches = content_lower.matches(&query_lower).count() as f32;
                1.0 / (1.0 + matches)
            } else {
                0.0
            };

            // Apply filters if provided
            if let Some(filters) = &query.filters {
                let mut matches = true;
                for (key, value) in filters {
                    if let Some(memory_value) = memory.metadata.get(key) {
                        if memory_value != value {
                            matches = false;
                            break;
                        }
                    } else {
                        matches = false;
                        break;
                    }
                }
                if !matches {
                    continue;
                }
            }

            if score > 0.0 {
                results.push(SearchResult {
                    memory: memory.clone(),
                    score,
                });
            }
        }

        // Sort by score (descending)
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    pub async fn list_all(&self) -> Vec<Memory> {
        let memories = self.memories.read().await;
        memories.values().cloned().collect()
    }

    pub async fn count(&self) -> usize {
        let memories = self.memories.read().await;
        memories.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store() {
        let store = MemoryStore::new();
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "npc".to_string());
        metadata.insert("name".to_string(), "Gandalf".to_string());

        let memory = store
            .store("Gandalf is a wizard".to_string(), metadata)
            .await
            .unwrap();
        assert!(!memory.id.is_empty());
        assert_eq!(memory.content, "Gandalf is a wizard");
    }

    #[tokio::test]
    async fn test_memory_get() {
        let store = MemoryStore::new();
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "npc".to_string());

        let memory = store
            .store("Test content".to_string(), metadata)
            .await
            .unwrap();
        let retrieved = store.get(&memory.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "Test content");
    }

    #[tokio::test]
    async fn test_memory_search() {
        let store = MemoryStore::new();
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "npc".to_string());

        store
            .store("Gandalf is a wizard".to_string(), metadata.clone())
            .await
            .unwrap();
        store
            .store("Frodo is a hobbit".to_string(), metadata.clone())
            .await
            .unwrap();

        let query = MemoryQuery {
            query: "wizard".to_string(),
            limit: Some(10),
            filters: None,
        };

        let results = store.search(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].memory.content.contains("wizard"));
    }

    #[tokio::test]
    async fn test_memory_delete() {
        let store = MemoryStore::new();
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "npc".to_string());

        let memory = store.store("Test".to_string(), metadata).await.unwrap();
        store.delete(&memory.id).await.unwrap();

        let retrieved = store.get(&memory.id).await.unwrap();
        assert!(retrieved.is_none());
    }
}
