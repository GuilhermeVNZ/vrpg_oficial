//! Integration tests for XTTS module
//! Tests XTTS integration with pipeline, Python bridge, and fallback scenarios

use tts_service::xtts::{XttsModel, SynthesisRequest, SharedXttsModel};
use tts_service::error::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Test if Coqui TTS is available in Python environment
/// This is a helper to skip tests if XTTS dependencies are not installed
fn check_coqui_tts_available() -> bool {
    use std::process::Command;
    
    let output = Command::new("python")
        .arg("-c")
        .arg("import TTS; print('OK')")
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("OK")
        }
        _ => false,
    }
}

#[tokio::test]
#[ignore] // Ignore by default, run with --ignored flag
async fn test_xtts_python_bridge_coqui_available() {
    if !check_coqui_tts_available() {
        eprintln!("⚠️  Coqui TTS not available, skipping test");
        eprintln!("   Install with: pip install TTS");
        return;
    }
    
    // This test requires actual Coqui XTTS implementation
    // For now, we'll test the Python bridge script structure
    
    let python_script = r#"
import sys
import json
try:
    from TTS.api import TTS
    print(json.dumps({"available": True}))
except ImportError:
    print(json.dumps({"available": False, "error": "TTS not installed"}))
    sys.exit(1)
"#;
    
    let output = tokio::process::Command::new("python")
        .arg("-c")
        .arg(python_script)
        .output()
        .await
        .expect("Failed to run Python");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value = serde_json::from_str(stdout.trim())
        .expect("Failed to parse Python output");
    
    assert!(result["available"].as_bool().unwrap_or(false), 
        "Coqui TTS should be available for this test");
}

#[tokio::test]
async fn test_xtts_shared_model() {
    let model: SharedXttsModel = Arc::new(RwLock::new(XttsModel::new()));
    
    {
        let mut model_guard = model.write().await;
        model_guard.load("test_model").await.unwrap();
    }
    
    // Test that we can use the shared model from different tasks
    let model_clone = model.clone();
    let handle = tokio::spawn(async move {
        let model_guard = model_clone.read().await;
        model_guard.is_loaded()
    });
    
    let is_loaded = handle.await.unwrap();
    assert!(is_loaded);
}

#[tokio::test]
async fn test_xtts_concurrent_synthesis() {
    let model: SharedXttsModel = Arc::new(RwLock::new({
        let mut m = XttsModel::new();
        // Note: This is async, but we're in a test context
        // In real usage, load would be done before spawning tasks
        m
    }));
    
    // Load model first
    {
        let mut model_guard = model.write().await;
        model_guard.load("test_model").await.unwrap();
    }
    
    // Spawn multiple synthesis tasks
    let mut handles = vec![];
    
    for i in 0..5 {
        let model_clone = model.clone();
        let handle = tokio::spawn(async move {
            let request = SynthesisRequest {
                text: format!("Test {}", i),
                voice_id: "dm".to_string(),
                speed: 1.0,
                pitch: 0.0,
            };
            
            let model_guard = model_clone.read().await;
            model_guard.synthesize(&request).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok(), "Concurrent synthesis should succeed");
        let audio = result.unwrap();
        assert!(!audio.samples.is_empty());
    }
}

#[tokio::test]
async fn test_xtts_pipeline_integration_structure() {
    // Test that XTTS can be integrated into pipeline structure
    // This is a structural test, actual pipeline integration would be in pipeline_test.rs
    
    let model: SharedXttsModel = Arc::new(RwLock::new({
        let mut m = XttsModel::new();
        // In real scenario, this would be loaded from config
        m
    }));
    
    // Verify model structure is compatible with pipeline expectations
    {
        let model_guard = model.read().await;
        assert!(!model_guard.is_loaded()); // Not loaded yet
        
        let voices = model_guard.list_voices();
        assert!(!voices.is_empty());
    }
}

#[tokio::test]
async fn test_xtts_error_propagation() {
    let mut model = XttsModel::new();
    
    // Test that errors are properly propagated
    let request = SynthesisRequest {
        text: "Test".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    // Should fail because model not loaded
    let result = model.synthesize(&request).await;
    assert!(result.is_err());
    
    // Load model
    model.load("test_model").await.unwrap();
    
    // Should fail because voice doesn't exist
    let bad_request = SynthesisRequest {
        text: "Test".to_string(),
        voice_id: "invalid_voice".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let result = model.synthesize(&bad_request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_xtts_audio_quality_basic() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Quality test audio".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let audio = model.synthesize(&request).await.unwrap();
    
    // Basic quality checks
    assert!(!audio.samples.is_empty());
    
    // Check for clipping (samples should be in valid range)
    let max_sample = audio.samples.iter()
        .map(|s| s.abs())
        .fold(0.0f32, f32::max);
    
    assert!(max_sample <= 1.0, "Audio should not clip (max: {})", max_sample);
    
    // Check for silence (should have some non-zero samples)
    let non_zero_count = audio.samples.iter()
        .filter(|&&s| s.abs() > 0.001)
        .count();
    
    assert!(non_zero_count > 0, "Audio should not be completely silent");
}

#[tokio::test]
async fn test_xtts_streaming_structure() {
    let mut model = XttsModel::new();
    model.load("test_model").await.unwrap();
    
    let request = SynthesisRequest {
        text: "Streaming test".to_string(),
        voice_id: "dm".to_string(),
        speed: 1.0,
        pitch: 0.0,
    };
    
    let mut stream = model.synthesize_streaming(&request, 100).await; // 100ms chunks
    
    let mut total_samples = 0;
    let mut chunk_count = 0;
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.unwrap();
        total_samples += chunk.samples.len();
        chunk_count += 1;
        
        assert!(!chunk.samples.is_empty());
        assert_eq!(chunk.sample_rate, 24000); // Current stub uses 24kHz
    }
    
    assert!(chunk_count > 0, "Should produce at least one chunk");
    assert!(total_samples > 0, "Should produce samples");
}

#[tokio::test]
#[ignore] // Requires actual Coqui XTTS
async fn test_xtts_multilingual_support() {
    if !check_coqui_tts_available() {
        eprintln!("⚠️  Coqui TTS not available, skipping test");
        return;
    }
    
    // This test would verify multilingual support once Coqui XTTS is integrated
    // For now, it's a placeholder
    
    let languages = vec!["en", "pt", "es", "fr"];
    
    for lang in languages {
        // In real implementation, would test XTTS with different languages
        // Current stub doesn't support language parameter
        eprintln!("Would test language: {}", lang);
    }
}

#[tokio::test]
#[ignore] // Requires actual Coqui XTTS
async fn test_xtts_speaker_cloning() {
    if !check_coqui_tts_available() {
        eprintln!("⚠️  Coqui TTS not available, skipping test");
        return;
    }
    
    // This test would verify speaker cloning once Coqui XTTS is integrated
    // XTTS supports speaker cloning from reference audio
    eprintln!("Would test speaker cloning");
}

use futures::StreamExt;


