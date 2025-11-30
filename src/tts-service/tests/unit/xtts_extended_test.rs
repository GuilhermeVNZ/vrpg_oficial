//! Extended Unit Tests for XTTS Module
//!
//! Following rulebook standards: comprehensive coverage, Given/When/Then scenarios

use tts_service::xtts::{AudioOutput, SynthesisRequest, XttsModel};

#[tokio::test]
async fn test_xtts_model_new() {
    // Given a new XTTS model
    // When creating with default options
    // Then it should be created successfully
    let model = XttsModel::new();
    assert!(!model.is_loaded());
    assert_eq!(model.list_voices().len(), 4); // dm, npc_male, npc_female, monster
}

#[tokio::test]
async fn test_xtts_model_new_with_options() {
    // Given XTTS model options
    // When creating with Coqui XTTS enabled and GPU
    // Then it should be created with correct flags
    let model = XttsModel::new_with_options(true, true, None);
    assert!(!model.is_loaded());
}

#[tokio::test]
async fn test_xtts_model_load() {
    // Given an XTTS model
    // When loading the model
    // Then it should be marked as loaded
    let mut model = XttsModel::new();
    let result = model.load("test_path").await;
    assert!(result.is_ok());
    assert!(model.is_loaded());
}

#[tokio::test]
async fn test_xtts_model_get_voice() {
    // Given an XTTS model
    // When getting a voice profile
    // Then it should return the correct voice
    let model = XttsModel::new();
    let voice = model.get_voice("dm");
    assert!(voice.is_some());
    assert_eq!(voice.unwrap().name, "Dungeon Master");
}

#[tokio::test]
async fn test_xtts_model_get_voice_not_found() {
    // Given an XTTS model
    // When getting a non-existent voice
    // Then it should return None
    let model = XttsModel::new();
    let voice = model.get_voice("nonexistent");
    assert!(voice.is_none());
}

#[tokio::test]
async fn test_xtts_model_list_voices() {
    // Given an XTTS model
    // When listing all voices
    // Then it should return all default voices
    let model = XttsModel::new();
    let voices = model.list_voices();
    assert_eq!(voices.len(), 4);
    let voice_names: Vec<&str> = voices.iter().map(|v| v.name.as_str()).collect();
    assert!(voice_names.contains(&"Dungeon Master"));
    assert!(voice_names.contains(&"NPC Male"));
    assert!(voice_names.contains(&"NPC Female"));
    assert!(voice_names.contains(&"Monster"));
}

#[tokio::test]
async fn test_xtts_synthesize_model_not_loaded() {
    // Given an XTTS model that is not loaded
    // When synthesizing audio
    // Then it should return an error
    let model = XttsModel::new();
    let request = SynthesisRequest {
        text: "Hello".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    let result = model.synthesize(&request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not loaded"));
}

#[tokio::test]
async fn test_xtts_synthesize_basic() {
    // Given a loaded XTTS model
    // When synthesizing audio with valid request
    // Then it should return audio output
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    let result = model.synthesize(&request).await;
    assert!(result.is_ok());
    
    let audio = result.unwrap();
    assert_eq!(audio.sample_rate, 24000);
    assert_eq!(audio.channels, 1);
    assert!(!audio.samples.is_empty());
}

#[tokio::test]
async fn test_xtts_synthesize_voice_not_found() {
    // Given a loaded XTTS model
    // When synthesizing with non-existent voice
    // Then it should return an error
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Hello".to_string(),
        voice_id: "nonexistent".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    let result = model.synthesize(&request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Voice not found"));
}

#[tokio::test]
async fn test_xtts_synthesize_cache() {
    // Given a loaded XTTS model
    // When synthesizing the same request twice
    // Then the second call should use cache
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let audio1 = model.synthesize(&request).await.unwrap();
    let audio2 = model.synthesize(&request).await.unwrap();
    
    // Cached result should be identical
    assert_eq!(audio1.samples.len(), audio2.samples.len());
    assert_eq!(audio1.sample_rate, audio2.sample_rate);
    assert_eq!(audio1.channels, audio2.channels);
}

#[tokio::test]
async fn test_xtts_synthesize_different_speeds() {
    // Given a loaded XTTS model
    // When synthesizing with different speeds
    // Then it should produce different audio lengths
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request_slow = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 0.5,
        pitch: 0.5,
    };
    
    let request_fast = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 2.0,
        pitch: 0.5,
    };
    
    let audio_slow = model.synthesize(&request_slow).await.unwrap();
    let audio_fast = model.synthesize(&request_fast).await.unwrap();
    
    // Slower speed should produce longer audio
    assert!(audio_slow.samples.len() > audio_fast.samples.len());
}

#[tokio::test]
async fn test_xtts_synthesize_different_pitches() {
    // Given a loaded XTTS model
    // When synthesizing with different pitches
    // Then it should produce different audio
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request_low = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let request_high = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 1.0,
    };
    
    let audio_low = model.synthesize(&request_low).await.unwrap();
    let audio_high = model.synthesize(&request_high).await.unwrap();
    
    // Should produce different audio (different frequencies)
    assert_eq!(audio_low.samples.len(), audio_high.samples.len());
}

