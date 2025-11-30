# IPC Protocol Technical Specification

## Purpose

The IPC Protocol defines the communication contract between the Electron client (main + renderer) and the Rust Orchestrator. This specification defines message formats, serialization, error handling, and bidirectional communication patterns.

## ADDED Requirements

### Requirement: IPC Message Format
The IPC Protocol SHALL use JSON for message serialization, with UTF-8 encoding and structured message types.

#### Scenario: Serialize PlayerAction Message
Given a PlayerAction with session_id="sess_123", player_id="player_1", kind=Voice, text="I attack the goblin"
When serializing the message
Then the IPC Protocol SHALL serialize to JSON
And the JSON SHALL include all required fields
And the JSON SHALL be UTF-8 encoded
And the JSON SHALL be valid and parseable

#### Scenario: Deserialize SceneUpdate Message
Given a JSON SceneUpdate message from Orchestrator
When deserializing the message
Then the IPC Protocol SHALL parse JSON correctly
And the IPC Protocol SHALL extract all fields
And the IPC Protocol SHALL validate required fields are present
And the IPC Protocol SHALL create SceneUpdate object

### Requirement: UI to Orchestrator Messages
The IPC Protocol SHALL support messages from UI (Electron renderer/main) to Orchestrator.

#### Scenario: Send PlayerAction from UI
Given UI wants to send a PlayerAction
When sending via IPC
Then the UI SHALL serialize PlayerAction to JSON
And the UI SHALL send via IPC channel "orchestrator:player-action"
And the Orchestrator SHALL receive and parse the message
And the Orchestrator SHALL validate the message
And the Orchestrator SHALL process the action

#### Scenario: Send RollResult from UI
Given UI has a RollResult from player dice roll
When sending via IPC
Then the UI SHALL serialize RollResult to JSON
And the UI SHALL send via IPC channel "orchestrator:roll-result"
And the Orchestrator SHALL receive and parse the message
And the Orchestrator SHALL validate request_id matches pending RollRequest
And the Orchestrator SHALL process the roll result

### Requirement: Orchestrator to UI Messages
The IPC Protocol SHALL support messages from Orchestrator to UI (Electron renderer).

#### Scenario: Send SceneUpdate to UI
Given Orchestrator needs to update UI with scene state
When sending SceneUpdate
Then the Orchestrator SHALL serialize SceneUpdate to JSON
And the Orchestrator SHALL send via IPC channel "orchestrator:scene-update"
And the UI SHALL receive and parse the message
And the UI SHALL update scene state
And the UI SHALL update UI components accordingly

#### Scenario: Send CombatUpdate to UI
Given Orchestrator needs to update UI with combat state
When sending CombatUpdate
Then the Orchestrator SHALL serialize CombatUpdate to JSON
And the Orchestrator SHALL send via IPC channel "orchestrator:combat-update"
And the UI SHALL receive and parse the message
And the UI SHALL update combat UI (initiative order, active creature, round)
And the UI SHALL highlight active creature

#### Scenario: Send RollRequest to UI
Given Orchestrator needs player to roll dice
When sending RollRequest
Then the Orchestrator SHALL serialize RollRequest to JSON
And the Orchestrator SHALL send via IPC channel "orchestrator:roll-request"
And the UI SHALL receive and parse the message
And the UI SHALL display dice roll UI
And the UI SHALL wait for player input
And the UI SHALL send RollResult back when complete

#### Scenario: Send Narration to UI
Given Orchestrator has narration text from Master AI
When sending Narration
Then the Orchestrator SHALL serialize Narration to JSON
And the Orchestrator SHALL send via IPC channel "orchestrator:narration"
And the UI SHALL receive and parse the message
And if tagged_for_tts=true, the UI SHALL send to TTS Service
And the UI SHALL display narration text
And the UI SHALL highlight speaking entity

### Requirement: Message Validation
The IPC Protocol SHALL validate all messages for required fields, types, and constraints.

#### Scenario: Validate Required Fields
Given a PlayerAction message missing session_id
When validating the message
Then the IPC Protocol SHALL detect missing required field
And the IPC Protocol SHALL return ValidationError
And the IPC Protocol SHALL indicate which field is missing
And the IPC Protocol SHALL NOT process the message

#### Scenario: Validate Field Types
Given a RollResult message with total="abc" (string instead of number)
When validating the message
Then the IPC Protocol SHALL detect type mismatch
And the IPC Protocol SHALL return ValidationError
And the IPC Protocol SHALL indicate expected type
And the IPC Protocol SHALL NOT process the message

