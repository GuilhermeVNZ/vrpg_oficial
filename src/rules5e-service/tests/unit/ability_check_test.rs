use rules5e_service::ability::{AbilityChecker, AbilityCheckRequest, Ability};

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
    assert_eq!(result.dc, 15);
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
fn test_ability_check_edge_cases() {
    let checker = AbilityChecker::new();
    
    // DC muito alto
    let request = AbilityCheckRequest {
        ability: Ability::Strength,
        ability_modifier: 0,
        proficiency_bonus: 0,
        has_proficiency: false,
        has_expertise: false,
        dc: 30,
        advantage: false,
        disadvantage: false,
        seed: Some(100),
    };
    let result = checker.check(&request).unwrap();
    assert!(!result.success || result.roll == 20);
    
    // DC muito baixo
    let request = AbilityCheckRequest {
        ability: Ability::Strength,
        ability_modifier: 20,
        proficiency_bonus: 0,
        has_proficiency: false,
        has_expertise: false,
        dc: 0,
        advantage: false,
        disadvantage: false,
        seed: Some(100),
    };
    let result = checker.check(&request).unwrap();
    assert!(result.success || result.roll == 1);
    
    // Vantagem + desvantagem (cancelam)
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


