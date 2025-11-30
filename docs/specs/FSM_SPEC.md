# Scene State Machine (FSM) Technical Specification

## Purpose

The Scene State Machine (FSM) manages the four main scene states of VRPG: SocialFreeFlow, Exploration, CombatTurnBased, and DowntimePreparation. This specification defines the state transitions, validation rules, and integration requirements for the FSM.

## ADDED Requirements

### Requirement: State Enum Definition
The FSM SHALL define four distinct scene states as an enum: SocialFreeFlow, Exploration, CombatTurnBased, and DowntimePreparation.

#### Scenario: State Enum Values
Given the FSM is initialized
When accessing the SceneState enum
Then the FSM SHALL provide SocialFreeFlow as a valid state
And the FSM SHALL provide Exploration as a valid state
And the FSM SHALL provide CombatTurnBased as a valid state
And the FSM SHALL provide DowntimePreparation as a valid state
And no other states SHALL be valid

#### Scenario: State Name Retrieval
Given a SceneState value
When requesting the human-readable name
Then the FSM SHALL return "SocialFreeFlow" for SocialFreeFlow state
And the FSM SHALL return "Exploration" for Exploration state
And the FSM SHALL return "CombatTurnBased" for CombatTurnBased state
And the FSM SHALL return "DowntimePreparation" for DowntimePreparation state

### Requirement: State Machine Initialization
The FSM SHALL initialize with a default state and support initialization with a specific state.

#### Scenario: Default Initialization
Given a new SceneStateMachine is created
When the FSM is initialized
Then the FSM SHALL default to SocialFreeFlow state
And the FSM SHALL be ready to accept transitions

#### Scenario: Initialization with Specific State
Given a request to create FSM with initial state Exploration
When the FSM is initialized with with_state(Exploration)
Then the FSM SHALL set current_state to Exploration
And the FSM SHALL be ready to accept transitions

### Requirement: Valid State Transitions
The FSM SHALL allow transitions between specific states according to defined rules.

#### Scenario: SocialFreeFlow to Exploration
Given current state is SocialFreeFlow
When transitioning to Exploration
Then the FSM SHALL allow the transition
And the FSM SHALL update current_state to Exploration
And the FSM SHALL return Ok(())

#### Scenario: SocialFreeFlow to CombatTurnBased
Given current state is SocialFreeFlow
When transitioning to CombatTurnBased
Then the FSM SHALL allow the transition
And the FSM SHALL update current_state to CombatTurnBased
And the FSM SHALL return Ok(())

#### Scenario: Exploration to SocialFreeFlow
Given current state is Exploration
When transitioning to SocialFreeFlow
Then the FSM SHALL allow the transition
And the FSM SHALL update current_state to SocialFreeFlow
And the FSM SHALL return Ok(())

#### Scenario: Exploration to CombatTurnBased
Given current state is Exploration
When transitioning to CombatTurnBased
Then the FSM SHALL allow the transition
And the FSM SHALL update current_state to CombatTurnBased
And the FSM SHALL return Ok(())

#### Scenario: CombatTurnBased to SocialFreeFlow
Given current state is CombatTurnBased
When transitioning to SocialFreeFlow
Then the FSM SHALL allow the transition
And the FSM SHALL update current_state to SocialFreeFlow
And the FSM SHALL return Ok(())

#### Scenario: CombatTurnBased to Exploration
Given current state is CombatTurnBased
When transitioning to Exploration
Then the FSM SHALL allow the transition
And the FSM SHALL update current_state to Exploration
And the FSM SHALL return Ok(())

#### Scenario: Same State Transition
Given current state is SocialFreeFlow
When transitioning to SocialFreeFlow (same state)
Then the FSM SHALL allow the transition (no-op)
And the FSM SHALL keep current_state as SocialFreeFlow
And the FSM SHALL return Ok(())

### Requirement: Invalid State Transitions
The FSM SHALL reject invalid state transitions and return appropriate errors.

#### Scenario: Invalid Transition Detection
Given current state is DowntimePreparation
When attempting to transition directly to CombatTurnBased
Then the FSM SHALL reject the transition
And the FSM SHALL return InvalidStateTransition error
And the FSM SHALL NOT change current_state
And the error message SHALL indicate the invalid transition

### Requirement: Force Transition
The FSM SHALL support forced transitions for recovery and initialization scenarios.

#### Scenario: Force Transition
Given current state is SocialFreeFlow
When force_transition is called with CombatTurnBased
Then the FSM SHALL change current_state to CombatTurnBased
And the FSM SHALL NOT validate the transition
And the FSM SHALL NOT return an error
And the FSM SHALL allow any state to be forced

### Requirement: State Persistence
The FSM SHALL support serialization and deserialization for state persistence.

#### Scenario: Serialize State Machine
Given a SceneStateMachine with current_state = CombatTurnBased
When serializing the FSM
Then the FSM SHALL serialize current_state correctly
And the serialized data SHALL be deserializable
And the deserialized FSM SHALL have the same current_state

#### Scenario: Deserialize State Machine
Given serialized FSM data with current_state = Exploration
When deserializing the FSM
Then the FSM SHALL restore current_state to Exploration
And the FSM SHALL be ready to accept transitions
And the FSM SHALL maintain all state information

### Requirement: State Query
The FSM SHALL provide methods to query the current state without modification.

#### Scenario: Get Current State
Given a SceneStateMachine with current_state = SocialFreeFlow
When querying current_state()
Then the FSM SHALL return SocialFreeFlow
And the FSM SHALL NOT modify the state
And the FSM SHALL allow multiple queries without side effects

## MODIFIED Requirements

### Requirement: State Transition Validation
The FSM SHALL validate transitions using a centralized validation function that checks all transition rules.

#### Scenario: Centralized Validation
Given any two states (from, to)
When can_transition_from is called
Then the FSM SHALL check all defined transition rules
And the FSM SHALL return true for valid transitions
And the FSM SHALL return false for invalid transitions
And the validation SHALL be consistent across all calls

## Technical Constraints

### Performance Requirements
- State transition SHALL complete in < 1ms
- State query SHALL complete in < 0.1ms
- Serialization SHALL complete in < 5ms
- Deserialization SHALL complete in < 5ms

### Memory Requirements
- FSM memory footprint SHALL be < 1KB per instance
- State enum size SHALL be minimal (single byte)

### Thread Safety
- FSM operations SHALL be thread-safe when used with appropriate synchronization
- Concurrent state queries SHALL be safe
- State transitions SHALL be atomic

## Implementation Notes

### Rust Implementation
```rust
pub enum SceneState {
    SocialFreeFlow,
    Exploration,
    CombatTurnBased,
    DowntimePreparation,
}

pub struct SceneStateMachine {
    current_state: SceneState,
}
```

### Transition Matrix
All states can transition to:
- SocialFreeFlow (from any state)
- Exploration (from any state)
- CombatTurnBased (from SocialFreeFlow or Exploration)
- DowntimePreparation (from any state)

Special rule:
- DowntimePreparation cannot transition directly to CombatTurnBased (must go through SocialFreeFlow or Exploration first)

### Integration Points
- FSM is used by Orchestrator for scene state management
- FSM state is included in GameSession for persistence
- FSM state changes trigger UI updates via SceneUpdate messages

### Testing Requirements
- Unit tests for all valid transitions (100% coverage)
- Unit tests for all invalid transitions (100% coverage)
- Unit tests for serialization/deserialization (100% coverage)
- Property-based tests for transition consistency











