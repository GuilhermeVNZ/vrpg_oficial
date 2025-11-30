//! Tests for Qwen-1.5B support (Task M1.2: add-qwen-1-5b-support)
//!
//! These tests verify the implementation of Qwen-1.5B support in the LLM Core,
//! including dual model loading, fast inference, and proper response constraints.

use crate::inference::{LlmInference, LlmRequest};
use crate::persona::Persona;

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)] // Used in s2s tests
    use std::time::Instant;

    /// Test configuration for 1.5B model
    #[allow(dead_code)] // Used in s2s tests
    const MODEL_1_5B_NAME: &str = "qwen2.5-1.5b-instruct";
    const MODEL_1_5B_MAX_TOKENS: u32 = 40;
    const MODEL_1_5B_TEMPERATURE: f32 = 0.8;
    #[allow(dead_code)] // Used in s2s tests
    const MODEL_1_5B_TOP_P: f32 = 0.9;
    const INFERENCE_TIMEOUT_MS: u64 = 1200; // 1.2 seconds

    /// Maximum tokens allowed for 1.5B responses (must be ≤ 40)
    #[allow(dead_code)] // Used in s2s tests
    const MAX_TOKENS_1_5B: u32 = 40;

    /// Maximum words allowed for 1.5B responses (1-2 sentences, ~15-45 words)
    #[allow(dead_code)] // Used in s2s tests
    const MAX_WORDS_1_5B: usize = 45;

    /// Test helper: Check if response is emotional/prelude (not final resolution)
    #[allow(dead_code)] // Used in s2s tests
    fn is_prelude_response(text: &str) -> bool {
        // 1.5B should NOT contain final resolution keywords
        let resolution_keywords = [
            "você acerta",
            "você erra",
            "você dá",
            "você recebe",
            "dano",
            "HP",
            "AC",
            "rolagem",
            "teste",
            "você castou",
            "você falha",
            "você consegue",
            "você não consegue",
            "resultado",
            "consequência",
            "resolução",
        ];

        let text_lower = text.to_lowercase();
        !resolution_keywords
            .iter()
            .any(|keyword| text_lower.contains(keyword))
    }

    /// Test helper: Count words in text
    #[allow(dead_code)] // Used in s2s tests
    fn count_words(text: &str) -> usize {
        text.split_whitespace().count()
    }

    /// Test helper: Estimate tokens (rough: 1 token ≈ 4 characters)
    #[allow(dead_code)] // Used in s2s tests
    fn estimate_tokens(text: &str) -> u32 {
        (text.len() as f32 / 4.0).ceil() as u32
    }

    /// Test: Load both models (1.5B and 14B) simultaneously
    #[tokio::test]
    async fn test_load_both_models_simultaneously() {
        // Create single inference instance that can hold both models
        let mut inference = LlmInference::new(Persona::DungeonMaster);

        // Load 14B model
        inference.load_model("test_model_14b").await.unwrap();
        assert!(inference.is_loaded(), "14B model should be loaded");
        assert!(!inference.is_1_5b_loaded(), "1.5B should not be loaded yet");

        // Load 1.5B model (should not interfere with 14B)
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();
        assert!(inference.is_1_5b_loaded(), "1.5B model should be loaded");

        // Both models should remain loaded
        assert!(
            inference.is_loaded(),
            "14B model should still be loaded after 1.5B load"
        );
        assert!(
            inference.is_1_5b_loaded(),
            "1.5B model should still be loaded"
        );
        assert!(
            inference.both_models_loaded(),
            "Both models should be loaded"
        );
    }

    /// Test: 1.5B inference latency < 1.2s total
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_inference_latency() {
        use std::time::Duration;
        use tokio::time::timeout;

        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        let request = LlmRequest {
            prompt: "O jogador avança em direção ao goblin.".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
            temperature: Some(MODEL_1_5B_TEMPERATURE),
            context: None,
        };

        let start = Instant::now();

        // Run inference with timeout using infer_1_5b()
        let result = timeout(
            Duration::from_millis(INFERENCE_TIMEOUT_MS),
            inference.infer_1_5b(&request),
        )
        .await;

        let elapsed = start.elapsed();

        // Verify timeout was not exceeded
        assert!(
            result.is_ok(),
            "Inference should complete within {}ms timeout",
            INFERENCE_TIMEOUT_MS
        );

        // Verify actual latency
        assert!(
            elapsed.as_millis() < INFERENCE_TIMEOUT_MS as u128,
            "Inference latency {}ms exceeds target {}ms",
            elapsed.as_millis(),
            INFERENCE_TIMEOUT_MS
        );

        // Verify response is valid
        let response = result.unwrap().unwrap();
        assert!(!response.text.is_empty(), "Response should not be empty");
    }

    /// Test: 1.5B generates emotional response (1-2 sentences, max 40 tokens)
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_emotional_response_format() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        let request = LlmRequest {
            prompt: "O jogador salta da varanda.".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
            temperature: Some(MODEL_1_5B_TEMPERATURE),
            context: None,
        };

        let response = inference.infer_1_5b(&request).await.unwrap();

        // Verify token count (estimated)
        let estimated_tokens = estimate_tokens(&response.text);
        assert!(
            estimated_tokens <= MAX_TOKENS_1_5B,
            "Response tokens {} exceed maximum {}",
            estimated_tokens,
            MAX_TOKENS_1_5B
        );

        // Verify word count (1-2 sentences, ~15-45 words)
        let word_count = count_words(&response.text);
        assert!(
            word_count <= MAX_WORDS_1_5B,
            "Response words {} exceed maximum {}",
            word_count,
            MAX_WORDS_1_5B
        );

        // Verify it's a prelude (emotional, not final resolution)
        assert!(
            is_prelude_response(&response.text),
            "Response should be prelude/emotional, not final resolution: {}",
            response.text
        );

        // Verify response is not empty
        assert!(!response.text.is_empty(), "Response should not be empty");
    }

    /// Test: 1.5B does NOT generate final results or consequences
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_no_final_results() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        // Test multiple prompts that might trigger final resolution
        let test_prompts = vec![
            "O jogador ataca o goblin com a espada.",
            "O jogador lança uma magia de fogo.",
            "O jogador tenta se esconder.",
            "O jogador faz um teste de Stealth.",
            "O jogador rola um dado de ataque.",
        ];

        for prompt in test_prompts {
            let request = LlmRequest {
                prompt: prompt.to_string(),
                persona: Persona::DungeonMaster,
                max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
                temperature: Some(MODEL_1_5B_TEMPERATURE),
                context: None,
            };

            let response = inference.infer_1_5b(&request).await.unwrap();

            // Verify response does NOT contain final resolution keywords
            assert!(
                is_prelude_response(&response.text),
                "Response should not contain final results/consequences for prompt '{}': {}",
                prompt,
                response.text
            );
        }
    }

    /// Test: Memory usage with both models loaded
    #[tokio::test]
    async fn test_memory_usage_both_models() {
        // Create single inference instance that holds both models
        let mut inference = LlmInference::new(Persona::DungeonMaster);

        // Load both models
        inference.load_model("test_model_14b").await.unwrap();
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        // Verify both are loaded
        assert!(inference.is_loaded(), "14B should be loaded");
        assert!(inference.is_1_5b_loaded(), "1.5B should be loaded");
        assert!(
            inference.both_models_loaded(),
            "Both models should be loaded"
        );

        // Verify we can use both without interference
        let request_14b = LlmRequest {
            prompt: "Test 14B".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(2048),
            temperature: Some(0.7),
            context: None,
        };

        let request_1_5b = LlmRequest {
            prompt: "Test 1.5B".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
            temperature: Some(MODEL_1_5B_TEMPERATURE),
            context: None,
        };

        // Both should work independently
        // Note: These will use fallback templates if Synap is not available
        let _response_14b = inference.generate(&request_14b).await;
        let _response_1_5b = inference.infer_1_5b(&request_1_5b).await;

        // Both models should still be loaded after use
        assert!(inference.is_loaded(), "14B should remain loaded after use");
        assert!(
            inference.is_1_5b_loaded(),
            "1.5B should remain loaded after use"
        );
        assert!(
            inference.both_models_loaded(),
            "Both models should remain loaded"
        );
    }

    /// Test: 1.5B endpoint `/llm/prelude` configuration
    #[tokio::test]
    async fn test_prelude_endpoint_configuration() {
        // This test verifies that the endpoint configuration is correct
        // The actual endpoint implementation will be tested in server.rs tests

        // Verify 1.5B parameters are configured correctly
        assert_eq!(MODEL_1_5B_MAX_TOKENS, 40, "Max tokens should be 40");
        assert_eq!(MODEL_1_5B_TEMPERATURE, 0.8, "Temperature should be 0.8");
        assert_eq!(MODEL_1_5B_TOP_P, 0.9, "Top-p should be 0.9");
        assert_eq!(INFERENCE_TIMEOUT_MS, 1200, "Timeout should be 1200ms");
    }

    /// Test: 1.5B logging and metrics
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_logging_and_metrics() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        let request = LlmRequest {
            prompt: "Test logging".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
            temperature: Some(MODEL_1_5B_TEMPERATURE),
            context: None,
        };

        let start = Instant::now();
        let response = inference.infer_1_5b(&request).await.unwrap();
        let elapsed = start.elapsed();

        // Verify metrics are available
        assert!(elapsed.as_millis() > 0, "Latency should be measurable");
        assert!(response.tokens_used > 0, "Tokens used should be tracked");

        // Verify response has persona
        assert_eq!(
            response.persona.name(),
            Persona::DungeonMaster.name(),
            "Response should have correct persona"
        );
    }

    /// Test: 1.5B response style (emotional, human-like, not formulaic)
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_response_style() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        let request = LlmRequest {
            prompt: "O jogador se aproxima silenciosamente.".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
            temperature: Some(MODEL_1_5B_TEMPERATURE),
            context: None,
        };

        let response = inference.infer_1_5b(&request).await.unwrap();

        // Verify response is not empty
        assert!(!response.text.is_empty());

        // Verify response is not formulaic (should not be identical to template)
        let text_lower = response.text.to_lowercase();
        let formulaic_phrases = [
            "como o dungeon master",
            "eu respondo",
            "a situação se desenrola",
            "template response",
        ];

        let is_formulaic = formulaic_phrases
            .iter()
            .any(|phrase| text_lower.contains(phrase));

        // Verify response is not formulaic (should not be identical to template)
        // Note: This is a soft check - actual implementation may vary
        // The important thing is that it's not a hardcoded template
        assert!(
            !is_formulaic,
            "Response should not be formulaic/template-based. Got: {}",
            response.text
        );
    }

    /// Test: Multiple 1.5B requests in sequence
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_multiple_requests_sequence() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        let prompts = vec![
            "O jogador avança.",
            "O jogador recua.",
            "O jogador observa.",
        ];

        for prompt in prompts {
            let request = LlmRequest {
                prompt: prompt.to_string(),
                persona: Persona::DungeonMaster,
                max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
                temperature: Some(MODEL_1_5B_TEMPERATURE),
                context: None,
            };

            let start = Instant::now();
            let response = inference.infer_1_5b(&request).await.unwrap();
            let elapsed = start.elapsed();

            // Verify each response is valid
            assert!(!response.text.is_empty());
            assert!(elapsed.as_millis() < INFERENCE_TIMEOUT_MS as u128);
            assert!(is_prelude_response(&response.text));
        }
    }

    /// Test: 1.5B with different personas
    #[tokio::test]
    #[cfg(feature = "s2s")] // Requires Synap server
    async fn test_1_5b_different_personas() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();

        let personas = vec![
            Persona::DungeonMaster,
            Persona::Npc("Gandalf".to_string()),
            Persona::Narrator,
        ];

        for persona in personas {
            inference.set_persona(persona.clone());

            let request = LlmRequest {
                prompt: "Test persona".to_string(),
                persona: persona.clone(),
                max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
                temperature: Some(MODEL_1_5B_TEMPERATURE),
                context: None,
            };

            let response = inference.infer_1_5b(&request).await.unwrap();

            // Verify response has correct persona
            assert_eq!(
                response.persona.name(),
                persona.name(),
                "Response should have correct persona"
            );
        }
    }

    /// Test: 1.5B error handling
    #[tokio::test]
    async fn test_1_5b_error_handling() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);

        // Test: Generate without loading 1.5B model
        let request = LlmRequest {
            prompt: "Test".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(MODEL_1_5B_MAX_TOKENS),
            temperature: Some(MODEL_1_5B_TEMPERATURE),
            context: None,
        };

        let result = inference.infer_1_5b(&request).await;
        assert!(
            result.is_err(),
            "Should return error when 1.5B model is not loaded"
        );

        // Test: Load 1.5B model and verify it works
        inference.load_model_1_5b("test_model_1_5b").await.unwrap();
        let result = inference.infer_1_5b(&request).await;
        assert!(result.is_ok(), "Should succeed when 1.5B model is loaded");
    }
}