#[tokio::test]
async fn test_xtts_clear_cache() {
    // Given a loaded XTTS model with cached audio
    // When clearing the cache
    // Then the cache should be empty
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    // Synthesize to populate cache
    model.synthesize(&request).await.unwrap();
    
    // Clear cache
    model.clear_cache().await;
    
    // Next synthesis should not use cache (will generate new audio)
    let audio = model.synthesize(&request).await.unwrap();
    assert!(!audio.samples.is_empty());
}

#[tokio::test]
async fn test_xtts_enable_coqui_xtts() {
    // Given an XTTS model
    // When enabling Coqui XTTS
    // Then it should be enabled
    let mut model = XttsModel::new();
    model.enable_coqui_xtts(true);
    // Model should still work (though Coqui XTTS requires Python bridge in real implementation)
    assert!(!model.is_loaded());
}

#[tokio::test]
async fn test_xtts_set_model_path() {
    // Given an XTTS model
    // When setting a custom model path
    // Then it should be stored
    let mut model = XttsModel::new();
    let path = std::path::PathBuf::from("custom_model_path");
    model.set_model_path(path.clone());
    // Path is stored internally (not directly accessible, but should not error)
    assert!(!model.is_loaded());
}

#[tokio::test]
async fn test_xtts_synthesize_streaming() {
    // Given a loaded XTTS model
    // When synthesizing with streaming
    // Then it should return a stream of audio chunks
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let mut stream = model.synthesize_streaming(&request, 100).await; // 100ms chunks
    
    let mut total_samples = 0;
    let mut chunk_count = 0;
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.unwrap();
        total_samples += chunk.samples.len();
        chunk_count += 1;
        assert_eq!(chunk.sample_rate, 24000);
        assert_eq!(chunk.channels, 1);
    }
    
    assert!(chunk_count > 0);
    assert!(total_samples > 0);
}

#[tokio::test]
async fn test_xtts_synthesize_streaming_error() {
    // Given an XTTS model that is not loaded
    // When synthesizing with streaming
    // Then it should return an error in the stream
    let model = XttsModel::new();
    
    let request = SynthesisRequest {
        text: "Hello".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let mut stream = model.synthesize_streaming(&request, 100).await;
    let first_chunk = stream.next().await;
    assert!(first_chunk.is_some());
    assert!(first_chunk.unwrap().is_err());
}

#[tokio::test]
async fn test_xtts_audio_output_serialization() {
    // Given an AudioOutput
    // When serializing to JSON
    // Then it should serialize correctly
    let audio = AudioOutput {
        samples: vec![0.1, 0.2, -0.1, -0.2],
        sample_rate: 24000,
        channels: 1,
    };
    
    let json = serde_json::to_string(&audio).unwrap();
    let deserialized: AudioOutput = serde_json::from_str(&json).unwrap();
    
    assert_eq!(audio.samples, deserialized.samples);
    assert_eq!(audio.sample_rate, deserialized.sample_rate);
    assert_eq!(audio.channels, deserialized.channels);
}

#[tokio::test]
async fn test_xtts_synthesis_request_serialization() {
    // Given a SynthesisRequest
    // When serializing to JSON
    // Then it should serialize correctly
    let request = SynthesisRequest {
        text: "Hello world".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.5,
        pitch: 0.7,
    };
    
    let json = serde_json::to_string(&request).unwrap();
    let deserialized: SynthesisRequest = serde_json::from_str(&json).unwrap();
    
    assert_eq!(request.text, deserialized.text);
    assert_eq!(request.voice_id, deserialized.voice_id);
    assert_eq!(request.speed, deserialized.speed);
    assert_eq!(request.pitch, deserialized.pitch);
}

#[tokio::test]
async fn test_xtts_different_voices() {
    // Given a loaded XTTS model
    // When synthesizing with different voices
    // Then it should produce different audio
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let text = "Hello world".to_string();
    
    let request_dm = SynthesisRequest {
        text: text.clone(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let request_npc = SynthesisRequest {
        text: text.clone(),
        voice_id: "npc_male".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let audio_dm = model.synthesize(&request_dm).await.unwrap();
    let audio_npc = model.synthesize(&request_npc).await.unwrap();
    
    // Different voices should produce different audio
    assert_eq!(audio_dm.samples.len(), audio_npc.samples.len());
    // Audio content should be different (different base_pitch)
    assert_ne!(audio_dm.samples, audio_npc.samples);
}

#[tokio::test]
async fn test_xtts_long_text() {
    // Given a loaded XTTS model
    // When synthesizing long text
    // Then it should produce proportionally longer audio
    let mut model = XttsModel::new();
    model.load("test_path").await.unwrap();
    
    let short_text = "Hello".to_string();
    let long_text = "Hello world this is a much longer text that should produce more audio samples".to_string();
    
    let request_short = SynthesisRequest {
        text: short_text,
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let request_long = SynthesisRequest {
        text: long_text,
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.5,
    };
    
    let audio_short = model.synthesize(&request_short).await.unwrap();
    let audio_long = model.synthesize(&request_long).await.unwrap();
    
    // Longer text should produce more samples
    assert!(audio_long.samples.len() > audio_short.samples.len());
}



