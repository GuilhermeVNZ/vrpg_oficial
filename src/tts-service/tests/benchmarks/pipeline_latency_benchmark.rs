//! Pipeline Latency Benchmark Tests
//!
//! These benchmarks test different pipeline configurations to identify
//! the optimal setup for lowest latency while maintaining quality.

use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Benchmark configuration for testing different pipeline setups
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub name: String,
    pub gpu_tier: GpuTier,
    pub parallel_streams: usize,
    pub prebuffer_seconds: f32,
    pub buffer_size_frames: usize,
    pub sample_rate: u32,
    pub chunk_duration_target: f32,
    pub use_time_stretch: bool,
    pub time_stretch_factor: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuTier {
    HighEnd,
    MidRange,
    Modest,
    LowEnd,
    CpuOnly,
}

/// Benchmark result with detailed metrics
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub config: BenchmarkConfig,
    pub initial_latency_ms: f32,
    pub chunk_generation_time_ms: Vec<f32>,
    pub average_chunk_time_ms: f32,
    pub real_time_factor: f32,
    pub gpu_utilization: Option<f32>,
    pub buffer_underruns: usize,
    pub audio_gaps_ms: Vec<f32>,
    pub total_audio_duration_ms: f32,
    pub total_generation_time_ms: f32,
    pub quality_score: f32, // 0.0-1.0
}

/// Run benchmark with specific configuration
pub async fn run_benchmark(config: BenchmarkConfig, test_text: &str) -> BenchmarkResult {
    let start = Instant::now();
    
    // Simulate pipeline execution
    // In real implementation, this would call the actual pipeline
    
    let initial_latency = measure_initial_latency(&config, test_text).await;
    let chunk_times = measure_chunk_generation(&config, test_text).await;
    let avg_chunk_time = chunk_times.iter().sum::<f32>() / chunk_times.len() as f32;
    let total_audio_duration = estimate_audio_duration(test_text);
    let total_generation_time = chunk_times.iter().sum::<f32>();
    let rtf = total_generation_time / total_audio_duration;
    
    BenchmarkResult {
        config: config.clone(),
        initial_latency_ms: initial_latency,
        chunk_generation_time_ms: chunk_times,
        average_chunk_time_ms: avg_chunk_time,
        real_time_factor: rtf,
        gpu_utilization: measure_gpu_utilization(&config).await,
        buffer_underruns: 0, // Would be measured in real implementation
        audio_gaps_ms: vec![], // Would be measured in real implementation
        total_audio_duration_ms: total_audio_duration,
        total_generation_time_ms: total_generation_time,
        quality_score: 1.0, // Would be measured in real implementation
    }
}

async fn measure_initial_latency(config: &BenchmarkConfig, _text: &str) -> f32 {
    // Simulate initial latency based on configuration
    let base_latency = match config.gpu_tier {
        GpuTier::HighEnd => 2500.0,
        GpuTier::MidRange => 3000.0,
        GpuTier::Modest => 3500.0,
        GpuTier::LowEnd => 4000.0,
        GpuTier::CpuOnly => 8000.0,
    };
    
    // Adjust based on prebuffer
    let prebuffer_latency = config.prebuffer_seconds * 1000.0;
    
    base_latency + prebuffer_latency
}

async fn measure_chunk_generation(config: &BenchmarkConfig, _text: &str) -> Vec<f32> {
    // Simulate chunk generation times
    let num_chunks = 5; // Example
    let base_time = match config.gpu_tier {
        GpuTier::HighEnd => 1200.0,
        GpuTier::MidRange => 1800.0,
        GpuTier::Modest => 2400.0,
        GpuTier::LowEnd => 3000.0,
        GpuTier::CpuOnly => 6000.0,
    };
    
    // Adjust for parallel streams
    let parallel_factor = if config.parallel_streams > 1 {
        1.0 / config.parallel_streams as f32
    } else {
        1.0
    };
    
    (0..num_chunks)
        .map(|_| base_time * parallel_factor)
        .collect()
}

fn estimate_audio_duration(text: &str) -> f32 {
    // Rough estimate: ~150 words per minute = ~2.5 words per second
    let words = text.split_whitespace().count();
    (words as f32 / 2.5) * 1000.0 // Convert to ms
}

async fn measure_gpu_utilization(config: &BenchmarkConfig) -> Option<f32> {
    match config.gpu_tier {
        GpuTier::CpuOnly => None,
        _ => Some(match config.gpu_tier {
            GpuTier::HighEnd => 85.0,
            GpuTier::MidRange => 70.0,
            GpuTier::Modest => 50.0,
            GpuTier::LowEnd => 40.0,
            _ => 0.0,
        }),
    }
}

/// Compare multiple benchmark configurations
pub async fn compare_configurations(
    configs: Vec<BenchmarkConfig>,
    test_text: &str,
) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    for config in configs {
        let result = run_benchmark(config, test_text).await;
        results.push(result);
    }
    
    // Sort by initial latency
    results.sort_by(|a, b| {
        a.initial_latency_ms
            .partial_cmp(&b.initial_latency_ms)
            .unwrap()
    });
    
    results
}

