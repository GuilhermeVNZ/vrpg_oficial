# INTENT DSL Technical Specification

## Purpose

The INTENT DSL (Domain Specific Language) is a structured language used by the Master AI to communicate intentions to the Orchestrator. This specification defines the formal grammar, syntax, semantics, and parsing requirements for the INTENT DSL.

## ADDED Requirements

### Requirement: DSL Block Structure
The INTENT DSL SHALL use a block-based structure with explicit opening and closing markers, allowing multiple INTENTs within a single block.

#### Scenario: Parse Valid INTENT Block
Given text containing:
```
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
END_INTENT
[/INTENTS]
```
When the parser processes the text
Then the parser SHALL recognize the [INTENTS] opening marker
And the parser SHALL extract all INTENTs within the block
And the parser SHALL recognize the [/INTENTS] closing marker
And the parser SHALL ignore any text outside the block markers

#### Scenario: Parse Multiple INTENTs in Single Block
Given text containing multiple INTENTs within one [INTENTS]...[/INTENTS] block
When the parser processes the text
Then the parser SHALL extract all INTENTs as separate objects
And the parser SHALL preserve the order of INTENTs
And the parser SHALL return a vector containing all parsed INTENTs

#### Scenario: Handle Missing Closing Marker
Given text containing [INTENTS] without a matching [/INTENTS]
When the parser processes the text
Then the parser SHALL return an IntentParseError
And the parser SHALL indicate the position of the error
And the parser SHALL not return partial INTENTs

### Requirement: INTENT Declaration Syntax
Each INTENT SHALL begin with "INTENT:" followed by the INTENT type, and SHALL end with "END_INTENT".

#### Scenario: Parse INTENT Declaration
Given text containing "INTENT: MELEE_ATTACK"
When the parser encounters this line
Then the parser SHALL recognize this as the start of a MELEE_ATTACK INTENT
And the parser SHALL expect field definitions until END_INTENT
And the parser SHALL create an Intent::MeleeAttack object

#### Scenario: Handle Missing END_INTENT
Given text containing "INTENT: SKILL_CHECK" followed by fields but no "END_INTENT"
When the parser processes the text
Then the parser SHALL return an IntentParseError
And the parser SHALL indicate that END_INTENT is missing
And the parser SHALL not create a partial INTENT object

### Requirement: Field Definition Syntax
INTENT fields SHALL be defined as "KEY: VALUE" pairs, with one field per line.

#### Scenario: Parse Field Definitions
Given text containing:
```
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
```
When the parser processes these lines
Then the parser SHALL extract "ACTOR" as the key and "player_1" as the value
And the parser SHALL extract "SKILL" as the key and "persuasion" as the value
And the parser SHALL extract "TARGET" as the key and "npc_guard_01" as the value
And the parser SHALL trim whitespace from keys and values

#### Scenario: Handle Empty Values
Given text containing "TARGET: " (empty value)
When the parser processes this field
Then the parser SHALL treat the value as an empty string
And the parser SHALL include the field in the parsed INTENT
And the parser SHALL not return an error

#### Scenario: Handle Multi-line Values
Given text containing a field with a quoted multi-line value
When the parser processes the field
Then the parser SHALL preserve the multi-line content
And the parser SHALL remove surrounding quotes if present
And the parser SHALL normalize line breaks

### Requirement: INTENT Type Support
The parser SHALL support all defined INTENT types: SKILL_CHECK, MELEE_ATTACK, RANGED_ATTACK, SPELL_CAST, LORE_QUERY, RULE_QUERY, COMBAT_START, COMBAT_END, and others as defined.

#### Scenario: Parse Skill Check INTENT
Given text containing:
```
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda"
SUGGEST_DC: YES
END_INTENT
```
When the parser processes this INTENT
Then the parser SHALL create an Intent::SkillCheck object
And the actor field SHALL be "player_1"
And the skill field SHALL be "persuasion"
And the target field SHALL be Some("npc_guard_01")
And the context field SHALL be Some("convencer o guarda")
And the suggest_dc field SHALL be true

#### Scenario: Parse Melee Attack INTENT
Given text containing:
```
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
WEAPON: weapon_longsword
MOVE_REQUIRED: YES
END_INTENT
```
When the parser processes this INTENT
Then the parser SHALL create an Intent::MeleeAttack object
And the actor field SHALL be "player_1"
And the target field SHALL be "npc_goblin_02"
And the weapon field SHALL be Some("weapon_longsword")
And the move_required field SHALL be true

#### Scenario: Parse Spell Cast INTENT
Given text containing:
```
INTENT: SPELL_CAST
ACTOR: player_1
SPELL: fireball
SLOT_LEVEL: 3
AREA_CENTER: "15,8"
TARGETS: npc_troll_01, npc_goblin_03
END_INTENT
```
When the parser processes this INTENT
Then the parser SHALL create an Intent::SpellCast object
And the actor field SHALL be "player_1"
And the spell field SHALL be "fireball"
And the slot_level field SHALL be 3
And the area_center field SHALL be Some((15, 8))
And the targets field SHALL contain ["npc_troll_01", "npc_goblin_03"]

#### Scenario: Parse Lore Query INTENT
Given text containing:
```
INTENT: LORE_QUERY
QUERY: "história dos Magos Rubros de Thay"
SCOPE: faction
END_INTENT
```
When the parser processes this INTENT
Then the parser SHALL create an Intent::LoreQuery object
And the query field SHALL be "história dos Magos Rubros de Thay"
And the scope field SHALL be Some("faction")

### Requirement: Boolean Value Parsing
Boolean fields SHALL accept "YES" or "NO" (case-insensitive) and SHALL convert them to true/false respectively.

