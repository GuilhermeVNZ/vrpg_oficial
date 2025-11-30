# Memory Service Technical Specification

## Purpose

The Memory Service provides long-term memory for the VRPG system using the Hive stack (Vectorizer, Nexus, Lexum). This specification defines the technical requirements for memory storage, retrieval, indexing, and integration with the Orchestrator and Master AI.

## ADDED Requirements

### Requirement: Memory Storage
The Memory Service SHALL store memories with metadata including scope, session_id, actor_id, and timestamp.

#### Scenario: Store Campaign Memory
Given a memory with text="The party saved the old wizard from the collapsing tower", scope="campaign", session_id="sess_123"
When storing the memory
Then the Memory Service SHALL create a memory entry with unique ID
And the Memory Service SHALL store text, scope, session_id, timestamp
And the Memory Service SHALL index the memory in Vectorizer
And the Memory Service SHALL create graph relations in Nexus
And the Memory Service SHALL index text in Lexum
And the Memory Service SHALL return the memory ID

#### Scenario: Store Session Memory
Given a memory with scope="session", session_id="sess_123"
When storing the memory
Then the Memory Service SHALL store with session scope
And the Memory Service SHALL associate with current session
And the Memory Service SHALL index in session-specific collection
And the Memory Service SHALL make memory available for current session queries

#### Scenario: Store Actor-Specific Memory
Given a memory with actor_id="npc_wizard", scope="campaign"
When storing the memory
Then the Memory Service SHALL associate memory with the actor
And the Memory Service SHALL create graph relation in Nexus linking memory to actor
And the Memory Service SHALL enable actor-specific queries

### Requirement: Memory Retrieval via Vectorizer
The Memory Service SHALL retrieve memories using Vectorizer for semantic similarity search.

#### Scenario: Semantic Search
Given a query "What does the wizard owe us?"
And scope=["campaign", "session"]
When searching for memories
Then the Memory Service SHALL call Vectorizer with the query
And the Memory Service SHALL search in specified scopes
And the Memory Service SHALL return memories ranked by similarity score
And the Memory Service SHALL filter results by min_score threshold (default 0.7)
And the Memory Service SHALL return top N results (default 10)

#### Scenario: Search with Multiple Scopes
Given a query with scopes=["global", "campaign", "session"]
When searching for memories
Then the Memory Service SHALL search across all specified scopes
And the Memory Service SHALL combine results from all scopes
And the Memory Service SHALL rank combined results by relevance
And the Memory Service SHALL return unified result set

### Requirement: Memory Retrieval via Lexum
The Memory Service SHALL retrieve memories using Lexum for full-text search.

#### Scenario: Full-Text Search
Given a query "wizard tower rescue"
When performing full-text search
Then the Memory Service SHALL call Lexum with the query
And the Memory Service SHALL search indexed text content
And the Memory Service SHALL return memories matching text terms
And the Memory Service SHALL rank by text relevance
And the Memory Service SHALL return results quickly (< 50ms)

### Requirement: Memory Retrieval via Nexus
The Memory Service SHALL retrieve memories using Nexus for graph-based relationship queries.

#### Scenario: Graph-Based Query
Given a query about relationships involving "npc_wizard"
When querying via Nexus
Then the Memory Service SHALL call Nexus to traverse graph relations
And the Memory Service SHALL find related entities (NPCs, locations, events)
And the Memory Service SHALL expand query with related terms
And the Memory Service SHALL return memories connected via graph
And the Memory Service SHALL include relationship context

#### Scenario: Query Enhancement via Nexus
Given a query "wizard favor"
When enhancing query via Nexus
Then the Memory Service SHALL call Nexus to expand query
And Nexus SHALL add related terms based on graph relations
And the enhanced query SHALL include: "wizard", "favor", "debt", "obligation", "gandros", "tower", "rescue"
And the Memory Service SHALL use enhanced query for Vectorizer/Lexum search

### Requirement: Hybrid Search Pipeline
The Memory Service SHALL combine results from Nexus, Lexum, and Vectorizer for optimal retrieval.

#### Scenario: Hybrid Search Flow
Given a query "What does the wizard owe us?"
When performing hybrid search
Then the Memory Service SHALL first call Nexus to enhance query and find graph relations
And the Memory Service SHALL then call Lexum for full-text search with enhanced query
And the Memory Service SHALL then call Vectorizer for semantic search with enhanced query
And the Memory Service SHALL combine and deduplicate results
And the Memory Service SHALL rank final results by relevance score
And the Memory Service SHALL return top N results

### Requirement: Memory Classification
The Memory Service SHALL classify memories using Classify before indexing.

#### Scenario: Classify Memory Before Storage
Given a memory text "The party saved the old wizard from the collapsing tower"
When storing the memory
Then the Memory Service SHALL call Classify to categorize the memory
And Classify SHALL return categories: ["event", "npc_relationship", "location", "favor_owed"]
And the Memory Service SHALL store categories as metadata
And the Memory Service SHALL use categories for filtering and organization
And the Memory Service SHALL store confidence score and metadata from Classify

### Requirement: Document Processing
The Memory Service SHALL process documents using Transmutation before indexing.

#### Scenario: Process PDF Document
Given a PDF document "campaign_notes.pdf"
When processing for memory storage
Then the Memory Service SHALL call Transmutation to convert PDF to Markdown
And Transmutation SHALL extract text content
And the Memory Service SHALL chunk the Markdown into segments
And the Memory Service SHALL classify each chunk
And the Memory Service SHALL index chunks in Vectorizer, Lexum, and Nexus

