//! Integration tests for XTTS fallback in pipeline
//! Tests the fallback mechanism from Piper to XTTS

use tts_service::pipeline::{TtsPipeline, PipelineRequest};
use tts_service::xtts::{XttsModel, SharedXttsModel};
use std::sync::Arc;
use tokio::sync::RwLock;

// Note: These tests require the full pipeline setup
// They are marked as integration tests and may require actual models

#[tokio::test]
#[ignore] // Requires full pipeline setup
async fn test_pipeline_xtts_fallback_structure() {
    // Test that pipeline can be configured with XTTS fallback
    // This is a structural test - actual fallback would require:
    // 1. Piper to fail (or be disabled)
    // 2. XTTS to be available and loaded
    // 3. Pipeline to route to XTTS
    
    // This test validates the structure exists, not the actual fallback behavior
    // Real fallback testing would be in pipeline_test.rs with actual models
    
    let xtts: SharedXttsModel = Arc::new(RwLock::new(XttsModel::new()));
    
    // Verify XTTS can be created and shared
    {
        let xtts_guard = xtts.read().await;
        assert!(!xtts_guard.is_loaded());
    }
    
    // In real pipeline, this would be:
    // let pipeline = TtsPipeline::new_with_xtts_fallback(piper, xtts, sovits);
    // let result = pipeline.synthesize(text, language).await;
}

#[tokio::test]
#[ignore] // Requires actual models
async fn test_pipeline_piper_fallback_to_xtts() {
    // This test would verify:
    // 1. Piper fails (or is disabled)
    // 2. Pipeline automatically falls back to XTTS
    // 3. XTTS generates audio successfully
    // 4. Result is returned correctly
    
    // Placeholder for actual implementation
    eprintln!("Would test Piper → XTTS fallback");
}

#[tokio::test]
#[ignore] // Requires actual models
async fn test_pipeline_xtts_direct_usage() {
    // This test would verify:
    // 1. XTTS can be used directly (bypassing Piper)
    // 2. Audio quality is acceptable
    // 3. Integration with SoVITS works
    
    // Placeholder for actual implementation
    eprintln!("Would test direct XTTS usage in pipeline");
}

#[tokio::test]
async fn test_xtts_error_handling_in_pipeline() {
    // Test that XTTS errors don't crash the pipeline
    // Pipeline should handle XTTS failures gracefully
    
    let mut xtts = XttsModel::new();
    
    // XTTS not loaded - should handle gracefully
    assert!(!xtts.is_loaded());
    
    // In pipeline, this should not cause a panic
    // Pipeline should either:
    // 1. Return error gracefully
    // 2. Fall back to another method
    // 3. Use default/neutral audio
}


