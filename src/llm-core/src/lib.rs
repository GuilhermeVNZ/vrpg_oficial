// LLM Core Service - Local LLM inference with persona support
// This module provides LLM inference capabilities with support for multiple personas

pub mod bridge_phrases;
pub mod error;
pub mod inference;
pub mod persona;
pub mod server;

pub use bridge_phrases::{BridgeCategory, BridgePhrasesManager};
pub use error::{LlmError, Result};
pub use inference::{LlmInference, LlmRequest, LlmResponse};
pub use persona::Persona;
pub use server::LlmServer;

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Basic compilation test - module compiles successfully
    }
}

// Include 1.5B tests module (only in test mode)
#[cfg(test)]
#[path = "inference_1_5b_tests.rs"]
mod inference_1_5b_tests;

// Include bridge phrases tests module (only in test mode)
#[cfg(test)]
#[path = "bridge_phrases_tests.rs"]
mod bridge_phrases_tests;
