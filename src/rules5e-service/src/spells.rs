//! Spell System - D&D 5e
//! Implements complete spell system including spell slots, casting, and effects

use crate::dice::{DiceExpression, DiceRoller};
use crate::error::{Result, RulesError};
use serde::{Deserialize, Serialize};

/// Spell level (0-9, where 0 is cantrip)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SpellLevel(pub u8);

impl SpellLevel {
    pub fn new(level: u8) -> Result<Self> {
        if level > 9 {
            return Err(RulesError::InvalidInput(
                "Spell level cannot exceed 9".to_string(),
            ));
        }
        Ok(Self(level))
    }

    pub fn is_cantrip(&self) -> bool {
        self.0 == 0
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Spell school
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl SpellSchool {
    pub fn name(&self) -> &'static str {
        match self {
            SpellSchool::Abjuration => "Abjuration",
            SpellSchool::Conjuration => "Conjuration",
            SpellSchool::Divination => "Divination",
            SpellSchool::Enchantment => "Enchantment",
            SpellSchool::Evocation => "Evocation",
            SpellSchool::Illusion => "Illusion",
            SpellSchool::Necromancy => "Necromancy",
            SpellSchool::Transmutation => "Transmutation",
        }
    }
}

/// Spell components
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellComponents {
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub material_description: Option<String>, // Required if material is true
    pub consumes_material: bool,              // True if material is consumed
}

impl SpellComponents {
    pub fn new() -> Self {
        Self {
            verbal: false,
            somatic: false,
            material: false,
            material_description: None,
            consumes_material: false,
        }
    }

    pub fn with_verbal(mut self) -> Self {
        self.verbal = true;
        self
    }

    pub fn with_somatic(mut self) -> Self {
        self.somatic = true;
        self
    }

    pub fn with_material(mut self, description: String, consumed: bool) -> Self {
        self.material = true;
        self.material_description = Some(description);
        self.consumes_material = consumed;
        self
    }
}

impl Default for SpellComponents {
    fn default() -> Self {
        Self::new()
    }
}

/// Casting time
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CastingTime {
    Action,
    BonusAction,
    Reaction { trigger: String },
    Minute(u32),
    Hour(u32),
    LongRest,
}

/// Spell range
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellRange {
    Touch,
    Self_,
    SelfRadius { radius: u32 }, // Radius in feet
    Feet(u32),                  // Range in feet
    Miles(u32),                 // Range in miles
    Unlimited,
    Special(String), // Special range description
}

/// Spell duration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellDuration {
    Instantaneous,
    Round(u32),
    Minute(u32),
    Hour(u32),
    Day(u32),
    UntilDispelled,
    UntilDispelledOrTriggered,
    Special(String), // Special duration description
}

/// Area of effect
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AreaOfEffect {
    None,
    Cone { length: u32 },      // Length in feet
    Cube { size: u32 },        // Size in feet
    Cylinder { radius: u32, height: u32 },
    Line { length: u32, width: u32 },
    Sphere { radius: u32 },
    Square { size: u32 },
}

/// Spell attack type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellAttackType {
    None,
    Melee,
    Ranged,
}

/// Saving throw information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellSavingThrow {
    pub ability: String, // "strength", "dexterity", etc.
    pub success: String, // What happens on success
    pub failure: String, // What happens on failure
}

/// Spell effect
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellEffect {
    pub damage: Option<DiceExpression>,     // Damage dice if spell deals damage
    pub damage_type: Option<String>,        // Type of damage
    pub healing: Option<DiceExpression>,    // Healing dice if spell heals
    pub condition: Option<String>,          // Condition applied
    pub description: String,                // Effect description
}

/// Spell structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub level: SpellLevel,
    pub school: SpellSchool,
    pub casting_time: CastingTime,
    pub range: SpellRange,
    pub components: SpellComponents,
    pub duration: SpellDuration,
    pub description: String,
    pub higher_levels: Option<String>, // Description of effects at higher levels
    pub classes: Vec<String>,          // Classes that can cast this spell
    pub ritual: bool,
    pub concentration: bool,
    pub area_of_effect: AreaOfEffect,
    pub attack_type: SpellAttackType,
    pub saving_throw: Option<SpellSavingThrow>,
    pub effect: SpellEffect,
}