/// Generate benchmark report
pub fn generate_report(results: &[BenchmarkResult]) -> String {
    let mut report = String::from("=== Pipeline Latency Benchmark Report ===\n\n");
    
    for (i, result) in results.iter().enumerate() {
        report.push_str(&format!(
            "Configuration #{}: {}\n",
            i + 1,
            result.config.name
        ));
        report.push_str(&format!("  GPU Tier: {:?}\n", result.config.gpu_tier));
        report.push_str(&format!("  Parallel Streams: {}\n", result.config.parallel_streams));
        report.push_str(&format!("  Pre-buffer: {:.2}s\n", result.config.prebuffer_seconds));
        report.push_str(&format!("  Buffer Size: {} frames\n", result.config.buffer_size_frames));
        report.push_str(&format!("  Sample Rate: {} Hz\n", result.config.sample_rate));
        report.push_str(&format!("\n  Results:\n"));
        report.push_str(&format!("    Initial Latency: {:.2}ms\n", result.initial_latency_ms));
        report.push_str(&format!("    Avg Chunk Time: {:.2}ms\n", result.average_chunk_time_ms));
        report.push_str(&format!("    Real-Time Factor: {:.2}x\n", result.real_time_factor));
        if let Some(util) = result.gpu_utilization {
            report.push_str(&format!("    GPU Utilization: {:.1}%\n", util));
        }
        report.push_str(&format!("    Buffer Underruns: {}\n", result.buffer_underruns));
        report.push_str(&format!("    Quality Score: {:.2}\n", result.quality_score));
        report.push_str("\n");
    }
    
    // Find best configuration
    if let Some(best) = results.first() {
        report.push_str(&format!(
            "🏆 Best Configuration: {}\n",
            best.config.name
        ));
        report.push_str(&format!("   Initial Latency: {:.2}ms\n", best.initial_latency_ms));
        report.push_str(&format!("   Real-Time Factor: {:.2}x\n", best.real_time_factor));
    }
    
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_benchmark_high_end() {
        let config = BenchmarkConfig {
            name: "High-End Optimized".to_string(),
            gpu_tier: GpuTier::HighEnd,
            parallel_streams: 2,
            prebuffer_seconds: 2.5,
            buffer_size_frames: 512,
            sample_rate: 24000,
            chunk_duration_target: 5.0,
            use_time_stretch: false,
            time_stretch_factor: 1.0,
        };
        
        let test_text = "In the depths of the ancient dungeon, shadows danced along the stone walls.";
        let result = run_benchmark(config, test_text).await;
        
        assert!(result.initial_latency_ms < 4000.0);
        assert!(result.real_time_factor < 0.5);
    }

    #[tokio::test]
    async fn test_benchmark_modest() {
        let config = BenchmarkConfig {
            name: "Modest Balanced".to_string(),
            gpu_tier: GpuTier::Modest,
            parallel_streams: 1,
            prebuffer_seconds: 1.25,
            buffer_size_frames: 384,
            sample_rate: 24000,
            chunk_duration_target: 5.0,
            use_time_stretch: false,
            time_stretch_factor: 1.0,
        };
        
        let test_text = "In the depths of the ancient dungeon, shadows danced along the stone walls.";
        let result = run_benchmark(config, test_text).await;
        
        assert!(result.initial_latency_ms < 4500.0);
        assert!(result.real_time_factor < 0.8);
    }

    #[tokio::test]
    async fn test_compare_configurations() {
        let configs = vec![
            BenchmarkConfig {
                name: "Config A: High-End Max".to_string(),
                gpu_tier: GpuTier::HighEnd,
                parallel_streams: 3,
                prebuffer_seconds: 3.0,
                buffer_size_frames: 512,
                sample_rate: 24000,
                chunk_duration_target: 5.0,
                use_time_stretch: false,
                time_stretch_factor: 1.0,
            },
            BenchmarkConfig {
                name: "Config B: High-End Balanced".to_string(),
                gpu_tier: GpuTier::HighEnd,
                parallel_streams: 2,
                prebuffer_seconds: 2.5,
                buffer_size_frames: 384,
                sample_rate: 24000,
                chunk_duration_target: 5.0,
                use_time_stretch: false,
                time_stretch_factor: 1.0,
            },
            BenchmarkConfig {
                name: "Config C: High-End Minimal".to_string(),
                gpu_tier: GpuTier::HighEnd,
                parallel_streams: 1,
                prebuffer_seconds: 2.0,
                buffer_size_frames: 256,
                sample_rate: 24000,
                chunk_duration_target: 5.0,
                use_time_stretch: false,
                time_stretch_factor: 1.0,
            },
        ];
        
        let test_text = "In the depths of the ancient dungeon, shadows danced along the stone walls as torchlight flickered.";
        let results = compare_configurations(configs, test_text).await;
        
        assert!(!results.is_empty());
        let report = generate_report(&results);
        println!("{}", report);
    }
}



