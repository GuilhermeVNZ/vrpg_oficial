//! INTENT DSL parsing and execution
//!
//! This module handles:
//! - Parsing INTENT DSL blocks from LLM output
//! - Validating INTENTs
//! - Executing INTENTs by calling appropriate services

pub mod actor_stats;
pub mod executor;
pub mod parser;
pub mod types;

pub use executor::IntentExecutor;
pub use parser::IntentParser;
pub use types::Intent;
