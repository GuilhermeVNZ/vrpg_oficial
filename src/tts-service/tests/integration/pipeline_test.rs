//! Integration tests for complete TTS pipeline

use tts_service::pipeline::{TtsPipeline, PipelineRequest};
use std::path::PathBuf;

#[tokio::test]
async fn test_pipeline_complete() {
    let pipeline = TtsPipeline::new(PathBuf::from("test_sovits").as_path());
    
    // Load Piper models
    pipeline.piper_pt.write().await.load("test_pt.onnx").await.unwrap();
    pipeline.piper_en.write().await.load("test_en.onnx").await.unwrap();
    
    // Test synthesis without SoVITS (fallback to Piper)
    let request = PipelineRequest {
        text: r#"<VOICE actor="NPC_Guard" emotion="skeptic" style="dry">
"Emissário? De qual reino?"
</VOICE>"#.to_string(),
        language: "pt".to_string(),
    };
    
    let result = pipeline.synthesize(request).await.unwrap();
    assert!(!result.audio.is_empty());
    assert_eq!(result.actor, "NPC_Guard");
    assert_eq!(result.emotion, "skeptic");
    assert_eq!(result.style, "dry");
}

#[tokio::test]
async fn test_pipeline_with_sovits() {
    let pipeline = TtsPipeline::new(PathBuf::from("test_sovits").as_path());
    
    // Load Piper models
    pipeline.piper_pt.write().await.load("test_pt.onnx").await.unwrap();
    pipeline.piper_en.write().await.load("test_en.onnx").await.unwrap();
    
    // Load SoVITS model
    pipeline.load_sovits_model("npc_guard", PathBuf::from("test_sovits/npc_guard.sovits").as_path()).await.unwrap();
    
    let request = PipelineRequest {
        text: r#"<VOICE actor="NPC_Guard" emotion="rage" style="crackled">
"TRAIDORES!"
</VOICE>"#.to_string(),
        language: "pt".to_string(),
    };
    
    let result = pipeline.synthesize(request).await.unwrap();
    assert!(!result.audio.is_empty());
    assert_eq!(result.actor, "NPC_Guard");
    assert_eq!(result.emotion, "rage");
}

#[tokio::test]
async fn test_pipeline_latency() {
    let pipeline = TtsPipeline::new(PathBuf::from("test_sovits").as_path());
    
    pipeline.piper_pt.write().await.load("test_pt.onnx").await.unwrap();
    pipeline.piper_en.write().await.load("test_en.onnx").await.unwrap();
    
    let request = PipelineRequest {
        text: "Teste rápido".to_string(),
        language: "pt".to_string(),
    };
    
    let start = std::time::Instant::now();
    let _result = pipeline.synthesize(request).await.unwrap();
    let duration = start.elapsed();
    
    // Total latency should be < 800ms (Qwen + Piper + SoVITS)
    // For now, without real models, we just check it's reasonable
    assert!(duration.as_millis() < 1000);
}

#[tokio::test]
async fn test_pipeline_multiple_characters() {
    let pipeline = TtsPipeline::new(PathBuf::from("test_sovits").as_path());
    
    pipeline.piper_pt.write().await.load("test_pt.onnx").await.unwrap();
    pipeline.piper_en.write().await.load("test_en.onnx").await.unwrap();
    
    // Load multiple SoVITS models
    pipeline.load_sovits_model("npc_guard", PathBuf::from("test_sovits/npc_guard.sovits").as_path()).await.unwrap();
    pipeline.load_sovits_model("npc_barkeep", PathBuf::from("test_sovits/npc_barkeep.sovits").as_path()).await.unwrap();
    
    let characters = pipeline.list_loaded_characters().await;
    assert!(characters.contains(&"npc_guard".to_string()));
    assert!(characters.contains(&"npc_barkeep".to_string()));
}

#[tokio::test]
async fn test_pipeline_fallback_to_piper() {
    let pipeline = TtsPipeline::new(PathBuf::from("test_sovits").as_path());
    
    pipeline.piper_pt.write().await.load("test_pt.onnx").await.unwrap();
    pipeline.piper_en.write().await.load("test_en.onnx").await.unwrap();
    
    // Don't load SoVITS model - should fallback to Piper
    let request = PipelineRequest {
        text: r#"<VOICE actor="Unknown_Character" emotion="neutral" style="neutral">
"Test"
</VOICE>"#.to_string(),
        language: "pt".to_string(),
    };
    
    let result = pipeline.synthesize(request).await.unwrap();
    // Should still work (fallback to Piper)
    assert!(!result.audio.is_empty());
}

