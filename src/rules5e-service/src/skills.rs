// Skills System - D&D 5e
// Implements the 18 skills system with proficiency and expertise

use crate::ability_scores::{AbilityScoreType, AbilityScores};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Skill {
    // Strength skills
    Athletics,
    // Dexterity skills
    Acrobatics,
    SleightOfHand,
    Stealth,
    // Intelligence skills
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    // Wisdom skills
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    // Charisma skills
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

impl Skill {
    pub fn associated_ability(&self) -> AbilityScoreType {
        match self {
            Skill::Athletics => AbilityScoreType::Strength,
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => {
                AbilityScoreType::Dexterity
            }
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => AbilityScoreType::Intelligence,
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => AbilityScoreType::Wisdom,
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                AbilityScoreType::Charisma
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Skill::Athletics => "Athletics",
            Skill::Acrobatics => "Acrobatics",
            Skill::SleightOfHand => "Sleight of Hand",
            Skill::Stealth => "Stealth",
            Skill::Arcana => "Arcana",
            Skill::History => "History",
            Skill::Investigation => "Investigation",
            Skill::Nature => "Nature",
            Skill::Religion => "Religion",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Insight => "Insight",
            Skill::Medicine => "Medicine",
            Skill::Perception => "Perception",
            Skill::Survival => "Survival",
            Skill::Deception => "Deception",
            Skill::Intimidation => "Intimidation",
            Skill::Performance => "Performance",
            Skill::Persuasion => "Persuasion",
        }
    }

    pub fn all() -> Vec<Skill> {
        vec![
            Skill::Athletics,
            Skill::Acrobatics,
            Skill::SleightOfHand,
            Skill::Stealth,
            Skill::Arcana,
            Skill::History,
            Skill::Investigation,
            Skill::Nature,
            Skill::Religion,
            Skill::AnimalHandling,
            Skill::Insight,
            Skill::Medicine,
            Skill::Perception,
            Skill::Survival,
            Skill::Deception,
            Skill::Intimidation,
            Skill::Performance,
            Skill::Persuasion,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProficiency {
    pub skill: Skill,
    pub has_proficiency: bool,
    pub has_expertise: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBonus {
    pub skill: Skill,
    pub ability_modifier: i32,
    pub proficiency_bonus: i32,
    pub total_bonus: i32,
    pub has_proficiency: bool,
    pub has_expertise: bool,
}

pub struct SkillCalculator;

impl SkillCalculator {
    pub fn calculate_skill_bonus(
        ability_scores: &AbilityScores,
        skill: Skill,
        proficiency_bonus: i32,
        has_proficiency: bool,
        has_expertise: bool,
    ) -> SkillBonus {
        let ability_type = skill.associated_ability();
        let ability_modifier = ability_scores.get_modifier(ability_type);

        let prof_bonus = if has_expertise {
            proficiency_bonus * 2
        } else if has_proficiency {
            proficiency_bonus
        } else {
            0
        };

        SkillBonus {
            skill,
            ability_modifier,
            proficiency_bonus: prof_bonus,
            total_bonus: ability_modifier + prof_bonus,
            has_proficiency,
            has_expertise,
        }
    }

    pub fn calculate_all_skills(
        ability_scores: &AbilityScores,
        proficiencies: &[SkillProficiency],
        proficiency_bonus: i32,
    ) -> Vec<SkillBonus> {
        let mut skill_map = std::collections::HashMap::new();
        for prof in proficiencies {
            skill_map.insert(prof.skill, (prof.has_proficiency, prof.has_expertise));
        }

        Skill::all()
            .into_iter()
            .map(|skill| {
                let (has_prof, has_exp) = skill_map.get(&skill).copied().unwrap_or((false, false));
                Self::calculate_skill_bonus(
                    ability_scores,
                    skill,
                    proficiency_bonus,
                    has_prof,
                    has_exp,
                )
            })
            .collect()
    }

    pub fn passive_perception(
        ability_scores: &AbilityScores,
        proficiency_bonus: i32,
        has_proficiency: bool,
        has_expertise: bool,
    ) -> i32 {
        let skill_bonus = Self::calculate_skill_bonus(
            ability_scores,
            Skill::Perception,
            proficiency_bonus,
            has_proficiency,
            has_expertise,
        );
        10 + skill_bonus.total_bonus
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCheckRequest {
    pub skill: Skill,
    pub ability_scores: AbilityScores,
    pub proficiency_bonus: i32,
    pub has_proficiency: bool,
    pub has_expertise: bool,
    pub dc: i32,
    pub advantage: bool,
    pub disadvantage: bool,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCheckResult {
    pub skill: Skill,
    pub skill_bonus: SkillBonus,
    pub roll: i32,
    pub natural_roll: u32,
    pub total: i32,
    pub dc: i32,
    pub success: bool,
}

impl SkillCheckResult {
    pub fn from_ability_check(
        skill: Skill,
        ability_scores: &AbilityScores,
        proficiency_bonus: i32,
        has_proficiency: bool,
        has_expertise: bool,
        ability_check_result: crate::ability::AbilityCheckResult,
    ) -> Self {
        let skill_bonus = SkillCalculator::calculate_skill_bonus(
            ability_scores,
            skill,
            proficiency_bonus,
            has_proficiency,
            has_expertise,
        );

        Self {
            skill,
            skill_bonus,
            roll: ability_check_result.roll,
            natural_roll: ability_check_result.natural_roll,
            total: ability_check_result.total,
            dc: ability_check_result.dc,
            success: ability_check_result.success,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_associated_abilities() {
        assert_eq!(
            Skill::Athletics.associated_ability(),
            AbilityScoreType::Strength
        );
        assert_eq!(
            Skill::Stealth.associated_ability(),
            AbilityScoreType::Dexterity
        );
        assert_eq!(
            Skill::Arcana.associated_ability(),
            AbilityScoreType::Intelligence
        );
        assert_eq!(
            Skill::Perception.associated_ability(),
            AbilityScoreType::Wisdom
        );
        assert_eq!(
            Skill::Persuasion.associated_ability(),
            AbilityScoreType::Charisma
        );
    }

    #[test]
    fn test_skill_bonus_calculation() {
        let scores = AbilityScores::new(15, 14, 13, 12, 10, 8);
        let bonus =
            SkillCalculator::calculate_skill_bonus(&scores, Skill::Athletics, 2, true, false);
        assert_eq!(bonus.ability_modifier, 2); // STR 15 = +2
        assert_eq!(bonus.proficiency_bonus, 2);
        assert_eq!(bonus.total_bonus, 4);

        let bonus_no_prof =
            SkillCalculator::calculate_skill_bonus(&scores, Skill::Athletics, 2, false, false);
        assert_eq!(bonus_no_prof.proficiency_bonus, 0);
        assert_eq!(bonus_no_prof.total_bonus, 2);

        let bonus_expertise =
            SkillCalculator::calculate_skill_bonus(&scores, Skill::Athletics, 2, true, true);
        assert_eq!(bonus_expertise.proficiency_bonus, 4); // 2 * 2
        assert_eq!(bonus_expertise.total_bonus, 6);
    }

    #[test]
    fn test_passive_perception() {
        let scores = AbilityScores::new(10, 10, 10, 10, 15, 10);
        // WIS 15 = +2 modifier, proficiency +2, total bonus = +4
        // Passive Perception = 10 + 4 = 14
        let passive = SkillCalculator::passive_perception(&scores, 2, true, false);
        assert_eq!(passive, 14);
    }

    #[test]
    fn test_all_skills() {
        let skills = Skill::all();
        assert_eq!(skills.len(), 18);
    }
}
