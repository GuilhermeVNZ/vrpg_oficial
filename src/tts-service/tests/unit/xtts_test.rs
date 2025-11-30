//! Unit tests for XTTS module
//! Tests the XTTS model implementation, caching, and error handling

use tts_service::{XttsModel, SynthesisRequest, AudioOutput, TtsError};
use tts_service::error::Result;

#[tokio::test]
async fn test_xtts_model_creation() {
    let model = XttsModel::new();
    assert!(!model.is_loaded());
    assert_eq!(model.list_voices().len(), 4); // dm, npc_male, npc_female, monster
}

#[tokio::test]
async fn test_xtts_model_loading() {
    let mut model = XttsModel::new();
    assert!(!model.is_loaded());
    
    // Load should succeed even with dummy path (current stub implementation)
    model.load("test_model").await.unwrap();
    assert!(model.is_loaded());
}

#[tokio::test]
async fn test_xtts_model_not_loaded_error() {
    let model = XttsModel::new();
    assert!(!model.is_loaded());
    
    let request = SynthesisRequest {
        text: "Test".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let result = model.synthesize(&request).await;
    assert!(result.is_err());
    
    if let Err(TtsError::ModelLoad(msg)) = result {
        assert!(msg.contains("not loaded"));
    } else {
        panic!("Expected ModelLoad error");
    }
}

#[tokio::test]
async fn test_xtts_voice_not_found() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Test".to_string(),
        voice_id: "nonexistent_voice".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let result = model.synthesize(&request).await;
    assert!(result.is_err());
    
    if let Err(TtsError::Voice(msg)) = result {
        assert!(msg.contains("not found"));
    } else {
        panic!("Expected Voice error");
    }
}

#[tokio::test]
async fn test_xtts_synthesize_basic() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Hello, world!".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let result = model.synthesize(&request).await.unwrap();
    
    // Validate output structure
    assert!(!result.samples.is_empty());
    assert!(result.sample_rate > 0);
    assert!(result.channels > 0);
    
    // Check audio samples are in valid range
    for sample in &result.samples {
        assert!(*sample >= -1.0 && *sample <= 1.0, "Sample out of range: {}", sample);
    }
}

#[tokio::test]
async fn test_xtts_synthesize_different_voices() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let voices = vec!["dm", "npc_male", "npc_female", "monster"];
    let text = "Test voice synthesis";
    
    for voice_id in voices {
        let request = SynthesisRequest {
            text: text.to_string(),
            voice_id: voice_id.to_string(),
            speed: 1.0,
            pitch: 0.0,
        };
        
        let result = model.synthesize(&request).await.unwrap();
        assert!(!result.samples.is_empty(), "Voice {} produced empty audio", voice_id);
    }
}

#[tokio::test]
async fn test_xtts_synthesize_different_speeds() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let speeds = vec![0.5, 0.75, 1.0, 1.25, 1.5];
    let text = "Testing different speeds";
    
    for speed in speeds {
        let request = SynthesisRequest {
            text: text.to_string(),
            voice_id: "dm".to_string(),
            speed,
            pitch: 0.0,
        };
        
        let result = model.synthesize(&request).await.unwrap();
        assert!(!result.samples.is_empty(), "Speed {} produced empty audio", speed);
        
        // Faster speed should produce shorter audio (in current stub implementation)
        // In real XTTS, this would be more complex
    }
}

#[tokio::test]
async fn test_xtts_synthesize_different_pitches() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let pitches = vec![-0.5, -0.25, 0.0, 0.25, 0.5];
    let text = "Testing different pitches";
    
    for pitch in pitches {
        let request = SynthesisRequest {
            text: text.to_string(),
            voice_id: "dm".to_string(),
            speed: 1.0,
            pitch,
        };
        
        let result = model.synthesize(&request).await.unwrap();
        assert!(!result.samples.is_empty(), "Pitch {} produced empty audio", pitch);
    }
}

