# Orchestrator Technical Specification

## Purpose

The Orchestrator is the systemic brain of VRPG, coordinating all non-AI pure tasks including scene state management, INTENT DSL parsing and execution, integration with services (rules5e, memory, game-engine), and communication with UI (IPC/WebSocket). This specification defines the technical requirements, interfaces, and behavior of the Orchestrator module.

## ADDED Requirements

### Requirement: Scene State Machine
The Orchestrator SHALL maintain a finite state machine (FSM) that manages four distinct scene states: SocialFreeFlow, Exploration, CombatTurnBased, and DowntimePreparation.

#### Scenario: State Transition from Social to Combat
Given a game session is in SocialFreeFlow state
When the Orchestrator receives a COMBAT_START INTENT from the Master AI
Then the Orchestrator SHALL transition to CombatTurnBased state
And the Orchestrator SHALL validate the transition is allowed
And the Orchestrator SHALL emit a CombatUpdate message to the UI

#### Scenario: Invalid State Transition
Given a game session is in DowntimePreparation state
When the Orchestrator attempts to transition directly to CombatTurnBased state
Then the Orchestrator SHALL reject the transition
And the Orchestrator SHALL return an InvalidStateTransition error
And the Orchestrator SHALL remain in DowntimePreparation state

#### Scenario: State Persistence
Given a game session with current state SocialFreeFlow
When the session is saved to disk
Then the Orchestrator SHALL persist the current state
And when the session is loaded from disk
Then the Orchestrator SHALL restore the exact same state

### Requirement: INTENT DSL Parsing
The Orchestrator SHALL parse INTENT DSL blocks from LLM output text, extracting structured INTENT objects that can be executed by the system.

#### Scenario: Parse Single INTENT Block
Given LLM output contains a valid INTENT block:
```
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
END_INTENT
[/INTENTS]
```
When the Orchestrator parses the text
Then the Orchestrator SHALL extract one Intent::SkillCheck object
And the actor field SHALL be "player_1"
And the skill field SHALL be "persuasion"
And the target field SHALL be Some("npc_guard_01")

#### Scenario: Parse Multiple INTENTs
Given LLM output contains multiple INTENT blocks within a single [INTENTS]...[/INTENTS] block
When the Orchestrator parses the text
Then the Orchestrator SHALL extract all INTENTs as separate objects
And the Orchestrator SHALL preserve the order of INTENTs
And the Orchestrator SHALL return a vector of Intent objects

#### Scenario: Handle Invalid INTENT Format
Given LLM output contains malformed INTENT syntax (missing END_INTENT, invalid field names)
When the Orchestrator attempts to parse the text
Then the Orchestrator SHALL return an IntentParseError
And the Orchestrator SHALL log the error with context
And the Orchestrator SHALL continue processing other valid INTENTs if present

### Requirement: INTENT Execution
The Orchestrator SHALL execute parsed INTENTs by calling appropriate services (rules5e-service, memory-service, game-engine) based on the INTENT type.

#### Scenario: Execute Skill Check INTENT
Given a parsed Intent::SkillCheck with actor="player_1", skill="persuasion"
When the Orchestrator executes the INTENT
Then the Orchestrator SHALL create a RollRequest message
And the Orchestrator SHALL send the RollRequest to the UI
And the Orchestrator SHALL wait for a RollResult response
And the Orchestrator SHALL call rules5e-service to resolve the skill check
And the Orchestrator SHALL generate a SkillOutcome event

#### Scenario: Execute Melee Attack INTENT
Given a parsed Intent::MeleeAttack with actor="player_1", target="npc_goblin_02"
When the Orchestrator executes the INTENT
And the current scene state is CombatTurnBased
Then the Orchestrator SHALL validate the actor has an available action
And the Orchestrator SHALL validate the target is alive and in range
And if a roll is required, the Orchestrator SHALL send RollRequest to UI
And the Orchestrator SHALL call rules5e-service to resolve attack and damage
And the Orchestrator SHALL update CombatState with results

