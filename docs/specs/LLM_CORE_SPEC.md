# LLM Core Technical Specification

## Purpose

The LLM Core Service provides local LLM inference for the VRPG system using a **pipeline architecture with 2 models**:

1. **Qwen-1.5B** → Fast human reaction ("prelude") - < 1.2s
2. **Qwen-14B** → Complete narrative, consequences, resolution - < 6s

This specification defines the technical requirements for dual-model inference, INTENT DSL generation, persona management, and integration with the Orchestrator.

**See [PIPELINE_ARCHITECTURE.md](../PIPELINE_ARCHITECTURE.md) for complete architecture details.**

## ADDED Requirements

### Requirement: Dual Model LLM Inference
The LLM Core SHALL provide local LLM inference using **two models**:
- **Qwen 2.5 1.5B** (Q4_K_M quantization) for fast prelude reactions
- **Qwen 2.5 14B** (Q4_K_M quantization) for complete narrative

#### Scenario: Load LLM Models
Given LLM Core service initialization
When loading the models
Then the LLM Core SHALL load Qwen 2.5 1.5B Q4_K_M from configured path
And the LLM Core SHALL load Qwen 2.5 14B Q4_K_M from configured path
And the LLM Core SHALL initialize inference engines (llama.cpp, Candle, or equivalent) for both models
And the LLM Core SHALL configure 1.5B parameters (max_tokens=40, temperature=0.8, top_p=0.9)
And the LLM Core SHALL configure 14B parameters (max_tokens=2048, context_size=8192, temperature=0.7, top_p=0.9)
And the LLM Core SHALL keep both models loaded in memory
And the LLM Core SHALL be ready for inference

#### Scenario: Perform 1.5B Inference (Prelude)
Given a LlmRequest with text input and role="prelude"
When performing inference with 1.5B
Then the LLM Core SHALL construct prompt for emotional reaction
And the LLM Core SHALL run inference on 1.5B model
And the LLM Core SHALL generate response text (1-2 sentences, max 40 tokens)
And the LLM Core SHALL complete inference in < 1.2s total (parse: 30-80ms, generation: 200-450ms, TTS: 150-350ms)
And the LLM Core SHALL return LlmResponse with prelude text
And the LLM Core SHALL NOT include final results, consequences, or rule applications

#### Scenario: Perform 14B Inference (Complete Narrative)
Given a LlmRequest with text input, role="narration", and fast_prelude from 1.5B
When performing inference with 14B
Then the LLM Core SHALL construct prompt with fast_prelude included
And the LLM Core SHALL include game_state, context_slice, and vectorizer results
And the LLM Core SHALL run inference on 14B model
And the LLM Core SHALL generate complete narrative response
And the LLM Core SHALL complete inference in < 6s total (ingest: 200-500ms, generation: 1.5-4s, TTS: 300-700ms)
And the LLM Core SHALL return LlmResponse with complete narrative
And the LLM Core SHALL NOT repeat or contradict the 1.5B prelude

### Requirement: Persona Switching
The LLM Core SHALL support multiple personas via prompt engineering, switching between DM, NPC, AI Player, Monster, and Narrator modes.

#### Scenario: Switch to DM Persona
Given current persona is "npc"
When switching to "dm" persona
Then the LLM Core SHALL update system prompt with DM instructions
And the LLM Core SHALL include DM mindset principles
And the LLM Core SHALL configure response style for narration
And the LLM Core SHALL NOT reload the model
And the LLM Core SHALL be ready for DM inference immediately

#### Scenario: Switch to NPC Persona
Given current persona is "dm"
And NPC profile with name="npc_guard", personality="stern", voice_style="gravel_low"
When switching to "npc" persona with NPC profile
Then the LLM Core SHALL update system prompt with NPC instructions
And the LLM Core SHALL include NPC personality and background
And the LLM Core SHALL configure response style for character dialogue
And the LLM Core SHALL maintain NPC consistency across sessions

#### Scenario: Switch to AI Player Persona
Given current persona is "dm"
And AI Player profile with character="rogue", personality="sarcastic"
When switching to "player_ai" persona
Then the LLM Core SHALL update system prompt with AI Player instructions
And the LLM Core SHALL include character personality and motivations
And the LLM Core SHALL configure response style for party member dialogue
And the LLM Core SHALL NOT generate INTENTs (only Master AI does)

### Requirement: INTENT DSL Generation
The LLM Core SHALL generate INTENT DSL blocks in LLM output, structured according to INTENT_DSL specification.

#### Scenario: Generate Skill Check INTENT
Given LLM input requires a skill check for persuasion
When generating response
Then the LLM Core SHALL include INTENT DSL block in output:
```
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda a liberar a entrada"
SUGGEST_DC: YES
END_INTENT
[/INTENTS]
```
And the LLM Core SHALL generate narrative text alongside INTENTs
And the LLM Core SHALL NOT explain INTENTs to the user

#### Scenario: Generate Combat INTENT
Given LLM input requires a melee attack
When generating response
Then the LLM Core SHALL include MELEE_ATTACK INTENT in DSL block
And the LLM Core SHALL include all required fields (ACTOR, TARGET, WEAPON if applicable)
And the LLM Core SHALL generate narrative description of the action
And the LLM Core SHALL NOT include mechanical numbers in narration

#### Scenario: Generate Multiple INTENTs
Given LLM input requires multiple actions
When generating response
Then the LLM Core SHALL include multiple INTENTs in single [INTENTS] block
And the LLM Core SHALL preserve order of INTENTs
And the LLM Core SHALL generate narrative text for context

