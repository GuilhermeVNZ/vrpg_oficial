// Memory Service - Long-term memory with Hive stack integration
// This module provides memory storage and retrieval using Vectorizer, Nexus, and Lexum

pub mod error;
pub mod lexum;
pub mod nexus;
pub mod server;
pub mod store;
pub mod vectorizer;

pub use error::{MemoryError, Result};
pub use lexum::LexumClient;
pub use nexus::NexusClient;
pub use server::MemoryServer;
pub use store::{Memory, MemoryQuery, MemoryStore, SearchResult};
pub use vectorizer::{SharedVectorizerClient, VectorizerClient};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_compiles() {
        assert!(true);
    }
}
