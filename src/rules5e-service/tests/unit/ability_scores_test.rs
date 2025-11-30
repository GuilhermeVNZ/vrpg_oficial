use rules5e_service::ability_scores::{
    AbilityScoreGenerator, AbilityScoreType, AbilityScores,
};

#[test]
fn test_ability_scores_new() {
    let scores = AbilityScores::new(15, 14, 13, 12, 10, 8);
    assert_eq!(scores.strength, 15);
    assert_eq!(scores.dexterity, 14);
    assert_eq!(scores.constitution, 13);
    assert_eq!(scores.intelligence, 12);
    assert_eq!(scores.wisdom, 10);
    assert_eq!(scores.charisma, 8);
}

#[test]
fn test_ability_modifiers() {
    let scores = AbilityScores::new(15, 10, 8, 20, 12, 16);
    assert_eq!(scores.get_modifier(AbilityScoreType::Strength), 2);
    assert_eq!(scores.get_modifier(AbilityScoreType::Dexterity), 0);
    assert_eq!(scores.get_modifier(AbilityScoreType::Constitution), -1);
    assert_eq!(scores.get_modifier(AbilityScoreType::Intelligence), 5);
    assert_eq!(scores.get_modifier(AbilityScoreType::Wisdom), 1);
    assert_eq!(scores.get_modifier(AbilityScoreType::Charisma), 3);
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
    assert_eq!(scores1.strength, scores2.strength);
    assert_eq!(scores1.dexterity, scores2.dexterity);
}

#[test]
fn test_score_limits() {
    let mut scores = AbilityScores::new(20, 20, 20, 20, 20, 20);
    assert!(scores.increase_score(AbilityScoreType::Strength, 10).is_ok());
    assert_eq!(scores.strength, 30);
    assert!(scores.increase_score(AbilityScoreType::Strength, 1).is_err());
}



