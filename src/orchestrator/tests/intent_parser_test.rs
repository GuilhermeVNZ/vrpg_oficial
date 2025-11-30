//! Comprehensive INTENT Parser tests

use orchestrator::intent::{Intent, IntentParser};

#[test]
fn test_parse_skill_check() {
    let text = r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda a liberar a entrada"
SUGGEST_DC: YES
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::SkillCheck {
        actor,
        skill,
        target,
        context,
        suggest_dc,
    } = &intents[0]
    {
        assert_eq!(actor, "player_1");
        assert_eq!(skill, "persuasion");
        assert_eq!(target.as_ref().unwrap(), "npc_guard_01");
        assert!(context.is_some());
        assert!(*suggest_dc);
    } else {
        panic!("Expected SkillCheck intent");
    }
}

#[test]
fn test_parse_melee_attack() {
    let text = r#"
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
WEAPON: weapon_longsword
MOVE_REQUIRED: YES
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::MeleeAttack {
        actor,
        target,
        weapon,
        move_required,
    } = &intents[0]
    {
        assert_eq!(actor, "player_1");
        assert_eq!(target, "npc_goblin_02");
        assert_eq!(weapon.as_ref().unwrap(), "weapon_longsword");
        assert!(*move_required);
    } else {
        panic!("Expected MeleeAttack intent");
    }
}

#[test]
fn test_parse_ranged_attack() {
    let text = r#"
[INTENTS]
INTENT: RANGED_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
WEAPON: weapon_longbow
MOVE_REQUIRED: NO
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::RangedAttack {
        actor,
        target,
        weapon,
        move_required,
    } = &intents[0]
    {
        assert_eq!(actor, "player_1");
        assert_eq!(target, "npc_goblin_02");
        assert_eq!(weapon.as_ref().unwrap(), "weapon_longbow");
        assert!(!*move_required);
    } else {
        panic!("Expected RangedAttack intent");
    }
}

#[test]
fn test_parse_spell_cast() {
    let text = r#"
[INTENTS]
INTENT: SPELL_CAST
ACTOR: player_1
SPELL: fireball
SLOT_LEVEL: 3
AREA_CENTER: 10, 15
TARGETS: npc_goblin_01, npc_goblin_02
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::SpellCast {
        actor,
        spell,
        slot_level,
        area_center,
        targets,
    } = &intents[0]
    {
        assert_eq!(actor, "player_1");
        assert_eq!(spell, "fireball");
        assert_eq!(*slot_level, 3);
        assert_eq!(*area_center, Some((10, 15)));
        assert_eq!(targets.len(), 2);
    } else {
        panic!("Expected SpellCast intent");
    }
}

#[test]
fn test_parse_combat_start() {
    let text = r#"
[INTENTS]
INTENT: COMBAT_START
REASON: "Goblins attack the party"
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::CombatStart { reason } = &intents[0] {
        // Parser preserves quotes in values
        assert_eq!(reason.as_ref().unwrap(), "\"Goblins attack the party\"");
    } else {
        panic!("Expected CombatStart intent");
    }
}

#[test]
fn test_parse_combat_end() {
    let text = r#"
[INTENTS]
INTENT: COMBAT_END
REASON: "All enemies defeated"
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::CombatEnd { reason } = &intents[0] {
        // Parser preserves quotes in values
        assert_eq!(reason.as_ref().unwrap(), "\"All enemies defeated\"");
    } else {
        panic!("Expected CombatEnd intent");
    }
}

#[test]
fn test_parse_lore_query() {
    let text = r#"
[INTENTS]
INTENT: LORE_QUERY
QUERY: "historia dos Magos Rubros de Thay"
SCOPE: faction
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::LoreQuery { query, scope } = &intents[0] {
        // Parser preserves quotes in values
        assert_eq!(query, "\"historia dos Magos Rubros de Thay\"");
        assert_eq!(scope.as_ref().unwrap(), "faction");
    } else {
        panic!("Expected LoreQuery intent");
    }
}

#[test]
fn test_parse_rule_query() {
    let text = r#"
[INTENTS]
INTENT: RULE_QUERY
QUERY: "como funciona o sistema de condições"
CONTEXT: combat
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 1);

    if let Intent::RuleQuery { query, context } = &intents[0] {
        // Parser preserves quotes in values
        assert_eq!(query, "\"como funciona o sistema de condições\"");
        assert_eq!(context.as_ref().unwrap(), "combat");
    } else {
        panic!("Expected RuleQuery intent");
    }
}

#[test]
fn test_parse_multiple_intents() {
    let text = r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: perception
END_INTENT
INTENT: LORE_QUERY
QUERY: "historia dos Magos Rubros de Thay"
SCOPE: faction
END_INTENT
[/INTENTS]
"#;

    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 2);

    if let Intent::SkillCheck { .. } = &intents[0] {
        // OK
    } else {
        panic!("Expected SkillCheck as first intent");
    }

    if let Intent::LoreQuery { .. } = &intents[1] {
        // OK
    } else {
        panic!("Expected LoreQuery as second intent");
    }
}

#[test]
fn test_parse_missing_required_field() {
    let text = r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
END_INTENT
[/INTENTS]
"#;

    let result = IntentParser::parse(text);
    assert!(result.is_err());
}

#[test]
fn test_parse_unknown_intent_type() {
    let text = r#"
[INTENTS]
INTENT: UNKNOWN_TYPE
ACTOR: player_1
END_INTENT
[/INTENTS]
"#;

    let result = IntentParser::parse(text);
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_text() {
    let intents = IntentParser::parse("").unwrap();
    assert_eq!(intents.len(), 0);
}

#[test]
fn test_parse_no_intents_block() {
    let text = "This is just regular text without INTENTS block";
    let intents = IntentParser::parse(text).unwrap();
    assert_eq!(intents.len(), 0);
}

#[test]
fn test_parse_malformed_block() {
    let text = r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: perception
[/INTENTS]
"#;

    // Missing END_INTENT, should still parse what it can
    let result = IntentParser::parse(text);
    // This might succeed or fail depending on implementation
    // For now, we'll just test it doesn't panic
    let _ = result;
}

#[test]
fn test_parse_bool_values() {
    let text1 = r#"
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
MOVE_REQUIRED: YES
END_INTENT
[/INTENTS]
"#;

    let intents1 = IntentParser::parse(text1).unwrap();
    if let Intent::MeleeAttack { move_required, .. } = &intents1[0] {
        assert!(*move_required);
    }

    let text2 = r#"
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
MOVE_REQUIRED: NO
END_INTENT
[/INTENTS]
"#;

    let intents2 = IntentParser::parse(text2).unwrap();
    if let Intent::MeleeAttack { move_required, .. } = &intents2[0] {
        assert!(!*move_required);
    }
}
