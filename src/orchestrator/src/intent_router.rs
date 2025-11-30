//! Intent Router - M2.4
//! Classifies player input to route to the correct path (FACT_QUERY, SIMPLE_RULE_QUERY, WORLD_ACTION, etc.)

use crate::error::{OrchestratorError, Result};
use crate::pipeline::PipelineState;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Intent classification types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntentClassification {
    /// Factual queries (HP, AC, position, etc.) - answered by GameState
    FactQuery,
    /// Simple rule queries (skill checks, ability checks) - answered by Vectorizer + 1.5B
    SimpleRuleQuery,
    /// Meta queries about the system
    MetaQuery,
    /// World actions (narrative actions)
    WorldAction,
    /// Combat actions (attacks, etc.)
    CombatAction,
    /// Spell casting
    SpellCast,
    /// Movement
    Move,
    /// Roll requests
    RollRequest,
    /// Uncertain - fallback to 1.5B for classification
    Uncertain,
}

/// Classification result
#[derive(Debug, Clone)]
pub struct ClassificationResult {
    /// Classified intent type
    pub intent_type: IntentClassification,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Classification method used
    pub method: String,
}

/// Intent Router
pub struct IntentRouter {
    /// Regex patterns for classification
    patterns: HashMap<IntentClassification, Vec<Regex>>,
    /// Cache for frequent classifications
    cache: Arc<Mutex<HashMap<String, ClassificationResult>>>,
}

impl IntentRouter {
    /// Create new Intent Router
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // FACT_QUERY patterns
        patterns.insert(
            IntentClassification::FactQuery,
            vec![
                Regex::new(r"(?i)(quantos|qual|quais).*(hp|vida|health)").unwrap(),
                Regex::new(r"(?i)(qual|quais).*(ac|classe.*armadura|armadura)").unwrap(),
                Regex::new(r"(?i)(quantos|qual).*(slot|slots).*(nível|level|nivel)").unwrap(),
                Regex::new(r"(?i)(qual|quais).*(posição|position|posicao)").unwrap(),
                Regex::new(r"(?i)(quantos|qual).*(recurso|recursos)").unwrap(),
                Regex::new(r"(?i)(tenho|tenho.*ainda).*(hp|vida|ac|slot)").unwrap(),
            ],
        );

        // SIMPLE_RULE_QUERY patterns
        patterns.insert(
            IntentClassification::SimpleRuleQuery,
            vec![
                Regex::new(r"(?i)(stealth|furtividade).*(usa|usa.*destreza|dexterity)").unwrap(),
                Regex::new(r"(?i)(investigation|investigação).*(usa|usa.*inteligência|intelligence|é.*inteligência)").unwrap(),
                Regex::new(r"(?i)(acrobatics|acrobacia).*(usa|usa.*destreza|dexterity)").unwrap(),
                Regex::new(r"(?i)(.*skill|.*perícia).*(usa|usa.*qual.*atributo)").unwrap(),
                Regex::new(r"(?i)(.*ability.*check|.*teste.*habilidade)").unwrap(),
            ],
        );

        // META_QUERY patterns
        patterns.insert(
            IntentClassification::MetaQuery,
            vec![
                Regex::new(r"(?i)(como.*funciona|como.*sistema|help|ajuda)").unwrap(),
                Regex::new(r"(?i)(o.*que.*é|what.*is|quem.*é)").unwrap(),
            ],
        );

        // WORLD_ACTION patterns
        patterns.insert(
            IntentClassification::WorldAction,
            vec![
                Regex::new(r"(?i)(eu.*quero|vou|vou.*fazer).*(abrir|fechar|interagir|examinar)")
                    .unwrap(),
                Regex::new(r"(?i)(eu.*quero|vou).*(conversar|falar|dizer)").unwrap(),
                Regex::new(r"(?i)(eu.*quero|vou).*(procurar|buscar|investigar)").unwrap(),
            ],
        );

        // COMBAT_ACTION patterns
        patterns.insert(
            IntentClassification::CombatAction,
            vec![
                Regex::new(r"(?i)(eu.*ataco|atacar|atacar.*com).*(goblin|inimigo|alvo)").unwrap(),
                Regex::new(r"(?i)(eu.*quero|vou).*(atacar|atacar.*com)").unwrap(),
                Regex::new(r"(?i)(ataque|attack).*(corpo.*corpo|melee)").unwrap(),
            ],
        );

