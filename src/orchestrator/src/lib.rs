//! VRPG Orchestrator - Central coordinator for the VRPG system
//!
//! The Orchestrator is the "systemic brain" of VRPG, coordinating all non-AI pure tasks:
//! - Scene state management (FSM)
//! - INTENT DSL parsing and execution
//! - Integration with services (rules5e, memory, game-engine)
//! - Communication with UI (IPC/WebSocket)

pub mod cache;
pub mod communication;
pub mod error;
pub mod fsm;
pub mod intent;
pub mod intent_router;
pub mod llm_client;
pub mod orchestrator;
pub mod pipeline;
pub mod services;
pub mod session;

pub use communication::CommunicationState;
pub use error::{OrchestratorError, Result};
pub use fsm::{SceneState, SceneStateMachine};
pub use intent::{Intent, IntentExecutor, IntentParser};
pub use intent_router::{
    classify_intent, ClassificationResult, IntentClassification, IntentRouter,
};
pub use orchestrator::Orchestrator;
pub use pipeline::{
    context_14b::{prepare_14b_context, Context14B, ContextEvent, VectorizerResult},
    flow::{handle_player_input, PipelineFlowError, PipelineFlowResult},
    objective_responses::answer_objective_question,
    simple_rule_query::{answer_simple_rule_query, SimpleRuleQueryResult},
    trigger::{should_trigger_1_5b, trigger_1_5b, trigger_1_5b_and_send_to_tts, TriggerCriteria},
    PipelineState, PipelineStateManager, PipelineStatus,
};
pub use session::{GameSession, SessionManager};
