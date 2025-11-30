use crate::dice::{DiceRoller, RollMode};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityCheckRequest {
    pub ability: Ability,
    pub ability_modifier: i32,
    pub proficiency_bonus: i32,
    pub has_proficiency: bool,
    pub has_expertise: bool,
    pub dc: i32,
    pub advantage: bool,
    pub disadvantage: bool,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityCheckResult {
    pub roll: i32,
    pub natural_roll: u32,
    pub ability_modifier: i32,
    pub proficiency_bonus: i32,
    pub total: i32,
    pub dc: i32,
    pub success: bool,
}

pub struct AbilityChecker;

impl Default for AbilityChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl AbilityChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, request: &AbilityCheckRequest) -> Result<AbilityCheckResult> {
        if request.advantage && request.disadvantage {
            // Cancel out - normal roll
            return self.check_normal(request);
        }

        let mode = if request.advantage {
            RollMode::Advantage
        } else if request.disadvantage {
            RollMode::Disadvantage
        } else {
            RollMode::Normal
        };

        let mut roller = if let Some(seed) = request.seed {
            DiceRoller::with_seed(seed)
        } else {
            DiceRoller::new()
        };

        let dice_expr = crate::dice::DiceExpression {
            count: if mode == RollMode::Normal { 1 } else { 2 },
            sides: 20,
            modifier: 0,
        };

        let roll_result = roller.roll(&dice_expr, mode)?;
        let natural_roll = if mode == RollMode::Normal {
            roll_result.rolls[0]
        } else if mode == RollMode::Advantage {
            *roll_result.rolls.iter().max().unwrap()
        } else {
            *roll_result.rolls.iter().min().unwrap()
        };

        let mut proficiency_bonus = 0;
        if request.has_proficiency {
            proficiency_bonus = request.proficiency_bonus;
            if request.has_expertise {
                proficiency_bonus *= 2;
            }
        }

        let total = natural_roll as i32 + request.ability_modifier + proficiency_bonus;
        let success = total >= request.dc;

        Ok(AbilityCheckResult {
            roll: natural_roll as i32,
            natural_roll,
            ability_modifier: request.ability_modifier,
            proficiency_bonus,
            total,
            dc: request.dc,
            success,
        })
    }

    fn check_normal(&self, request: &AbilityCheckRequest) -> Result<AbilityCheckResult> {
        let mut roller = if let Some(seed) = request.seed {
            DiceRoller::with_seed(seed)
        } else {
            DiceRoller::new()
        };

        let dice_expr = crate::dice::DiceExpression {
            count: 1,
            sides: 20,
            modifier: 0,
        };

        let roll_result = roller.roll(&dice_expr, RollMode::Normal)?;
        let natural_roll = roll_result.rolls[0];

        let mut proficiency_bonus = 0;
        if request.has_proficiency {
            proficiency_bonus = request.proficiency_bonus;
            if request.has_expertise {
                proficiency_bonus *= 2;
            }
        }

        let total = natural_roll as i32 + request.ability_modifier + proficiency_bonus;
        let success = total >= request.dc;

        Ok(AbilityCheckResult {
            roll: natural_roll as i32,
            natural_roll,
            ability_modifier: request.ability_modifier,
            proficiency_bonus,
            total,
            dc: request.dc,
            success,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_check() {
        let checker = AbilityChecker::new();
        let request = AbilityCheckRequest {
            ability: Ability::Strength,
            ability_modifier: 3,
            proficiency_bonus: 2,
            has_proficiency: false,
            has_expertise: false,
            dc: 15,
            advantage: false,
            disadvantage: false,
            seed: Some(100),
        };
        let result = checker.check(&request).unwrap();
        assert!(result.roll >= 1 && result.roll <= 20);
        assert_eq!(result.ability_modifier, 3);
    }

    #[test]
    fn test_ability_check_with_proficiency() {
        let checker = AbilityChecker::new();
        let request = AbilityCheckRequest {
            ability: Ability::Strength,
            ability_modifier: 3,
            proficiency_bonus: 2,
            has_proficiency: true,
            has_expertise: false,
            dc: 15,
            advantage: false,
            disadvantage: false,
            seed: Some(100),
        };
        let result = checker.check(&request).unwrap();
        assert_eq!(result.proficiency_bonus, 2);
        assert_eq!(result.total, result.roll + 3 + 2);
    }

    #[test]
    fn test_ability_check_with_expertise() {
        let checker = AbilityChecker::new();
        let request = AbilityCheckRequest {
            ability: Ability::Strength,
            ability_modifier: 3,
            proficiency_bonus: 2,
            has_proficiency: true,
            has_expertise: true,
            dc: 15,
            advantage: false,
            disadvantage: false,
            seed: Some(100),
        };
        let result = checker.check(&request).unwrap();
        assert_eq!(result.proficiency_bonus, 4); // 2 * 2
        assert_eq!(result.total, result.roll + 3 + 4);
    }

    #[test]
    fn test_ability_check_advantage_disadvantage_cancel() {
        let checker = AbilityChecker::new();
        let request = AbilityCheckRequest {
            ability: Ability::Strength,
            ability_modifier: 3,
            proficiency_bonus: 2,
            has_proficiency: false,
            has_expertise: false,
            dc: 15,
            advantage: true,
            disadvantage: true,
            seed: Some(100),
        };
        let result = checker.check(&request).unwrap();
        // Should be normal roll (single die)
        assert!(result.roll >= 1 && result.roll <= 20);
    }
}
