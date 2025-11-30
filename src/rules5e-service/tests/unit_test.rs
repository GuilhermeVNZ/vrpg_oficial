mod unit {
    mod dice_roll_test {
        use rules5e_service::dice::{DiceExpression, DiceRoller, RollMode};

        #[test]
        fn test_dice_roll_1d20() {
            let mut roller = DiceRoller::new();
            let expr = DiceExpression {
                count: 1,
                sides: 20,
                modifier: 0,
            };
            let result = roller.roll(&expr, RollMode::Normal).unwrap();
            assert!(result.total >= 1 && result.total <= 20);
            assert_eq!(result.rolls.len(), 1);
        }

        #[test]
        fn test_dice_roll_2d8_plus_3() {
            let mut roller = DiceRoller::new();
            let expr = DiceExpression {
                count: 2,
                sides: 8,
                modifier: 3,
            };
            let result = roller.roll(&expr, RollMode::Normal).unwrap();
            assert!(result.total >= 5 && result.total <= 19);
            assert_eq!(result.rolls.len(), 2);
        }

        #[test]
        fn test_dice_roll_deterministic_seed() {
            let seed = 12345;
            let mut roller1 = DiceRoller::with_seed(seed);
            let mut roller2 = DiceRoller::with_seed(seed);

            let expr = DiceExpression {
                count: 1,
                sides: 20,
                modifier: 0,
            };

            let result1 = roller1.roll(&expr, RollMode::Normal).unwrap();
            let result2 = roller2.roll(&expr, RollMode::Normal).unwrap();

            assert_eq!(result1.total, result2.total);
            assert_eq!(result1.rolls, result2.rolls);
        }

        #[test]
        fn test_dice_roll_edge_cases() {
            let mut roller = DiceRoller::new();

            // 1d1 (resultado sempre 1)
            let expr = DiceExpression {
                count: 1,
                sides: 1,
                modifier: 0,
            };
            let result = roller.roll(&expr, RollMode::Normal).unwrap();
            assert_eq!(result.total, 1);

            // 0d20 (resultado sempre 0 + modifier)
            let expr = DiceExpression {
                count: 0,
                sides: 20,
                modifier: 5,
            };
            let result = roller.roll(&expr, RollMode::Normal).unwrap();
            assert_eq!(result.total, 5);

            // Modificador negativo
            let expr = DiceExpression {
                count: 1,
                sides: 20,
                modifier: -5,
            };
            let result = roller.roll(&expr, RollMode::Normal).unwrap();
            assert!(result.total >= -4 && result.total <= 15);
        }

        #[test]
        fn test_dice_roll_distribution() {
            let mut roller = DiceRoller::new();
            let expr = DiceExpression {
                count: 1,
                sides: 20,
                modifier: 0,
            };

            let mut counts = [0; 21]; // 0-20
            for _ in 0..10000 {
                let result = roller.roll(&expr, RollMode::Normal).unwrap();
                let value = result.total as usize;
                if value <= 20 {
                    counts[value] += 1;
                }
            }

            // Verificar que todos os valores ocorreram (aproximadamente uniforme)
            let min_count = counts[1..=20].iter().min().unwrap();
            let max_count = counts[1..=20].iter().max().unwrap();

            // Com 10000 rolagens, cada valor deveria aparecer ~500 vezes
            // Aceitamos uma variação de ±200 (300-700)
            assert!(
                *min_count >= 300,
                "Distribution too skewed: min={}",
                min_count
            );
            assert!(
                *max_count <= 700,
                "Distribution too skewed: max={}",
                max_count
            );
        }

        #[test]
        fn test_dice_roll_advantage() {
            let mut roller = DiceRoller::with_seed(42);
            let expr = DiceExpression {
                count: 2,
                sides: 20,
                modifier: 0,
            };
            let result = roller.roll(&expr, RollMode::Advantage).unwrap();
            assert_eq!(result.rolls.len(), 2);
            assert_eq!(result.total, *result.rolls.iter().max().unwrap() as i32);
        }

        #[test]
        fn test_dice_roll_disadvantage() {
            let mut roller = DiceRoller::with_seed(42);
            let expr = DiceExpression {
                count: 2,
                sides: 20,
                modifier: 0,
            };
            let result = roller.roll(&expr, RollMode::Disadvantage).unwrap();
            assert_eq!(result.rolls.len(), 2);
            assert_eq!(result.total, *result.rolls.iter().min().unwrap() as i32);
        }
    }

    mod attack_test {
        use rules5e_service::attack::{AttackRequest, AttackResolver};

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
    }

    mod damage_test {
        use rules5e_service::damage::{Damage, DamageRequest, DamageResolver, DamageType};

        #[test]
        fn test_damage_calculation() {
            let resolver = DamageResolver::new();
            let request = DamageRequest {
                damage: vec![Damage {
                    amount: 10,
                    damage_type: DamageType::Fire,
                }],
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
                damage: vec![Damage {
                    amount: 10,
                    damage_type: DamageType::Fire,
                }],
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
                damage: vec![Damage {
                    amount: 10,
                    damage_type: DamageType::Fire,
                }],
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
                damage: vec![Damage {
                    amount: 10,
                    damage_type: DamageType::Fire,
                }],
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
                damage: vec![Damage {
                    amount: 10,
                    damage_type: DamageType::Fire,
                }],
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
                damage: vec![Damage {
                    amount: 11,
                    damage_type: DamageType::Fire,
                }],
                resistances: vec![DamageType::Fire],
                vulnerabilities: vec![],
                immunities: vec![],
            };
            let result = resolver.resolve(&request);
            assert_eq!(result.total, 5); // 11 / 2 = 5 (round down)
        }
    }

    mod ability_check_test {
        use rules5e_service::ability::{Ability, AbilityCheckRequest, AbilityChecker};

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
    }

    mod saving_throw_test {
        use rules5e_service::ability::{Ability, AbilityCheckRequest, AbilityChecker};

        #[test]
        fn test_saving_throw() {
            let checker = AbilityChecker::new();
            let request = AbilityCheckRequest {
                ability: Ability::Dexterity,
                ability_modifier: 2,
                proficiency_bonus: 3,
                has_proficiency: false,
                has_expertise: false,
                dc: 15,
                advantage: false,
                disadvantage: false,
                seed: Some(100),
            };
            let result = checker.check(&request).unwrap();
            assert!(result.roll >= 1 && result.roll <= 20);
            assert_eq!(result.ability_modifier, 2);
            assert_eq!(result.dc, 15);
        }

        #[test]
        fn test_saving_throw_with_proficiency() {
            let checker = AbilityChecker::new();
            let request = AbilityCheckRequest {
                ability: Ability::Dexterity,
                ability_modifier: 2,
                proficiency_bonus: 3,
                has_proficiency: true,
                has_expertise: false,
                dc: 15,
                advantage: false,
                disadvantage: false,
                seed: Some(100),
            };
            let result = checker.check(&request).unwrap();
            assert_eq!(result.proficiency_bonus, 3);
            assert_eq!(result.total, result.roll + 2 + 3);
        }

        #[test]
        fn test_saving_throw_edge_cases() {
            let checker = AbilityChecker::new();

            // DC muito alto
            let request = AbilityCheckRequest {
                ability: Ability::Constitution,
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
                ability: Ability::Constitution,
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
        }
    }

    mod condition_test {
        use rules5e_service::condition::{ConditionApplication, ConditionManager, ConditionType};

        #[test]
        fn test_condition_poisoned() {
            let mut manager = ConditionManager::new();
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Poisoned,
                duration_rounds: Some(10),
                permanent: false,
            });
            assert!(manager.has(ConditionType::Poisoned));
        }

        #[test]
        fn test_condition_stunned() {
            let mut manager = ConditionManager::new();
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Stunned,
                duration_rounds: Some(5),
                permanent: false,
            });
            assert!(manager.has(ConditionType::Stunned));
        }

        #[test]
        fn test_condition_multiple() {
            let mut manager = ConditionManager::new();
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Poisoned,
                duration_rounds: Some(10),
                permanent: false,
            });
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Stunned,
                duration_rounds: Some(5),
                permanent: false,
            });
            assert!(manager.has(ConditionType::Poisoned));
            assert!(manager.has(ConditionType::Stunned));
            assert_eq!(manager.get_all().len(), 2);
        }

        #[test]
        fn test_condition_duplicate() {
            let mut manager = ConditionManager::new();
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Poisoned,
                duration_rounds: Some(10),
                permanent: false,
            });
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Poisoned,
                duration_rounds: Some(5),
                permanent: false,
            });
            // Should only have one condition
            assert_eq!(
                manager
                    .get_all()
                    .iter()
                    .filter(|c| c.condition_type == ConditionType::Poisoned)
                    .count(),
                1
            );
        }

        #[test]
        fn test_condition_remove() {
            let mut manager = ConditionManager::new();
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Poisoned,
                duration_rounds: Some(10),
                permanent: false,
            });
            manager.remove(ConditionType::Poisoned);
            assert!(!manager.has(ConditionType::Poisoned));
        }

        #[test]
        fn test_condition_permanent() {
            let mut manager = ConditionManager::new();
            manager.apply(&ConditionApplication {
                condition_type: ConditionType::Blinded,
                duration_rounds: None,
                permanent: true,
            });
            assert!(manager.has(ConditionType::Blinded));
            manager.expire_conditions();
            // Permanent conditions should not expire
            assert!(manager.has(ConditionType::Blinded));
        }
    }
}
