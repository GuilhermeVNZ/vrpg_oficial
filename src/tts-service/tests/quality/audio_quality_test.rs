//! Quality tests for audio output

#[cfg(test)]
mod tests {
    use super::super::super::streaming_pipeline::*;

    #[tokio::test]
    async fn test_raw_quality_preservation() {
        // Given: XTTS generated audio (RAW)
        let config = PipelineConfig {
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "Test audio quality preservation.";
        
        // When: Processing through pipeline
        let result = pipeline.process_text(text).await;
        
        // Then: Audio quality SHOULD be preserved
        assert!(result.quality_score >= 0.95, 
               "Quality score {:.2} below threshold 0.95", result.quality_score);
        
        // And: SHOULD not have metallic artifacts
        assert!(!result.has_metallic_artifacts);
        
        // And: SHOULD not have distortion
        assert!(!result.has_distortion);
        
        // And: SHOULD maintain natural voice quality
        assert!(result.natural_voice_quality);
    }

    #[tokio::test]
    async fn test_format_conversion_quality() {
        // Given: Float32 audio from XTTS
        let config = PipelineConfig {
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "Test format conversion quality.";
        
        // When: Converting to int16 for I/O
        let result = pipeline.process_text(text).await;
        
        // Then: Audio quality SHOULD be preserved
        assert!(result.format_conversion_quality >= 0.98);
        
        // And: SHOULD not introduce artifacts
        assert!(!result.has_conversion_artifacts);
        
        // And: SHOULD be sufficient for voice
        assert!(result.sufficient_for_voice);
    }

    #[tokio::test]
    async fn test_zero_gap_playback() {
        // Given: Multiple audio chunks
        let config = PipelineConfig {
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "First chunk. Second chunk. Third chunk.";
        
        // When: Playing chunks sequentially
        let result = pipeline.process_text(text).await;
        
        // Then: SHOULD have zero gaps between chunks
        assert_eq!(result.audio_gaps_ms.len(), 0);
        
        // And: SHOULD maintain audio continuity
        assert!(result.audio_continuous);
        
        // And: SHOULD not have clicks or pops
        assert!(!result.has_clicks_or_pops);
    }
}



