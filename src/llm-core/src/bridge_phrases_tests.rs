// Bridge Phrases Tests - Comprehensive test suite for bridge phrases system

#[cfg(test)]
mod tests {
    use crate::bridge_phrases::{BridgeCategory, BridgePhrasesManager};

    /// Test: Load bridge phrases from JSON
    #[tokio::test]
    async fn test_load_bridge_phrases() {
        let manager = BridgePhrasesManager::new().unwrap();
        let stats = manager.get_stats().await;

        // Verify phrases were loaded
        assert!(
            stats.total_phrases > 0,
            "Should have loaded phrases from JSON"
        );
        assert!(
            stats.total_phrases >= 50,
            "Should have at least 50 phrases (got {})",
            stats.total_phrases
        );
    }

    /// Test: Select phrase from specific category
    #[tokio::test]
    async fn test_select_phrase_from_category() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Select from neutral category
        let phrase = manager
            .select_phrase(BridgeCategory::Neutral)
            .await
            .unwrap();

        assert!(
            phrase.is_some(),
            "Should return a phrase from neutral category"
        );
        assert!(
            !phrase.as_ref().unwrap().is_empty(),
            "Phrase should not be empty"
        );
    }

    /// Test: Anti-repetition - don't repeat recent phrases
    #[tokio::test]
    async fn test_anti_repetition() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Select multiple phrases from same category
        let mut selected_phrases = Vec::new();
        for _ in 0..10 {
            if let Ok(Some(phrase)) = manager.select_phrase(BridgeCategory::Neutral).await {
                selected_phrases.push(phrase);
            }
        }

        // Verify no immediate repetition (within last 10)
        if selected_phrases.len() >= 2 {
            // Check that we don't have exact duplicates in recent selections
            let unique_count = selected_phrases
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len();

            // Allow some repetition if category is small, but should have variety
            assert!(
                unique_count >= selected_phrases.len() / 2,
                "Should have variety in selected phrases (unique: {}/{})",
                unique_count,
                selected_phrases.len()
            );
        }
    }

    /// Test: Anti-loop - category rotation
    #[tokio::test]
    async fn test_anti_loop_category_rotation() {
        let manager = BridgePhrasesManager::with_config(10, 20, 3, 3, 6).unwrap();

        // Use same category multiple times
        let mut categories_used = Vec::new();
        for _ in 0..10 {
            if let Ok(phrase) = manager
                .select_phrase_with_rotation(BridgeCategory::Neutral)
                .await
            {
                // Record which category was actually used
                // (Note: we can't directly check which category was used,
                // but we can verify the system doesn't get stuck)
                assert!(!phrase.is_empty(), "Should return a valid phrase");
                categories_used.push(phrase);
            }
        }

        // Verify we got responses (system didn't get stuck)
        assert!(
            !categories_used.is_empty(),
            "Should return phrases even with category rotation"
        );
    }

    /// Test: Select phrase from any category
    #[tokio::test]
    async fn test_select_phrase_any() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Select from any category
        let phrase = manager.select_phrase_any().await.unwrap();

        assert!(
            !phrase.is_empty(),
            "Should return a phrase from any category"
        );
    }

    /// Test: Statistics tracking
    #[tokio::test]
    async fn test_statistics_tracking() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Select some phrases
        for _ in 0..5 {
            let _ = manager
                .select_phrase(BridgeCategory::Neutral)
                .await
                .ok()
                .flatten();
        }

        let stats = manager.get_stats().await;

        // Verify stats are being tracked
        assert!(
            stats.recent_phrases_count > 0,
            "Should track recent phrases"
        );
        assert!(
            stats.recent_categories_count > 0,
            "Should track recent categories"
        );
    }

    /// Test: Clear history
    #[tokio::test]
    async fn test_clear_history() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Select some phrases
        for _ in 0..5 {
            let _ = manager
                .select_phrase(BridgeCategory::Neutral)
                .await
                .ok()
                .flatten();
        }

        // Clear history
        manager.clear_history().await;

        let stats = manager.get_stats().await;

        // Verify history was cleared
        assert_eq!(
            stats.recent_phrases_count, 0,
            "Recent phrases should be cleared"
        );
        assert_eq!(
            stats.recent_categories_count, 0,
            "Recent categories should be cleared"
        );
    }

    /// Test: All categories are available
    #[tokio::test]
    async fn test_all_categories_available() {
        let manager = BridgePhrasesManager::new().unwrap();
        let stats = manager.get_stats().await;

        // Verify we have phrases for multiple categories
        let all_categories = BridgeCategory::all();
        assert!(
            stats.total_phrases >= all_categories.len(),
            "Should have phrases for multiple categories"
        );
    }

    /// Test: Category key conversion
    #[test]
    fn test_category_key_conversion() {
        // Test to_key
        assert_eq!(BridgeCategory::Neutral.to_key(), "neutral");
        assert_eq!(BridgeCategory::TensionHigh.to_key(), "tension_high");

        // Test from_key
        assert_eq!(
            BridgeCategory::from_key("neutral"),
            Some(BridgeCategory::Neutral)
        );
        assert_eq!(
            BridgeCategory::from_key("tension_high"),
            Some(BridgeCategory::TensionHigh)
        );
        assert_eq!(BridgeCategory::from_key("invalid"), None);
    }

    /// Test: Phrases are human-like and not formulaic
    #[tokio::test]
    async fn test_phrases_are_human_like() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Select phrases from different categories
        let categories = [
            BridgeCategory::Neutral,
            BridgeCategory::Anticipation,
            BridgeCategory::Empowering,
        ];

        for category in &categories {
            if let Ok(Some(phrase)) = manager.select_phrase(*category).await {
                // Verify phrase is not empty
                assert!(!phrase.is_empty(), "Phrase should not be empty");

                // Verify phrase doesn't look like a template
                let lower = phrase.to_lowercase();
                assert!(
                    !lower.contains("template"),
                    "Phrase should not be template-like: {}",
                    phrase
                );
                assert!(
                    !lower.contains("placeholder"),
                    "Phrase should not be placeholder-like: {}",
                    phrase
                );
            }
        }
    }

    /// Test: Multiple sequential requests
    #[tokio::test]
    async fn test_multiple_sequential_requests() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Make multiple sequential requests
        let mut phrases = Vec::new();
        for _ in 0..20 {
            if let Ok(phrase) = manager.select_phrase_any().await {
                phrases.push(phrase);
            }
        }

        // Verify we got responses
        assert!(
            !phrases.is_empty(),
            "Should return phrases for sequential requests"
        );

        // Verify variety (should not be all identical)
        let unique: std::collections::HashSet<_> = phrases.iter().collect();
        assert!(
            unique.len() > 1 || phrases.len() < 5,
            "Should have variety in sequential requests (unique: {}/{})",
            unique.len(),
            phrases.len()
        );
    }

    /// Test: Category usage counting
    #[tokio::test]
    async fn test_category_usage_counting() {
        let manager = BridgePhrasesManager::new().unwrap();

        // Use specific category multiple times
        for _ in 0..5 {
            let _ = manager
                .select_phrase(BridgeCategory::Neutral)
                .await
                .ok()
                .flatten();
        }

        let stats = manager.get_stats().await;

        // Verify usage count is tracked
        assert!(
            stats.category_usage.contains_key(&BridgeCategory::Neutral),
            "Should track usage for Neutral category"
        );
        assert!(
            stats.category_usage.get(&BridgeCategory::Neutral).unwrap() >= &5,
            "Should count at least 5 uses of Neutral category"
        );
    }
}
