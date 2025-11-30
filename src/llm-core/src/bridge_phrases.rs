// Bridge Phrases Module - Human-like bridge phrases for 1.5B model
// Prevents cognitive silence and provides natural transitions

use crate::error::{LlmError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Categories of bridge phrases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BridgeCategory {
    Neutral,
    GentlePrompt,
    Anticipation,
    TensionLow,
    TensionHigh,
    CinematicLow,
    CinematicHigh,
    Empowering,
    Empathetic,
    RoleplayPositive,
    RoleplayMysterious,
    Validation,
    Momentum,
}

impl BridgeCategory {
    /// Get all available categories
    pub fn all() -> Vec<BridgeCategory> {
        vec![
            BridgeCategory::Neutral,
            BridgeCategory::GentlePrompt,
            BridgeCategory::Anticipation,
            BridgeCategory::TensionLow,
            BridgeCategory::TensionHigh,
            BridgeCategory::CinematicLow,
            BridgeCategory::CinematicHigh,
            BridgeCategory::Empowering,
            BridgeCategory::Empathetic,
            BridgeCategory::RoleplayPositive,
            BridgeCategory::RoleplayMysterious,
            BridgeCategory::Validation,
            BridgeCategory::Momentum,
        ]
    }

    /// Convert to string key (matches JSON keys)
    pub fn to_key(&self) -> &'static str {
        match self {
            BridgeCategory::Neutral => "neutral",
            BridgeCategory::GentlePrompt => "gentle_prompt",
            BridgeCategory::Anticipation => "anticipation",
            BridgeCategory::TensionLow => "tension_low",
            BridgeCategory::TensionHigh => "tension_high",
            BridgeCategory::CinematicLow => "cinematic_low",
            BridgeCategory::CinematicHigh => "cinematic_high",
            BridgeCategory::Empowering => "empowering",
            BridgeCategory::Empathetic => "empathetic",
            BridgeCategory::RoleplayPositive => "roleplay_positive",
            BridgeCategory::RoleplayMysterious => "roleplay_mysterious",
            BridgeCategory::Validation => "validation",
            BridgeCategory::Momentum => "momentum",
        }
    }

    /// Parse from string key
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "neutral" => Some(BridgeCategory::Neutral),
            "gentle_prompt" => Some(BridgeCategory::GentlePrompt),
            "anticipation" => Some(BridgeCategory::Anticipation),
            "tension_low" => Some(BridgeCategory::TensionLow),
            "tension_high" => Some(BridgeCategory::TensionHigh),
            "cinematic_low" => Some(BridgeCategory::CinematicLow),
            "cinematic_high" => Some(BridgeCategory::CinematicHigh),
            "empowering" => Some(BridgeCategory::Empowering),
            "empathetic" => Some(BridgeCategory::Empathetic),
            "roleplay_positive" => Some(BridgeCategory::RoleplayPositive),
            "roleplay_mysterious" => Some(BridgeCategory::RoleplayMysterious),
            "validation" => Some(BridgeCategory::Validation),
            "momentum" => Some(BridgeCategory::Momentum),
            _ => None,
        }
    }
}

/// Bridge phrases database loaded from JSON
#[derive(Debug, Clone, Deserialize)]
struct BridgePhrasesData {
    #[serde(flatten)]
    categories: HashMap<String, Vec<String>>,
}

/// Bridge phrases manager with anti-repetition and anti-loop systems
pub struct BridgePhrasesManager {
    phrases: HashMap<BridgeCategory, Vec<String>>,
    recent_phrases: Arc<RwLock<VecDeque<String>>>, // Last 10 phrases used (anti-loop)
    recent_categories: Arc<RwLock<VecDeque<BridgeCategory>>>, // Last 20 categories used
    category_usage_count: Arc<RwLock<HashMap<BridgeCategory, usize>>>, // Usage count per category
    phrase_repetition_score: Arc<RwLock<HashMap<String, usize>>>, // Repetition score per phrase
    max_recent_phrases: usize,                     // Default: 10 (anti-loop requirement)
    max_recent_categories: usize,                  // Default: 20
    min_category_rotation: usize,                  // Minimum responses before reusing category
    freeze_threshold: usize,                       // Freeze if score > 3
    removal_threshold: usize,                      // Remove if score > 6
    frozen_phrases: Arc<RwLock<std::collections::HashSet<String>>>, // Frozen phrases
}