#### Scenario: Execute Lore Query INTENT
Given a parsed Intent::LoreQuery with query="history of Waterdeep"
When the Orchestrator executes the INTENT
Then the Orchestrator SHALL call memory-service with the query
And the Orchestrator SHALL wait for search results from Vectorizer/Nexus/Lexum
And the Orchestrator SHALL format the results for injection into LLM context
And the Orchestrator SHALL return the results to the Master AI

### Requirement: XTTS Streaming Integration
The Orchestrator SHALL integrate with XTTS streaming pipeline for real-time cinematic audio output.

#### Scenario: Send Text to XTTS Streaming
Given text output from Qwen 1.5B or Qwen 14B
When sending to TTS Service
Then the Orchestrator SHALL send text to XTTS streaming pipeline
And the Orchestrator SHALL NOT wait for full audio generation
And the Orchestrator SHALL allow streaming to start playback immediately
And the Orchestrator SHALL maintain non-blocking communication

#### Scenario: Cancel XTTS Streaming
Given XTTS streaming is active
When player interrupts or new input arrives
Then the Orchestrator SHALL cancel XTTS streaming immediately
And the Orchestrator SHALL clear AudioBuffer FIFO
And the Orchestrator SHALL cancel chunks in generation
And the Orchestrator SHALL stop audio playback gracefully

#### Scenario: GPU Adaptive Control Integration
Given Orchestrator initialization
When coordinating with TTS Service
Then the Orchestrator SHALL respect GPU tier configuration
And the Orchestrator SHALL NOT override GPU adaptive settings
And the Orchestrator SHALL handle GPU unavailable scenarios gracefully

### Requirement: Session Management
The Orchestrator SHALL manage game sessions, including creation, state persistence, and lifecycle management.

#### Scenario: Create New Session
Given a request to create a new game session
When the Orchestrator creates a session
Then the Orchestrator SHALL generate a unique session ID (UUID)
And the Orchestrator SHALL initialize the session with SocialFreeFlow state
And the Orchestrator SHALL set created_at and updated_at timestamps
And the Orchestrator SHALL return the session ID

#### Scenario: Get Session State
Given a valid session ID
When the Orchestrator retrieves the session
Then the Orchestrator SHALL return the current scene state
And the Orchestrator SHALL return session metadata (created_at, updated_at)
And the Orchestrator SHALL return all participants in the session

#### Scenario: Remove Session
Given a valid session ID
When the Orchestrator removes the session
Then the Orchestrator SHALL delete the session from memory
And the Orchestrator SHALL clean up any associated resources
And subsequent requests for that session ID SHALL return None

### Requirement: IPC/WebSocket Communication
The Orchestrator SHALL communicate with the Electron UI via IPC (preferred) or WebSocket, handling bidirectional message passing.

#### Scenario: Receive Player Action from UI
Given the UI sends a PlayerAction message via IPC
When the Orchestrator receives the message
Then the Orchestrator SHALL validate the session ID exists
And the Orchestrator SHALL validate the player ID is valid
And the Orchestrator SHALL process the action according to current scene state
And the Orchestrator SHALL send appropriate response messages

#### Scenario: Send Scene Update to UI
Given the scene state has changed (e.g., new participant, state transition)
When the Orchestrator needs to notify the UI
Then the Orchestrator SHALL create a SceneUpdate message
And the Orchestrator SHALL serialize the message to JSON
And the Orchestrator SHALL send the message via IPC to the UI
And the UI SHALL receive and process the update

#### Scenario: Handle Communication Error
Given a communication error occurs (IPC channel closed, WebSocket disconnected)
When the Orchestrator detects the error
Then the Orchestrator SHALL log the error with context
And the Orchestrator SHALL attempt to reconnect if possible
And the Orchestrator SHALL queue messages for retry if connection is temporarily lost

### Requirement: Service Integration
The Orchestrator SHALL integrate with external services (rules5e-service, memory-service, game-engine) via HTTP/GRPC interfaces.

#### Scenario: Call Rules5e Service
Given an INTENT requires D&D 5e rule resolution (attack, damage, skill check)
When the Orchestrator needs to resolve the rule
Then the Orchestrator SHALL construct an HTTP request to rules5e-service
And the Orchestrator SHALL include all necessary parameters
And the Orchestrator SHALL wait for the response (with timeout)
And the Orchestrator SHALL handle errors gracefully (service unavailable, timeout)