#### Scenario: Process Image with OCR
Given an image file "map_tavern.png"
When processing for memory storage
Then the Memory Service SHALL call Transmutation with OCR enabled
And Transmutation SHALL extract text from image using Tesseract
And the Memory Service SHALL process extracted text as memory
And the Memory Service SHALL index the text content

### Requirement: Memory Scopes
The Memory Service SHALL organize memories into scopes: global, campaign, session, and actor.

#### Scenario: Query Global Scope
Given a query with scope="global"
When searching memories
Then the Memory Service SHALL search only in global scope
And the Memory Service SHALL return lore, rules, and general knowledge
And the Memory Service SHALL NOT return campaign-specific or session-specific memories

#### Scenario: Query Campaign Scope
Given a query with scope="campaign"
When searching memories
Then the Memory Service SHALL search in campaign scope
And the Memory Service SHALL return campaign-specific events, NPCs, locations
And the Memory Service SHALL include memories from all sessions in the campaign

#### Scenario: Query Session Scope
Given a query with scope="session", session_id="sess_123"
When searching memories
Then the Memory Service SHALL search only in session_123 scope
And the Memory Service SHALL return memories from current session
And the Memory Service SHALL NOT return memories from other sessions

#### Scenario: Query Actor Scope
Given a query with actor_id="npc_wizard"
When searching memories
Then the Memory Service SHALL search memories associated with npc_wizard
And the Memory Service SHALL use Nexus to find actor-related memories
And the Memory Service SHALL return memories about the actor

### Requirement: Memory Indexing
The Memory Service SHALL index memories in Vectorizer, Lexum, and Nexus for efficient retrieval.

#### Scenario: Index Memory in Vectorizer
Given a stored memory
When indexing in Vectorizer
Then the Memory Service SHALL generate embedding for the memory text
And the Memory Service SHALL store embedding in Vectorizer collection
And the Memory Service SHALL associate embedding with memory ID
And the Memory Service SHALL store metadata (scope, session_id, actor_id, timestamp)
And the Memory Service SHALL enable semantic search on the memory

#### Scenario: Index Memory in Lexum
Given a stored memory
When indexing in Lexum
Then the Memory Service SHALL index text content for full-text search
And the Memory Service SHALL create searchable index
And the Memory Service SHALL enable fast text-based queries
And the Memory Service SHALL associate index with memory ID

#### Scenario: Index Memory in Nexus
Given a stored memory mentioning entities (NPCs, locations)
When indexing in Nexus
Then the Memory Service SHALL extract entities from memory text
And the Memory Service SHALL create graph nodes for entities
And the Memory Service SHALL create graph edges for relationships
And the Memory Service SHALL link memory to entities
And the Memory Service SHALL enable graph traversal queries

### Requirement: HTTP API
The Memory Service SHALL expose HTTP endpoints for memory operations.

#### Scenario: Health Check Endpoint
Given Memory Service is running
When GET /health is called
Then the Memory Service SHALL return status 200
And the response SHALL include service status
And the response SHALL include Hive stack connectivity status
And the response SHALL include index statistics

#### Scenario: Store Memory Endpoint
Given POST /memory with memory data
When the endpoint is called
Then the Memory Service SHALL validate the memory data
And the Memory Service SHALL classify the memory
And the Memory Service SHALL store and index the memory
And the Memory Service SHALL return memory ID

#### Scenario: Search Memory Endpoint
Given POST /memory/search with query and scope
When the endpoint is called
Then the Memory Service SHALL perform hybrid search (Nexus + Lexum + Vectorizer)
And the Memory Service SHALL return ranked results
And the Memory Service SHALL include relevance scores
And the Memory Service SHALL include metadata for each result

## Technical Constraints

### Performance Requirements
- Memory storage: < 100ms per memory
- Semantic search (Vectorizer): < 200ms for typical queries
- Full-text search (Lexum): < 50ms for typical queries
- Graph query (Nexus): < 150ms for typical queries
- Hybrid search: < 400ms total

### Integration Requirements
- Memory Service SHALL integrate with Vectorizer MCP
- Memory Service SHALL integrate with Nexus MCP
- Memory Service SHALL integrate with Lexum MCP
- Memory Service SHALL integrate with Classify MCP
- Memory Service SHALL integrate with Transmutation library

### Storage Requirements
- Memory entries SHALL be stored persistently
- Indexes SHALL be maintained in Hive stack services
- Memory metadata SHALL be queryable
- Memory content SHALL support text up to 10KB per entry

## Implementation Notes

### Rust Module Structure
```
src/memory-service/
├── lib.rs              # Public API
├── error.rs            # Error types
├── server.rs           # HTTP server
├── store.rs            # Memory storage
├── vectorizer.rs       # Vectorizer integration
├── nexus.rs            # Nexus integration
├── lexum.rs            # Lexum integration
└── classify.rs         # Classify integration
```

### Dependencies
- `serde` for serialization
- `tokio` for async runtime
- `tracing` for logging
- MCP client libraries for Hive stack

### Testing Requirements
- Unit tests for memory storage (100% coverage)
- Integration tests for Vectorizer search
- Integration tests for Lexum search
- Integration tests for Nexus graph queries
- Integration tests for hybrid search pipeline
- Performance benchmarks for search operations











