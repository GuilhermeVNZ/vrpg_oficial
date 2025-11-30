# Turn Engine Technical Specification

## Purpose

The Turn Engine is the tactical combat system of VRPG, managing initiative order, turn progression, action economy, and deterministic resolution of combat actions in D&D 5e. This specification defines the technical requirements for turn-based combat flow, client vs server roll handling, and integration with the Orchestrator and Rules Engine.

## ADDED Requirements

### Requirement: Initiative Management
The Turn Engine SHALL manage initiative order for all combat participants, calculating initiative values and ordering creatures by initiative.

#### Scenario: Calculate Initiative for Player
Given a player character with DEX modifier +3 and no initiative bonus
When the Turn Engine calculates initiative
Then the Turn Engine SHALL request a roll from the client UI (RollRequest)
And the Turn Engine SHALL wait for RollResult from the client
And the Turn Engine SHALL add the DEX modifier to the natural roll
And the Turn Engine SHALL store the total initiative value

#### Scenario: Calculate Initiative for NPC
Given an NPC creature with DEX modifier +1
When the Turn Engine calculates initiative
Then the Turn Engine SHALL roll 1d20 + DEX modifier server-side
And the Turn Engine SHALL store the initiative value
And the Turn Engine SHALL NOT request a roll from the client

#### Scenario: Order Initiative
Given multiple creatures with calculated initiative values: player_1 (18), npc_goblin_01 (15), player_2 (20)
When the Turn Engine orders initiative
Then the Turn Engine SHALL sort by initiative value (descending)
And the Turn Engine SHALL create InitiativeEntry for each creature
And the Turn Engine SHALL set the order as: [player_2 (20), player_1 (18), npc_goblin_01 (15)]
And the Turn Engine SHALL set active_index to 0 (first in order)

### Requirement: Turn Progression
The Turn Engine SHALL manage turn progression, advancing to the next creature in initiative order and handling round transitions.

#### Scenario: Advance to Next Turn
Given a combat state with active_index = 0 and initiative_order with 3 creatures
When the current turn ends
Then the Turn Engine SHALL increment active_index to 1
And the Turn Engine SHALL reset action economy for the new active creature
And the Turn Engine SHALL emit TURN_START event for the new active creature
And the Turn Engine SHALL NOT increment round (still in round 1)

#### Scenario: Advance to Next Round
Given a combat state with active_index at the last creature in initiative_order
When the current turn ends
Then the Turn Engine SHALL wrap active_index to 0
And the Turn Engine SHALL increment round by 1
And the Turn Engine SHALL reset action economy for all creatures
And the Turn Engine SHALL emit NEW_ROUND event
And the Turn Engine SHALL emit TURN_START event for the first creature

#### Scenario: Skip Unconscious Creatures
Given a combat state where creature at index 1 is unconscious (is_conscious = false)
When advancing initiative
Then the Turn Engine SHALL skip the unconscious creature
And the Turn Engine SHALL advance to the next conscious creature
And the Turn Engine SHALL NOT increment round if wrapping within the same round

### Requirement: Action Economy
The Turn Engine SHALL track and enforce action economy (Action, Bonus Action, Movement, Reaction) for each creature per turn.

#### Scenario: Consume Action for Melee Attack
Given a creature with used_action = false
When a MELEE_ATTACK INTENT is executed
Then the Turn Engine SHALL set used_action = true
And the Turn Engine SHALL validate that used_action was false before execution
And the Turn Engine SHALL allow the attack to proceed

#### Scenario: Reject Action When Already Used
Given a creature with used_action = true
When a MELEE_ATTACK INTENT is attempted
Then the Turn Engine SHALL reject the INTENT
And the Turn Engine SHALL return an ActionEconomyError
And the Turn Engine SHALL notify the Master AI that the action is unavailable