#### Scenario: Parse YES Boolean
Given text containing "SUGGEST_DC: YES"
When the parser processes this field
Then the parser SHALL convert "YES" to true
And the parser SHALL be case-insensitive ("yes", "Yes", "YES" all convert to true)

#### Scenario: Parse NO Boolean
Given text containing "MOVE_REQUIRED: NO"
When the parser processes this field
Then the parser SHALL convert "NO" to false
And the parser SHALL be case-insensitive ("no", "No", "NO" all convert to false)

#### Scenario: Handle Invalid Boolean Value
Given text containing "SUGGEST_DC: maybe" (invalid boolean)
When the parser processes this field
Then the parser SHALL treat it as false (default)
And the parser SHALL log a warning
And the parser SHALL not return an error

### Requirement: Numeric Value Parsing
Numeric fields SHALL be parsed according to their expected type (u8, i32, etc.) with appropriate validation.

#### Scenario: Parse Valid Integer
Given text containing "SLOT_LEVEL: 3"
When the parser processes this field for a u8 type
Then the parser SHALL parse "3" as u8
And the parser SHALL store the value correctly

#### Scenario: Handle Invalid Integer
Given text containing "SLOT_LEVEL: abc" (non-numeric)
When the parser processes this field
Then the parser SHALL use the default value (1 for slot_level)
And the parser SHALL log a warning
And the parser SHALL not return an error

#### Scenario: Parse Coordinate Pair
Given text containing "AREA_CENTER: 15,8"
When the parser processes this field
Then the parser SHALL split by comma
And the parser SHALL parse "15" as i32 for x coordinate
And the parser SHALL parse "8" as i32 for y coordinate
And the parser SHALL create a tuple (15, 8)

### Requirement: List Value Parsing
List fields SHALL accept comma-separated values and SHALL parse them into vectors.

#### Scenario: Parse Comma-Separated List
Given text containing "TARGETS: npc_troll_01, npc_goblin_03, npc_goblin_04"
When the parser processes this field
Then the parser SHALL split by comma
And the parser SHALL trim whitespace from each value
And the parser SHALL create a vector ["npc_troll_01", "npc_goblin_03", "npc_goblin_04"]

#### Scenario: Parse Single Item List
Given text containing "TARGETS: npc_troll_01" (single item, no comma)
When the parser processes this field
Then the parser SHALL create a vector with one element ["npc_troll_01"]
And the parser SHALL handle this as a valid list

### Requirement: Field Inference and Defaults
The parser SHALL apply default values for optional fields when they are omitted.

#### Scenario: Omitted Optional Field
Given text containing an INTENT without the MOVE_REQUIRED field
When the parser processes the INTENT
Then the parser SHALL use the default value (false for MOVE_REQUIRED)
And the parser SHALL not return an error

#### Scenario: Omitted Required Field
Given text containing an INTENT without a required field (e.g., ACTOR in MELEE_ATTACK)
When the parser processes the INTENT
Then the parser SHALL return an IntentParseError
And the parser SHALL indicate which required field is missing

### Requirement: Error Reporting
The parser SHALL provide detailed error messages indicating the location and nature of parsing errors.

#### Scenario: Syntax Error Reporting
Given text containing invalid INTENT syntax
When the parser encounters the error
Then the parser SHALL return an IntentParseError
And the error message SHALL indicate the line number or position
And the error message SHALL describe the nature of the error
And the error message SHALL include context (surrounding text if possible)

#### Scenario: Unknown INTENT Type
Given text containing "INTENT: UNKNOWN_TYPE"
When the parser processes this INTENT
Then the parser SHALL return an IntentParseError
And the error message SHALL indicate that UNKNOWN_TYPE is not a recognized INTENT type
And the error message SHALL list valid INTENT types if possible

## Technical Constraints

### Performance Requirements
- Parsing a single INTENT block SHALL complete in < 10ms
- Parsing a block with 10 INTENTs SHALL complete in < 50ms
- Memory usage for parsing SHALL be O(n) where n is input text length

### Grammar Definition
```
Block := "[INTENTS]" { Intent } "[/INTENTS]"
Intent := "INTENT:" TYPE { KeyVal } "END_INTENT"
KeyVal := KEY ":" VALUE
KEY := [A-Z_]+
VALUE := .* (until newline or next KEY)
TYPE := SKILL_CHECK | MELEE_ATTACK | RANGED_ATTACK | SPELL_CAST | LORE_QUERY | RULE_QUERY | COMBAT_START | COMBAT_END | ...
```

### Case Sensitivity
- INTENT types SHALL be case-insensitive (SKILL_CHECK = skill_check = Skill_Check)
- Field keys SHALL be case-insensitive (ACTOR = actor = Actor)
- Field values SHALL preserve case unless specified otherwise

### Whitespace Handling
- Leading and trailing whitespace SHALL be trimmed from keys and values
- Empty lines SHALL be ignored
- Multiple spaces SHALL be normalized to single space within values

## Implementation Notes

### Rust Parser Structure
The parser SHALL be implemented as a deterministic line-by-line parser, not using regex for main parsing logic.

### Error Types
```rust
pub enum IntentParseError {
    MissingClosingMarker,
    MissingEndIntent,
    UnknownIntentType(String),
    MissingRequiredField(String),
    InvalidValueType { field: String, expected: String, got: String },
    ParseError { message: String, position: usize },
}
```

### Testing Requirements
- Unit tests for each INTENT type (100% coverage)
- Unit tests for edge cases (empty values, missing fields, invalid syntax)
- Property-based tests for parser robustness
- Performance benchmarks for large INTENT blocks