### Requirement: Context Management
The LLM Core SHALL manage conversation context, including game state, memory, and recent dialogue.

#### Scenario: Include Game State in Context
Given a LlmRequest with game_context
When constructing prompt
Then the LLM Core SHALL include game state snapshot
And the LLM Core SHALL include current scene state
And the LLM Core SHALL include active participants
And the LLM Core SHALL format context for LLM consumption
And the LLM Core SHALL maintain context within token limits

#### Scenario: Include Memory in Context
Given memory search results from Memory Service
When constructing prompt
Then the LLM Core SHALL include relevant memories as LORE_SNIPPETS
And the LLM Core SHALL format memories for narrative use
And the LLM Core SHALL prioritize most relevant memories
And the LLM Core SHALL maintain context size within limits

#### Scenario: Manage Context Window
Given conversation history exceeds context window
When managing context
Then the LLM Core SHALL prioritize recent messages
And the LLM Core SHALL summarize or truncate older messages
And the LLM Core SHALL maintain essential context (game state, active scene)
And the LLM Core SHALL NOT exceed model context limit (8192 tokens)

### Requirement: Master AI Mindset
The LLM Core SHALL enforce Master AI mindset principles: never calculate mechanics, only narrate and generate INTENTs.

#### Scenario: Master AI Narration
Given Master AI needs to describe an attack result
When generating narration
Then the LLM Core SHALL generate narrative description
And the LLM Core SHALL NOT include mechanical numbers (AC, damage, modifiers)
And the LLM Core SHALL use metaphorical, diegetic language
And the LLM Core SHALL maintain immersion

#### Scenario: Master AI INTENT Generation
Given Master AI needs to request a skill check
When generating response
Then the LLM Core SHALL generate SKILL_CHECK INTENT
And the LLM Core SHALL NOT specify DC (Engine decides)
And the LLM Core SHALL include narrative context
And the LLM Core SHALL NOT explain mechanics to user

### Requirement: Integration with Memory Service
The LLM Core SHALL integrate with Memory Service to enrich context with long-term memory.

#### Scenario: Enrich Context with Memory
Given a LlmRequest that may benefit from memory lookup
When preparing context
Then the LLM Core SHALL call Memory Service with relevant query
And the LLM Core SHALL wait for memory search results
And the LLM Core SHALL include memories in context as LORE_SNIPPETS
And the LLM Core SHALL use memories to inform narrative

#### Scenario: Memory Query via INTENT
Given LLM determines it needs lore information
When generating response
Then the LLM Core SHALL generate LORE_QUERY INTENT
And the Orchestrator SHALL process the INTENT and query Memory Service
And the Memory Service SHALL return results
And the LLM Core SHALL receive enriched context for next inference

### Requirement: HTTP API
The LLM Core SHALL expose HTTP endpoints for inference requests.

#### Scenario: Health Check Endpoint
Given LLM Core service is running
When GET /health is called
Then the LLM Core SHALL return status 200
And the response SHALL include service status
And the response SHALL include model status (loaded/not loaded)
And the response SHALL include current persona

#### Scenario: Inference Endpoint
Given POST /llm with LlmRequest:
```json
{
  "session_id": "sess_123",
  "speaker": {"type": "human_player", "id": "pc_1"},
  "llm_role": "dm",
  "game_context": {...},
  "player_input": "I sneak towards the door and listen carefully."
}
```
When the endpoint is called
Then the LLM Core SHALL validate the request
And the LLM Core SHALL construct prompt with persona and context
And the LLM Core SHALL perform inference
And the LLM Core SHALL return LlmResponse with generated text and INTENTs
And the response SHALL include INTENT DSL blocks if applicable

### Requirement: Response Format
The LLM Core SHALL return responses in a structured format including narrative text and INTENT DSL blocks.

#### Scenario: Parse Response with INTENTs
Given LLM generates response with INTENT DSL block
When returning LlmResponse
Then the LLM Core SHALL include narrative text
And the LLM Core SHALL include INTENT DSL block as separate field
And the LLM Core SHALL structure response for Orchestrator parsing
And the LLM Core SHALL maintain text and INTENTs separately

## Technical Constraints

### Performance Requirements
- Model loading: < 30s (first time), < 5s (subsequent)
- Inference latency: < 3s (target: 1-2s) for typical requests
- Persona switching: < 100ms
- Context preparation: < 50ms

### Resource Requirements
- Model size: ~8GB (Qwen 2.5 14B Q4_K_M)
- GPU memory: < 8GB (if using GPU acceleration)
- CPU memory: < 16GB (if using CPU)
- Context window: 8192 tokens maximum

### Quality Requirements
- Response coherence: High (model-dependent)
- INTENT DSL accuracy: > 90% correct format
- Persona consistency: Maintained across sessions
- Narrative quality: Immersive, diegetic, non-mechanical

## Implementation Notes

### Rust Module Structure
```
src/llm-core/
├── lib.rs              # Public API
├── error.rs            # Error types
├── server.rs           # HTTP server
├── inference.rs        # LLM inference engine
├── persona.rs          # Persona management
└── prompt.rs          # Prompt construction
```

### Dependencies
- LLM inference library (llama.cpp bindings, Candle, or equivalent)
- `serde` for serialization
- `tokio` for async runtime
- `tracing` for logging

### Testing Requirements
- Unit tests for persona switching (100% coverage)
- Unit tests for prompt construction (100% coverage)
- Integration tests for inference
- Tests for INTENT DSL generation accuracy
- Performance benchmarks for inference latency











