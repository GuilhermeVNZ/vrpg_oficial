# XTTS GPU Adaptive Control Specification

## Purpose

This specification defines adaptive GPU usage control for XTTS to prevent system overload on modest hardware while maintaining voice response performance. The system SHALL automatically detect hardware capabilities and adjust GPU usage accordingly, ensuring smooth operation on both high-end (RTX 5090) and modest GPUs.

## ADDED Requirements

### Requirement: Hardware Capability Detection

The TTS Service SHALL automatically detect GPU capabilities and classify hardware into performance tiers for adaptive configuration.

#### Scenario: GPU Capability Detection
Given TTS Service initialization
When detecting GPU capabilities
Then the TTS Service SHALL query GPU name, VRAM, and compute capability
And the TTS Service SHALL classify GPU into tier:
  - **High-End**: RTX 4090/5090, A100, H100 (32GB+ VRAM, compute 8.0+)
  - **Mid-Range**: RTX 3060/3070/3080, RTX 4060/4070 (8-16GB VRAM, compute 7.0+)
  - **Modest**: GTX 1660, RTX 3050, integrated GPUs (4-8GB VRAM, compute 6.0+)
  - **Low-End**: < 4GB VRAM or compute < 6.0
And the TTS Service SHALL store tier classification for runtime use

#### Scenario: CPU Fallback Detection
Given no GPU available or GPU below minimum requirements
When initializing TTS Service
Then the TTS Service SHALL detect CPU-only mode
And the TTS Service SHALL configure for CPU inference
And the TTS Service SHALL adjust latency expectations (3-30s per chunk)
And the TTS Service SHALL disable parallel inference

### Requirement: Adaptive GPU Usage Control

The TTS Service SHALL control GPU usage based on hardware tier, preventing system overload while maintaining voice response performance.

#### Scenario: High-End GPU Configuration (RTX 5090)
Given GPU classified as High-End (RTX 5090)
When configuring XTTS
Then the TTS Service SHALL enable parallel inference (2-3 CUDA streams)
And the TTS Service SHALL allow pre-buffering of 2-3 chunks
And the TTS Service SHALL target GPU utilization 80-95%
And the TTS Service SHALL maintain < 0.5x real-time factor
And the TTS Service SHALL NOT limit VRAM usage (use available)

#### Scenario: Mid-Range GPU Configuration (RTX 3070)
Given GPU classified as Mid-Range (RTX 3070, 8GB VRAM)
When configuring XTTS
Then the TTS Service SHALL enable limited parallel inference (1-2 CUDA streams)
And the TTS Service SHALL allow pre-buffering of 1-2 chunks
And the TTS Service SHALL target GPU utilization 60-80%
And the TTS Service SHALL limit VRAM usage to 6GB maximum
And the TTS Service SHALL maintain < 0.6x real-time factor

#### Scenario: Modest GPU Configuration (RTX 3050)
Given GPU classified as Modest (RTX 3050, 4-8GB VRAM)
When configuring XTTS
Then the TTS Service SHALL disable parallel inference (1 CUDA stream only)
And the TTS Service SHALL allow pre-buffering of 1 chunk only
And the TTS Service SHALL target GPU utilization 40-60%
And the TTS Service SHALL limit VRAM usage to 3GB maximum
And the TTS Service SHALL maintain < 0.8x real-time factor
And the TTS Service SHALL yield GPU to other processes

#### Scenario: Low-End GPU Configuration
Given GPU classified as Low-End (< 4GB VRAM)
When configuring XTTS
Then the TTS Service SHALL disable parallel inference
And the TTS Service SHALL use minimal pre-buffering (0.5-1.0s)
And the TTS Service SHALL target GPU utilization 30-50%
And the TTS Service SHALL limit VRAM usage to 2GB maximum
And the TTS Service SHALL allow CPU fallback for some operations
And the TTS Service SHALL maintain < 1.0x real-time factor

### Requirement: GPU Memory Management

The TTS Service SHALL manage GPU memory to prevent OOM (Out of Memory) errors and system slowdown.

#### Scenario: VRAM Limit Enforcement
Given configured VRAM limit (e.g., 3GB for modest GPU)
When allocating GPU memory
Then the TTS Service SHALL track VRAM usage
And the TTS Service SHALL NOT exceed configured limit
And the TTS Service SHALL free unused memory immediately
And the TTS Service SHALL clear CUDA cache if approaching limit

#### Scenario: GPU Memory Monitoring
Given TTS Service running
When monitoring GPU memory
Then the TTS Service SHALL track allocated VRAM
And the TTS Service SHALL track reserved VRAM
And the TTS Service SHALL log warnings if usage > 80% of limit
And the TTS Service SHALL trigger cleanup if usage > 90% of limit

