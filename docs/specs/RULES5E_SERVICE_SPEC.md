# Rules5e Service Technical Specification

## Purpose

The Rules5e Service is the deterministic D&D 5e rules engine, providing mechanical resolution for attacks, damage, ability checks, saving throws, conditions, and all numerical calculations. This specification defines the technical requirements for rule resolution, determinism, and integration with the Orchestrator.

## ADDED Requirements

### Requirement: Deterministic Dice Rolling
The Rules5e Service SHALL provide deterministic dice rolling using seed-based random number generation.

#### Scenario: Deterministic Roll with Seed
Given a dice expression "1d20+5" and a seed value
When rolling the dice
Then the Rules5e Service SHALL use the seed for RNG
And the Rules5e Service SHALL return a RollResult with total, natural, and breakdown
And the same seed SHALL always produce the same result
And the result SHALL be reproducible for replay

#### Scenario: Parse Dice Expression
Given a dice expression "2d8+3"
When parsing the expression
Then the Rules5e Service SHALL identify 2 dice of d8
And the Rules5e Service SHALL identify modifier +3
And the Rules5e Service SHALL validate the expression syntax
And the Rules5e Service SHALL return parsed components

#### Scenario: Roll with Advantage
Given a dice expression "1d20+5" with advantage
When rolling with advantage
Then the Rules5e Service SHALL roll 2d20
And the Rules5e Service SHALL take the higher result
And the Rules5e Service SHALL add the modifier to the higher roll
And the Rules5e Service SHALL indicate advantage was used

#### Scenario: Roll with Disadvantage
Given a dice expression "1d20+5" with disadvantage
When rolling with disadvantage
Then the Rules5e Service SHALL roll 2d20
And the Rules5e Service SHALL take the lower result
And the Rules5e Service SHALL add the modifier to the lower roll
And the Rules5e Service SHALL indicate disadvantage was used

### Requirement: Attack Resolution
The Rules5e Service SHALL resolve attacks, determining hit/miss, critical hits, and damage.

#### Scenario: Resolve Melee Attack
Given an AttackRequest with attacker (attack_bonus=+5), target (AC=15), attack_roll=18
When resolving the attack
Then the Rules5e Service SHALL compare attack_roll (18) to target AC (15)
And the Rules5e Service SHALL determine hit (18 >= 15)
And the Rules5e Service SHALL return AttackResult with hit=true
And the Rules5e Service SHALL NOT calculate damage (damage is separate)

#### Scenario: Resolve Critical Hit
Given an AttackRequest with natural roll=20
When resolving the attack
Then the Rules5e Service SHALL identify critical hit
And the Rules5e Service SHALL return AttackResult with critical=true
And the Rules5e Service SHALL indicate double damage dice

#### Scenario: Resolve Miss
Given an AttackRequest with attack_roll=12, target AC=15
When resolving the attack
Then the Rules5e Service SHALL compare 12 to 15
And the Rules5e Service SHALL determine miss (12 < 15)
And the Rules5e Service SHALL return AttackResult with hit=false
And the Rules5e Service SHALL NOT calculate damage

### Requirement: Damage Calculation
The Rules5e Service SHALL calculate damage, applying damage types, resistances, and vulnerabilities.

#### Scenario: Calculate Basic Damage
Given a DamageRequest with damage_roll="1d8+3", damage_type=Slashing
When calculating damage
Then the Rules5e Service SHALL roll 1d8+3
And the Rules5e Service SHALL return DamageResult with total damage
And the Rules5e Service SHALL include damage_type in result

#### Scenario: Apply Resistance
Given a DamageRequest with damage=10, damage_type=Fire
And target has Fire resistance
When calculating damage
Then the Rules5e Service SHALL apply resistance (half damage)
And the Rules5e Service SHALL return DamageResult with total=5
And the Rules5e Service SHALL indicate resistance was applied

#### Scenario: Apply Vulnerability
Given a DamageRequest with damage=10, damage_type=Cold
And target has Cold vulnerability
When calculating damage
Then the Rules5e Service SHALL apply vulnerability (double damage)
And the Rules5e Service SHALL return DamageResult with total=20
And the Rules5e Service SHALL indicate vulnerability was applied

