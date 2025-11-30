use crate::bridge_phrases::{BridgeCategory, BridgePhrasesManager};
use crate::error::{LlmError, Result};
use crate::persona::Persona;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub prompt: String,
    pub persona: Persona,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub context: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
    pub intents: Option<String>, // INTENT DSL block if present
    pub tokens_used: u32,
    pub finish_reason: String,
    pub persona: Persona,
}

pub struct LlmInference {
    persona: Persona,
    model_loaded: bool,
    model_1_5b_loaded: bool, // Track 1.5B model separately
    cache: Arc<RwLock<HashMap<String, LlmResponse>>>,
    conversation_history: Arc<RwLock<Vec<(Persona, String)>>>,
    synap_endpoint: String,
    synap_client: Client,
    model_name: String,                        // 14B model name
    model_1_5b_name: String,                   // 1.5B model name
    bridge_phrases: Arc<BridgePhrasesManager>, // Bridge phrases manager
}

impl LlmInference {
    pub fn new(persona: Persona) -> Self {
        let synap_endpoint = std::env::var("SYNAP_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:15500".to_string());
        
        let synap_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        let model_name =
            std::env::var("VRPG_LLM_MODEL").unwrap_or_else(|_| "qwen2.5-14b-instruct".to_string());

        let model_1_5b_name = std::env::var("VRPG_LLM_MODEL_1_5B")
            .unwrap_or_else(|_| "qwen2.5-1.5b-instruct".to_string());

        // Initialize bridge phrases manager
        let bridge_phrases = Arc::new(BridgePhrasesManager::new().unwrap_or_else(|e| {
            warn!(
                "Failed to load bridge phrases: {}. Continuing without bridge phrases.",
                e
            );
            // Create a minimal manager that will fail gracefully
            BridgePhrasesManager::with_config(10, 20, 5, 3, 6)
                .unwrap_or_else(|_| panic!("Failed to create fallback bridge phrases manager"))
        }));
        
        Self {
            persona,
            model_loaded: false,
            model_1_5b_loaded: false,
            cache: Arc::new(RwLock::new(HashMap::new())),
            conversation_history: Arc::new(RwLock::new(Vec::new())),
            synap_endpoint,
            synap_client,
            model_name,
            model_1_5b_name,
            bridge_phrases,
        }
    }

    pub async fn load_model(&mut self, _model_path: &str) -> Result<()> {
        // In a real implementation, this would load the LLM model (GGUF, ONNX, etc.)
        // For now, we mark as loaded and use template-based responses
        // This loads the 14B model
        self.model_loaded = true;
        Ok(())
    }

    /// Load the 1.5B model for fast prelude inference
    pub async fn load_model_1_5b(&mut self, _model_path: &str) -> Result<()> {
        // In a real implementation, this would load the 1.5B model
        // For now, we mark as loaded
        self.model_1_5b_loaded = true;
        info!("✅ Qwen-1.5B model loaded (Mestre Reflexo)");
        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }

    pub fn is_1_5b_loaded(&self) -> bool {
        self.model_1_5b_loaded
    }

    /// Check if both models are loaded
    pub fn both_models_loaded(&self) -> bool {
        self.model_loaded && self.model_1_5b_loaded
    }

    pub fn persona(&self) -> &Persona {
        &self.persona
    }

    pub fn set_persona(&mut self, persona: Persona) {
        self.persona = persona;
    }

    pub async fn generate(&self, request: &LlmRequest) -> Result<LlmResponse> {
        if !self.model_loaded {
            return Err(LlmError::ModelLoad("Model not loaded".to_string()));
        }

        // Check cache
        let cache_key = format!("{}:{}", request.persona.name(), request.prompt);
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Generate response using Synap (Qwen)
        let response_text = self.generate_with_synap(request).await?;

        // Extract INTENT DSL block from response if present
        let intents = Self::extract_intent_block(&response_text);
        
        // Remove INTENT block from narrative text if present
        let narrative_text = Self::remove_intent_block(&response_text);

        // Estimate tokens (rough approximation: 1 token ≈ 4 characters)
        let tokens_used = (response_text.len() as f32 / 4.0).ceil() as u32;

        let response = LlmResponse {
            text: narrative_text,
            intents,
            tokens_used,
            finish_reason: "stop".to_string(),
            persona: request.persona.clone(),
        };

        // Cache the response
        {
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, response.clone());
        }

        // Add to conversation history
        {
            let mut history = self.conversation_history.write().await;
            history.push((request.persona.clone(), request.prompt.clone()));
            history.push((request.persona.clone(), response.text.clone()));
            // Keep only last 20 exchanges
            let len = history.len();
            if len > 40 {
                let remove_count = len - 40;
                history.drain(0..remove_count);
            }
        }