impl Spell {
    pub fn can_be_cast_as_ritual(&self) -> bool {
        self.ritual
    }

    pub fn requires_concentration(&self) -> bool {
        self.concentration
    }

    pub fn can_be_upcast(&self) -> bool {
        self.higher_levels.is_some() && !self.level.is_cantrip()
    }

    pub fn is_cantrip(&self) -> bool {
        self.level.is_cantrip()
    }
}

/// Spell slots structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellSlots {
    /// Available spell slots by level (1-9)
    /// Key is spell level, value is (used, total)
    pub slots: std::collections::HashMap<u8, (u32, u32)>,
    /// Maximum spell level available
    pub max_level: u8,
}

impl SpellSlots {
    pub fn new(max_level: u8) -> Result<Self> {
        if max_level > 9 {
            return Err(RulesError::InvalidInput(
                "Spell level cannot exceed 9".to_string(),
            ));
        }

        let mut slots = std::collections::HashMap::new();
        for level in 1..=max_level {
            slots.insert(level, (0, 0));
        }

        Ok(Self { slots, max_level })
    }

    /// Create spell slots for a full caster (Wizard, Cleric, etc.)
    pub fn for_full_caster(level: u8) -> Result<Self> {
        if level > 20 {
            return Err(RulesError::InvalidInput(
                "Character level cannot exceed 20".to_string(),
            ));
        }

        // D&D 5e spell slots table for full casters
        let table: &[(u8, &[(u8, u32)])] = &[
            // Level -> [(Spell Level, Slots)]
            (1, &[(1, 2)]),
            (2, &[(1, 3)]),
            (3, &[(1, 4), (2, 2)]),
            (4, &[(1, 4), (2, 3)]),
            (5, &[(1, 4), (2, 3), (3, 2)]),
            (6, &[(1, 4), (2, 3), (3, 3)]),
            (7, &[(1, 4), (2, 3), (3, 3), (4, 1)]),
            (8, &[(1, 4), (2, 3), (3, 3), (4, 2)]),
            (9, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 1)]),
            (10, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2)]),
            (11, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1)]),
            (12, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1)]),
            (13, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1)]),
            (14, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1)]),
            (15, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1), (8, 1)]),
            (16, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1), (8, 1)]),
            (17, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1), (8, 1), (9, 1)]),
            (18, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 3), (6, 1), (7, 1), (8, 1), (9, 1)]),
            (19, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 3), (6, 2), (7, 1), (8, 1), (9, 1)]),
            (20, &[(1, 4), (2, 3), (3, 3), (4, 3), (5, 3), (6, 2), (7, 2), (8, 1), (9, 1)]),
        ];

        let slots_data = table
            .iter()
            .rev()
            .find(|(lvl, _)| *lvl <= level)
            .map(|(_, data)| *data)
            .unwrap_or(&[(1, 2)]);

        let mut slots = std::collections::HashMap::new();
        let mut max_level = 0;

        for (spell_level, slot_count) in slots_data {
            slots.insert(*spell_level, (0, *slot_count));
            if *spell_level > max_level {
                max_level = *spell_level;
            }
        }

        Ok(Self { slots, max_level })
    }

    /// Get available slots for a spell level
    pub fn available(&self, level: u8) -> u32 {
        self.slots
            .get(&level)
            .map(|(used, total)| total.saturating_sub(*used))
            .unwrap_or(0)
    }

    /// Use a spell slot
    pub fn use_slot(&mut self, level: u8) -> Result<()> {
        if level == 0 {
            // Cantrips don't use slots
            return Ok(());
        }

        let (used, total) = self
            .slots
            .get(&level)
            .copied()
            .ok_or_else(|| RulesError::InvalidInput(format!("No spell slots at level {}", level)))?;

        if used >= total {
            return Err(RulesError::InvalidInput(format!(
                "No available spell slots at level {}",
                level
            )));
        }

        self.slots.insert(level, (used + 1, total));
        Ok(())
    }

    /// Restore a spell slot (short rest, etc.)
    pub fn restore_slot(&mut self, level: u8) -> Result<()> {
        if level == 0 {
            return Ok(());
        }

        let (used, total) = self
            .slots
            .get(&level)
            .copied()
            .ok_or_else(|| RulesError::InvalidInput(format!("No spell slots at level {}", level)))?;

        if used == 0 {
            return Ok(()); // Already at max
        }

        self.slots.insert(level, (used - 1, total));
        Ok(())
    }

    /// Restore all spell slots (long rest)
    pub fn restore_all(&mut self) {
        let levels_and_totals: Vec<(u8, u32)> = self
            .slots
            .iter()
            .map(|(level, (_, total))| (*level, *total))
            .collect();
        for (level, total) in levels_and_totals {
            self.slots.insert(level, (0, total));
        }
    }

    /// Get all used slots count
    pub fn total_used(&self) -> u32 {
        self.slots.values().map(|(used, _)| used).sum()
    }

    /// Get all total slots count
    pub fn total_slots(&self) -> u32 {
        self.slots.values().map(|(_, total)| total).sum()
    }
}

