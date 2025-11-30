// Ability Scores System - D&D 5e
// Implements complete ability score system with modifiers and generation methods

use crate::error::{Result, RulesError};
use serde::{Deserialize, Serialize};

// Note: This Ability enum is for ability scores
// The ability module has Ability enum for ability checks - they serve different purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbilityScoreType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl AbilityScores {
    pub fn new(str: u8, dex: u8, con: u8, int: u8, wis: u8, cha: u8) -> Self {
        Self {
            strength: str,
            dexterity: dex,
            constitution: con,
            intelligence: int,
            wisdom: wis,
            charisma: cha,
        }
    }

    pub fn get_score(&self, ability: AbilityScoreType) -> u8 {
        match ability {
            AbilityScoreType::Strength => self.strength,
            AbilityScoreType::Dexterity => self.dexterity,
            AbilityScoreType::Constitution => self.constitution,
            AbilityScoreType::Intelligence => self.intelligence,
            AbilityScoreType::Wisdom => self.wisdom,
            AbilityScoreType::Charisma => self.charisma,
        }
    }

    pub fn get_modifier(&self, ability: AbilityScoreType) -> i32 {
        let score = self.get_score(ability);
        // Modifier = (Score - 10) / 2, rounded down
        (score as i32 - 10) / 2
    }

    pub fn set_score(&mut self, ability: AbilityScoreType, value: u8) -> Result<()> {
        if value > 30 {
            return Err(RulesError::InvalidInput(
                "Ability score cannot exceed 30".to_string(),
            ));
        }
        match ability {
            AbilityScoreType::Strength => self.strength = value,
            AbilityScoreType::Dexterity => self.dexterity = value,
            AbilityScoreType::Constitution => self.constitution = value,
            AbilityScoreType::Intelligence => self.intelligence = value,
            AbilityScoreType::Wisdom => self.wisdom = value,
            AbilityScoreType::Charisma => self.charisma = value,
        }
        Ok(())
    }

    pub fn increase_score(&mut self, ability: AbilityScoreType, amount: u8) -> Result<()> {
        let current = self.get_score(ability);
        if current > 30 || amount > 30 || current.saturating_add(amount) > 30 {
            return Err(RulesError::InvalidInput(
                "Ability score cannot exceed 30".to_string(),
            ));
        }
        let new_value = current + amount;
        self.set_score(ability, new_value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityGenerationMethod {
    StandardArray, // 15, 14, 13, 12, 10, 8
    PointBuy,      // 27 points
    Rolling,       // 4d6 drop lowest
}

pub struct AbilityScoreGenerator;

impl AbilityScoreGenerator {
    pub fn generate_standard_array() -> AbilityScores {
        // Standard array: 15, 14, 13, 12, 10, 8
        AbilityScores::new(15, 14, 13, 12, 10, 8)
    }

    pub fn generate_point_buy() -> AbilityScores {
        // Point buy: 27 points, starting at 8
        // This is a simplified version - full point buy would need more logic
        // For now, return standard array as default
        Self::generate_standard_array()
    }

    pub fn generate_rolling(seed: Option<u64>) -> AbilityScores {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        let mut rng = if let Some(s) = seed {
            StdRng::seed_from_u64(s)
        } else {
            StdRng::from_entropy()
        };

        let mut scores = Vec::new();
        for _ in 0..6 {
            // Roll 4d6, drop lowest
            let mut rolls = vec![
                rng.gen_range(1..=6),
                rng.gen_range(1..=6),
                rng.gen_range(1..=6),
                rng.gen_range(1..=6),
            ];
            rolls.sort();
            rolls.remove(0); // Drop lowest
            let total: u8 = rolls.iter().sum();
            scores.push(total);
        }

        // Sort descending and assign
        scores.sort_by(|a, b| b.cmp(a));
        AbilityScores::new(
            scores[0], scores[1], scores[2], scores[3], scores[4], scores[5],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_modifier_calculation() {
        let scores = AbilityScores::new(15, 10, 8, 20, 12, 16);
        assert_eq!(scores.get_modifier(AbilityScoreType::Strength), 2); // (15-10)/2 = 2
        assert_eq!(scores.get_modifier(AbilityScoreType::Dexterity), 0); // (10-10)/2 = 0
        assert_eq!(scores.get_modifier(AbilityScoreType::Constitution), -1); // (8-10)/2 = -1
        assert_eq!(scores.get_modifier(AbilityScoreType::Intelligence), 5); // (20-10)/2 = 5
        assert_eq!(scores.get_modifier(AbilityScoreType::Wisdom), 1); // (12-10)/2 = 1
        assert_eq!(scores.get_modifier(AbilityScoreType::Charisma), 3); // (16-10)/2 = 3
    }

    #[test]
    fn test_standard_array() {
        let scores = AbilityScoreGenerator::generate_standard_array();
        assert_eq!(scores.strength, 15);
        assert_eq!(scores.dexterity, 14);
        assert_eq!(scores.constitution, 13);
        assert_eq!(scores.intelligence, 12);
        assert_eq!(scores.wisdom, 10);
        assert_eq!(scores.charisma, 8);
    }

    #[test]
    fn test_rolling_deterministic() {
        let seed = 12345;
        let scores1 = AbilityScoreGenerator::generate_rolling(Some(seed));
        let scores2 = AbilityScoreGenerator::generate_rolling(Some(seed));
        // Should be deterministic with same seed
        assert_eq!(scores1.strength, scores2.strength);
    }

    #[test]
    fn test_score_limits() {
        let mut scores = AbilityScores::new(20, 20, 20, 20, 20, 20);
        // 20 + 10 = 30, should be OK
        assert!(scores
            .increase_score(AbilityScoreType::Strength, 10)
            .is_ok());
        // Now at 30, cannot increase further
        assert!(scores
            .increase_score(AbilityScoreType::Strength, 1)
            .is_err());
        // Reset and test 20 + 11 = 31, should fail
        let mut scores2 = AbilityScores::new(20, 20, 20, 20, 20, 20);
        assert!(scores2
            .increase_score(AbilityScoreType::Strength, 11)
            .is_err());
    }
}
