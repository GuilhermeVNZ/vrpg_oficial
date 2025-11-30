use rules5e_service::damage::{DamageResolver, DamageRequest, Damage, DamageType};

#[test]
fn test_damage_calculation() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 10,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![],
        vulnerabilities: vec![],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 10);
}

#[test]
fn test_damage_resistance() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 10,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![DamageType::Fire],
        vulnerabilities: vec![],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 5); // 10 / 2 = 5
}

#[test]
fn test_damage_vulnerability() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 10,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![],
        vulnerabilities: vec![DamageType::Fire],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 20); // 10 * 2 = 20
}

#[test]
fn test_damage_immunity() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 10,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![],
        vulnerabilities: vec![],
        immunities: vec![DamageType::Fire],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 0);
}

#[test]
fn test_damage_multiple_types() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 10,
                damage_type: DamageType::Fire,
            },
            Damage {
                amount: 5,
                damage_type: DamageType::Cold,
            },
        ],
        resistances: vec![DamageType::Fire],
        vulnerabilities: vec![],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 10); // 5 (fire) + 5 (cold) = 10
}

#[test]
fn test_damage_resistance_vulnerability_cancel() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 10,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![DamageType::Fire],
        vulnerabilities: vec![DamageType::Fire],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 10); // Cancel out
}

#[test]
fn test_damage_round_down() {
    let resolver = DamageResolver::new();
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 11,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![DamageType::Fire],
        vulnerabilities: vec![],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 5); // 11 / 2 = 5 (round down)
}

#[test]
fn test_damage_edge_cases() {
    let resolver = DamageResolver::new();
    
    // Dano 0
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 0,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![],
        vulnerabilities: vec![],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 0);
    
    // Dano muito alto
    let request = DamageRequest {
        damage: vec![
            Damage {
                amount: 1000,
                damage_type: DamageType::Fire,
            },
        ],
        resistances: vec![],
        vulnerabilities: vec![],
        immunities: vec![],
    };
    let result = resolver.resolve(&request);
    assert_eq!(result.total, 1000);
}


