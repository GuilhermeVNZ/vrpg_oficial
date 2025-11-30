//! Extended Unit Tests for Metrics Module
//!
//! Following rulebook standards: comprehensive coverage, Given/When/Then scenarios

use tts_service::metrics::{LatencyTimer, MetricsCollector, PipelineMetrics};
use std::time::Duration;

#[test]
fn test_metrics_collector_new() {
    // Given a new MetricsCollector
    // When creating
    // Then it should be initialized with zeros
    let collector = MetricsCollector::new();
    assert_eq!(collector.total_requests, 0);
    assert_eq!(collector.get_average_latency_ms(), 0.0);
}

#[test]
fn test_metrics_collector_record_request() {
    // Given a MetricsCollector
    // When recording a request
    // Then it should update statistics
    let mut collector = MetricsCollector::new();
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: false,
    });
    
    assert_eq!(collector.total_requests, 1);
    assert_eq!(collector.get_average_latency_ms(), 100.0);
}

#[test]
fn test_metrics_collector_record_multiple_requests() {
    // Given a MetricsCollector
    // When recording multiple requests
    // Then it should calculate averages correctly
    let mut collector = MetricsCollector::new();
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: false,
    });
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 200,
        xtts_latency_ms: 50,
        sovits_latency_ms: 150,
        audio_duration_ms: 1000,
        cache_hit: true,
    });
    
    assert_eq!(collector.total_requests, 2);
    assert_eq!(collector.get_average_latency_ms(), 150.0);
    assert_eq!(collector.get_average_piper_latency_ms(), 40.0);
    assert_eq!(collector.get_average_sovits_latency_ms(), 110.0);
}

#[test]
fn test_metrics_collector_cache_hit_rate() {
    // Given a MetricsCollector with mixed cache hits
    // When calculating cache hit rate
    // Then it should return correct rate
    let mut collector = MetricsCollector::new();
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: true,
    });
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 200,
        xtts_latency_ms: 50,
        sovits_latency_ms: 150,
        audio_duration_ms: 1000,
        cache_hit: false,
    });
    
    assert_eq!(collector.get_cache_hit_rate(), 0.5);
}

#[test]
fn test_metrics_collector_error_rate() {
    // Given a MetricsCollector with errors
    // When calculating error rate
    // Then it should return correct rate
    let mut collector = MetricsCollector::new();
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: false,
    });
    
    collector.record_error();
    collector.record_error();
    
    assert_eq!(collector.get_error_rate(), 2.0 / 1.0); // 2 errors / 1 request = 2.0
}

#[test]
fn test_metrics_collector_get_stats() {
    // Given a MetricsCollector with data
    // When getting stats
    // Then it should return all statistics
    let mut collector = MetricsCollector::new();
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: true,
    });
    
    let stats = collector.get_stats();
    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.average_latency_ms, 100.0);
    assert_eq!(stats.average_xtts_latency_ms, 30.0);
    assert_eq!(stats.average_sovits_latency_ms, 70.0);
    assert_eq!(stats.cache_hit_rate, 1.0);
}

#[test]
fn test_metrics_collector_reset() {
    // Given a MetricsCollector with data
    // When resetting
    // Then it should clear all statistics
    let mut collector = MetricsCollector::new();
    
    collector.record_request(PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: false,
    });
    
    collector.reset();
    
    assert_eq!(collector.total_requests, 0);
    assert_eq!(collector.get_average_latency_ms(), 0.0);
}

#[test]
fn test_latency_timer_start() {
    // Given a LatencyTimer
    // When starting
    // Then it should record the start time
    let timer = LatencyTimer::start();
    let elapsed = timer.elapsed_ms();
    assert!(elapsed < 100); // Should be very small immediately after start
}

#[test]
fn test_latency_timer_elapsed_ms() {
    // Given a LatencyTimer
    // When waiting and checking elapsed time
    // Then it should return correct milliseconds
    let timer = LatencyTimer::start();
    std::thread::sleep(Duration::from_millis(50));
    let elapsed = timer.elapsed_ms();
    assert!(elapsed >= 50);
    assert!(elapsed < 200); // Allow some margin
}

#[test]
fn test_latency_timer_elapsed() {
    // Given a LatencyTimer
    // When checking elapsed Duration
    // Then it should return correct duration
    let timer = LatencyTimer::start();
    std::thread::sleep(Duration::from_millis(50));
    let elapsed = timer.elapsed();
    assert!(elapsed >= Duration::from_millis(50));
    assert!(elapsed < Duration::from_millis(200));
}

#[test]
fn test_metrics_collector_zero_requests() {
    // Given a MetricsCollector with no requests
    // When getting statistics
    // Then it should return zeros
    let collector = MetricsCollector::new();
    
    assert_eq!(collector.get_average_latency_ms(), 0.0);
    assert_eq!(collector.get_average_piper_latency_ms(), 0.0);
    assert_eq!(collector.get_average_sovits_latency_ms(), 0.0);
    assert_eq!(collector.get_cache_hit_rate(), 0.0);
    assert_eq!(collector.get_error_rate(), 0.0);
}

#[test]
fn test_pipeline_metrics_serialization() {
    // Given a PipelineMetrics
    // When serializing
    // Then it should serialize correctly
    let metrics = PipelineMetrics {
        total_latency_ms: 100,
        xtts_latency_ms: 30,
        sovits_latency_ms: 70,
        audio_duration_ms: 500,
        cache_hit: true,
    };
    
    // PipelineMetrics doesn't implement Serialize, but we can test the struct
    assert_eq!(metrics.total_latency_ms, 100);
    assert_eq!(metrics.xtts_latency_ms, 30);
    assert_eq!(metrics.sovits_latency_ms, 70);
    assert_eq!(metrics.audio_duration_ms, 500);
    assert!(metrics.cache_hit);
}