#### Scenario: Memory Cleanup
Given GPU memory usage approaching limit
When cleanup is triggered
Then the TTS Service SHALL clear CUDA cache
And the TTS Service SHALL free completed chunk buffers
And the TTS Service SHALL reduce pre-buffer size if needed
And the TTS Service SHALL maintain minimum buffer for playback

### Requirement: Sequential vs Parallel Inference Control

The TTS Service SHALL control whether XTTS uses sequential or parallel inference based on hardware tier.

#### Scenario: High-End Parallel Inference
Given High-End GPU (RTX 5090)
When generating multiple chunks
Then the TTS Service SHALL use 2-3 CUDA streams in parallel
And the TTS Service SHALL generate chunk N+1 and N+2 simultaneously
And the TTS Service SHALL maximize GPU utilization
And the TTS Service SHALL NOT serialize chunk generation

#### Scenario: Modest Sequential Inference
Given Modest GPU (RTX 3050)
When generating multiple chunks
Then the TTS Service SHALL use 1 CUDA stream only (sequential)
And the TTS Service SHALL generate chunks one at a time
And the TTS Service SHALL yield GPU between chunks
And the TTS Service SHALL prevent GPU saturation

#### Scenario: CPU Fallback for Some Operations
Given Low-End GPU or CPU-only mode
When generating chunks
Then the TTS Service SHALL use CPU for preprocessing
And the TTS Service SHALL use GPU only for core inference (if available)
And the TTS Service SHALL use CPU for post-processing
And the TTS Service SHALL balance CPU/GPU workload

### Requirement: Pre-Buffer Size Adaptation

The TTS Service SHALL adapt pre-buffer size based on hardware capabilities to prevent system overload.

#### Scenario: High-End Pre-Buffering
Given High-End GPU
When configuring pre-buffer
Then the TTS Service SHALL use 2.0-3.0 seconds pre-buffer
And the TTS Service SHALL maintain 2-3 chunks ahead
And the TTS Service SHALL NOT limit buffer size

#### Scenario: Modest Pre-Buffering
Given Modest GPU
When configuring pre-buffer
Then the TTS Service SHALL use 1.0-1.5 seconds pre-buffer
And the TTS Service SHALL maintain 1 chunk ahead
And the TTS Service SHALL reduce buffer if VRAM pressure

#### Scenario: Low-End Pre-Buffering
Given Low-End GPU
When configuring pre-buffer
Then the TTS Service SHALL use 0.5-1.0 seconds pre-buffer
And the TTS Service SHALL maintain minimal buffer
And the TTS Service SHALL start playback earlier (lower threshold)

### Requirement: GPU Yield to System

The TTS Service SHALL yield GPU resources to other processes on modest hardware to prevent system slowdown.

#### Scenario: GPU Yield Between Chunks
Given Modest GPU configuration
When generating chunks
Then the TTS Service SHALL yield GPU between chunks (small delay)
And the TTS Service SHALL allow other processes to use GPU
And the TTS Service SHALL NOT monopolize GPU resources
And the TTS Service SHALL maintain acceptable latency (< 1.0x RTF)

#### Scenario: Adaptive Yield Based on System Load
Given system GPU load monitoring
When system GPU load > 80%
Then the TTS Service SHALL reduce parallel streams
And the TTS Service SHALL increase yield time between chunks
And the TTS Service SHALL reduce pre-buffer size
And the TTS Service SHALL maintain voice response performance

### Requirement: Performance Profile Configuration

The TTS Service SHALL support manual performance profile selection for fine-tuning.

#### Scenario: Performance Profile Selection
Given TTS Service configuration
When user selects performance profile
Then the TTS Service SHALL support profiles:
  - **"high_performance"**: Maximum GPU usage, parallel inference, large buffers
  - **"balanced"**: Moderate GPU usage, limited parallel, medium buffers
  - **"modest"**: Minimal GPU usage, sequential inference, small buffers
  - **"auto"**: Auto-detect based on hardware (default)
And the TTS Service SHALL apply profile settings
And the TTS Service SHALL override hardware detection if profile specified

#### Scenario: Environment Variable Override
Given environment variables set
When TTS Service initializes
Then the TTS Service SHALL respect:
  - `VRPG_XTTS_GPU_STREAMS`: Number of parallel CUDA streams (0-3)
  - `VRPG_XTTS_GPU_VRAM_LIMIT_MB`: VRAM limit in MB
  - `VRPG_XTTS_GPU_UTILIZATION_TARGET`: Target GPU utilization (0.3-0.95)
  - `VRPG_XTTS_PREBUFFER_SECONDS`: Pre-buffer size in seconds
And the TTS Service SHALL override auto-detection if variables set

## MODIFIED Requirements

### Requirement: XTTS Streaming Worker (Updated)

The XTTS Streaming Worker SHALL adapt parallel inference based on hardware tier and configuration, NOT always use maximum parallelization.

