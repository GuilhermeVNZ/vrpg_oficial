# Benchmark Tests

## Purpose

Benchmark tests help identify the optimal pipeline configuration for lowest latency while maintaining quality.

## Running Benchmarks

### Quick Benchmark (CI/CD)
```bash
cargo test --test benchmark_suite quick_benchmark -- --nocapture
```

### Full Benchmark Suite
```bash
cargo test --test benchmark_suite run_full_benchmark_suite -- --ignored --nocapture
```

This will:
1. Test all configuration combinations
2. Generate a detailed report
3. Save results to `benchmark_report.txt`
4. Identify the best configuration

## Benchmark Configurations

The suite tests:
- **GPU Tiers**: High-End, Mid-Range, Modest, Low-End, CPU-Only
- **Parallel Streams**: 0-3
- **Pre-buffer Sizes**: 0.5s - 3.0s
- **Buffer Sizes**: 256, 384, 512 frames
- **Sample Rates**: 16kHz, 24kHz
- **Time-Stretch**: Enabled/Disabled

## Interpreting Results

### Key Metrics
- **Initial Latency**: Time to first audio (target: < 4.0s)
- **Real-Time Factor**: Generation speed vs playback (target: < 0.5x High-End)
- **GPU Utilization**: GPU usage percentage (target: tier-dependent)
- **Buffer Underruns**: Number of playback interruptions (target: 0)
- **Quality Score**: Audio quality preservation (target: > 0.95)

### Best Configuration Selection

The benchmark automatically:
1. Sorts configurations by initial latency
2. Identifies the best configuration
3. Validates it meets all targets
4. Reports detailed metrics

## Example Output

```
=== Pipeline Latency Benchmark Report ===

Configuration #1: High-End: Minimal Latency
  Initial Latency: 2850.00ms
  Real-Time Factor: 0.42x
  GPU Utilization: 87.5%
  Buffer Underruns: 0
  Quality Score: 0.98

🏆 Best Configuration: High-End: Minimal Latency
   Initial Latency: 2850.00ms
   Real-Time Factor: 0.42x
```