/// Spell casting request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCastRequest {
    pub spell_name: String,
    pub slot_level: Option<u8>, // None for cantrips, Some(level) for upcasting
    pub caster_level: u8,
    pub spell_ability_modifier: i32,
    pub spell_save_dc: i32, // 8 + proficiency + ability modifier
    pub spell_attack_bonus: i32, // Proficiency + ability modifier
    pub seed: Option<u64>,
}

/// Spell casting result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCastResult {
    pub spell_name: String,
    pub slot_used: Option<u8>,
    pub success: bool,
    pub attack_roll: Option<i32>,        // If spell requires attack roll
    pub saving_throw_result: Option<bool>, // If spell requires saving throw (true = save, false = fail)
    pub damage: Option<i32>,
    pub healing: Option<i32>,
    pub effects_applied: Vec<String>,
}

/// Spell caster
pub struct SpellCaster;

impl SpellCaster {
    pub fn new() -> Self {
        Self
    }

    /// Calculate upcast damage based on spell level difference
    /// Most spells scale by +1 die per level above base level
    fn calculate_upcast_damage(
        &self,
        base_expr: &DiceExpression,
        base_level: u8,
        cast_level: u8,
    ) -> DiceExpression {
        if cast_level <= base_level {
            return base_expr.clone();
        }

        let level_diff = cast_level - base_level;

        // Try to parse upcasting from higher_levels description
        // For now, default to +1 die per level above base
        // Common patterns:
        // - "+1d6 per level above Xth" -> add level_diff dice
        // - "+1dX per level above Xth" -> add level_diff dice of same type

        // For now, assume standard scaling: +1 die per level
        DiceExpression {
            count: base_expr.count + level_diff as u32,
            sides: base_expr.sides,
            modifier: base_expr.modifier,
        }
    }

    /// Calculate upcast healing based on spell level difference
    fn calculate_upcast_healing(
        &self,
        base_expr: &DiceExpression,
        base_level: u8,
        cast_level: u8,
    ) -> DiceExpression {
        if cast_level <= base_level {
            return base_expr.clone();
        }

        let level_diff = cast_level - base_level;

        // Standard healing scaling: +1 die per level above base
        DiceExpression {
            count: base_expr.count + level_diff as u32,
            sides: base_expr.sides,
            modifier: base_expr.modifier,
        }
    }

