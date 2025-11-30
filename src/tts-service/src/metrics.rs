//! Metrics Module - Latency tracking and performance metrics
//!
//! This module provides metrics collection for TTS pipeline performance,
//! including latency tracking, throughput, and quality metrics.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct PipelineMetrics {
    pub total_latency_ms: u64,
    pub xtts_latency_ms: u64,
    pub audio_duration_ms: u64,
    pub cache_hit: bool,
}

#[derive(Debug, Default)]
pub struct MetricsCollector {
    total_requests: u64,
    total_latency_ms: u64,
    xtts_total_ms: u64,
    cache_hits: u64,
    errors: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_request(&mut self, metrics: PipelineMetrics) {
        self.total_requests += 1;
        self.total_latency_ms += metrics.total_latency_ms;
        self.xtts_total_ms += metrics.xtts_latency_ms;

        if metrics.cache_hit {
            self.cache_hits += 1;
        }
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn get_average_latency_ms(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_latency_ms as f64 / self.total_requests as f64
    }

    pub fn get_average_xtts_latency_ms(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.xtts_total_ms as f64 / self.total_requests as f64
    }

    pub fn get_cache_hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.cache_hits as f64 / self.total_requests as f64
    }

    pub fn get_error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.errors as f64 / self.total_requests as f64
    }

    pub fn get_stats(&self) -> MetricsStats {
        MetricsStats {
            total_requests: self.total_requests,
            average_latency_ms: self.get_average_latency_ms(),
            average_xtts_latency_ms: self.get_average_xtts_latency_ms(),
            cache_hit_rate: self.get_cache_hit_rate(),
            error_rate: self.get_error_rate(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsStats {
    pub total_requests: u64,
    pub average_latency_ms: f64,
    pub average_xtts_latency_ms: f64,
    pub cache_hit_rate: f64,
    pub error_rate: f64,
}

pub type SharedMetricsCollector = Arc<RwLock<MetricsCollector>>;

/// Helper struct to measure latency of a pipeline stage
pub struct LatencyTimer {
    start: Instant,
}

impl LatencyTimer {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();

        collector.record_request(PipelineMetrics {
            total_latency_ms: 100,
            xtts_latency_ms: 30,
            audio_duration_ms: 500,
            cache_hit: false,
        });

        collector.record_request(PipelineMetrics {
            total_latency_ms: 200,
            xtts_latency_ms: 50,
            audio_duration_ms: 1000,
            cache_hit: true,
        });

        assert_eq!(collector.total_requests, 2);
        assert_eq!(collector.get_average_latency_ms(), 150.0);
        assert_eq!(collector.get_cache_hit_rate(), 0.5);
    }

    #[test]
    fn test_latency_timer() {
        let timer = LatencyTimer::start();
        std::thread::sleep(std::time::Duration::from_millis(10));

        let elapsed = timer.elapsed_ms();
        assert!(elapsed >= 10);
    }
}
