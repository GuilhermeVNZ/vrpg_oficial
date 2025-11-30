//! INTENT DSL Parser
//!
//! Parses INTENT DSL blocks from LLM output:
//! [INTENTS]
//! INTENT: MELEE_ATTACK
//! ACTOR: player_1
//! TARGET: npc_goblin_02
//! WEAPON: weapon_longsword
//! MOVE_REQUIRED: YES
//! END_INTENT
//! [/INTENTS]

use super::types::Intent;
use crate::error::{OrchestratorError, Result};

/// INTENT DSL Parser
pub struct IntentParser;

impl IntentParser {
    /// Parse INTENT DSL block from text
    ///
    /// Extracts all INTENT blocks from the text and parses them
    pub fn parse(text: &str) -> Result<Vec<Intent>> {
        let mut intents = Vec::new();

        // Find all [INTENTS] blocks
        let start_marker = "[INTENTS]";
        let end_marker = "[/INTENTS]";

        let mut remaining = text;

        while let Some(start_idx) = remaining.find(start_marker) {
            let block_start = start_idx + start_marker.len();
            let block_text = &remaining[block_start..];

            if let Some(end_idx) = block_text.find(end_marker) {
                let block_content = block_text[..end_idx].trim();
                let parsed = Self::parse_block(block_content)?;
                intents.extend(parsed);

                // Move past this block
                remaining = &block_text[end_idx + end_marker.len()..];
            } else {
                // No closing marker found, break
                break;
            }
        }

        Ok(intents)
    }