#### Scenario: Reset Action Economy on Turn Start
Given a creature ending their turn with used_action = true, used_bonus_action = true
When the next turn starts for that creature
Then the Turn Engine SHALL reset used_action = false
And the Turn Engine SHALL reset used_bonus_action = false
And the Turn Engine SHALL reset remaining_movement_ft to creature's speed
And the Turn Engine SHALL NOT reset used_reaction (reactions reset at start of creature's turn)

### Requirement: INTENT Processing in Combat
The Turn Engine SHALL process combat INTENTs, validate prerequisites, and coordinate with the Rules Engine for resolution.

#### Scenario: Process Melee Attack INTENT
Given a MELEE_ATTACK INTENT with actor="player_1", target="npc_goblin_02"
And the current scene state is CombatTurnBased
And player_1 has an available action
And npc_goblin_02 is alive and in range
When the Turn Engine processes the INTENT
Then the Turn Engine SHALL validate the actor is the active creature
And the Turn Engine SHALL validate the target is alive
And the Turn Engine SHALL validate range and line of sight
And if a roll is required, the Turn Engine SHALL send RollRequest to UI
And the Turn Engine SHALL wait for RollResult
And the Turn Engine SHALL call rules5e-service to resolve attack and damage
And the Turn Engine SHALL update CombatState with results
And the Turn Engine SHALL consume the action
And the Turn Engine SHALL emit DAMAGE_DEALT event

#### Scenario: Process Spell Cast INTENT
Given a SPELL_CAST INTENT with actor="player_1", spell="fireball", slot_level=3, area_center=(15,8)
And the current scene state is CombatTurnBased
And player_1 has an available action and a 3rd level spell slot
When the Turn Engine processes the INTENT
Then the Turn Engine SHALL validate the actor has the spell
And the Turn Engine SHALL validate the actor has the spell slot
And the Turn Engine SHALL calculate affected creatures in the area
And the Turn Engine SHALL call rules5e-service for each affected creature (saves, damage)
And the Turn Engine SHALL update CombatState with results
And the Turn Engine SHALL consume the action and spell slot
And the Turn Engine SHALL emit SPELL_CAST event

#### Scenario: Reject Invalid INTENT
Given a MELEE_ATTACK INTENT where the target is dead
When the Turn Engine processes the INTENT
Then the Turn Engine SHALL validate the target is alive
And the Turn Engine SHALL reject the INTENT
And the Turn Engine SHALL return a ValidationError
And the Turn Engine SHALL notify the Master AI with error context

### Requirement: Line of Sight and Range
The Turn Engine SHALL validate line of sight (LoS) and range for ranged attacks and spells.

#### Scenario: Validate Line of Sight
Given a ranged attack from attacker at position (5, 5) to target at position (10, 10)
And a grid with obstacles between them
When the Turn Engine checks line of sight
Then the Turn Engine SHALL use Bresenham algorithm to trace the path
And the Turn Engine SHALL check each cell for blocks_los = true
And if any cell blocks LoS, the Turn Engine SHALL return LoS blocked
And the Turn Engine SHALL reject the attack INTENT

#### Scenario: Validate Range
Given a ranged weapon with range 30ft (6 grid cells)
And attacker at (0, 0) and target at (10, 0) (distance = 50ft)
When the Turn Engine checks range
Then the Turn Engine SHALL calculate distance as grid_distance * 5ft
And the Turn Engine SHALL compare to weapon_range
And the Turn Engine SHALL reject the attack if out of range
And the Turn Engine SHALL suggest movement or apply disadvantage if applicable

### Requirement: Area of Effect (AoE)
The Turn Engine SHALL calculate and apply area of effect spells and abilities to all affected creatures.

#### Scenario: Calculate AoE Circle
Given a SPELL_CAST INTENT with spell="fireball", area_center=(15, 8), radius=20ft
And a grid with multiple creatures
When the Turn Engine calculates affected area
Then the Turn Engine SHALL identify all cells within 20ft radius of center
And the Turn Engine SHALL identify all creatures in those cells
And the Turn Engine SHALL create SpellOutcomeCreature for each affected creature
And the Turn Engine SHALL apply saves and damage according to D&D 5e rules

#### Scenario: Calculate AoE Cone
Given a SPELL_CAST INTENT with spell="burning_hands", area_center=(10, 10), length=15ft, angle=60deg
And a grid with creatures
When the Turn Engine calculates affected area
Then the Turn Engine SHALL identify all cells within the cone shape
And the Turn Engine SHALL identify all creatures in those cells
And the Turn Engine SHALL apply effects to each creature

### Requirement: Condition Management
The Turn Engine SHALL automatically manage conditions, applying effects at turn start/end and removing expired conditions.

#### Scenario: Apply Condition at Turn Start
Given a creature with condition "poisoned" (UntilStartOfTurn)
When the creature's turn starts
Then the Turn Engine SHALL apply poison damage
And the Turn Engine SHALL check if the condition kills the creature
And the Turn Engine SHALL emit ConditionTicked event if damage is applied
And the Turn Engine SHALL remove the condition after applying effects

#### Scenario: Reduce Condition Duration
Given a creature with condition "frightened" (Rounds(3))
When the creature's turn ends
Then the Turn Engine SHALL reduce the duration from 3 to 2
And the Turn Engine SHALL keep the condition active
And the Turn Engine SHALL NOT emit ConditionEnded event

#### Scenario: Remove Expired Condition
Given a creature with condition "frightened" (Rounds(1))
When the creature's turn ends
Then the Turn Engine SHALL reduce duration from 1 to 0
And the Turn Engine SHALL remove the condition
And the Turn Engine SHALL emit ConditionEnded event
And the Turn Engine SHALL notify the Orchestrator for Master AI narration

### Requirement: Combat State Transitions
The Turn Engine SHALL handle transitions into and out of combat, initializing and cleaning up combat state.

#### Scenario: Start Combat
Given a COMBAT_START INTENT from the Master AI
And a list of creatures involved in combat
When the Turn Engine initializes combat
Then the Turn Engine SHALL create a CombatState with unique encounter_id
And the Turn Engine SHALL calculate initiative for all creatures
And the Turn Engine SHALL order creatures by initiative
And the Turn Engine SHALL set active_index to 0
And the Turn Engine SHALL set round to 1
And the Turn Engine SHALL emit CombatUpdate to UI
And the Turn Engine SHALL emit COMBAT_START event
And the Turn Engine SHALL trigger music state change to combat

#### Scenario: End Combat
Given a combat state where all hostile creatures are dead or unconscious
When the Turn Engine detects combat end
Then the Turn Engine SHALL set ended_at timestamp
And the Turn Engine SHALL emit COMBAT_END event
And the Turn Engine SHALL transition scene state to SocialFreeFlow
And the Turn Engine SHALL emit CombatUpdate with inCombat=false
And the Turn Engine SHALL trigger music state change to post-combat
And the Turn Engine SHALL clean up temporary combat state

### Requirement: Client vs Server Rolls
The Turn Engine SHALL distinguish between player rolls (client-side) and NPC/engine rolls (server-side).

#### Scenario: Request Player Roll
Given a skill check required for player_1
When the Turn Engine needs a roll
Then the Turn Engine SHALL create a RollRequest with actor_id="player_1"
And the Turn Engine SHALL include roll_kind, skill, formula_hint, reason
And the Turn Engine SHALL send RollRequest to UI via Orchestrator
And the Turn Engine SHALL wait for RollResult response
And the Turn Engine SHALL validate the RollResult matches the request_id

#### Scenario: Server-Side NPC Roll
Given an NPC needs to roll initiative
When the Turn Engine needs a roll
Then the Turn Engine SHALL call rules5e-service to roll server-side
And the Turn Engine SHALL NOT send RollRequest to UI
And the Turn Engine SHALL use the server roll result directly

#### Scenario: Validate Roll Result
Given a RollResult from client with total=18, natural=15, breakdown={d20: 15, modifier: 3}
And a RollRequest with formula_hint="1d20 + 3"
When the Turn Engine validates the result
Then the Turn Engine SHALL verify total matches natural + modifiers
And the Turn Engine SHALL verify breakdown is consistent
And the Turn Engine SHALL log the roll for replay/debugging
And if validation fails, the Turn Engine SHALL request a re-roll

## MODIFIED Requirements

### Requirement: Turn Engine Thread Safety
The Turn Engine SHALL ensure thread-safe access to CombatState when handling concurrent INTENTs and UI updates.

#### Scenario: Concurrent Combat State Access
Given multiple threads attempting to read or modify CombatState simultaneously
When combat operations occur
Then the Turn Engine SHALL use appropriate synchronization primitives
And the Turn Engine SHALL prevent race conditions
And the Turn Engine SHALL maintain state consistency

## Technical Constraints

### Performance Requirements
- Initiative calculation SHALL complete in < 10ms for 10 creatures
- Turn advancement SHALL complete in < 5ms
- LoS calculation SHALL complete in < 20ms for typical grid sizes
- AoE calculation SHALL complete in < 50ms for typical areas
- Condition processing SHALL complete in < 5ms per creature

### Grid Requirements
- Grid cell size SHALL be 5ft (standard D&D scale)
- Grid coordinates SHALL use (x, y) integer pairs
- Maximum grid size SHALL be 100x100 cells (500ft x 500ft)
- Grid cells SHALL support elevation (i16 range)

### Integration Requirements
- Turn Engine SHALL integrate with Orchestrator for INTENT processing
- Turn Engine SHALL integrate with rules5e-service for rule resolution
- Turn Engine SHALL emit events for Master AI narration
- Turn Engine SHALL send CombatUpdate messages to UI

## Implementation Notes

### Rust Module Structure
The Turn Engine is part of the game-engine module but coordinates closely with the Orchestrator:
```
src/game-engine/
├── turn.rs              # Turn order and progression
├── combat.rs            # Combat state management
└── grid.rs              # Grid, LoS, AoE calculations
```

### Dependencies
- `orchestrator` for INTENT processing
- `rules5e-service` for rule resolution
- `serde` for serialization
- `uuid` for encounter IDs

### Testing Requirements
- Unit tests for initiative calculation (100% coverage)
- Unit tests for turn progression (100% coverage)
- Unit tests for action economy (100% coverage)
- Unit tests for LoS and range validation (100% coverage)
- Integration tests for combat flow
- E2E tests for full combat scenarios











