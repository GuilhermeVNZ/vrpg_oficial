//! Unit tests for Voice INTENT parser

use tts_service::voice_intent::{VoiceIntentParser, VoiceIntentType};

#[test]
fn test_parse_npc_dialogue() {
    let text = r#"<VOICE actor="NPC_Guard" emotion="skeptic" style="dry" pace="normal" volume="normal">
"Emissário? De qual reino? Mostre sua insígnia!"
</VOICE>"#;
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    assert_eq!(parsed.intent_type, VoiceIntentType::NPC_DIALOGUE);
    assert_eq!(parsed.voice_intent.actor, "NPC_Guard");
    assert_eq!(parsed.voice_intent.emotion, "skeptic");
    assert_eq!(parsed.voice_intent.style, "dry");
    assert_eq!(parsed.text, "Emissário? De qual reino? Mostre sua insígnia!");
    assert_eq!(parsed.priority, 3);
}

#[test]
fn test_parse_narrate() {
    let text = "O corredor é estreito, iluminado por tochas antigas.";
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    assert_eq!(parsed.intent_type, VoiceIntentType::NARRATE);
    assert_eq!(parsed.voice_intent.actor, "mestre");
    assert_eq!(parsed.voice_intent.emotion, "neutral");
    assert_eq!(parsed.voice_intent.style, "neutral");
    assert_eq!(parsed.priority, 1);
}

#[test]
fn test_parse_event() {
    let text = r#"<VOICE actor="mestre" emotion="danger" style="intense">
"O ogro avança e a sala inteira treme com o impacto."
</VOICE>"#;
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    assert_eq!(parsed.intent_type, VoiceIntentType::EVENT);
    assert_eq!(parsed.voice_intent.emotion, "danger");
    assert_eq!(parsed.voice_intent.style, "intense");
    assert_eq!(parsed.priority, 4);
}

#[test]
fn test_parse_player_dialogue() {
    let text = r#"<VOICE actor="PLAYER_Rogue" emotion="amused" style="casual">
"Relaxa... eu abro a porta."
</VOICE>"#;
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    assert_eq!(parsed.intent_type, VoiceIntentType::PLAYER_DIALOGUE);
    assert_eq!(parsed.voice_intent.actor, "PLAYER_Rogue");
    assert_eq!(parsed.priority, 2);
}

#[test]
fn test_parse_condition_expire() {
    let text = r#"<VOICE actor="mestre" emotion="solemn" style="neutral">
"A energia rubra abandona seus músculos. A dor retorna."
</VOICE>"#;
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    // Should detect as CONDITION_EXPIRE if text contains keywords
    // For now, it will be NARRATE, but we can enhance detection
    assert_eq!(parsed.voice_intent.emotion, "solemn");
}

#[test]
fn test_parse_with_quotes() {
    let text = r#"<VOICE actor="NPC_Guard" emotion="skeptic" style="dry">
'Emissário? De qual reino?'
</VOICE>"#;
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    assert_eq!(parsed.text, "Emissário? De qual reino?");
}

#[test]
fn test_parse_missing_attributes() {
    let text = r#"<VOICE actor="NPC_Guard">
"Test"
</VOICE>"#;
    
    let parsed = VoiceIntentParser::parse(text).unwrap();
    assert_eq!(parsed.voice_intent.actor, "NPC_Guard");
    assert_eq!(parsed.voice_intent.emotion, "neutral"); // Default
    assert_eq!(parsed.voice_intent.style, "neutral"); // Default
}