#### Scenario: Apply Multiple Resistances
Given a DamageRequest with damage=10, damage_type=Fire
And target has Fire resistance and Cold resistance (but not Fire)
When calculating damage
Then the Rules5e Service SHALL apply only Fire resistance
And the Rules5e Service SHALL return DamageResult with total=5
And the Rules5e Service SHALL NOT apply Cold resistance

### Requirement: Ability Checks
The Rules5e Service SHALL resolve ability checks, comparing roll results to Difficulty Class (DC).

#### Scenario: Resolve Ability Check Success
Given an AbilityCheckRequest with ability=Strength, roll_total=18, DC=15
When resolving the check
Then the Rules5e Service SHALL compare 18 to DC 15
And the Rules5e Service SHALL determine success (18 >= 15)
And the Rules5e Service SHALL return AbilityCheckResult with success=true

#### Scenario: Resolve Ability Check Failure
Given an AbilityCheckRequest with ability=Dexterity, roll_total=12, DC=15
When resolving the check
Then the Rules5e Service SHALL compare 12 to DC 15
And the Rules5e Service SHALL determine failure (12 < 15)
And the Rules5e Service SHALL return AbilityCheckResult with success=false

#### Scenario: Resolve Ability Check with Advantage
Given an AbilityCheckRequest with advantage=true
When resolving the check
Then the Rules5e Service SHALL roll 2d20 and take higher
And the Rules5e Service SHALL add ability modifier to higher roll
And the Rules5e Service SHALL compare to DC

### Requirement: Saving Throws
The Rules5e Service SHALL resolve saving throws, applying proficiency bonuses when applicable.

#### Scenario: Resolve Saving Throw with Proficiency
Given a SavingThrowRequest with ability=Wisdom, proficiency=true, proficiency_bonus=+3, ability_mod=+2
When resolving the saving throw
Then the Rules5e Service SHALL roll 1d20
And the Rules5e Service SHALL add ability_mod (+2) and proficiency_bonus (+3)
And the Rules5e Service SHALL compare total to DC
And the Rules5e Service SHALL return SavingThrowResult

#### Scenario: Resolve Saving Throw without Proficiency
Given a SavingThrowRequest with ability=Constitution, proficiency=false, ability_mod=+1
When resolving the saving throw
Then the Rules5e Service SHALL roll 1d20
And the Rules5e Service SHALL add only ability_mod (+1)
And the Rules5e Service SHALL NOT add proficiency bonus
And the Rules5e Service SHALL compare total to DC

### Requirement: Condition Management (AUTO)
The Rules5e Service SHALL automatically manage conditions, applying, maintaining, and removing them according to D&D 5e rules.

#### Scenario: Apply Condition
Given a ConditionApplication with kind=Poisoned, duration=Rounds(3)
When applying the condition
Then the Rules5e Service SHALL add the condition to the creature
And the Rules5e Service SHALL store duration information
And the Rules5e Service SHALL emit ConditionApplied event
And the Rules5e Service SHALL track condition source

#### Scenario: Apply Condition Effect at Turn Start
Given a creature with condition "poisoned" (UntilStartOfTurn)
When the creature's turn starts
Then the Rules5e Service SHALL apply poison damage
And the Rules5e Service SHALL check if damage kills the creature
And the Rules5e Service SHALL emit ConditionTicked event
And the Rules5e Service SHALL remove condition if duration expires

#### Scenario: Reduce Condition Duration
Given a creature with condition "frightened" (Rounds(3))
When the creature's turn ends
Then the Rules5e Service SHALL reduce duration from 3 to 2
And the Rules5e Service SHALL keep condition active
And the Rules5e Service SHALL NOT emit ConditionEnded event

#### Scenario: Remove Expired Condition
Given a creature with condition "frightened" (Rounds(1))
When the creature's turn ends
Then the Rules5e Service SHALL reduce duration from 1 to 0
And the Rules5e Service SHALL remove the condition
And the Rules5e Service SHALL emit ConditionEnded event
And the Rules5e Service SHALL notify Orchestrator for Master AI narration

### Requirement: Skill Checks
The Rules5e Service SHALL resolve skill checks, applying skill proficiency and ability modifiers.

#### Scenario: Resolve Skill Check with Proficiency
Given a SkillCheckRequest with skill=Persuasion, proficiency=true, proficiency_bonus=+3, ability_mod=+2
When resolving the skill check
Then the Rules5e Service SHALL roll 1d20
And the Rules5e Service SHALL add ability_mod (+2) and proficiency_bonus (+3)
And the Rules5e Service SHALL compare total to DC
And the Rules5e Service SHALL return SkillCheckResult

