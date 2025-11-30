//! Helper functions for getting actor statistics from game session

use crate::error::Result;
use crate::session::GameSession;
use uuid::Uuid;

/// Actor statistics for combat and skill checks
#[derive(Debug, Clone)]
pub struct ActorStats {
    pub actor_id: Uuid,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub ac: i32,
    // Ability scores (defaults for now, will be extended when Character system is implemented)
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    // Proficiency bonus (calculated from level, default for now)
    pub proficiency_bonus: i32,
    // Level (default for now)
    pub level: i32,
}

impl ActorStats {
    /// Calculate ability modifier from ability score
    pub fn ability_modifier(&self, ability: &str) -> i32 {
        let score = match ability.to_lowercase().as_str() {
            "strength" | "str" => self.strength,
            "dexterity" | "dex" => self.dexterity,
            "constitution" | "con" => self.constitution,
            "intelligence" | "int" => self.intelligence,
            "wisdom" | "wis" => self.wisdom,
            "charisma" | "cha" => self.charisma,
            _ => 10, // Default
        };
        (score - 10) / 2
    }

    /// Calculate attack bonus (STR or DEX modifier + proficiency)
    pub fn attack_bonus(&self, use_dexterity: bool) -> i32 {
        let ability_mod = if use_dexterity {
            self.ability_modifier("dex")
        } else {
            self.ability_modifier("str")
        };
        ability_mod + self.proficiency_bonus
    }

    /// Check if actor has proficiency in a skill
    /// TODO: This will be replaced with actual proficiency tracking
    pub fn has_proficiency(&self, _skill: &str) -> bool {
        // For now, assume no proficiency
        // In the future, this will check against actor's skill proficiencies
        false
    }
}

/// Get actor statistics from game session
pub fn get_actor_stats(game_session: &GameSession, actor_id: &str) -> Result<Option<ActorStats>> {
    // Try to parse as UUID first
    let actor_uuid = Uuid::parse_str(actor_id).ok();

    if let Some(engine) = game_session.engine_session() {
        if let Some(scene) = engine.get_current_scene() {
            // Find actor by UUID or name
            let actor = if let Some(uuid) = actor_uuid {
                scene.get_actor(uuid)
            } else {
                scene
                    .all_actors()
                    .iter()
                    .find(|a| a.name == actor_id)
                    .map(|a| a.id)
                    .and_then(|id| scene.get_actor(id))
            };

            if let Some(actor) = actor {
                // For now, use default ability scores (10 = +0 modifier)
                // TODO: When Character system is implemented, get real scores
                let stats = ActorStats {
                    actor_id: actor.id,
                    name: actor.name.clone(),
                    hp: actor.hp,
                    max_hp: actor.max_hp,
                    ac: actor.ac,
                    strength: 10,
                    dexterity: 10,
                    constitution: 10,
                    intelligence: 10,
                    wisdom: 10,
                    charisma: 10,
                    proficiency_bonus: 2, // Default for level 1-4
                    level: 1,             // Default
                };
                return Ok(Some(stats));
            }
        }
    }

    Ok(None)
}

/// Get skill ability modifier for a skill
pub fn skill_ability_modifier(stats: &ActorStats, skill: &str) -> i32 {
    // Map skills to their primary ability
    let ability = match skill.to_lowercase().as_str() {
        "athletics" => "strength",
        "acrobatics" | "sleight_of_hand" | "stealth" => "dexterity",
        "arcana" | "history" | "investigation" | "nature" | "religion" => "intelligence",
        "animal_handling" | "insight" | "medicine" | "perception" | "survival" => "wisdom",
        "deception" | "intimidation" | "performance" | "persuasion" => "charisma",
        _ => "intelligence", // Default
    };
    stats.ability_modifier(ability)
}

/// Get DC suggestion based on context
pub fn suggest_dc(difficulty: &str) -> i32 {
    match difficulty.to_lowercase().as_str() {
        "very_easy" | "trivial" => 5,
        "easy" => 10,
        "medium" | "moderate" => 15,
        "hard" | "difficult" => 20,
        "very_hard" | "nearly_impossible" => 25,
        _ => 15, // Default medium difficulty
    }
}