#### Scenario: Adaptive Parallel Inference
Given hardware tier and configuration
When generating chunks
Then the TTS Service SHALL use parallel streams only if:
  - GPU tier is High-End OR
  - `VRPG_XTTS_GPU_STREAMS` > 1 OR
  - Performance profile is "high_performance"
And the TTS Service SHALL use sequential inference if:
  - GPU tier is Modest/Low-End OR
  - VRAM pressure detected OR
  - System GPU load > 80%

## Technical Constraints

### GPU Usage Limits by Tier

| Tier | VRAM Limit | Parallel Streams | Pre-Buffer | GPU Utilization | RTF Target |
|------|------------|-----------------|------------|-----------------|------------|
| High-End | Unlimited | 2-3 | 2.0-3.0s | 80-95% | < 0.5x |
| Mid-Range | 6GB | 1-2 | 1.5-2.0s | 60-80% | < 0.6x |
| Modest | 3GB | 1 | 1.0-1.5s | 40-60% | < 0.8x |
| Low-End | 2GB | 0-1 | 0.5-1.0s | 30-50% | < 1.0x |

### Performance Targets (Maintained Across Tiers)

- **Initial latency**: 2.5-4.0s (all tiers)
- **Streaming continuity**: Zero gaps (all tiers)
- **Voice response**: Maintained (all tiers)
- **System responsiveness**: No slowdown (all tiers)

### Environment Variables

```bash
# GPU Control
VRPG_XTTS_GPU_STREAMS=1              # 0=CPU, 1=Sequential, 2-3=Parallel
VRPG_XTTS_GPU_VRAM_LIMIT_MB=3072     # VRAM limit in MB (0=unlimited)
VRPG_XTTS_GPU_UTILIZATION_TARGET=0.6 # Target GPU utilization (0.3-0.95)
VRPG_XTTS_PREBUFFER_SECONDS=1.5      # Pre-buffer size in seconds
VRPG_XTTS_PERFORMANCE_PROFILE=auto    # auto|high_performance|balanced|modest
```

## Implementation Architecture

### Component: GPU Capability Detector

```rust
struct GpuCapabilityDetector {
    gpu_name: String,
    vram_total_gb: f32,
    compute_capability: (u32, u32),
    tier: GpuTier,
}

enum GpuTier {
    HighEnd,    // RTX 4090/5090, A100, H100
    MidRange,   // RTX 3060-3080, RTX 4060-4070
    Modest,     // RTX 3050, GTX 1660
    LowEnd,     // < 4GB VRAM
}

impl GpuCapabilityDetector {
    fn detect() -> Self {
        // Query GPU capabilities
        // Classify into tier
        // Return detector
    }
    
    fn get_config(&self) -> GpuConfig {
        // Return configuration based on tier
    }
}
```

### Component: Adaptive GPU Config

```rust
struct GpuConfig {
    max_parallel_streams: usize,  // 0-3
    vram_limit_mb: usize,          // 0 = unlimited
    utilization_target: f32,      // 0.3-0.95
    prebuffer_seconds: f32,       // 0.5-3.0
    yield_between_chunks: bool,    // Yield GPU on modest hardware
    cpu_fallback_enabled: bool,    // Allow CPU for some ops
}
```

### Component: GPU Usage Monitor

```rust
struct GpuUsageMonitor {
    vram_allocated: f32,
    vram_limit: f32,
    utilization_current: f32,
    utilization_target: f32,
}

impl GpuUsageMonitor {
    fn check_vram_pressure(&self) -> bool {
        // Check if VRAM usage > 80% of limit
    }
    
    fn should_reduce_usage(&self) -> bool {
        // Check if should reduce parallel streams
    }
    
    fn cleanup_if_needed(&self) {
        // Clear CUDA cache if needed
    }
}
```

## Testing Requirements

### Unit Tests
- GPU capability detection (100% coverage)
- Tier classification (all tiers)
- Config generation per tier (100% coverage)
- VRAM limit enforcement (100% coverage)

### Integration Tests
- High-End GPU configuration (RTX 5090)
- Mid-Range GPU configuration (RTX 3070)
- Modest GPU configuration (RTX 3050)
- CPU fallback mode
- Environment variable overrides

### Performance Tests
- GPU utilization measurement (verify targets)
- VRAM usage measurement (verify limits)
- System responsiveness (no slowdown)
- Voice latency (maintain < 4.0s initial)

## Migration Notes

### Breaking Changes
- GPU usage now adaptive (not always maximum)
- Parallel inference disabled by default on modest hardware
- Pre-buffer size now tier-dependent

### Migration Path
1. Implement GPU capability detection
2. Implement adaptive configuration
3. Add environment variable support
4. Test on different hardware tiers
5. Document configuration options
6. Update default settings

## References

- XTTS Real-Time Factor: 0.4x (advantage to stay ahead)
- GPU Tiers: Based on VRAM and compute capability
- Performance Target: Maintain voice response < 4.0s on all tiers
- System Goal: No slowdown on modest hardware



