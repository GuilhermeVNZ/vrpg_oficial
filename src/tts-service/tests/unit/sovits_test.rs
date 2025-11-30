//! Unit tests for SoVITS module

use tts_service::sovits::{SoVITSModel, VoiceIntent};
use std::path::PathBuf;

#[tokio::test]
async fn test_sovits_model_loading() {
    let mut model = SoVITSModel::new("npc_guard", PathBuf::from("test_guard.sovits").as_path());
    assert!(!model.is_loaded());
    
    model.load().await.unwrap();
    assert!(model.is_loaded());
    assert_eq!(model.character_id(), "npc_guard");
}

#[tokio::test]
async fn test_sovits_convert() {
    let mut model = SoVITSModel::new("npc_guard", PathBuf::from("test_guard.sovits").as_path());
    model.load().await.unwrap();
    
    let neutral_audio = vec![0.0f32; 1000];
    let intent = VoiceIntent {
        actor: "NPC_Guard".to_string(),
        emotion: "skeptic".to_string(),
        style: "dry".to_string(),
        pace: Some("normal".to_string()),
        volume: Some("normal".to_string()),
        context: None,
    };
    
    let result = model.convert(&neutral_audio, 22050, &intent).await.unwrap();
    assert_eq!(result.samples.len(), neutral_audio.len());
    assert_eq!(result.sample_rate, 22050);
    assert_eq!(result.channels, 1);
}

#[tokio::test]
async fn test_sovits_emotion_support() {
    let model = SoVITSModel::new("npc_guard", PathBuf::from("test_guard.sovits").as_path());
    
    assert!(model.supports_emotion("neutral"));
    assert!(model.supports_emotion("rage"));
    assert!(model.supports_emotion("skeptic"));
    assert!(!model.supports_emotion("nonexistent"));
}

#[tokio::test]
async fn test_sovits_style_support() {
    let model = SoVITSModel::new("npc_guard", PathBuf::from("test_guard.sovits").as_path());
    
    assert!(model.supports_style("neutral"));
    assert!(model.supports_style("dry"));
    assert!(model.supports_style("authoritative"));
    assert!(!model.supports_style("nonexistent"));
}

#[tokio::test]
async fn test_sovits_emotion_application() {
    let mut model = SoVITSModel::new("npc_guard", PathBuf::from("test_guard.sovits").as_path());
    model.load().await.unwrap();
    
    let neutral_audio = vec![0.1f32; 1000];
    
    // Test different emotions
    let intent_rage = VoiceIntent {
        actor: "NPC_Guard".to_string(),
        emotion: "rage".to_string(),
        style: "neutral".to_string(),
        pace: None,
        volume: None,
        context: None,
    };
    
    let intent_calm = VoiceIntent {
        actor: "NPC_Guard".to_string(),
        emotion: "calm".to_string(),
        style: "neutral".to_string(),
        pace: None,
        volume: None,
        context: None,
    };
    
    let audio_rage = model.convert(&neutral_audio, 22050, &intent_rage).await.unwrap();
    let audio_calm = model.convert(&neutral_audio, 22050, &intent_calm).await.unwrap();
    
    // Different emotions should produce different audio
    assert_ne!(audio_rage.samples, audio_calm.samples);
}

#[tokio::test]
async fn test_sovits_not_loaded() {
    let model = SoVITSModel::new("npc_guard", PathBuf::from("test_guard.sovits").as_path());
    
    let neutral_audio = vec![0.0f32; 100];
    let intent = VoiceIntent {
        actor: "NPC_Guard".to_string(),
        emotion: "neutral".to_string(),
        style: "neutral".to_string(),
        pace: None,
        volume: None,
        context: None,
    };
    
    let result = model.convert(&neutral_audio, 22050, &intent).await;
    assert!(result.is_err());
}

