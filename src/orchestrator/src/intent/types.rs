//! INTENT types and definitions

use serde::{Deserialize, Serialize};

/// INTENT enum representing all possible intents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    // Social/Roleplay
    SkillCheck {
        actor: String,
        skill: String,
        target: Option<String>,
        context: Option<String>,
        suggest_dc: bool,
    },
    LoreQuery {
        query: String,
        scope: Option<String>,
    },
    RuleQuery {
        query: String,
        context: Option<String>,
    },
    NpcDialogue {
        npc_id: String,
        text: String,
    },
    SceneEvent {
        event_type: String,
        description: String,
    },

    // Exploration
    InvestigateArea {
        actor: String,
        area: String,
    },
    SearchItem {
        actor: String,
        item: Option<String>,
    },
    InteractObject {
        actor: String,
        object_id: String,
    },

    // Combat
    MeleeAttack {
        actor: String,
        target: String,
        weapon: Option<String>,
        move_required: bool,
    },
    RangedAttack {
        actor: String,
        target: String,
        weapon: Option<String>,
        move_required: bool,
    },
    SpellCast {
        actor: String,
        spell: String,
        slot_level: u8,
        area_center: Option<(i32, i32)>,
        targets: Vec<String>,
    },
    UseItem {
        actor: String,
        item_id: String,
    },
    ReadyAction {
        actor: String,
        action: String,
    },
    Dash {
        actor: String,
    },
    Disengage {
        actor: String,
    },
    Help {
        actor: String,
        target: String,
    },
    CombatStart {
        reason: Option<String>,
    },
    CombatEnd {
        reason: Option<String>,
    },

    // Assets
    GeneratePortrait {
        character_id: String,
        style: Option<String>,
    },
    GenerateScene {
        scene_id: String,
        style: Option<String>,
        prompts: Vec<String>,
    },
    GenerateBattlemap {
        map_id: String,
        style: Option<String>,
    },
}

impl Intent {
    /// Get the intent type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Intent::SkillCheck { .. } => "SKILL_CHECK",
            Intent::LoreQuery { .. } => "LORE_QUERY",
            Intent::RuleQuery { .. } => "RULE_QUERY",
            Intent::NpcDialogue { .. } => "NPC_DIALOGUE",
            Intent::SceneEvent { .. } => "SCENE_EVENT",
            Intent::InvestigateArea { .. } => "INVESTIGATE_AREA",
            Intent::SearchItem { .. } => "SEARCH_ITEM",
            Intent::InteractObject { .. } => "INTERACT_OBJECT",
            Intent::MeleeAttack { .. } => "MELEE_ATTACK",
            Intent::RangedAttack { .. } => "RANGED_ATTACK",
            Intent::SpellCast { .. } => "SPELL_CAST",
            Intent::UseItem { .. } => "USE_ITEM",
            Intent::ReadyAction { .. } => "READY_ACTION",
            Intent::Dash { .. } => "DASH",
            Intent::Disengage { .. } => "DISENGAGE",
            Intent::Help { .. } => "HELP",
            Intent::CombatStart { .. } => "COMBAT_START",
            Intent::CombatEnd { .. } => "COMBAT_END",
            Intent::GeneratePortrait { .. } => "GENERATE_PORTRAIT",
            Intent::GenerateScene { .. } => "GENERATE_SCENE",
            Intent::GenerateBattlemap { .. } => "GENERATE_BATTLEMAP",
        }
    }
}
