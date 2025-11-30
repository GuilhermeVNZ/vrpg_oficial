use crate::config::RuntimeConfig;
use crate::error::{InfraError, Result};
use crate::health::{HealthChecker, HealthStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceState {
    pub name: String,
    pub running: bool,
    pub health_status: Option<HealthStatus>,
    pub restart_count: u32,
    pub last_start: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct ServiceManager {
    config: Arc<RuntimeConfig>,
    states: Arc<RwLock<HashMap<String, ServiceState>>>,
    health_checker: Arc<HealthChecker>,
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceManager {
    pub fn new() -> Self {
        let config = Arc::new(RuntimeConfig::new());
        let mut states = HashMap::new();

        // Initialize states for all configured services
        for (name, _) in &config.services {
            states.insert(
                name.clone(),
                ServiceState {
                    name: name.clone(),
                    running: false,
                    health_status: None,
                    restart_count: 0,
                    last_start: None,
                },
            );
        }

        Self {
            config,
            states: Arc::new(RwLock::new(states)),
            health_checker: Arc::new(HealthChecker::new()),
        }
    }

    pub fn with_config(config: RuntimeConfig) -> Self {
        let config = Arc::new(config);
        let mut states = HashMap::new();

        for (name, _) in &config.services {
            states.insert(
                name.clone(),
                ServiceState {
                    name: name.clone(),
                    running: false,
                    health_status: None,
                    restart_count: 0,
                    last_start: None,
                },
            );
        }

        Self {
            config,
            states: Arc::new(RwLock::new(states)),
            health_checker: Arc::new(HealthChecker::new()),
        }
    }

    pub async fn start_service(&self, name: &str) -> Result<()> {
        let service_config = self
            .config
            .get_service(name)
            .ok_or_else(|| InfraError::Service(format!("Service not found: {}", name)))?;

        let port = service_config.port;
        let health_endpoint = service_config.health_endpoint.clone();
        let startup_timeout_secs = service_config.startup_timeout_secs;

        // In a real implementation, this would start the service process
        // For now, we just mark it as running
        {
            let mut states = self.states.write().await;
            if let Some(state) = states.get_mut(name) {
                state.running = true;
                state.last_start = Some(chrono::Utc::now());
            }
        }

        // Wait for service to be healthy
        let timeout = Duration::from_secs(startup_timeout_secs);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            let status = self
                .health_checker
                .check(name, port, &health_endpoint)
                .await?;

            if status.healthy {
                {
                    let mut states = self.states.write().await;
                    if let Some(state) = states.get_mut(name) {
                        state.health_status = Some(status);
                    }
                }
                return Ok(());
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Err(InfraError::Service(format!(
            "Service {} failed to start within timeout",
            name
        )))
    }

    pub async fn stop_service(&self, name: &str) -> Result<()> {
        // In a real implementation, this would stop the service process
        {
            let mut states = self.states.write().await;
            if let Some(state) = states.get_mut(name) {
                state.running = false;
                state.health_status = None;
            }
        }

        Ok(())
    }

    pub async fn start_all(&self) -> Result<()> {
        for service_name in self.config.get_startup_order() {
            if let Err(e) = self.start_service(service_name).await {
                tracing::warn!("Failed to start service {}: {}", service_name, e);
                // Continue with other services
            }
        }
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<()> {
        let service_names: Vec<String> = {
            let states = self.states.read().await;
            states.keys().cloned().collect()
        };

        for service_name in service_names {
            let _ = self.stop_service(&service_name).await;
        }

        Ok(())
    }

    pub async fn check_health(&self, name: &str) -> Result<HealthStatus> {
        let service_config = self
            .config
            .get_service(name)
            .ok_or_else(|| InfraError::Service(format!("Service not found: {}", name)))?;

        let port = service_config.port;
        let health_endpoint = service_config.health_endpoint.clone();

        let status = self
            .health_checker
            .check(name, port, &health_endpoint)
            .await?;

        // Update state
        {
            let mut states = self.states.write().await;
            if let Some(state) = states.get_mut(name) {
                state.health_status = Some(status.clone());
            }
        }

        Ok(status)
    }

    pub async fn check_all_health(&self) -> Vec<HealthStatus> {
        let services: Vec<(String, u16, String)> = {
            let config = &*self.config;
            config
                .services
                .iter()
                .map(|(name, config)| (name.clone(), config.port, config.health_endpoint.clone()))
                .collect()
        };

        self.health_checker.check_multiple(&services).await
    }

    pub async fn get_state(&self, name: &str) -> Option<ServiceState> {
        let states = self.states.read().await;
        states.get(name).cloned()
    }

    pub async fn get_all_states(&self) -> Vec<ServiceState> {
        let states = self.states.read().await;
        states.values().cloned().collect()
    }

    pub async fn start_health_monitoring(&self) {
        let config = self.config.clone();
        let states = self.states.clone();
        let health_checker = self.health_checker.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30));

            loop {
                interval.tick().await;

                for (name, service_config) in &config.services {
                    let status = health_checker
                        .check(name, service_config.port, &service_config.health_endpoint)
                        .await;

                    if let Ok(status) = status {
                        let mut states = states.write().await;
                        if let Some(state) = states.get_mut(name) {
                            state.health_status = Some(status.clone());

                            // Auto-restart if unhealthy and under max restarts
                            if !status.healthy
                                && state.running
                                && state.restart_count < service_config.max_restarts
                            {
                                tracing::warn!("Service {} is unhealthy, attempting restart", name);
                                state.restart_count += 1;
                                state.running = false;
                                // In real implementation, would trigger restart
                            }
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_manager_creation() {
        let manager = ServiceManager::new();
        let states = manager.get_all_states().await;
        assert!(!states.is_empty());
    }

    #[tokio::test]
    async fn test_service_manager_get_state() {
        let manager = ServiceManager::new();
        let state = manager.get_state("rules5e-service").await;
        assert!(state.is_some());
        assert_eq!(state.unwrap().name, "rules5e-service");
    }
}
