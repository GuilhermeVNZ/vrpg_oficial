// Infra Runtime - Service orchestration and observability
// This module provides service management and health monitoring

pub mod config;
pub mod error;
pub mod health;
pub mod manager;

pub use config::{RuntimeConfig, ServiceConfig};
pub use error::Result;
pub use health::{HealthChecker, HealthStatus};
pub use manager::{ServiceManager, ServiceState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_compiles() {
        assert!(true);
    }
}