#### Scenario: Validate Session ID
Given a message with session_id that doesn't exist
When validating the message
Then the Orchestrator SHALL check if session exists
And if session doesn't exist, the Orchestrator SHALL return SessionNotFoundError
And the Orchestrator SHALL NOT process the message

### Requirement: Error Handling
The IPC Protocol SHALL handle errors gracefully, returning structured error responses.

#### Scenario: Handle Parsing Error
Given malformed JSON message
When parsing the message
Then the IPC Protocol SHALL catch parsing error
And the IPC Protocol SHALL return ParseError
And the IPC Protocol SHALL include error message
And the IPC Protocol SHALL NOT crash the service

#### Scenario: Handle Processing Error
Given a valid message that fails during processing
When processing the message
Then the IPC Protocol SHALL catch processing error
And the IPC Protocol SHALL return ProcessingError
And the IPC Protocol SHALL include error context
And the IPC Protocol SHALL log the error
And the IPC Protocol SHALL send error response to UI

### Requirement: Message Correlation
The IPC Protocol SHALL support request/response correlation using request IDs.

#### Scenario: Correlate RollRequest and RollResult
Given a RollRequest with request_id="req_123"
When UI sends RollResult
Then the RollResult SHALL include the same request_id="req_123"
And the Orchestrator SHALL match RollResult to pending RollRequest
And the Orchestrator SHALL process the correlated result
And the Orchestrator SHALL remove pending request from queue

#### Scenario: Handle Orphaned RollResult
Given a RollResult with request_id that doesn't match any pending request
When processing the RollResult
Then the Orchestrator SHALL detect orphaned result
And the Orchestrator SHALL return CorrelationError
And the Orchestrator SHALL log the error
And the Orchestrator SHALL NOT process the result

### Requirement: Async Message Handling
The IPC Protocol SHALL support asynchronous message handling without blocking.

#### Scenario: Non-Blocking Message Send
Given UI wants to send PlayerAction
When sending via IPC
Then the IPC Protocol SHALL send asynchronously
And the UI SHALL NOT block waiting for response
And the UI SHALL continue processing other events
And the Orchestrator SHALL process message in background

#### Scenario: Async Response Handling
Given Orchestrator sends SceneUpdate
When UI receives the message
Then the UI SHALL handle message asynchronously
And the UI SHALL NOT block main thread
And the UI SHALL update state in next render cycle
And the UI SHALL maintain responsiveness

### Requirement: Message Batching
The IPC Protocol SHALL support batching multiple messages for efficiency.

#### Scenario: Batch Multiple Updates
Given multiple SceneUpdate messages need to be sent
When batching messages
Then the IPC Protocol SHALL combine messages into batch
And the IPC Protocol SHALL send as single batch message
And the UI SHALL receive and process all updates
And the UI SHALL apply updates atomically

## Technical Constraints

### Performance Requirements
- Message serialization: < 2ms
- Message deserialization: < 2ms
- IPC send latency: < 5ms
- IPC receive latency: < 5ms
- Total round-trip: < 20ms

### Message Size Limits
- Maximum message size: 1MB
- Typical message size: < 10KB
- Batch message size: < 100KB

### Reliability Requirements
- Message delivery SHALL be guaranteed (Electron IPC is reliable)
- Message order SHALL be preserved
- Duplicate messages SHALL be detected and handled

## Implementation Notes

### Electron IPC Channels
```
Main Process → Orchestrator:
  - orchestrator:player-action
  - orchestrator:roll-result

Orchestrator → Main Process:
  - orchestrator:scene-update
  - orchestrator:combat-update
  - orchestrator:roll-request
  - orchestrator:narration
```

### Message Types
All message types are defined in `orchestrator/src/communication.rs`:
- PlayerAction
- RollResult
- SceneUpdate
- CombatUpdate
- RollRequest
- Narration

### Serialization Format
- Format: JSON (UTF-8)
- Schema: Defined in Rust types, serialized via serde
- Validation: JSON schema validation (optional)

### Error Response Format
```json
{
  "error": {
    "type": "ValidationError",
    "message": "Missing required field: session_id",
    "code": "MISSING_FIELD",
    "field": "session_id"
  }
}
```

### Testing Requirements
- Unit tests for message serialization (100% coverage)
- Unit tests for message deserialization (100% coverage)
- Unit tests for validation (100% coverage)
- Integration tests for IPC communication
- E2E tests for full message flow











