use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    pub amount: i32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageRequest {
    pub damage: Vec<Damage>,
    pub resistances: Vec<DamageType>,
    pub vulnerabilities: Vec<DamageType>,
    pub immunities: Vec<DamageType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResult {
    pub original_damage: Vec<Damage>,
    pub final_damage: Vec<Damage>,
    pub total: i32,
}

pub struct DamageResolver;

impl Default for DamageResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DamageResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve(&self, request: &DamageRequest) -> DamageResult {
        let mut final_damage = Vec::new();
        let mut total = 0;

        for damage in &request.damage {
            let mut amount = damage.amount;

            // Check immunity first (overrides everything)
            if request.immunities.contains(&damage.damage_type) {
                amount = 0;
            } else {
                // Check vulnerability and resistance
                let has_vulnerability = request.vulnerabilities.contains(&damage.damage_type);
                let has_resistance = request.resistances.contains(&damage.damage_type);

                if has_vulnerability && has_resistance {
                    // They cancel out - normal damage
                } else if has_vulnerability {
                    amount *= 2;
                } else if has_resistance {
                    amount /= 2; // Round down
                }
            }

            final_damage.push(Damage {
                amount,
                damage_type: damage.damage_type,
            });
            total += amount;
        }

        DamageResult {
            original_damage: request.damage.clone(),
            final_damage,
            total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
