//! Voice INTENT Parser
//!
//! Parses Voice INTENTs from Qwen LLM output and extracts
//! emotional tags and metadata for XTTS voice synthesis.

use crate::error::{Result, TtsError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceIntent {
    pub actor: String,
    pub emotion: String,
    pub style: String,
    pub pace: Option<String>,   // "slow", "normal", "fast"
    pub volume: Option<String>, // "low", "normal", "high"
    pub context: Option<String>,
}

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum VoiceIntentType {
    NARRATE,
    NPC_DIALOGUE,
    PLAYER_DIALOGUE,
    EVENT,
    CONDITION_EXPIRE,
    SYSTEM,
}

impl FromStr for VoiceIntentType {
    type Err = TtsError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "NARRATE" => Ok(VoiceIntentType::NARRATE),
            "NPC_DIALOGUE" => Ok(VoiceIntentType::NPC_DIALOGUE),
            "PLAYER_DIALOGUE" => Ok(VoiceIntentType::PLAYER_DIALOGUE),
            "EVENT" => Ok(VoiceIntentType::EVENT),
            "CONDITION_EXPIRE" => Ok(VoiceIntentType::CONDITION_EXPIRE),
            "SYSTEM" => Ok(VoiceIntentType::SYSTEM),
            _ => Err(TtsError::Synthesis(format!(
                "Unknown voice intent type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedVoiceIntent {
    pub intent_type: VoiceIntentType,
    pub text: String,
    pub voice_intent: VoiceIntent,
    pub priority: u8, // 1-9, higher = more priority
}

pub struct VoiceIntentParser;

impl VoiceIntentParser {
    /// Parse Voice INTENT from Qwen LLM output
    ///
    /// Expected format:
    /// ```xml
    /// <VOICE actor="NPC_Guard" emotion="skeptic" style="dry" pace="normal" volume="normal">
    /// "Emissário? De qual reino? Mostre sua insígnia!"
    /// </VOICE>
    /// ```
    pub fn parse(text: &str) -> Result<ParsedVoiceIntent> {
        // Extract VOICE tag
        let voice_tag_start = text.find("<VOICE");
        let voice_tag_end = text.find("</VOICE>");

        if voice_tag_start.is_none() || voice_tag_end.is_none() {
            // No VOICE tag found, treat as plain narration
            return Ok(ParsedVoiceIntent {
                intent_type: VoiceIntentType::NARRATE,
                text: text.trim().to_string(),
                voice_intent: VoiceIntent {
                    actor: "mestre".to_string(),
                    emotion: "neutral".to_string(),
                    style: "neutral".to_string(),
                    pace: Some("normal".to_string()),
                    volume: Some("normal".to_string()),
                    context: None,
                },
                priority: 1,
            });
        }

        let start = voice_tag_start.unwrap();
        let end = voice_tag_end.unwrap() + "</VOICE>".len();
        let tag_content = &text[start..end];

        // Extract attributes from the opening tag (before >)
        let tag_end_pos = tag_content.find('>').unwrap_or(tag_content.len());
        let attributes_section = &tag_content[..tag_end_pos];

        // Extract attributes
        let actor = Self::extract_attribute(attributes_section, "actor")
            .unwrap_or_else(|| "mestre".to_string());
        let emotion = Self::extract_attribute(attributes_section, "emotion")
            .unwrap_or_else(|| "neutral".to_string());
        let style = Self::extract_attribute(attributes_section, "style")
            .unwrap_or_else(|| "neutral".to_string());
        let pace = Self::extract_attribute(attributes_section, "pace");
        let volume = Self::extract_attribute(attributes_section, "volume");
        let context = Self::extract_attribute(attributes_section, "context");

        // Extract text content (between > and </VOICE>)
        let text_start = tag_content.find('>').map(|i| i + 1).unwrap_or(0);
        let text_end = tag_content.rfind("</VOICE>").unwrap_or(tag_content.len());
        let dialogue_text = tag_content[text_start..text_end].trim();
        // Remove quotes if present
        let dialogue_text = dialogue_text.trim_matches('"').trim_matches('\'').trim();

        // Determine intent type from actor and context
        // Priority: NPC/PLAYER > EVENT > CONDITION_EXPIRE > NARRATE
        let intent_type = if actor.starts_with("NPC_") || actor.to_lowercase().starts_with("npc_") {
            VoiceIntentType::NPC_DIALOGUE
        } else if actor.starts_with("PLAYER_") || actor.to_lowercase().starts_with("player_") {
            VoiceIntentType::PLAYER_DIALOGUE
        } else if emotion == "danger" || style == "intense" {
            VoiceIntentType::EVENT
        } else if emotion == "solemn"
            && (dialogue_text.contains("acaba") || dialogue_text.contains("termina"))
        {
            VoiceIntentType::CONDITION_EXPIRE
        } else {
            VoiceIntentType::NARRATE
        };

        // Determine priority
        let priority = match intent_type {
            VoiceIntentType::NARRATE => 1,
            VoiceIntentType::PLAYER_DIALOGUE => 2,
            VoiceIntentType::NPC_DIALOGUE => 3,
            VoiceIntentType::EVENT => 4,
            VoiceIntentType::CONDITION_EXPIRE => 5,
            VoiceIntentType::SYSTEM => 6,
        };

        Ok(ParsedVoiceIntent {
            intent_type,
            text: dialogue_text.to_string(),
            voice_intent: VoiceIntent {
                actor,
                emotion,
                style,
                pace,
                volume,
                context,
            },
            priority,
        })
    }

    fn extract_attribute(text: &str, attr_name: &str) -> Option<String> {
        // Simple attribute extraction without regex dependency
        // Handle both " and ' quotes, and whitespace
        // Pattern 1: attr_name="value" or attr_name='value'
        // Pattern 2: attr_name = "value" or attr_name = 'value'

        // Try pattern with = and quote
        let pattern1 = format!(r#"{}="#, attr_name);
        if let Some(start) = text.find(&pattern1) {
            let value_start = start + pattern1.len();
            // Find closing quote (skip the opening quote if present)
            let search_start =
                if text[value_start..].starts_with('"') || text[value_start..].starts_with('\'') {
                    value_start + 1
                } else {
                    value_start
                };
            // Find closing quote
            if let Some(end) = text[search_start..].find('"') {
                let value = text[search_start..search_start + end].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
            if let Some(end) = text[search_start..].find('\'') {
                let value = text[search_start..search_start + end].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }

        // Try pattern with = and space
        let pattern2 = format!(r#"{} = "#, attr_name);
        if let Some(start) = text.find(&pattern2) {
            let value_start = start + pattern2.len();
            // Skip opening quote if present
            let search_start =
                if text[value_start..].starts_with('"') || text[value_start..].starts_with('\'') {
                    value_start + 1
                } else {
                    value_start
                };
            // Find closing quote
            if let Some(end) = text[search_start..].find('"') {
                let value = text[search_start..search_start + end].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
            if let Some(end) = text[search_start..].find('\'') {
                let value = text[search_start..search_start + end].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_voice_intent() {
        let text = r#"<VOICE actor="NPC_Guard" emotion="skeptic" style="dry" pace="normal" volume="normal">
"Emissário? De qual reino? Mostre sua insígnia!"
</VOICE>"#;

        let parsed = VoiceIntentParser::parse(text).unwrap();
        assert_eq!(parsed.intent_type, VoiceIntentType::NPC_DIALOGUE);
        assert_eq!(parsed.voice_intent.actor, "NPC_Guard");
        assert_eq!(parsed.voice_intent.emotion, "skeptic");
        assert_eq!(parsed.voice_intent.style, "dry");
        assert_eq!(
            parsed.text,
            "Emissário? De qual reino? Mostre sua insígnia!"
        );
    }

    #[test]
    fn test_parse_narrate() {
        let text = "O corredor é estreito, iluminado por tochas antigas.";

        let parsed = VoiceIntentParser::parse(text).unwrap();
        assert_eq!(parsed.intent_type, VoiceIntentType::NARRATE);
        assert_eq!(parsed.voice_intent.actor, "mestre");
        assert_eq!(parsed.voice_intent.emotion, "neutral");
    }

    #[test]
    fn test_parse_event() {
        let text = r#"<VOICE actor="mestre" emotion="danger" style="intense">
"O ogro avança e a sala inteira treme com o impacto."
</VOICE>"#;

        let parsed = VoiceIntentParser::parse(text).unwrap();
        assert_eq!(parsed.intent_type, VoiceIntentType::EVENT);
    }
}