impl BridgePhrasesManager {
    /// Create a new bridge phrases manager
    pub fn new() -> Result<Self> {
        Self::with_config(10, 20, 5, 3, 6)
    }

    /// Create with custom configuration
    pub fn with_config(
        max_recent_phrases: usize,
        max_recent_categories: usize,
        min_category_rotation: usize,
        freeze_threshold: usize,
        removal_threshold: usize,
    ) -> Result<Self> {
        let phrases = Self::load_phrases()?;

        Ok(Self {
            phrases,
            recent_phrases: Arc::new(RwLock::new(VecDeque::with_capacity(max_recent_phrases))),
            recent_categories: Arc::new(RwLock::new(VecDeque::with_capacity(
                max_recent_categories,
            ))),
            category_usage_count: Arc::new(RwLock::new(HashMap::new())),
            phrase_repetition_score: Arc::new(RwLock::new(HashMap::new())),
            max_recent_phrases,
            max_recent_categories,
            min_category_rotation,
            freeze_threshold,
            removal_threshold,
            frozen_phrases: Arc::new(RwLock::new(std::collections::HashSet::new())),
        })
    }

    /// Load phrases from JSON file
    fn load_phrases() -> Result<HashMap<BridgeCategory, Vec<String>>> {
        // Try multiple possible paths (for tests and production)
        let possible_paths = vec![
            std::env::var("BRIDGE_PHRASES_PATH").ok(),
            Some("config/bridge_phrases.json".to_string()),
            Some("../config/bridge_phrases.json".to_string()),
            Some("../../config/bridge_phrases.json".to_string()),
            Some("vrpg-client/config/bridge_phrases.json".to_string()),
        ];

        let mut last_error = None;
        for config_path in possible_paths.into_iter().flatten() {
            match std::fs::read_to_string(&config_path) {
                Ok(content) => {
                    let data: BridgePhrasesData = serde_json::from_str(&content).map_err(|e| {
                        LlmError::ModelLoad(format!("Failed to parse bridge phrases: {}", e))
                    })?;

                    let mut phrases = HashMap::new();
                    for (key, phrase_list) in data.categories {
                        if let Some(category) = BridgeCategory::from_key(&key) {
                            phrases.insert(category, phrase_list);
                        } else {
                            warn!("Unknown bridge phrase category: {}", key);
                        }
                    }

                    debug!(
                        "Loaded {} bridge phrase categories from {}",
                        phrases.len(),
                        config_path
                    );
                    return Ok(phrases);
                }
                Err(e) => {
                    last_error = Some((config_path, e));
                    continue;
                }
            }
        }

        // If all paths failed, return error
        if let Some((path, err)) = last_error {
            return Err(LlmError::ModelLoad(format!(
                "Failed to read bridge phrases from any path. Last attempt: {}: {}",
                path, err
            )));
        }

        Err(LlmError::ModelLoad(
            "No bridge phrases path configured".to_string(),
        ))
    }