#### Scenario: Call Memory Service
Given an INTENT requires memory lookup (LoreQuery, RuleQuery)
When the Orchestrator needs to query memory
Then the Orchestrator SHALL construct a query to memory-service
And the Orchestrator SHALL specify the search scope (Vectorizer, Nexus, Lexum)
And the Orchestrator SHALL wait for search results
And the Orchestrator SHALL format results for LLM context injection

### Requirement: Error Handling and Recovery
The Orchestrator SHALL handle errors gracefully, providing fallback behavior and logging for debugging.

#### Scenario: INTENT Parsing Failure
Given LLM output contains unparseable INTENT syntax
When the Orchestrator attempts to parse the INTENT
Then the Orchestrator SHALL log the error with the raw text
And the Orchestrator SHALL continue processing other INTENTs if present
And the Orchestrator SHALL notify the Master AI of the parsing failure
And the Orchestrator SHALL request a retry with error context

#### Scenario: Service Unavailable
Given a required service (rules5e-service, memory-service) is unavailable
When the Orchestrator attempts to call the service
Then the Orchestrator SHALL detect the unavailability (timeout, connection error)
And the Orchestrator SHALL log the error
And the Orchestrator SHALL provide fallback behavior (e.g., default DC for skill checks)
And the Orchestrator SHALL notify the UI of degraded functionality

#### Scenario: Invalid INTENT Execution
Given an INTENT cannot be executed (invalid actor, target dead, out of range)
When the Orchestrator attempts to execute the INTENT
Then the Orchestrator SHALL validate all prerequisites
And the Orchestrator SHALL return a validation error
And the Orchestrator SHALL provide error context to the Master AI
And the Orchestrator SHALL not modify game state

## MODIFIED Requirements

### Requirement: State Machine Thread Safety
The Orchestrator SHALL ensure thread-safe access to the scene state machine when handling concurrent requests from UI and services.

#### Scenario: Concurrent State Access
Given multiple threads attempt to read or modify scene state simultaneously
When state operations occur
Then the Orchestrator SHALL use appropriate synchronization primitives (mutex, RwLock)
And the Orchestrator SHALL prevent race conditions
And the Orchestrator SHALL maintain state consistency

## Technical Constraints

### Performance Requirements
- INTENT parsing SHALL complete in < 10ms for typical INTENT blocks
- State transitions SHALL complete in < 5ms
- IPC message serialization/deserialization SHALL complete in < 2ms
- Service call timeouts SHALL be configurable (default: 5 seconds)
- XTTS streaming integration SHALL be non-blocking
- Audio streaming cancellation SHALL complete in < 100ms

### Resource Requirements
- Memory usage SHALL not exceed 100MB per active session
- CPU usage SHALL remain below 5% when idle
- File I/O for session persistence SHALL be asynchronous

### Compatibility Requirements
- IPC interface SHALL be compatible with Electron IPC
- WebSocket interface SHALL use standard WebSocket protocol (RFC 6455)
- Service interfaces SHALL use HTTP/1.1 or HTTP/2
- Serialization format SHALL be JSON (UTF-8)

## Implementation Notes

### Rust Module Structure
```
src/orchestrator/
├── lib.rs              # Public API
├── error.rs            # Error types
├── fsm.rs              # Finite State Machine
├── intent/
│   ├── mod.rs
│   ├── types.rs        # Intent enum
│   ├── parser.rs       # DSL parser
│   └── executor.rs    # INTENT executor
├── session.rs          # Session management
└── communication.rs    # IPC/WebSocket handlers
```

### Dependencies
- `serde` for serialization
- `uuid` for session IDs
- `chrono` for timestamps
- `tokio` for async runtime
- `tracing` for logging

### Testing Requirements
- Unit tests for FSM transitions (100% coverage)
- Unit tests for INTENT parser (100% coverage)
- Integration tests for service calls
- E2E tests for IPC communication