    /// Validate if a spell can be cast
    pub fn validate_cast(
        &self,
        spell: &Spell,
        slots: &SpellSlots,
        slot_level: Option<u8>,
    ) -> Result<()> {
        // Cantrips don't require slots
        if spell.is_cantrip() {
            return Ok(());
        }

        // Determine the level to use
        let cast_level = slot_level.unwrap_or(spell.level.value());

        // Can't cast at lower level than spell's base level
        if cast_level < spell.level.value() {
            return Err(RulesError::InvalidInput(format!(
                "Cannot cast {} at level {} (minimum level {})",
                spell.name,
                cast_level,
                spell.level.value()
            )));
        }

        // Check if slot is available
        if slots.available(cast_level) == 0 {
            return Err(RulesError::InvalidInput(format!(
                "No available spell slots at level {}",
                cast_level
            )));
        }

        Ok(())
    }

    /// Cast a spell
    pub fn cast(
        &self,
        spell: &Spell,
        request: &SpellCastRequest,
        slots: &mut SpellSlots,
    ) -> Result<SpellCastResult> {
        // Validate cast
        self.validate_cast(spell, slots, request.slot_level)?;

        // Determine slot level to use
        let slot_level = if spell.is_cantrip() {
            None
        } else {
            Some(request.slot_level.unwrap_or(spell.level.value()))
        };

        // Use spell slot (if not cantrip)
        if let Some(level) = slot_level {
            slots.use_slot(level)?;
        }

        // Calculate effects
        let mut result = SpellCastResult {
            spell_name: spell.name.clone(),
            slot_used: slot_level,
            success: true,
            attack_roll: None,
            saving_throw_result: None,
            damage: None,
            healing: None,
            effects_applied: vec![],
        };

        // Handle attack roll spells
        if spell.attack_type != SpellAttackType::None {
            let mut roller = if let Some(seed) = request.seed {
                DiceRoller::with_seed(seed)
            } else {
                DiceRoller::new()
            };

            let dice_expr = crate::dice::DiceExpression {
                count: 1,
                sides: 20,
                modifier: request.spell_attack_bonus,
            };

            let roll_result = roller.roll(&dice_expr, crate::dice::RollMode::Normal)?;
            result.attack_roll = Some(roll_result.total);
            result.success = roll_result.total >= 10; // Default AC 10, can be overridden
        }

        // Handle saving throw spells
        if spell.saving_throw.is_some() {
            // Saving throw will be resolved externally with actual DC and targets
            // For now, just mark that it requires one
            result.saving_throw_result = None; // To be filled by caller
        }

        // Calculate damage/healing if applicable
        // Handle upcasting: calculate damage at cast level
        let base_level = spell.level.value();
        let cast_level = slot_level.unwrap_or(base_level);

        if let Some(ref damage_expr) = spell.effect.damage {
            let mut roller = if let Some(seed) = request.seed {
                DiceRoller::with_seed(seed)
            } else {
                DiceRoller::new()
            };

            // Calculate upcast damage (only if cast at higher level than base)
            let damage_expr_to_use = if cast_level > base_level {
                self.calculate_upcast_damage(damage_expr, base_level, cast_level)
            } else {
                damage_expr.clone()
            };

            let roll_result = roller.roll(&damage_expr_to_use, crate::dice::RollMode::Normal)?;
            result.damage = Some(roll_result.total);
        }

        if let Some(ref healing_expr) = spell.effect.healing {
            let mut roller = if let Some(seed) = request.seed {
                DiceRoller::with_seed(seed)
            } else {
                DiceRoller::new()
            };

            // Calculate upcast healing (only if cast at higher level than base)
            let healing_expr_to_use = if cast_level > base_level {
                self.calculate_upcast_healing(healing_expr, base_level, cast_level)
            } else {
                healing_expr.clone()
            };

            let roll_result = roller.roll(&healing_expr_to_use, crate::dice::RollMode::Normal)?;
            result.healing = Some(roll_result.total);
        }

        Ok(result)
    }
}

impl Default for SpellCaster {
    fn default() -> Self {
        Self::new()
    }
}

/// Spell database
pub struct SpellDatabase {
    spells: std::collections::HashMap<String, Spell>,
}

