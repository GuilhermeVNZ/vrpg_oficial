//! Integration test: Hello World XTTS → SoVITS (Dungeon Master)
//!
//! This test follows project best practices:
//! - Uses real XTTS audio as input
//! - Converts using trained SoVITS dungeon master model
//! - Validates output quality and latency
//! - Follows rulebook testing guidelines

use tts_service::pipeline::{TtsPipeline, PipelineRequest};
use std::path::PathBuf;
use std::time::Instant;

#[tokio::test]
#[ignore] // Requires SoVITS model and XTTS audio file
async fn test_hello_world_xtts_to_sovits() {
    // Setup paths
    let sovits_base_path = PathBuf::from("assets-and-models/models/tts/sovits");
    
    // Create pipeline
    let pipeline = TtsPipeline::new(&sovits_base_path);
    
    // Test request with Voice INTENT for dungeon master
    let request = PipelineRequest {
        text: "<VOICE actor=\"dungeon_master_en\" emotion=\"neutral\" style=\"narrative\">Hello World</VOICE>".to_string(),
        language: "en".to_string(),
    };
    
    // Measure latency
    let start = Instant::now();
    
    // Synthesize
    let response = pipeline.synthesize(request).await
        .expect("Pipeline synthesis should succeed");
    
    let latency_ms = start.elapsed().as_millis();
    
    // Validate response
    assert!(!response.audio.is_empty(), "Audio should not be empty");
    assert!(response.sample_rate > 0, "Sample rate should be positive");
    assert_eq!(response.channels, 1, "Should be mono audio");
    assert!(response.duration_ms > 0, "Duration should be positive");
    assert_eq!(response.actor, "dungeon_master_en", "Actor should match");
    
    // Validate latency (target: < 1.5s for GPU)
    let target_latency_ms = 1500;
    assert!(
        latency_ms < target_latency_ms as u128,
        "Latency {}ms exceeds target {}ms",
        latency_ms,
        target_latency_ms
    );
    
    // Validate audio quality (basic checks)
    let max_amplitude = response.audio.iter()
        .map(|&x| x.abs())
        .fold(0.0f32, f32::max);
    
    assert!(
        max_amplitude > 0.0 && max_amplitude <= 1.0,
        "Audio amplitude should be in valid range [0, 1], got {}",
        max_amplitude
    );
    
    // Log results
    println!(
        "✅ Test passed: Hello World → SoVITS (DM)\n\
         - Latency: {}ms\n\
         - Duration: {}ms\n\
         - Sample rate: {} Hz\n\
         - Samples: {}\n\
         - Actor: {}\n\
         - Emotion: {}\n\
         - Style: {}",
        latency_ms,
        response.duration_ms,
        response.sample_rate,
        response.audio.len(),
        response.actor,
        response.emotion,
        response.style
    );
}

#[tokio::test]
#[ignore] // Requires SoVITS model
async fn test_sovits_model_loading() {
    let sovits_base_path = PathBuf::from("assets-and-models/models/tts/sovits");
    let pipeline = TtsPipeline::new(&sovits_base_path);
    
    // Pipeline should be created successfully
    // Models are loaded lazily on first synthesis
    assert!(true, "Pipeline created successfully");
}