    /// Select a random phrase from a specific category
    /// Returns None if category is empty or all phrases were recently used/frozen
    /// Direct JSON selection (no LLM generation) - human listening to human
    pub async fn select_phrase(&self, category: BridgeCategory) -> Result<Option<String>> {
        let phrases = self
            .phrases
            .get(&category)
            .ok_or_else(|| LlmError::ModelLoad("Category not found".to_string()))?;

        if phrases.is_empty() {
            return Ok(None);
        }

        // Get available phrases (not in recent history, not frozen, not removed)
        let recent = self.recent_phrases.read().await;
        let frozen = self.frozen_phrases.read().await;
        let scores = self.phrase_repetition_score.read().await;

        let available: Vec<&String> = phrases
            .iter()
            .filter(|phrase| {
                // Not in recent history (last 10)
                !recent.contains(*phrase)
                    // Not frozen (score > 3)
                    && !frozen.contains(*phrase)
                    // Not removed (score <= 6)
                    && scores.get(*phrase).copied().unwrap_or(0) <= self.removal_threshold
            })
            .collect();

        drop(recent);
        drop(frozen);
        drop(scores);

        // If all phrases in category were recently used/frozen, allow reuse but prefer others
        let phrase = if available.is_empty() {
            // Anti-loop: if this category was used too recently, try another
            if self.is_category_too_recent(&category).await {
                return Ok(None); // Force category rotation
            }
            // Fallback: use any phrase from category (all were used recently)
            // But still avoid frozen/removed phrases
            let scores = self.phrase_repetition_score.read().await;
            let frozen = self.frozen_phrases.read().await;
            let fallback: Vec<&String> = phrases
                .iter()
                .filter(|phrase| {
                    scores.get(*phrase).copied().unwrap_or(0) <= self.removal_threshold
                        && !frozen.contains(*phrase)
                })
                .collect();
            drop(scores);
            drop(frozen);
            fallback
                .get(fastrand::usize(..fallback.len()))
                .copied()
                .cloned()
        } else {
            available
                .get(fastrand::usize(..available.len()))
                .map(|s| (*s).clone())
        };

        if let Some(selected) = phrase {
            self.record_usage(&selected, category).await;
            Ok(Some(selected))
        } else {
            Ok(None)
        }
    }

    /// Select a phrase from any category (smart selection with anti-loop)
    pub async fn select_phrase_any(&self) -> Result<String> {
        // Get categories sorted by usage (least used first)
        let categories = self.get_categories_by_usage().await;

        // Try each category until we find an available phrase
        for category in &categories {
            if let Ok(Some(phrase)) = self.select_phrase(*category).await {
                return Ok(phrase);
            }
        }

        // If all categories failed (shouldn't happen), use neutral as fallback
        self.select_phrase(BridgeCategory::Neutral)
            .await?
            .ok_or_else(|| LlmError::ModelLoad("No bridge phrases available".to_string()))
    }

    /// Select a phrase from a category, with automatic category rotation if needed
    pub async fn select_phrase_with_rotation(
        &self,
        preferred_category: BridgeCategory,
    ) -> Result<String> {
        // Check if preferred category is too recent
        if self.is_category_too_recent(&preferred_category).await {
            // Force rotation: select from different category
            let categories = self.get_categories_by_usage().await;
            for category in &categories {
                if *category != preferred_category {
                    if let Ok(Some(phrase)) = self.select_phrase(*category).await {
                        return Ok(phrase);
                    }
                }
            }
        }

        // Try preferred category first
        if let Ok(Some(phrase)) = self.select_phrase(preferred_category).await {
            return Ok(phrase);
        }

        // Fallback to any available
        self.select_phrase_any().await
    }

    /// Check if a category was used too recently (anti-loop)
    async fn is_category_too_recent(&self, category: &BridgeCategory) -> bool {
        let recent = self.recent_categories.read().await;
        let recent_vec: Vec<&BridgeCategory> = recent.iter().collect();

        // Check if category appears in last min_category_rotation entries
        if recent_vec.len() < self.min_category_rotation {
            return false;
        }

        let last_n: Vec<BridgeCategory> = recent_vec
            .iter()
            .rev()
            .take(self.min_category_rotation)
            .copied()
            .copied()
            .collect();

        last_n.contains(category)
    }

    /// Get categories sorted by usage (least used first)
    async fn get_categories_by_usage(&self) -> Vec<BridgeCategory> {
        let usage = self.category_usage_count.read().await;
        let mut categories: Vec<BridgeCategory> = BridgeCategory::all();

        categories.sort_by_key(|cat| usage.get(cat).copied().unwrap_or(0));
        categories
    }