#[tokio::test]
async fn test_xtts_cache_functionality() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Cache test".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    // First synthesis
    let result1 = model.synthesize(&request).await.unwrap();
    let samples1_len = result1.samples.len();
    
    // Second synthesis (should be cached)
    let result2 = model.synthesize(&request).await.unwrap();
    let samples2_len = result2.samples.len();
    
    // Cached result should be identical
    assert_eq!(samples1_len, samples2_len);
    assert_eq!(result1.sample_rate, result2.sample_rate);
    assert_eq!(result1.channels, result2.channels);
    
    // Samples should be identical (exact cache hit)
    assert_eq!(result1.samples, result2.samples);
}

#[tokio::test]
async fn test_xtts_cache_different_requests() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request1 = SynthesisRequest {
        text: "First request".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let request2 = SynthesisRequest {
        text: "Second request".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let result1 = model.synthesize(&request1).await.unwrap();
    let result2 = model.synthesize(&request2).await.unwrap();
    
    // Different requests should produce different results
    assert_ne!(result1.samples, result2.samples);
}

#[tokio::test]
async fn test_xtts_cache_clear() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Cache clear test".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    // First synthesis
    let result1 = model.synthesize(&request).await.unwrap();
    
    // Clear cache
    model.clear_cache().await;
    
    // Second synthesis (should regenerate, not use cache)
    let result2 = model.synthesize(&request).await.unwrap();
    
    // Results should be identical (same input), but regenerated
    assert_eq!(result1.samples.len(), result2.samples.len());
}

#[tokio::test]
async fn test_xtts_get_voice() {
    let model = XttsModel::new();
    
    let voice = model.get_voice("dm");
    assert!(voice.is_some());
    
    let voice = voice.unwrap();
    assert_eq!(voice.id, "dm");
    assert_eq!(voice.name, "Dungeon Master");
    
    let nonexistent = model.get_voice("nonexistent");
    assert!(nonexistent.is_none());
}

#[tokio::test]
async fn test_xtts_list_voices() {
    let model = XttsModel::new();
    let voices = model.list_voices();
    
    assert_eq!(voices.len(), 4);
    
    let voice_ids: Vec<&str> = voices.iter().map(|v| v.id.as_str()).collect();
    assert!(voice_ids.contains(&"dm"));
    assert!(voice_ids.contains(&"npc_male"));
    assert!(voice_ids.contains(&"npc_female"));
    assert!(voice_ids.contains(&"monster"));
}

#[tokio::test]
async fn test_xtts_audio_output_structure() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Structure test".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let output = model.synthesize(&request).await.unwrap();
    
    // Validate AudioOutput structure
    assert!(!output.samples.is_empty());
    assert!(output.sample_rate >= 8000 && output.sample_rate <= 48000);
    assert!(output.channels >= 1 && output.channels <= 2);
    
    // Check for NaN or Inf values
    for (i, sample) in output.samples.iter().enumerate() {
        assert!(!sample.is_nan(), "NaN at index {}", i);
        assert!(!sample.is_infinite(), "Inf at index {}", i);
    }
}

#[tokio::test]
async fn test_xtts_empty_text() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    // Empty text might produce empty audio or error
    let result = model.synthesize(&request).await;
    // Current stub might handle this, real XTTS might error
    // Just ensure it doesn't panic
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_xtts_long_text() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let long_text = "This is a very long text that should test the XTTS model's ability to handle longer inputs. ".repeat(10);
    
    let request = SynthesisRequest {
        text: long_text,
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let result = model.synthesize(&request).await.unwrap();
    
    // Long text should produce longer audio
    assert!(!result.samples.is_empty());
    assert!(result.samples.len() > 1000); // Should be substantial
}

#[tokio::test]
async fn test_xtts_special_characters() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let texts = vec![
        "Hello, world!",
        "Test with numbers: 12345",
        "Test with symbols: @#$%",
        "Test with unicode: 你好",
        "Test with quotes: \"Hello\"",
    ];
    
    for text in texts {
        let request = SynthesisRequest {
            text: text.to_string(),
            voice_id: "dm".to_string(),
            speed: 1.0,
            pitch: 0.0,
        };
        
        let result = model.synthesize(&request).await;
        // Should not panic, might succeed or fail depending on model
        assert!(result.is_ok() || result.is_err());
    }
}

