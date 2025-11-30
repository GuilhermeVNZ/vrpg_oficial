use rules5e_service::attack::{AttackResolver, AttackRequest};

#[test]
fn test_attack_hit_calculation() {
    let resolver = AttackResolver::new();
    let request = AttackRequest {
        attack_bonus: 5,
        ac: 15,
        advantage: false,
        disadvantage: false,
        seed: Some(100),
    };
    let result = resolver.resolve(&request).unwrap();
    assert!(result.roll >= 1 && result.roll <= 20);
    assert_eq!(result.total, result.roll + 5);
    assert_eq!(result.ac, 15);
}

#[test]
fn test_attack_critical_hit() {
    let resolver = AttackResolver::new();
    // Testar com vários seeds até encontrar um que dê 20
    for seed in 0..1000 {
        let request = AttackRequest {
            attack_bonus: 0,
            ac: 30,
            advantage: false,
            disadvantage: false,
            seed: Some(seed),
        };
        let result = resolver.resolve(&request).unwrap();
        if result.natural_roll == 20 {
            assert!(result.critical_hit);
            assert!(result.hit);
            return;
        }
    }
    // Se não encontrou em 1000 tentativas, algo está errado
    panic!("Could not find natural 20 in 1000 attempts");
}

#[test]
fn test_attack_critical_miss() {
    let resolver = AttackResolver::new();
    // Testar com vários seeds até encontrar um que dê 1
    for seed in 0..1000 {
        let request = AttackRequest {
            attack_bonus: 10,
            ac: 5,
            advantage: false,
            disadvantage: false,
            seed: Some(seed),
        };
        let result = resolver.resolve(&request).unwrap();
        if result.natural_roll == 1 {
            assert!(result.critical_miss);
            assert!(!result.hit);
            return;
        }
    }
    panic!("Could not find natural 1 in 1000 attempts");
}

#[test]
fn test_attack_with_advantage() {
    let resolver = AttackResolver::new();
    let request = AttackRequest {
        attack_bonus: 5,
        ac: 15,
        advantage: true,
        disadvantage: false,
        seed: Some(50),
    };
    let result = resolver.resolve(&request).unwrap();
    assert!(result.total >= request.attack_bonus + 1);
    assert!(result.total <= request.attack_bonus + 20);
}

#[test]
fn test_attack_with_disadvantage() {
    let resolver = AttackResolver::new();
    let request = AttackRequest {
        attack_bonus: 5,
        ac: 15,
        advantage: false,
        disadvantage: true,
        seed: Some(50),
    };
    let result = resolver.resolve(&request).unwrap();
    assert!(result.total >= request.attack_bonus + 1);
    assert!(result.total <= request.attack_bonus + 20);
}

#[test]
fn test_attack_advantage_disadvantage_cancel() {
    let resolver = AttackResolver::new();
    let request = AttackRequest {
        attack_bonus: 5,
        ac: 15,
        advantage: true,
        disadvantage: true,
        seed: Some(50),
    };
    let result = resolver.resolve(&request).unwrap();
    // Should resolve as normal (single roll)
    assert!(result.roll >= 1 && result.roll <= 20);
}

#[test]
fn test_attack_edge_cases() {
    let resolver = AttackResolver::new();
    
    // Very high AC
    let request = AttackRequest {
        attack_bonus: 0,
        ac: 30,
        advantage: false,
        disadvantage: false,
        seed: Some(100),
    };
    let result = resolver.resolve(&request).unwrap();
    // Only natural 20 should hit
    assert!(!result.hit || result.critical_hit);
    
    // Very low AC
    let request = AttackRequest {
        attack_bonus: 20,
        ac: 0,
        advantage: false,
        disadvantage: false,
        seed: Some(100),
    };
    let result = resolver.resolve(&request).unwrap();
    // Only natural 1 should miss
    assert!(result.hit || result.critical_miss);
}