    /// Record phrase and category usage
    /// Implements anti-loop scoring: freeze if score > 3, remove if score > 6
    async fn record_usage(&self, phrase: &str, category: BridgeCategory) {
        // Check if phrase was recently used (increases repetition score)
        let was_recent = {
            let recent = self.recent_phrases.read().await;
            recent.contains(&phrase.to_string())
        };

        // Update repetition score
        let mut scores = self.phrase_repetition_score.write().await;
        if was_recent {
            // Phrase was reused - increment score
            let score = scores.entry(phrase.to_string()).or_insert(0);
            *score += 1;

            // Apply freeze if score > 3
            if *score > self.freeze_threshold {
                let mut frozen = self.frozen_phrases.write().await;
                frozen.insert(phrase.to_string());
                warn!("Bridge phrase frozen (score {}): {}", score, phrase);
            }

            // Apply removal if score > 6 (manual removal - phrase won't be selected)
            if *score > self.removal_threshold {
                warn!(
                    "Bridge phrase should be manually removed (score {}): {}",
                    score, phrase
                );
            }
        } else {
            // Phrase is new - reset score if it exists
            scores.remove(phrase);
            // Remove from frozen if it was frozen
            let mut frozen = self.frozen_phrases.write().await;
            frozen.remove(phrase);
        }
        drop(scores);

        // Record phrase in recent history (last 10 for anti-loop)
        let mut recent_phrases = self.recent_phrases.write().await;
        recent_phrases.push_back(phrase.to_string());
        if recent_phrases.len() > self.max_recent_phrases {
            let removed = recent_phrases.pop_front();
            // Clean up score for removed phrase if it's been a while
            if let Some(removed_phrase) = removed {
                let mut scores = self.phrase_repetition_score.write().await;
                // Only remove score if it's low (high scores should persist for monitoring)
                if scores.get(&removed_phrase).copied().unwrap_or(0) <= 2 {
                    scores.remove(&removed_phrase);
                }
            }
        }
        drop(recent_phrases);

        // Record category
        let mut recent_categories = self.recent_categories.write().await;
        recent_categories.push_back(category);
        if recent_categories.len() > self.max_recent_categories {
            recent_categories.pop_front();
        }
        drop(recent_categories);

        // Update usage count
        let mut usage = self.category_usage_count.write().await;
        *usage.entry(category).or_insert(0) += 1;
    }

    /// Get statistics about phrase usage
    pub async fn get_stats(&self) -> BridgePhrasesStats {
        let recent_phrases = self.recent_phrases.read().await;
        let recent_categories = self.recent_categories.read().await;
        let usage = self.category_usage_count.read().await;
        let scores = self.phrase_repetition_score.read().await;
        let frozen = self.frozen_phrases.read().await;

        BridgePhrasesStats {
            total_phrases: self.phrases.values().map(|v| v.len()).sum::<usize>(),
            recent_phrases_count: recent_phrases.len(),
            recent_categories_count: recent_categories.len(),
            category_usage: usage.clone(),
            frozen_phrases_count: frozen.len(),
            high_score_phrases: scores
                .iter()
                .filter(|(_, &score)| score > self.removal_threshold)
                .count(),
        }
    }

    /// Clear recent history (useful for testing or reset)
    pub async fn clear_history(&self) {
        let mut recent_phrases = self.recent_phrases.write().await;
        recent_phrases.clear();
        drop(recent_phrases);

        let mut recent_categories = self.recent_categories.write().await;
        recent_categories.clear();
        drop(recent_categories);

        let mut usage = self.category_usage_count.write().await;
        usage.clear();
        drop(usage);

        let mut scores = self.phrase_repetition_score.write().await;
        scores.clear();
        drop(scores);

        let mut frozen = self.frozen_phrases.write().await;
        frozen.clear();
    }
}

impl Default for BridgePhrasesManager {
    fn default() -> Self {
        Self::new().expect("Failed to create BridgePhrasesManager")
    }
}

/// Statistics about bridge phrases usage
#[derive(Debug, Clone)]
pub struct BridgePhrasesStats {
    pub total_phrases: usize,
    pub recent_phrases_count: usize,
    pub recent_categories_count: usize,
    pub category_usage: HashMap<BridgeCategory, usize>,
    pub frozen_phrases_count: usize,
    pub high_score_phrases: usize, // Phrases with score > 6 (should be manually removed)
}