#### Scenario: Resolve Skill Check with Expertise
Given a SkillCheckRequest with skill=Stealth, expertise=true, proficiency_bonus=+3
When resolving the skill check
Then the Rules5e Service SHALL apply double proficiency bonus (+6 instead of +3)
And the Rules5e Service SHALL add ability modifier
And the Rules5e Service SHALL compare to DC

### Requirement: Weapon Database
The Rules5e Service SHALL maintain a database of D&D 5e weapons with properties, damage, and range.

#### Scenario: Get Weapon Information
Given a weapon ID "weapon_longsword"
When querying weapon information
Then the Rules5e Service SHALL return Weapon with name, damage, properties
And the Rules5e Service SHALL include weapon type (melee/ranged)
And the Rules5e Service SHALL include range information if applicable

#### Scenario: List Weapons by Category
Given a request for melee weapons
When querying weapons
Then the Rules5e Service SHALL return all melee weapons
And the Rules5e Service SHALL include weapon properties
And the Rules5e Service SHALL filter by category correctly

### Requirement: HTTP API
The Rules5e Service SHALL expose HTTP endpoints for all rule operations.

#### Scenario: Health Check Endpoint
Given Rules5e Service is running
When GET /health is called
Then the Rules5e Service SHALL return status 200
And the response SHALL include service status
And the response SHALL include loaded weapon database count

#### Scenario: Roll Endpoint
Given POST /roll with {"expression": "2d8+3"}
When the endpoint is called
Then the Rules5e Service SHALL parse the expression
And the Rules5e Service SHALL roll the dice
And the Rules5e Service SHALL return RollResult with total, natural, breakdown
And the response SHALL be JSON formatted

#### Scenario: Attack Endpoint
Given POST /attack with AttackRequest
When the endpoint is called
Then the Rules5e Service SHALL resolve the attack
And the Rules5e Service SHALL return AttackResult
And the Rules5e Service SHALL include hit/miss, critical, and outcome

#### Scenario: Damage Endpoint
Given POST /damage with DamageRequest
When the endpoint is called
Then the Rules5e Service SHALL calculate damage
And the Rules5e Service SHALL apply resistances/vulnerabilities
And the Rules5e Service SHALL return DamageResult

#### Scenario: Ability Check Endpoint
Given POST /ability-check with AbilityCheckRequest
When the endpoint is called
Then the Rules5e Service SHALL resolve the ability check
And the Rules5e Service SHALL return AbilityCheckResult
And the Rules5e Service SHALL include success/failure and margin

#### Scenario: Saving Throw Endpoint
Given POST /saving-throw with SavingThrowRequest
When the endpoint is called
Then the Rules5e Service SHALL resolve the saving throw
And the Rules5e Service SHALL return SavingThrowResult
And the Rules5e Service SHALL include success/failure

## Technical Constraints

### Performance Requirements
- Dice roll resolution: < 1ms
- Attack resolution: < 5ms
- Damage calculation: < 2ms
- Ability check resolution: < 3ms
- Condition processing: < 5ms per creature

### Determinism Requirements
- Same seed + same expression = same result (100% reproducible)
- RNG SHALL use seed-based algorithm
- All calculations SHALL be deterministic (no floating point errors)

### Accuracy Requirements
- All calculations SHALL match D&D 5e SRD rules exactly
- Rounding SHALL follow D&D 5e conventions (round down)
- Edge cases SHALL be handled according to official rules

## Implementation Notes

### Rust Module Structure
```
src/rules5e-service/
├── lib.rs              # Public API
├── error.rs            # Error types
├── server.rs           # HTTP server
├── dice.rs             # Dice rolling
├── attack.rs           # Attack resolution
├── damage.rs           # Damage calculation
├── ability.rs          # Ability checks
├── condition.rs        # Condition management
├── skills.rs           # Skill checks
└── weapons.rs          # Weapon database
```

### Dependencies
- `serde` for serialization
- `tokio` for async runtime
- `tracing` for logging
- Deterministic RNG library

### Testing Requirements
- Unit tests for all rule calculations (100% coverage)
- Deterministic tests (same seed = same result)
- Edge case tests (critical hits, resistances, vulnerabilities)
- Integration tests for HTTP endpoints
- Performance benchmarks











