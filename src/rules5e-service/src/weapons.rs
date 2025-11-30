// Weapons System - D&D 5e
// Implements complete weapons system with properties and damage

use crate::damage::DamageType;
use crate::dice::DiceExpression;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponCategory {
    Simple,
    Martial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Melee,
    Ranged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponProperty {
    Light,
    Finesse,
    TwoHanded,
    Versatile,
    Heavy,
    Reach,
    Thrown,
    Ammunition,
    Loading,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub category: WeaponCategory,
    pub weapon_type: WeaponType,
    pub damage: DiceExpression,
    pub damage_type: DamageType,
    pub properties: Vec<WeaponProperty>,
    pub cost: u32,                                // in copper pieces
    pub weight: f32,                              // in pounds
    pub range_normal: Option<u32>,                // for ranged weapons
    pub range_long: Option<u32>,                  // for ranged weapons
    pub versatile_damage: Option<DiceExpression>, // for versatile weapons
}

impl Weapon {
    pub fn calculate_damage(&self, use_versatile: bool) -> DiceExpression {
        if use_versatile && self.properties.contains(&WeaponProperty::Versatile) {
            self.versatile_damage.clone().unwrap_or(self.damage.clone())
        } else {
            self.damage.clone()
        }
    }

    pub fn uses_strength(&self) -> bool {
        !self.properties.contains(&WeaponProperty::Finesse) && self.weapon_type == WeaponType::Melee
    }

    pub fn uses_dexterity(&self) -> bool {
        self.properties.contains(&WeaponProperty::Finesse) || self.weapon_type == WeaponType::Ranged
    }
}

pub struct WeaponDatabase;

impl WeaponDatabase {
    pub fn get_weapon(name: &str) -> Option<Weapon> {
        Self::all_weapons()
            .into_iter()
            .find(|w| w.name.to_lowercase() == name.to_lowercase())
    }

    pub fn all_weapons() -> Vec<Weapon> {
        vec![
            // Simple Melee Weapons
            Weapon {
                name: "Club".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Light],
                cost: 10,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Dagger".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![
                    WeaponProperty::Finesse,
                    WeaponProperty::Light,
                    WeaponProperty::Thrown,
                ],
                cost: 200,
                weight: 1.0,
                range_normal: Some(20),
                range_long: Some(60),
                versatile_damage: None,
            },
            Weapon {
                name: "Greatclub".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::TwoHanded],
                cost: 20,
                weight: 10.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Handaxe".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Light, WeaponProperty::Thrown],
                cost: 500,
                weight: 2.0,
                range_normal: Some(20),
                range_long: Some(60),
                versatile_damage: None,
            },
            Weapon {
                name: "Javelin".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Thrown],
                cost: 50,
                weight: 2.0,
                range_normal: Some(30),
                range_long: Some(120),
                versatile_damage: None,
            },
            Weapon {
                name: "Light Hammer".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Light, WeaponProperty::Thrown],
                cost: 200,
                weight: 2.0,
                range_normal: Some(20),
                range_long: Some(60),
                versatile_damage: None,
            },
            Weapon {
                name: "Mace".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![],
                cost: 500,
                weight: 4.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Quarterstaff".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Versatile],
                cost: 20,
                weight: 4.0,
                range_normal: None,
                range_long: None,
                versatile_damage: Some(DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                }),
            },
            Weapon {
                name: "Sickle".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Light],
                cost: 100,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Spear".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Thrown, WeaponProperty::Versatile],
                cost: 100,
                weight: 3.0,
                range_normal: Some(20),
                range_long: Some(60),
                versatile_damage: Some(DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                }),
            },
            // Simple Ranged Weapons
            Weapon {
                name: "Light Crossbow".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![
                    WeaponProperty::Ammunition,
                    WeaponProperty::Loading,
                    WeaponProperty::TwoHanded,
                ],
                cost: 2500,
                weight: 5.0,
                range_normal: Some(80),
                range_long: Some(320),
                versatile_damage: None,
            },
            Weapon {
                name: "Dart".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Finesse, WeaponProperty::Thrown],
                cost: 5,
                weight: 0.25,
                range_normal: Some(20),
                range_long: Some(60),
                versatile_damage: None,
            },
            Weapon {
                name: "Shortbow".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Ammunition, WeaponProperty::TwoHanded],
                cost: 2500,
                weight: 2.0,
                range_normal: Some(80),
                range_long: Some(320),
                versatile_damage: None,
            },
            Weapon {
                name: "Sling".to_string(),
                category: WeaponCategory::Simple,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Ammunition],
                cost: 10,
                weight: 0.0,
                range_normal: Some(30),
                range_long: Some(120),
                versatile_damage: None,
            },
            // Martial Melee Weapons
            Weapon {
                name: "Battleaxe".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Versatile],
                cost: 1000,
                weight: 4.0,
                range_normal: None,
                range_long: None,
                versatile_damage: Some(DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                }),
            },
            Weapon {
                name: "Flail".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![],
                cost: 1000,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Glaive".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![
                    WeaponProperty::Heavy,
                    WeaponProperty::Reach,
                    WeaponProperty::TwoHanded,
                ],
                cost: 2000,
                weight: 6.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Greataxe".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 12,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
                cost: 3000,
                weight: 7.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Greatsword".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 2,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
                cost: 5000,
                weight: 6.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Halberd".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![
                    WeaponProperty::Heavy,
                    WeaponProperty::Reach,
                    WeaponProperty::TwoHanded,
                ],
                cost: 2000,
                weight: 6.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Lance".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 12,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Reach, WeaponProperty::Special],
                cost: 1000,
                weight: 6.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Longsword".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Versatile],
                cost: 1500,
                weight: 3.0,
                range_normal: None,
                range_long: None,
                versatile_damage: Some(DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                }),
            },
            Weapon {
                name: "Maul".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 2,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
                cost: 1000,
                weight: 10.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Morningstar".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![],
                cost: 1500,
                weight: 4.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Pike".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![
                    WeaponProperty::Heavy,
                    WeaponProperty::Reach,
                    WeaponProperty::TwoHanded,
                ],
                cost: 500,
                weight: 18.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Rapier".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Finesse],
                cost: 2500,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Scimitar".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Finesse, WeaponProperty::Light],
                cost: 2500,
                weight: 3.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Shortsword".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Finesse, WeaponProperty::Light],
                cost: 1000,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Trident".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Thrown, WeaponProperty::Versatile],
                cost: 500,
                weight: 4.0,
                range_normal: Some(20),
                range_long: Some(60),
                versatile_damage: Some(DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                }),
            },
            Weapon {
                name: "War Pick".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![],
                cost: 500,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            Weapon {
                name: "Warhammer".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Versatile],
                cost: 1500,
                weight: 2.0,
                range_normal: None,
                range_long: None,
                versatile_damage: Some(DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                }),
            },
            Weapon {
                name: "Whip".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Melee,
                damage: DiceExpression {
                    count: 1,
                    sides: 4,
                    modifier: 0,
                },
                damage_type: DamageType::Slashing,
                properties: vec![WeaponProperty::Finesse, WeaponProperty::Reach],
                cost: 200,
                weight: 3.0,
                range_normal: None,
                range_long: None,
                versatile_damage: None,
            },
            // Martial Ranged Weapons
            Weapon {
                name: "Blowgun".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 1,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![WeaponProperty::Ammunition, WeaponProperty::Loading],
                cost: 1000,
                weight: 1.0,
                range_normal: Some(25),
                range_long: Some(100),
                versatile_damage: None,
            },
            Weapon {
                name: "Hand Crossbow".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 6,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![
                    WeaponProperty::Ammunition,
                    WeaponProperty::Light,
                    WeaponProperty::Loading,
                ],
                cost: 7500,
                weight: 3.0,
                range_normal: Some(30),
                range_long: Some(120),
                versatile_damage: None,
            },
            Weapon {
                name: "Heavy Crossbow".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 10,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![
                    WeaponProperty::Ammunition,
                    WeaponProperty::Heavy,
                    WeaponProperty::Loading,
                    WeaponProperty::TwoHanded,
                ],
                cost: 5000,
                weight: 18.0,
                range_normal: Some(100),
                range_long: Some(400),
                versatile_damage: None,
            },
            Weapon {
                name: "Longbow".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 1,
                    sides: 8,
                    modifier: 0,
                },
                damage_type: DamageType::Piercing,
                properties: vec![
                    WeaponProperty::Ammunition,
                    WeaponProperty::Heavy,
                    WeaponProperty::TwoHanded,
                ],
                cost: 5000,
                weight: 2.0,
                range_normal: Some(150),
                range_long: Some(600),
                versatile_damage: None,
            },
            Weapon {
                name: "Net".to_string(),
                category: WeaponCategory::Martial,
                weapon_type: WeaponType::Ranged,
                damage: DiceExpression {
                    count: 0,
                    sides: 0,
                    modifier: 0,
                },
                damage_type: DamageType::Bludgeoning,
                properties: vec![WeaponProperty::Special, WeaponProperty::Thrown],
                cost: 100,
                weight: 3.0,
                range_normal: Some(5),
                range_long: Some(15),
                versatile_damage: None,
            },
        ]
    }

    pub fn simple_melee() -> Vec<Weapon> {
        Self::all_weapons()
            .into_iter()
            .filter(|w| w.category == WeaponCategory::Simple && w.weapon_type == WeaponType::Melee)
            .collect()
    }

    pub fn simple_ranged() -> Vec<Weapon> {
        Self::all_weapons()
            .into_iter()
            .filter(|w| w.category == WeaponCategory::Simple && w.weapon_type == WeaponType::Ranged)
            .collect()
    }

    pub fn martial_melee() -> Vec<Weapon> {
        Self::all_weapons()
            .into_iter()
            .filter(|w| w.category == WeaponCategory::Martial && w.weapon_type == WeaponType::Melee)
            .collect()
    }

    pub fn martial_ranged() -> Vec<Weapon> {
        Self::all_weapons()
            .into_iter()
            .filter(|w| {
                w.category == WeaponCategory::Martial && w.weapon_type == WeaponType::Ranged
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_database() {
        let weapons = WeaponDatabase::all_weapons();
        assert!(weapons.len() > 0);

        let longsword = WeaponDatabase::get_weapon("Longsword");
        assert!(longsword.is_some());
        let ls = longsword.unwrap();
        assert_eq!(ls.category, WeaponCategory::Martial);
        assert_eq!(ls.weapon_type, WeaponType::Melee);
        assert!(ls.properties.contains(&WeaponProperty::Versatile));
    }

    #[test]
    fn test_versatile_damage() {
        let longsword = WeaponDatabase::get_weapon("Longsword").unwrap();
        let one_handed = longsword.calculate_damage(false);
        let two_handed = longsword.calculate_damage(true);

        assert_eq!(one_handed.sides, 8);
        assert_eq!(two_handed.sides, 10);
    }

    #[test]
    fn test_finesse_weapon() {
        let rapier = WeaponDatabase::get_weapon("Rapier").unwrap();
        assert!(rapier.uses_dexterity());
        assert!(!rapier.uses_strength());

        let longsword = WeaponDatabase::get_weapon("Longsword").unwrap();
        assert!(!longsword.uses_dexterity());
        assert!(longsword.uses_strength());
    }

    #[test]
    fn test_ranged_weapon() {
        let longbow = WeaponDatabase::get_weapon("Longbow").unwrap();
        assert_eq!(longbow.weapon_type, WeaponType::Ranged);
        assert!(longbow.uses_dexterity());
        assert!(longbow.range_normal.is_some());
        assert_eq!(longbow.range_normal.unwrap(), 150);
    }
}
