//! Extended Unit Tests for Voice Intent Parser
//!
//! Following rulebook standards: comprehensive coverage, Given/When/Then scenarios

use tts_service::voice_intent::{ParsedVoiceIntent, VoiceIntentParser, VoiceIntentType};

#[test]
fn test_voice_intent_parser_parse_no_tag() {
    // Given text without VOICE tag
    // When parsing
    // Then it should return default narration intent
    let text = "This is plain narration text.";
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.intent_type, VoiceIntentType::NARRATE);
    assert_eq!(result.text, "This is plain narration text.");
    assert_eq!(result.voice_intent.actor, "mestre");
    assert_eq!(result.voice_intent.emotion, "neutral");
    assert_eq!(result.voice_intent.style, "neutral");
}

#[test]
fn test_voice_intent_parser_parse_with_actor() {
    // Given text with VOICE tag containing actor
    // When parsing
    // Then it should extract the actor
    let text = r#"<VOICE actor="NPC_Guard">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.actor, "NPC_Guard");
}

#[test]
fn test_voice_intent_parser_parse_with_emotion() {
    // Given text with VOICE tag containing emotion
    // When parsing
    // Then it should extract the emotion
    let text = r#"<VOICE actor="NPC_Guard" emotion="angry">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.emotion, "angry");
}

#[test]
fn test_voice_intent_parser_parse_with_style() {
    // Given text with VOICE tag containing style
    // When parsing
    // Then it should extract the style
    let text = r#"<VOICE actor="NPC_Guard" style="dramatic">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.style, "dramatic");
}

#[test]
fn test_voice_intent_parser_parse_with_pace() {
    // Given text with VOICE tag containing pace
    // When parsing
    // Then it should extract the pace
    let text = r#"<VOICE actor="NPC_Guard" pace="fast">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.pace, Some("fast".to_string()));
}

#[test]
fn test_voice_intent_parser_parse_with_volume() {
    // Given text with VOICE tag containing volume
    // When parsing
    // Then it should extract the volume
    let text = r#"<VOICE actor="NPC_Guard" volume="loud">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.volume, Some("loud".to_string()));
}

#[test]
fn test_voice_intent_parser_parse_with_context() {
    // Given text with VOICE tag containing context
    // When parsing
    // Then it should extract the context
    let text = r#"<VOICE actor="NPC_Guard" context="combat">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.context, Some("combat".to_string()));
}

#[test]
fn test_voice_intent_parser_parse_all_attributes() {
    // Given text with VOICE tag containing all attributes
    // When parsing
    // Then it should extract all attributes
    let text = r#"<VOICE actor="NPC_Guard" emotion="angry" style="dramatic" pace="fast" volume="loud" context="combat">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.actor, "NPC_Guard");
    assert_eq!(result.voice_intent.emotion, "angry");
    assert_eq!(result.voice_intent.style, "dramatic");
    assert_eq!(result.voice_intent.pace, Some("fast".to_string()));
    assert_eq!(result.voice_intent.volume, Some("loud".to_string()));
    assert_eq!(result.voice_intent.context, Some("combat".to_string()));
}

#[test]
fn test_voice_intent_parser_parse_text_content() {
    // Given text with VOICE tag
    // When parsing
    // Then it should extract the text content
    let text = r#"<VOICE actor="NPC_Guard">Hello, world!</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.text, "Hello, world!");
}

#[test]
fn test_voice_intent_parser_parse_multiline_text() {
    // Given text with VOICE tag containing multiline text
    // When parsing
    // Then it should extract all text content
    let text = r#"<VOICE actor="NPC_Guard">
Hello,
this is a multiline
text.
</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert!(result.text.contains("Hello"));
    assert!(result.text.contains("multiline"));
}

#[test]
fn test_voice_intent_parser_parse_defaults() {
    // Given text with VOICE tag but missing attributes
    // When parsing
    // Then it should use default values
    let text = r#"<VOICE>Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert_eq!(result.voice_intent.actor, "mestre");
    assert_eq!(result.voice_intent.emotion, "neutral");
    assert_eq!(result.voice_intent.style, "neutral");
}