        Ok(response)
    }

    /// Extract INTENT DSL block from LLM response
    fn extract_intent_block(text: &str) -> Option<String> {
        // Look for [INTENTS]...[/INTENTS] block
        let start_marker = "[INTENTS]";
        let end_marker = "[/INTENTS]";
        
        if let Some(start_idx) = text.find(start_marker) {
            if let Some(end_idx) = text[start_idx..].find(end_marker) {
                let block = &text[start_idx..start_idx + end_idx + end_marker.len()];
                return Some(block.to_string());
            }
        }
        None
    }

    /// Remove INTENT DSL block from narrative text
    fn remove_intent_block(text: &str) -> String {
        // Remove [INTENTS]...[/INTENTS] block if present
        let start_marker = "[INTENTS]";
        let end_marker = "[/INTENTS]";
        
        if let Some(start_idx) = text.find(start_marker) {
            if let Some(end_idx) = text[start_idx..].find(end_marker) {
                let before = &text[..start_idx];
                let after = &text[start_idx + end_idx + end_marker.len()..];
                return format!("{}{}", before.trim(), after.trim())
                    .trim()
                    .to_string();
            }
        }
        text.to_string()
    }

    /// Generate response using Synap (Qwen)
    async fn generate_with_synap(&self, request: &LlmRequest) -> Result<String> {
        use serde_json::json;
        
        // Construir prompt completo com persona e contexto
        let system_prompt = self.build_system_prompt(&request.persona);
        let user_prompt = if let Some(context) = &request.context {
            let context_str = context.join("\n");
            format!("{}\n\nContext:\n{}", request.prompt, context_str)
        } else {
            request.prompt.clone()
        };
        
        // Formato Synap StreamableHTTP
        let request_id = format!(
            "llm-{}",
            std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
                .as_nanos()
        );
        
        let synap_request = json!({
            "command": "llm.generate",
            "request_id": request_id,
            "payload": {
                "model": self.model_name,
                "messages": [
                    {
                        "role": "system",
                        "content": system_prompt
                    },
                    {
                        "role": "user",
                        "content": user_prompt
                    }
                ],
                "max_tokens": request.max_tokens.unwrap_or(512),
                "temperature": request.temperature.unwrap_or(0.7),
            }
        });
        
        info!(
            "🤖 Calling Synap for LLM inference (model: {})",
            self.model_name
        );
        
        let start_time = std::time::Instant::now();
        
        // Chamar Synap
        let response = self
            .synap_client
            .post(format!("{}/api/v1/command", self.synap_endpoint))
            .json(&synap_request)
            .send()
            .await
            .map_err(|e| LlmError::ModelLoad(format!("Synap request failed: {}", e)))?;
        
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| LlmError::ModelLoad(format!("Failed to read Synap response: {}", e)))?;
        
        if !status.is_success() {
            warn!("Synap returned error status {}: {}", status, body);
            // Fallback para template-based response
            return self
                .generate_response(
                    &request.persona,
                    &request.prompt,
                    request.context.as_deref(),
                )
                .await;
        }
        
        let result: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            LlmError::ModelLoad(format!("Failed to parse Synap response: {}: {}", e, body))
        })?;
        
        // Verificar sucesso
        if !result
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            warn!("Synap returned error: {}", error);
            // Fallback para template-based response
            return self
                .generate_response(
                    &request.persona,
                    &request.prompt,
                    request.context.as_deref(),
                )
                .await;
        }
        
        // Extrair resposta do payload
        let payload = result
            .get("payload")
            .ok_or_else(|| LlmError::ModelLoad("Missing payload in Synap response".to_string()))?;
        
        // Synap pode retornar em diferentes formatos, tentar extrair texto
        let response_text = if let Some(choices) = payload.get("choices") {
            // Formato OpenAI-compatible
            choices
                .get(0)
                .and_then(|c| c.get("message"))
                .and_then(|m| m.get("content"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else if let Some(text) = payload.get("text") {
            // Formato direto
            text.as_str().map(|s| s.to_string())
        } else if let Some(content) = payload.get("content") {
            // Formato alternativo
            content.as_str().map(|s| s.to_string())
        } else {
            None
        };
        
        let response_text = response_text.ok_or_else(|| {
            LlmError::ModelLoad(format!(
                "Could not extract text from Synap response: {}",
                body
            ))
        })?;
        
        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        info!("✅ Synap LLM inference complete in {}ms", elapsed_ms);
        
        Ok(response_text)
    }
    
    /// Build system prompt based on persona
    fn build_system_prompt(&self, persona: &Persona) -> String {
        let base_prompt = match persona {
            Persona::DungeonMaster => {
                "You are a Dungeon Master for a tabletop RPG game. You narrate the story, control NPCs, and manage the game world. Be descriptive, immersive, and engaging."
            }
            Persona::Npc(name) => {
                &format!("You are {}, an NPC in the game. Respond in character.", name)
            }
            Persona::PlayerIa(name) => {
                &format!("You are {}, a player character. Respond as this character would.", name)
            }
            Persona::Monster(name) => {
                &format!("You are a {}, a monster in the game. Respond appropriately.", name)
            }
            Persona::Narrator => {
                "You are a narrator. Provide descriptive, atmospheric narration."
            }
        };
        
        // Adicionar instruções de INTENT DSL para Dungeon Master
        if matches!(persona, Persona::DungeonMaster) {
            format!("{}\n\nIMPORTANT: When your response requires mechanical actions (combat, skill checks, etc.), include an INTENT DSL block using this format:\n\n[INTENTS]\nINTENT: <TYPE>\n<KEY>: <VALUE>\n...\nEND_INTENT\n[/INTENTS]\n\nThe narrative text should come before or after the INTENT block. Only include INTENTs when mechanical actions are needed.", base_prompt)
        } else {
            base_prompt.to_string()
        }
    }

    async fn generate_response(
        &self,
        persona: &Persona,
        prompt: &str,
        context: Option<&[String]>,
    ) -> Result<String> {
        // In a real implementation, this would call the actual LLM model (Qwen 2.5 14B)
        // For now, we generate persona-appropriate responses using templates
        
        // Build system prompt with INTENT DSL instructions for Dungeon Master
        let system_instruction = if matches!(persona, Persona::DungeonMaster) {
            "\n\nIMPORTANT: When your response requires mechanical actions (combat, skill checks, etc.), \
            include an INTENT DSL block using this format:\n\n\
            [INTENTS]\n\
            INTENT: <TYPE>\n\
            <KEY>: <VALUE>\n\
            ...\n\
            END_INTENT\n\
            [/INTENTS]\n\n\
            The narrative text should come before or after the INTENT block. \
            Only include INTENTs when mechanical actions are needed.\n"
        } else {
            ""
        };
        
        let base_response = match persona {
            Persona::DungeonMaster => {
                format!("As the Dungeon Master, I respond to your query: \"{}\". The situation unfolds as follows...{}", 
                    prompt, system_instruction)
            }
            Persona::Npc(name) => {
                format!("{} says: \"{}\"", name, prompt)
            }
            Persona::PlayerIa(name) => {
                format!("{} responds: \"{}\"", name, prompt)
            }
            Persona::Monster(name) => {
                format!("The {} growls: \"{}\"", name, prompt)
            }
            Persona::Narrator => {
                format!("Narrator: \"{}\"", prompt)
            }
        };

        // Add context if provided
        let response = if let Some(ctx) = context {
            let context_str = ctx.join(" ");
            format!("{} [Context: {}]", base_response, context_str)
        } else {
            base_response
        };

        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        Ok(response)
    }

    pub async fn get_conversation_history(&self) -> Vec<(Persona, String)> {
        let history = self.conversation_history.read().await;
        history.clone()
    }

    pub async fn clear_history(&self) {
        let mut history = self.conversation_history.write().await;
        history.clear();
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Generate fast prelude response using 1.5B model
    /// This is the "Mestre Reflexo" - fast emotional reaction (< 1.2s)
    pub async fn infer_1_5b(&self, request: &LlmRequest) -> Result<LlmResponse> {
        if !self.model_1_5b_loaded {
            return Err(LlmError::ModelLoad("1.5B model not loaded".to_string()));
        }

        let start_time = std::time::Instant::now();

        // Check cache with 1.5B-specific key
        let cache_key = format!("1.5b:{}:{}", request.persona.name(), request.prompt);
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                info!("📦 1.5B cache hit");
                return Ok(cached.clone());
            }
        }

        // Select a bridge phrase (human-like transition to prevent cognitive silence)
        // Use neutral category as default, but allow rotation for variety
        let bridge_category = BridgeCategory::Neutral; // Can be made context-aware later
        let bridge_phrase = self
            .bridge_phrases
            .select_phrase_with_rotation(bridge_category)
            .await
            .unwrap_or_else(|e| {
                warn!(
                    "Failed to select bridge phrase: {}. Continuing without bridge phrase.",
                    e
                );
                String::new()
            });

        // Generate response using 1.5B via Synap (with bridge phrase)
        let response_text = self
            .generate_with_synap_1_5b(request, Some(&bridge_phrase))
            .await?;

        // Verify response constraints (1-2 sentences, max 40 tokens)
        let word_count = response_text.split_whitespace().count();
        let estimated_tokens = (response_text.len() as f32 / 4.0).ceil() as u32;

        // Log warning if response exceeds constraints
        if estimated_tokens > 40 {
            warn!(
                "⚠️ 1.5B response exceeds token limit: {} tokens (max: 40)",
                estimated_tokens
            );
        }
        if word_count > 45 {
            warn!(
                "⚠️ 1.5B response exceeds word limit: {} words (max: 45)",
                word_count
            );
        }

        // Estimate tokens
        let tokens_used = estimated_tokens.min(40); // Cap at 40

        let response = LlmResponse {
            text: response_text,
            intents: None, // 1.5B should NEVER generate INTENTs
            tokens_used,
            finish_reason: "stop".to_string(),
            persona: request.persona.clone(),
        };

        // Cache the response
        {
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, response.clone());
        }

        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        info!(
            "⚡ 1.5B inference complete in {}ms (target: <1200ms, tokens: {})",
            elapsed_ms, tokens_used
        );

        // Log warning if latency exceeds target
        if elapsed_ms > 1200 {
            warn!(
                "⚠️ 1.5B inference latency {}ms exceeds target 1200ms",
                elapsed_ms
            );
        }

        Ok(response)
    }

    /// Generate response using Synap with 1.5B model (fast prelude)
    async fn generate_with_synap_1_5b(
        &self,
        request: &LlmRequest,
        bridge_phrase: Option<&str>,
    ) -> Result<String> {
        use serde_json::json;

        // Build prompt for 1.5B (emotional reaction, not final resolution)
        let system_prompt = self.build_system_prompt_1_5b(&request.persona, bridge_phrase);
        let user_prompt = if let Some(context) = &request.context {
            let context_str = context.join("\n");
            format!("{}\n\nContext:\n{}", request.prompt, context_str)
        } else {
            request.prompt.clone()
        };

        // Formato Synap StreamableHTTP
        let request_id = format!(
            "llm-1.5b-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let synap_request = json!({
            "command": "llm.generate",
            "request_id": request_id,
            "payload": {
                "model": self.model_1_5b_name,
                "role": "prelude",
                "messages": [
                    {
                        "role": "system",
                        "content": system_prompt
                    },
                    {
                        "role": "user",
                        "content": user_prompt
                    }
                ],
                "max_tokens": 40, // Hard limit for 1.5B
                "temperature": 0.8,
                "top_p": 0.9,
                "use_gpu": true
            }
        });

        info!(
            "⚡ Calling Synap for 1.5B inference (model: {})",
            self.model_1_5b_name
        );

        let start_time = std::time::Instant::now();

        // Call Synap
        let response = self
            .synap_client
            .post(format!("{}/api/v1/command", self.synap_endpoint))
            .json(&synap_request)
            .send()
            .await
            .map_err(|e| LlmError::ModelLoad(format!("Synap 1.5B request failed: {}", e)))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| {
            LlmError::ModelLoad(format!("Failed to read Synap 1.5B response: {}", e))
        })?;

        if !status.is_success() {
            warn!("Synap 1.5B returned error status {}: {}", status, body);
            // Fallback to template-based response
            return self
                .generate_prelude_response(
                    &request.persona,
                    &request.prompt,
                    request.context.as_deref(),
                )
                .await;
        }

        let result: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            LlmError::ModelLoad(format!(
                "Failed to parse Synap 1.5B response: {}: {}",
                e, body
            ))
        })?;

        // Check success
        if !result
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            warn!("Synap 1.5B returned error: {}", error);
            // Fallback to template-based response
            return self
                .generate_prelude_response(
                    &request.persona,
                    &request.prompt,
                    request.context.as_deref(),
                )
                .await;
        }

        // Extract response from payload
        let payload = result.get("payload").ok_or_else(|| {
            LlmError::ModelLoad("Missing payload in Synap 1.5B response".to_string())
        })?;

        // Extract text from response
        let response_text = if let Some(choices) = payload.get("choices") {
            choices
                .get(0)
                .and_then(|c| c.get("message"))
                .and_then(|m| m.get("content"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else if let Some(text) = payload.get("text") {
            text.as_str().map(|s| s.to_string())
        } else if let Some(content) = payload.get("content") {
            content.as_str().map(|s| s.to_string())
        } else {
            None
        };

        let response_text = response_text.ok_or_else(|| {
            LlmError::ModelLoad(format!(
                "Could not extract text from Synap 1.5B response: {}",
                body
            ))
        })?;

        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        info!("✅ Synap 1.5B inference complete in {}ms", elapsed_ms);

        Ok(response_text)
    }

    /// Build system prompt for 1.5B (emotional reaction, NOT final resolution)
    fn build_system_prompt_1_5b(&self, persona: &Persona, bridge_phrase: Option<&str>) -> String {
        let base_prompt = match persona {
            Persona::DungeonMaster => {
                "You are a Dungeon Master providing a brief, emotional reaction to player actions. \
                Your role is to provide a human-like immediate response (1-2 sentences, max 40 tokens) \
                that acknowledges the action emotionally without resolving it. \
                You are NOT the narrator - you are the immediate human reaction. \
                Be perceptive, intimate, sensory. Open space, don't close it. \
                NEVER give numbers, calculate damage, request rolls, decide success/failure, \
                interpret spells, describe NPC reactions, resolve combat, narrate mechanical results, \
                resolve scenes, explain rules, cite causality, interpret death saves, or announce criticals/failures."
            }
            Persona::Npc(name) => {
                &format!("You are {}, an NPC. Provide a brief emotional reaction (1-2 sentences, max 40 tokens).", name)
            }
            Persona::PlayerIa(name) => {
                &format!("You are {}, a player character. Provide a brief emotional reaction (1-2 sentences, max 40 tokens).", name)
            }
            Persona::Monster(name) => {
                &format!("You are a {}. Provide a brief emotional reaction (1-2 sentences, max 40 tokens).", name)
            }
            Persona::Narrator => {
                "You are a narrator. Provide a brief, atmospheric emotional reaction (1-2 sentences, max 40 tokens)."
            }
        };

        // Include bridge phrase if provided (human-like transition)
        if let Some(phrase) = bridge_phrase {
            if !phrase.is_empty() {
                format!(
                    "{}\n\nUse this as inspiration for your response style: \"{}\"",
                    base_prompt, phrase
                )
            } else {
                base_prompt.to_string()
            }
        } else {
            base_prompt.to_string()
        }
    }

    /// Generate prelude response (fallback template for 1.5B)
    async fn generate_prelude_response(
        &self,
        persona: &Persona,
        _prompt: &str,
        context: Option<&[String]>,
    ) -> Result<String> {
        // Generate brief emotional reaction (1-2 sentences)
        let base_response = match persona {
            Persona::DungeonMaster => {
                "A weight settles in the air as you move forward. The moment stretches.".to_string()
            }
            Persona::Npc(name) => {
                format!("{}'s eyes flicker with recognition.", name)
            }
            Persona::PlayerIa(name) => {
                format!("{} feels the tension build.", name)
            }
            Persona::Monster(name) => {
                format!("The {} shifts, sensing movement.", name)
            }
            Persona::Narrator => "The silence deepens, heavy with anticipation.".to_string(),
        };

        // Add context if provided (minimal)
        let response = if let Some(ctx) = context {
            let context_str = ctx.first().map(|s| s.as_str()).unwrap_or("");
            if !context_str.is_empty() {
                format!("{} {}", base_response, context_str)
            } else {
                base_response
            }
        } else {
            base_response
        };

        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_llm_inference_loading() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        assert!(!inference.is_loaded());
        
        inference.load_model("test_model").await.unwrap();
        assert!(inference.is_loaded());
    }

    #[tokio::test]
    async fn test_llm_generate() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model("test_model").await.unwrap();
        
        let request = LlmRequest {
            prompt: "What happens next?".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: Some(100),
            temperature: Some(0.7),
            context: None,
        };
        
        let response = inference.generate(&request).await.unwrap();
        assert!(!response.text.is_empty());
        assert!(response.tokens_used > 0);
        // INTENTs may or may not be present depending on the response
    }

    #[tokio::test]
    async fn test_llm_cache() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model("test_model").await.unwrap();
        
        let request = LlmRequest {
            prompt: "Test prompt".to_string(),
            persona: Persona::DungeonMaster,
            max_tokens: None,
            temperature: None,
            context: None,
        };
        
        let response1 = inference.generate(&request).await.unwrap();
        let response2 = inference.generate(&request).await.unwrap();
        
        // Cached response should be identical
        assert_eq!(response1.text, response2.text);
    }

    #[tokio::test]
    async fn test_llm_persona_switching() {
        let mut inference = LlmInference::new(Persona::DungeonMaster);
        inference.load_model("test_model").await.unwrap();
        
        inference.set_persona(Persona::Npc("Gandalf".to_string()));
        assert_eq!(inference.persona().name(), "Gandalf");
    }
}