    /// Parse a single INTENT block
    fn parse_block(block: &str) -> Result<Vec<Intent>> {
        let mut intents = Vec::new();
        let lines: Vec<&str> = block
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let mut i = 0;
        while i < lines.len() {
            if lines[i].starts_with("INTENT:") {
                let intent_type = lines[i]
                    .strip_prefix("INTENT:")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Invalid INTENT line".to_string())
                    })?
                    .trim();

                let mut fields = std::collections::HashMap::new();
                i += 1;

                // Parse fields until END_INTENT
                while i < lines.len() && lines[i] != "END_INTENT" {
                    if let Some(colon_idx) = lines[i].find(':') {
                        let key = lines[i][..colon_idx].trim().to_uppercase();
                        let value = lines[i][colon_idx + 1..].trim();
                        fields.insert(key, value.to_string());
                    }
                    i += 1;
                }

                // Parse the intent
                let intent = Self::parse_intent(intent_type, &fields)?;
                intents.push(intent);
            }
            i += 1;
        }

        Ok(intents)
    }

    /// Parse a single INTENT from its type and fields
    fn parse_intent(
        intent_type: &str,
        fields: &std::collections::HashMap<String, String>,
    ) -> Result<Intent> {
        match intent_type {
            "SKILL_CHECK" => Ok(Intent::SkillCheck {
                actor: fields
                    .get("ACTOR")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing ACTOR".to_string())
                    })?
                    .clone(),
                skill: fields
                    .get("SKILL")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing SKILL".to_string())
                    })?
                    .clone(),
                target: fields.get("TARGET").cloned(),
                context: fields.get("CONTEXT").cloned(),
                suggest_dc: Self::parse_bool(
                    fields.get("SUGGEST_DC").unwrap_or(&"YES".to_string()),
                ),
            }),
            "MELEE_ATTACK" => Ok(Intent::MeleeAttack {
                actor: fields
                    .get("ACTOR")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing ACTOR".to_string())
                    })?
                    .clone(),
                target: fields
                    .get("TARGET")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing TARGET".to_string())
                    })?
                    .clone(),
                weapon: fields.get("WEAPON").cloned(),
                move_required: Self::parse_bool(
                    fields.get("MOVE_REQUIRED").unwrap_or(&"NO".to_string()),
                ),
            }),
            "RANGED_ATTACK" => Ok(Intent::RangedAttack {
                actor: fields
                    .get("ACTOR")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing ACTOR".to_string())
                    })?
                    .clone(),
                target: fields
                    .get("TARGET")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing TARGET".to_string())
                    })?
                    .clone(),
                weapon: fields.get("WEAPON").cloned(),
                move_required: Self::parse_bool(
                    fields.get("MOVE_REQUIRED").unwrap_or(&"NO".to_string()),
                ),
            }),
            "SPELL_CAST" => {
                let area_center = fields.get("AREA_CENTER").and_then(|s| {
                    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
                    if parts.len() == 2 {
                        parts[0]
                            .parse::<i32>()
                            .ok()
                            .and_then(|x| parts[1].parse::<i32>().ok().map(|y| (x, y)))
                    } else {
                        None
                    }
                });

                let targets = fields
                    .get("TARGETS")
                    .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                    .unwrap_or_default();

                Ok(Intent::SpellCast {
                    actor: fields
                        .get("ACTOR")
                        .ok_or_else(|| {
                            OrchestratorError::IntentParseError("Missing ACTOR".to_string())
                        })?
                        .clone(),
                    spell: fields
                        .get("SPELL")
                        .ok_or_else(|| {
                            OrchestratorError::IntentParseError("Missing SPELL".to_string())
                        })?
                        .clone(),
                    slot_level: fields
                        .get("SLOT_LEVEL")
                        .and_then(|s| s.parse::<u8>().ok())
                        .unwrap_or(1),
                    area_center,
                    targets,
                })
            }
            "LORE_QUERY" => Ok(Intent::LoreQuery {
                query: fields
                    .get("QUERY")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing QUERY".to_string())
                    })?
                    .clone(),
                scope: fields.get("SCOPE").cloned(),
            }),
            "RULE_QUERY" => Ok(Intent::RuleQuery {
                query: fields
                    .get("QUERY")
                    .ok_or_else(|| {
                        OrchestratorError::IntentParseError("Missing QUERY".to_string())
                    })?
                    .clone(),
                context: fields.get("CONTEXT").cloned(),
            }),
            "COMBAT_START" => Ok(Intent::CombatStart {
                reason: fields.get("REASON").cloned(),
            }),
            "COMBAT_END" => Ok(Intent::CombatEnd {
                reason: fields.get("REASON").cloned(),
            }),
            _ => Err(OrchestratorError::IntentParseError(format!(
                "Unknown INTENT type: {}",
                intent_type
            ))),
        }
    }

    /// Parse boolean value (YES/NO)
    fn parse_bool(value: &str) -> bool {
        value.trim().to_uppercase() == "YES"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill_check() {
        let text = r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda a liberar a entrada"
SUGGEST_DC: YES
END_INTENT
[/INTENTS]
"#;

        let intents = IntentParser::parse(text).unwrap();
        assert_eq!(intents.len(), 1);

        if let Intent::SkillCheck {
            actor,
            skill,
            target,
            context,
            suggest_dc,
        } = &intents[0]
        {
            assert_eq!(actor, "player_1");
            assert_eq!(skill, "persuasion");
            assert_eq!(target.as_ref().unwrap(), "npc_guard_01");
            assert!(context.is_some());
            assert!(*suggest_dc);
        } else {
            panic!("Expected SkillCheck intent");
        }
    }

    #[test]
    fn test_parse_melee_attack() {
        let text = r#"
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
WEAPON: weapon_longsword
MOVE_REQUIRED: YES
END_INTENT
[/INTENTS]
"#;

        let intents = IntentParser::parse(text).unwrap();
        assert_eq!(intents.len(), 1);

        if let Intent::MeleeAttack {
            actor,
            target,
            weapon,
            move_required,
        } = &intents[0]
        {
            assert_eq!(actor, "player_1");
            assert_eq!(target, "npc_goblin_02");
            assert_eq!(weapon.as_ref().unwrap(), "weapon_longsword");
            assert!(*move_required);
        } else {
            panic!("Expected MeleeAttack intent");
        }
    }

    #[test]
    fn test_parse_multiple_intents() {
        let text = r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: perception
END_INTENT
INTENT: LORE_QUERY
QUERY: "historia dos Magos Rubros de Thay"
SCOPE: faction
END_INTENT
[/INTENTS]
"#;

        let intents = IntentParser::parse(text).unwrap();
        assert_eq!(intents.len(), 2);
    }
}