impl SpellDatabase {
    pub fn new() -> Self {
        Self {
            spells: std::collections::HashMap::new(),
        }
    }

    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.insert(spell.name.clone(), spell);
    }

    pub fn get_spell(&self, name: &str) -> Option<&Spell> {
        self.spells.get(name)
    }

    pub fn list_spells(&self) -> Vec<&Spell> {
        self.spells.values().collect()
    }

    pub fn search_spells(&self, query: &str) -> Vec<&Spell> {
        let query_lower = query.to_lowercase();
        self.spells
            .values()
            .filter(|spell| {
                spell.name.to_lowercase().contains(&query_lower)
                    || spell.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn get_spells_by_level(&self, level: u8) -> Vec<&Spell> {
        self.spells
            .values()
            .filter(|spell| spell.level.value() == level)
            .collect()
    }

    pub fn get_spells_by_school(&self, school: SpellSchool) -> Vec<&Spell> {
        self.spells
            .values()
            .filter(|spell| spell.school == school)
            .collect()
    }

    pub fn get_spells_by_class(&self, class: &str) -> Vec<&Spell> {
        let class_lower = class.to_lowercase();
        self.spells
            .values()
            .filter(|spell| {
                spell
                    .classes
                    .iter()
                    .any(|c| c.to_lowercase() == class_lower)
            })
            .collect()
    }
}

impl Default for SpellDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_level() {
        let level1 = SpellLevel::new(1).unwrap();
        assert!(!level1.is_cantrip());
        assert_eq!(level1.value(), 1);

        let cantrip = SpellLevel::new(0).unwrap();
        assert!(cantrip.is_cantrip());
    }

    #[test]
    fn test_spell_slots_for_full_caster() {
        let slots = SpellSlots::for_full_caster(5).unwrap();
        assert_eq!(slots.available(1), 4);
        assert_eq!(slots.available(2), 3);
        assert_eq!(slots.available(3), 2);
    }

    #[test]
    fn test_spell_slots_use() {
        let mut slots = SpellSlots::for_full_caster(5).unwrap();
        assert_eq!(slots.available(1), 4);
        
        slots.use_slot(1).unwrap();
        assert_eq!(slots.available(1), 3);
        
        slots.use_slot(1).unwrap();
        assert_eq!(slots.available(1), 2);
    }

    #[test]
    fn test_spell_slots_restore_all() {
        let mut slots = SpellSlots::for_full_caster(5).unwrap();
        slots.use_slot(1).unwrap();
        slots.use_slot(2).unwrap();
        
        assert_eq!(slots.available(1), 3);
        assert_eq!(slots.available(2), 2);
        
        slots.restore_all();
        assert_eq!(slots.available(1), 4);
        assert_eq!(slots.available(2), 3);
    }

    #[test]
    fn test_spell_database() {
        let mut db = SpellDatabase::new();
        
        let spell = Spell {
            name: "Fireball".to_string(),
            level: SpellLevel::new(3).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(150),
            components: SpellComponents::new()
                .with_verbal()
                .with_somatic()
                .with_material("A tiny ball of bat guano and sulfur".to_string(), false),
            duration: SpellDuration::Instantaneous,
            description: "A bright streak flashes from your pointing finger...".to_string(),
            higher_levels: Some("At higher levels: +1d6 damage per level above 3rd".to_string()),
            classes: vec!["Sorcerer".to_string(), "Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::Sphere { radius: 20 },
            attack_type: SpellAttackType::None,
            saving_throw: Some(SpellSavingThrow {
                ability: "dexterity".to_string(),
                success: "Half damage".to_string(),
                failure: "Full damage".to_string(),
            }),
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 8,
                    sides: 6,
                    modifier: 0,
                }),
                damage_type: Some("fire".to_string()),
                healing: None,
                condition: None,
                description: "Deals fire damage in a 20-foot radius sphere".to_string(),
            },
        };

        db.add_spell(spell);
        
        let found = db.get_spell("Fireball");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Fireball");
    }

    #[test]
    fn test_spell_caster_validate_cantrip() {
        let caster = SpellCaster::new();
        let slots = SpellSlots::for_full_caster(5).unwrap();

        let cantrip = Spell {
            name: "Fire Bolt".to_string(),
            level: SpellLevel::new(0).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(120),
            components: SpellComponents::new().with_verbal().with_somatic(),
            duration: SpellDuration::Instantaneous,
            description: "A bolt of fire...".to_string(),
            higher_levels: None,
            classes: vec!["Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::None,
            attack_type: SpellAttackType::Ranged,
            saving_throw: None,
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                }),
                damage_type: Some("fire".to_string()),
                healing: None,
                condition: None,
                description: "Deals fire damage".to_string(),
            },
        };

        // Cantrips don't require slots
        assert!(caster.validate_cast(&cantrip, &slots, None).is_ok());
    }

    #[test]
    fn test_spell_caster_validate_requires_slot() {
        let caster = SpellCaster::new();
        let mut slots = SpellSlots::for_full_caster(5).unwrap();

        let fireball = Spell {
            name: "Fireball".to_string(),
            level: SpellLevel::new(3).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(150),
            components: SpellComponents::new()
                .with_verbal()
                .with_somatic()
                .with_material("A tiny ball of bat guano and sulfur".to_string(), false),
            duration: SpellDuration::Instantaneous,
            description: "A bright streak flashes...".to_string(),
            higher_levels: Some("At higher levels: +1d6 damage per level above 3rd".to_string()),
            classes: vec!["Sorcerer".to_string(), "Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::Sphere { radius: 20 },
            attack_type: SpellAttackType::None,
            saving_throw: Some(SpellSavingThrow {
                ability: "dexterity".to_string(),
                success: "Half damage".to_string(),
                failure: "Full damage".to_string(),
            }),
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 8,
                    sides: 6,
                    modifier: 0,
                }),
                damage_type: Some("fire".to_string()),
                healing: None,
                condition: None,
                description: "Deals fire damage".to_string(),
            },
        };

        // Should validate with available slots
        assert!(caster.validate_cast(&fireball, &slots, None).is_ok());

        // Use all 3rd level slots
        slots.use_slot(3).unwrap();
        slots.use_slot(3).unwrap();

        // Should fail validation
        assert!(caster.validate_cast(&fireball, &slots, None).is_err());
    }

    #[test]
    fn test_spell_caster_cast_cantrip() {
        let caster = SpellCaster::new();
        let mut slots = SpellSlots::for_full_caster(5).unwrap();

        let cantrip = Spell {
            name: "Fire Bolt".to_string(),
            level: SpellLevel::new(0).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(120),
            components: SpellComponents::new().with_verbal().with_somatic(),
            duration: SpellDuration::Instantaneous,
            description: "A bolt of fire...".to_string(),
            higher_levels: None,
            classes: vec!["Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::None,
            attack_type: SpellAttackType::Ranged,
            saving_throw: None,
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                }),
                damage_type: Some("fire".to_string()),
                healing: None,
                condition: None,
                description: "Deals fire damage".to_string(),
            },
        };

        let request = SpellCastRequest {
            spell_name: "Fire Bolt".to_string(),
            slot_level: None,
            caster_level: 5,
            spell_ability_modifier: 3,
            spell_save_dc: 14,
            spell_attack_bonus: 6,
            seed: Some(12345),
        };

        let result = caster.cast(&cantrip, &request, &mut slots).unwrap();
        assert_eq!(result.spell_name, "Fire Bolt");
        assert_eq!(result.slot_used, None); // Cantrips don't use slots
        assert!(result.attack_roll.is_some());
        assert!(result.damage.is_some());
    }

    #[test]
    fn test_spell_database_search() {
        let mut db = SpellDatabase::new();

        let fireball = Spell {
            name: "Fireball".to_string(),
            level: SpellLevel::new(3).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(150),
            components: SpellComponents::new().with_verbal().with_somatic(),
            duration: SpellDuration::Instantaneous,
            description: "A bright streak flashes from your pointing finger...".to_string(),
            higher_levels: None,
            classes: vec!["Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::Sphere { radius: 20 },
            attack_type: SpellAttackType::None,
            saving_throw: None,
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 8,
                    sides: 6,
                    modifier: 0,
                }),
                damage_type: Some("fire".to_string()),
                healing: None,
                condition: None,
                description: "Deals fire damage".to_string(),
            },
        };

        let magic_missile = Spell {
            name: "Magic Missile".to_string(),
            level: SpellLevel::new(1).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(120),
            components: SpellComponents::new().with_verbal().with_somatic(),
            duration: SpellDuration::Instantaneous,
            description: "You create three glowing darts of magical force...".to_string(),
            higher_levels: None,
            classes: vec!["Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::None,
            attack_type: SpellAttackType::None,
            saving_throw: None,
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 3,
                    sides: 4,
                    modifier: 3,
                }),
                damage_type: Some("force".to_string()),
                healing: None,
                condition: None,
                description: "Deals force damage".to_string(),
            },
        };

        db.add_spell(fireball);
        db.add_spell(magic_missile);

        let results = db.search_spells("fire");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Fireball");

        let results = db.search_spells("missile");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Magic Missile");
    }

    #[test]
    fn test_spell_database_by_level() {
        let mut db = SpellDatabase::new();

        let fireball = Spell {
            name: "Fireball".to_string(),
            level: SpellLevel::new(3).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(150),
            components: SpellComponents::new().with_verbal().with_somatic(),
            duration: SpellDuration::Instantaneous,
            description: "A bright streak...".to_string(),
            higher_levels: None,
            classes: vec!["Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::None,
            attack_type: SpellAttackType::None,
            saving_throw: None,
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 8,
                    sides: 6,
                    modifier: 0,
                }),
                damage_type: Some("fire".to_string()),
                healing: None,
                condition: None,
                description: "Deals fire damage".to_string(),
            },
        };

        let magic_missile = Spell {
            name: "Magic Missile".to_string(),
            level: SpellLevel::new(1).unwrap(),
            school: SpellSchool::Evocation,
            casting_time: CastingTime::Action,
            range: SpellRange::Feet(120),
            components: SpellComponents::new().with_verbal().with_somatic(),
            duration: SpellDuration::Instantaneous,
            description: "You create three glowing darts...".to_string(),
            higher_levels: None,
            classes: vec!["Wizard".to_string()],
            ritual: false,
            concentration: false,
            area_of_effect: AreaOfEffect::None,
            attack_type: SpellAttackType::None,
            saving_throw: None,
            effect: SpellEffect {
                damage: Some(DiceExpression {
                    count: 3,
                    sides: 4,
                    modifier: 3,
                }),
                damage_type: Some("force".to_string()),
                healing: None,
                condition: None,
                description: "Deals force damage".to_string(),
            },
        };

        db.add_spell(fireball);
        db.add_spell(magic_missile);

        let level_1 = db.get_spells_by_level(1);
        assert_eq!(level_1.len(), 1);
        assert_eq!(level_1[0].name, "Magic Missile");

        let level_3 = db.get_spells_by_level(3);
        assert_eq!(level_3.len(), 1);
        assert_eq!(level_3[0].name, "Fireball");
    }

    #[test]
    fn test_spell_components() {
        let components = SpellComponents::new()
            .with_verbal()
            .with_somatic()
            .with_material("A piece of iron".to_string(), false);

        assert!(components.verbal);
        assert!(components.somatic);
        assert!(components.material);
        assert_eq!(components.material_description, Some("A piece of iron".to_string()));
        assert!(!components.consumes_material);
    }

    #[test]
    fn test_spell_slots_restore() {
        let mut slots = SpellSlots::for_full_caster(5).unwrap();
        
        slots.use_slot(1).unwrap();
        slots.use_slot(2).unwrap();
        
        assert_eq!(slots.available(1), 3);
        assert_eq!(slots.available(2), 2);
        
        slots.restore_slot(1).unwrap();
        assert_eq!(slots.available(1), 4);
        assert_eq!(slots.available(2), 2);
    }
}

