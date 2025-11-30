//! Performance tests for latency measurement

#[cfg(test)]
mod tests {
    use super::super::super::streaming_pipeline::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_initial_latency_high_end() {
        // Given: High-End GPU configuration
        let config = PipelineConfig {
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "Test text for latency measurement.";
        
        // When: Starting audio playback
        let start = Instant::now();
        let result = pipeline.process_text(text).await;
        let latency = start.elapsed().as_millis() as f32;
        
        // Then: Initial latency SHOULD be < 3.8s
        assert!(latency < 3800.0, 
               "Initial latency {}ms exceeds target 3800ms", latency);
        
        // And: SHOULD be measured and logged
        println!("Initial latency: {:.2}ms", latency);
    }

    #[tokio::test]
    async fn test_initial_latency_modest() {
        // Given: Modest GPU configuration
        let config = PipelineConfig {
            gpu_tier: GpuTier::Modest,
            parallel_streams: 1,
            prebuffer_seconds: 1.25,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "Test text for latency measurement.";
        
        // When: Starting audio playback
        let start = Instant::now();
        let result = pipeline.process_text(text).await;
        let latency = start.elapsed().as_millis() as f32;
        
        // Then: Initial latency SHOULD be < 4.5s
        assert!(latency < 4500.0, 
               "Initial latency {}ms exceeds target 4500ms", latency);
        
        // And: SHOULD be measured and logged
        println!("Initial latency: {:.2}ms", latency);
    }

    #[tokio::test]
    async fn test_chunk_generation_latency() {
        // Given: Pipeline configuration
        let config = PipelineConfig {
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "Chunk one. Chunk two. Chunk three.";
        
        // When: Generating chunks
        let result = pipeline.process_text(text).await;
        
        // Then: Each chunk SHOULD meet latency targets
        for (i, chunk_time) in result.chunk_generation_times.iter().enumerate() {
            assert!(*chunk_time < 3000.0, 
                   "Chunk {} generation time {}ms exceeds target", i, chunk_time);
        }
    }

    #[tokio::test]
    async fn test_real_time_factor() {
        // Given: Audio generation
        let config = PipelineConfig {
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 384,
            sample_rate: 24000,
        };
        
        let pipeline = StreamingPipeline::new(config).await;
        let text = "Test text for RTF measurement.";
        
        // When: Measuring RTF
        let result = pipeline.process_text(text).await;
        
        // Then: High-End SHOULD have RTF < 0.5x
        assert!(result.real_time_factor < 0.5, 
               "RTF {:.2}x exceeds target 0.5x", result.real_time_factor);
    }
}



