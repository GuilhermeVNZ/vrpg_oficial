// Rules5e Service - Deterministic D&D 5e rules engine
// This module provides deterministic D&D 5e rule calculations

pub mod ability;
pub mod ability_scores;
pub mod attack;
pub mod condition;
pub mod cr_xp;
pub mod damage;
pub mod dice;
pub mod error;
pub mod server;
pub mod skills;
pub mod spells;
pub mod weapons;

pub use ability::{Ability, AbilityCheckRequest, AbilityCheckResult, AbilityChecker};
pub use ability_scores::{
    AbilityGenerationMethod, AbilityScoreGenerator, AbilityScoreType, AbilityScores,
};
pub use attack::{AttackRequest, AttackResolver, AttackResult};
pub use condition::{Condition, ConditionApplication, ConditionManager, ConditionType};
pub use cr_xp::{xp_to_cr_approximate, ChallengeRating};
pub use damage::{DamageRequest, DamageResolver, DamageResult, DamageType};
pub use dice::{DiceExpression, DiceRoller, RollMode, RollResult};
pub use error::{Result, RulesError};
pub use server::RulesServer;
pub use skills::{
    Skill, SkillBonus, SkillCalculator, SkillCheckRequest, SkillCheckResult, SkillProficiency,
};
pub use spells::{
    AreaOfEffect, CastingTime, Spell, SpellAttackType, SpellCastRequest, SpellCastResult,
    SpellCaster, SpellComponents, SpellDatabase, SpellDuration, SpellEffect, SpellLevel,
    SpellRange, SpellSavingThrow, SpellSchool, SpellSlots,
};
pub use weapons::{Weapon, WeaponCategory, WeaponDatabase, WeaponProperty, WeaponType};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        assert!(true);
    }
}