        // SPELL_CAST patterns
        patterns.insert(
            IntentClassification::SpellCast,
            vec![
                Regex::new(r"(?i)(eu.*lanço|lançar|cast|lançar.*magia).*(fireball|magic|spell)")
                    .unwrap(),
                Regex::new(r"(?i)(eu.*uso|usar).*(magia|spell|feitiço)").unwrap(),
                Regex::new(r"(?i)(cast|lançar).*(.*spell|.*magia)").unwrap(),
            ],
        );

        // MOVE patterns
        patterns.insert(
            IntentClassification::Move,
            vec![
                Regex::new(r"(?i)(eu.*me.*movo|mover|movimento).*(para|até|em.*direção)").unwrap(),
                Regex::new(r"(?i)(eu.*vou|ir).*(para|até|em.*direção)").unwrap(),
            ],
        );

        // ROLL_REQUEST patterns
        patterns.insert(
            IntentClassification::RollRequest,
            vec![
                Regex::new(r"(?i)(eu.*rolo|rolar|roll).*(d20|dado|dice)").unwrap(),
                Regex::new(r"(?i)(rolar|roll).*(.*dado|.*dice)").unwrap(),
            ],
        );

        Self {
            patterns,
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Classify input text
    pub fn classify(
        &self,
        input: &str,
        _pipeline_state: &PipelineState,
    ) -> Result<ClassificationResult> {
        let start = Instant::now();

        // Check cache first
        let cache_key = input.to_lowercase().trim().to_string();
        {
            let cache = self.cache.lock().map_err(|_| {
                OrchestratorError::ServiceError("Failed to acquire cache lock".to_string())
            })?;
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Try regex-based classification
        let mut best_match: Option<(IntentClassification, f32)> = None;

        for (intent_type, regexes) in &self.patterns {
            for regex in regexes {
                if regex.is_match(input) {
                    let confidence = 0.95; // High confidence for regex matches
                    if best_match.is_none() || best_match.as_ref().unwrap().1 < confidence {
                        best_match = Some((intent_type.clone(), confidence));
                    }
                }
            }
        }

        let result = if let Some((intent_type, confidence)) = best_match {
            ClassificationResult {
                intent_type,
                confidence,
                method: "regex".to_string(),
            }
        } else {
            // Fallback to Uncertain (will trigger 1.5B for classification)
            ClassificationResult {
                intent_type: IntentClassification::Uncertain,
                confidence: 0.5, // Low confidence for uncertain cases
                method: "uncertain".to_string(),
            }
        };

        // Cache the result
        {
            let mut cache = self.cache.lock().map_err(|_| {
                OrchestratorError::ServiceError("Failed to acquire cache lock".to_string())
            })?;
            cache.insert(cache_key, result.clone());
        }

        let elapsed = start.elapsed();
        if elapsed.as_millis() >= 10 {
            tracing::warn!(
                "Intent classification took longer than expected: {:?}",
                elapsed
            );
        }

        Ok(result)
    }

    /// Clear cache
    pub fn clear_cache(&self) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| {
            OrchestratorError::ServiceError("Failed to acquire cache lock".to_string())
        })?;
        cache.clear();
        Ok(())
    }

    /// Get cache size
    pub fn cache_size(&self) -> Result<usize> {
        let cache = self.cache.lock().map_err(|_| {
            OrchestratorError::ServiceError("Failed to acquire cache lock".to_string())
        })?;
        Ok(cache.len())
    }
}

impl Default for IntentRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for classification
pub fn classify_intent(
    input: &str,
    pipeline_state: &PipelineState,
) -> Result<ClassificationResult> {
    let router = IntentRouter::new();
    router.classify(input, pipeline_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_router_new() {
        let router = IntentRouter::new();
        assert!(!router.patterns.is_empty());
    }

    #[test]
    fn test_classify_fact_query() {
        let router = IntentRouter::new();
        let result = router
            .classify("Quantos HP eu tenho?", &PipelineState::new())
            .unwrap();
        assert_eq!(result.intent_type, IntentClassification::FactQuery);
    }
}