#[test]
fn test_voice_intent_type_from_str() {
    // Given a string representation of VoiceIntentType
    // When parsing
    // Then it should return the correct type
    assert_eq!(
        VoiceIntentType::from_str("NARRATE").unwrap(),
        VoiceIntentType::NARRATE
    );
    assert_eq!(
        VoiceIntentType::from_str("NPC_DIALOGUE").unwrap(),
        VoiceIntentType::NPC_DIALOGUE
    );
    assert_eq!(
        VoiceIntentType::from_str("PLAYER_DIALOGUE").unwrap(),
        VoiceIntentType::PLAYER_DIALOGUE
    );
    assert_eq!(
        VoiceIntentType::from_str("EVENT").unwrap(),
        VoiceIntentType::EVENT
    );
    assert_eq!(
        VoiceIntentType::from_str("CONDITION_EXPIRE").unwrap(),
        VoiceIntentType::CONDITION_EXPIRE
    );
    assert_eq!(
        VoiceIntentType::from_str("SYSTEM").unwrap(),
        VoiceIntentType::SYSTEM
    );
}

#[test]
fn test_voice_intent_type_from_str_case_insensitive() {
    // Given a string representation in different case
    // When parsing
    // Then it should be case-insensitive
    assert_eq!(
        VoiceIntentType::from_str("narrate").unwrap(),
        VoiceIntentType::NARRATE
    );
    assert_eq!(
        VoiceIntentType::from_str("Narrate").unwrap(),
        VoiceIntentType::NARRATE
    );
    assert_eq!(
        VoiceIntentType::from_str("NaRrAtE").unwrap(),
        VoiceIntentType::NARRATE
    );
}

#[test]
fn test_voice_intent_type_from_str_invalid() {
    // Given an invalid string
    // When parsing
    // Then it should return an error
    let result = VoiceIntentType::from_str("INVALID");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown voice intent type"));
}

#[test]
fn test_voice_intent_parser_extract_attribute() {
    // Given an attributes section with a specific attribute
    // When extracting the attribute
    // Then it should return the correct value
    let attributes = r#"actor="NPC_Guard" emotion="angry""#;
    let actor = VoiceIntentParser::extract_attribute(attributes, "actor");
    assert_eq!(actor, Some("NPC_Guard".to_string()));
    
    let emotion = VoiceIntentParser::extract_attribute(attributes, "emotion");
    assert_eq!(emotion, Some("angry".to_string()));
}

#[test]
fn test_voice_intent_parser_extract_attribute_not_found() {
    // Given an attributes section without a specific attribute
    // When extracting the attribute
    // Then it should return None
    let attributes = r#"actor="NPC_Guard""#;
    let emotion = VoiceIntentParser::extract_attribute(attributes, "emotion");
    assert_eq!(emotion, None);
}

#[test]
fn test_voice_intent_parser_extract_attribute_quoted() {
    // Given an attributes section with quoted values
    // When extracting the attribute
    // Then it should handle quotes correctly
    let attributes = r#"actor="NPC_Guard" emotion='angry' style="dramatic""#;
    let actor = VoiceIntentParser::extract_attribute(attributes, "actor");
    assert_eq!(actor, Some("NPC_Guard".to_string()));
    
    let emotion = VoiceIntentParser::extract_attribute(attributes, "emotion");
    assert_eq!(emotion, Some("angry".to_string()));
}

#[test]
fn test_voice_intent_parser_parse_priority() {
    // Given text with VOICE tag
    // When parsing
    // Then it should have a priority value
    let text = r#"<VOICE actor="NPC_Guard">Hello</VOICE>"#;
    let result = VoiceIntentParser::parse(text).unwrap();
    
    assert!(result.priority >= 1 && result.priority <= 9);
}

#[test]
fn test_voice_intent_parser_parse_serialization() {
    // Given a ParsedVoiceIntent
    // When serializing to JSON
    // Then it should serialize correctly
    let text = r#"<VOICE actor="NPC_Guard" emotion="angry">Hello</VOICE>"#;
    let parsed = VoiceIntentParser::parse(text).unwrap();
    
    let json = serde_json::to_string(&parsed).unwrap();
    let deserialized: ParsedVoiceIntent = serde_json::from_str(&json).unwrap();
    
    assert_eq!(parsed.intent_type, deserialized.intent_type);
    assert_eq!(parsed.text, deserialized.text);
    assert_eq!(parsed.voice_intent.actor, deserialized.voice_intent.actor);
    assert_eq!(parsed.voice_intent.emotion, deserialized.voice_intent.emotion);
    assert_eq!(parsed.voice_intent.style, deserialized.voice_intent.style);
}



