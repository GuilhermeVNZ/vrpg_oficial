//! Service integrations
//!
//! This module provides integration clients for external services:
//! - Rules5e Service (combat, dice, skill checks)
//! - Memory Service (lore, rules, knowledge)
//! - TTS Service (voice synthesis)
//! - ASR Service (speech recognition)

pub mod memory;
pub mod rules5e;
pub mod synap_client;
pub mod tts;

pub use memory::MemoryClient;
pub use rules5e::Rules5eClient;
pub use synap_client::{AsrResult, LlmResult, SharedSynapClient, SynapClient, TtsResult};
pub use tts::{SharedTtsClient, TtsClient, TtsResponse};
