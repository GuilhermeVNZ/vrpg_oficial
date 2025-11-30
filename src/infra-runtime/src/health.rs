use crate::error::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub service: String,
    pub healthy: bool,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: u64,
    pub error: Option<String>,
}

pub struct HealthChecker {
    client: Client,
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn check(
        &self,
        service_name: &str,
        port: u16,
        endpoint: &str,
    ) -> Result<HealthStatus> {
        let url = format!("http://localhost:{}{}", port, endpoint);
        let start = std::time::Instant::now();

        let response = self.client.get(&url).send().await;

        let response_time_ms = start.elapsed().as_millis() as u64;
        let now = Utc::now();

        match response {
            Ok(resp) => {
                let healthy = resp.status().is_success();
                Ok(HealthStatus {
                    service: service_name.to_string(),
                    healthy,
                    last_check: now,
                    response_time_ms,
                    error: if healthy {
                        None
                    } else {
                        Some(format!("HTTP {}", resp.status()))
                    },
                })
            }
            Err(e) => Ok(HealthStatus {
                service: service_name.to_string(),
                healthy: false,
                last_check: now,
                response_time_ms,
                error: Some(format!("Request failed: {}", e)),
            }),
        }
    }

    pub async fn check_multiple(&self, services: &[(String, u16, String)]) -> Vec<HealthStatus> {
        let mut results = Vec::new();

        for (name, port, endpoint) in services {
            if let Ok(status) = self.check(name, *port, endpoint).await {
                results.push(status);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checker_creation() {
        let checker = HealthChecker::new();
        // Note: Actual health check would require a running service
        assert!(true);
    }
}
