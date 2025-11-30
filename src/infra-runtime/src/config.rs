use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub port: u16,
    pub health_endpoint: String,
    pub startup_timeout_secs: u64,
    pub health_check_interval_secs: u64,
    pub max_restarts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub services: HashMap<String, ServiceConfig>,
    pub startup_order: Vec<String>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeConfig {
    pub fn new() -> Self {
        let mut services = HashMap::new();

        // Default service configurations
        services.insert(
            "rules5e-service".to_string(),
            ServiceConfig {
                name: "rules5e-service".to_string(),
                port: 7004,
                health_endpoint: "/health".to_string(),
                startup_timeout_secs: 10,
                health_check_interval_secs: 30,
                max_restarts: 5,
            },
        );

        services.insert(
            "asr-service".to_string(),
            ServiceConfig {
                name: "asr-service".to_string(),
                port: 7001,
                health_endpoint: "/health".to_string(),
                startup_timeout_secs: 30,
                health_check_interval_secs: 30,
                max_restarts: 3,
            },
        );

        services.insert(
            "tts-service".to_string(),
            ServiceConfig {
                name: "tts-service".to_string(),
                port: 7003,
                health_endpoint: "/health".to_string(),
                startup_timeout_secs: 30,
                health_check_interval_secs: 30,
                max_restarts: 3,
            },
        );

        services.insert(
            "llm-core".to_string(),
            ServiceConfig {
                name: "llm-core".to_string(),
                port: 7002,
                health_endpoint: "/health".to_string(),
                startup_timeout_secs: 60,
                health_check_interval_secs: 30,
                max_restarts: 3,
            },
        );

        services.insert(
            "memory-service".to_string(),
            ServiceConfig {
                name: "memory-service".to_string(),
                port: 7005,
                health_endpoint: "/health".to_string(),
                startup_timeout_secs: 10,
                health_check_interval_secs: 30,
                max_restarts: 5,
            },
        );

        Self {
            services,
            startup_order: vec![
                "rules5e-service".to_string(),
                "memory-service".to_string(),
                "asr-service".to_string(),
                "tts-service".to_string(),
                "llm-core".to_string(),
            ],
        }
    }

    pub fn get_service(&self, name: &str) -> Option<&ServiceConfig> {
        self.services.get(name)
    }

    pub fn get_startup_order(&self) -> &[String] {
        &self.startup_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = RuntimeConfig::new();
        assert!(!config.services.is_empty());
        assert!(!config.startup_order.is_empty());
    }

    #[test]
    fn test_config_get_service() {
        let config = RuntimeConfig::new();
        let service = config.get_service("rules5e-service");
        assert!(service.is_some());
        assert_eq!(service.unwrap().port, 7004);
    }
}
