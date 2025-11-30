//! Comprehensive Benchmark Suite
//!
//! Tests different pipeline configurations to find optimal latency setup.

use crate::benchmarks::pipeline_latency_benchmark::*;

/// Standard test text for benchmarks
const BENCHMARK_TEXT: &str = "In the depths of the ancient dungeon, shadows danced along the stone walls as torchlight flickered. The air was thick with the scent of damp earth and something else—something that made the hairs on the back of your neck stand on end.";

/// Generate all benchmark configurations to test
pub fn generate_all_configurations() -> Vec<BenchmarkConfig> {
    let mut configs = Vec::new();
    
    // High-End GPU Configurations
    configs.push(BenchmarkConfig {
        name: "High-End: Max Parallel (3 streams, 3.0s buffer)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 3,
        prebuffer_seconds: 3.0,
        buffer_size_frames: 512,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "High-End: Balanced (2 streams, 2.5s buffer)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 384,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "High-End: Minimal Latency (1 stream, 2.0s buffer, 256 frames)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 1,
        prebuffer_seconds: 2.0,
        buffer_size_frames: 256,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "High-End: Time-Stretch (2 streams, 2.5s buffer, 1.1x stretch)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 384,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: true,
        time_stretch_factor: 1.1,
    });
    
    // Mid-Range GPU Configurations
    configs.push(BenchmarkConfig {
        name: "Mid-Range: Balanced (1-2 streams, 1.75s buffer)".to_string(),
        gpu_tier: GpuTier::MidRange,
        parallel_streams: 1,
        prebuffer_seconds: 1.75,
        buffer_size_frames: 384,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    // Modest GPU Configurations
    configs.push(BenchmarkConfig {
        name: "Modest: Sequential (1 stream, 1.25s buffer)".to_string(),
        gpu_tier: GpuTier::Modest,
        parallel_streams: 1,
        prebuffer_seconds: 1.25,
        buffer_size_frames: 384,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "Modest: Minimal Buffer (1 stream, 1.0s buffer, 256 frames)".to_string(),
        gpu_tier: GpuTier::Modest,
        parallel_streams: 1,
        prebuffer_seconds: 1.0,
        buffer_size_frames: 256,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    // Low-End GPU Configurations
    configs.push(BenchmarkConfig {
        name: "Low-End: Minimal (0-1 stream, 0.75s buffer)".to_string(),
        gpu_tier: GpuTier::LowEnd,
        parallel_streams: 0,
        prebuffer_seconds: 0.75,
        buffer_size_frames: 256,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    // CPU-Only Configurations
    configs.push(BenchmarkConfig {
        name: "CPU-Only: Fallback (0 streams, 0.5s buffer)".to_string(),
        gpu_tier: GpuTier::CpuOnly,
        parallel_streams: 0,
        prebuffer_seconds: 0.5,
        buffer_size_frames: 256,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    // Buffer Size Variations (High-End)
    configs.push(BenchmarkConfig {
        name: "High-End: Small Buffer (2 streams, 2.5s buffer, 256 frames)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 256,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "High-End: Medium Buffer (2 streams, 2.5s buffer, 384 frames)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 384,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "High-End: Large Buffer (2 streams, 2.5s buffer, 512 frames)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 512,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    // Sample Rate Variations
    configs.push(BenchmarkConfig {
        name: "High-End: 16kHz (2 streams, 2.5s buffer, 16kHz)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 384,
        sample_rate: 16000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs.push(BenchmarkConfig {
        name: "High-End: 24kHz (2 streams, 2.5s buffer, 24kHz)".to_string(),
        gpu_tier: GpuTier::HighEnd,
        parallel_streams: 2,
        prebuffer_seconds: 2.5,
        buffer_size_frames: 384,
        sample_rate: 24000,
        chunk_duration_target: 5.0,
        use_time_stretch: false,
        time_stretch_factor: 1.0,
    });
    
    configs
}

/// Run full benchmark suite
#[tokio::test]
#[ignore] // Only run when explicitly requested
async fn run_full_benchmark_suite() {
    let configs = generate_all_configurations();
    let results = compare_configurations(configs, BENCHMARK_TEXT).await;
    
    let report = generate_report(&results);
    println!("{}", report);
    
    // Save report to file
    std::fs::write("benchmark_report.txt", &report).unwrap();
    
    // Assert best configuration meets targets
    if let Some(best) = results.first() {
        assert!(
            best.initial_latency_ms < 4000.0,
            "Best configuration should have initial latency < 4000ms, got {}ms",
            best.initial_latency_ms
        );
        assert!(
            best.real_time_factor < 0.5,
            "Best configuration should have RTF < 0.5x, got {:.2}x",
            best.real_time_factor
        );
    }
}

/// Quick benchmark for CI/CD
#[tokio::test]
async fn quick_benchmark() {
    let configs = vec![
        BenchmarkConfig {
            name: "Quick: High-End Balanced".to_string(),
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
            name: "Quick: Modest Balanced".to_string(),
            gpu_tier: GpuTier::Modest,
            parallel_streams: 1,
            prebuffer_seconds: 1.25,
            buffer_size_frames: 384,
            sample_rate: 24000,
            chunk_duration_target: 5.0,
            use_time_stretch: false,
            time_stretch_factor: 1.0,
        },
    ];
    
    let results = compare_configurations(configs, BENCHMARK_TEXT).await;
    
    for result in &results {
        println!("{}: {:.2}ms initial latency, {:.2}x RTF", 
            result.config.name, 
            result.initial_latency_ms,
            result.real_time_factor
        );
    }
}



