use crate::error::{Result, RulesError};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DiceExpression {
    pub count: u32,
    pub sides: u32,
    pub modifier: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollResult {
    pub expression: DiceExpression,
    pub rolls: Vec<u32>,
    pub total: i32,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollMode {
    Normal,
    Advantage,
    Disadvantage,
}

pub struct DiceRoller {
    rng: StdRng,
    seed: Option<u64>,
}

impl Default for DiceRoller {
    fn default() -> Self {
        Self::new()
    }
}

impl DiceRoller {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
            seed: None,
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed: Some(seed),
        }
    }

    pub fn roll(&mut self, expression: &DiceExpression, mode: RollMode) -> Result<RollResult> {
        if expression.count == 0 {
            return Ok(RollResult {
                expression: expression.clone(),
                rolls: vec![],
                total: expression.modifier,
                seed: self.seed,
            });
        }

        if expression.sides == 0 {
            return Err(RulesError::DiceParse("Dice sides cannot be 0".to_string()));
        }

        let mut rolls = Vec::with_capacity(expression.count as usize);

        for _ in 0..expression.count {
            let roll = if expression.sides == 1 {
                1
            } else {
                self.rng.gen_range(1..=expression.sides)
            };
            rolls.push(roll);
        }

        let total = match mode {
            RollMode::Normal => rolls.iter().sum::<u32>() as i32 + expression.modifier,
            RollMode::Advantage => {
                if rolls.len() < 2 {
                    rolls[0] as i32 + expression.modifier
                } else {
                    *rolls.iter().max().unwrap() as i32 + expression.modifier
                }
            }
            RollMode::Disadvantage => {
                if rolls.len() < 2 {
                    rolls[0] as i32 + expression.modifier
                } else {
                    *rolls.iter().min().unwrap() as i32 + expression.modifier
                }
            }
        };

        Ok(RollResult {
            expression: expression.clone(),
            rolls,
            total,
            seed: self.seed,
        })
    }

    pub fn parse(expr: &str) -> Result<DiceExpression> {
        let expr = expr.trim().to_lowercase();

        // Handle simple cases
        if expr == "0" {
            return Ok(DiceExpression {
                count: 0,
                sides: 0,
                modifier: 0,
            });
        }

        // Parse format: NdM+K or NdM-K or NdM
        let parts: Vec<&str> = expr.split('d').collect();

        if parts.len() != 2 {
            return Err(RulesError::DiceParse(format!(
                "Invalid dice expression: {}",
                expr
            )));
        }

        let count = parts[0]
            .parse::<u32>()
            .map_err(|_| RulesError::DiceParse(format!("Invalid dice count: {}", parts[0])))?;

        let modifier_part = parts[1];
        let (sides_str, modifier) = if modifier_part.contains('+') {
            let subparts: Vec<&str> = modifier_part.split('+').collect();
            if subparts.len() != 2 {
                return Err(RulesError::DiceParse(format!(
                    "Invalid modifier: {}",
                    modifier_part
                )));
            }
            (
                subparts[0],
                subparts[1].parse::<i32>().map_err(|_| {
                    RulesError::DiceParse(format!("Invalid modifier value: {}", subparts[1]))
                })?,
            )
        } else if modifier_part.contains('-') {
            let subparts: Vec<&str> = modifier_part.split('-').collect();
            if subparts.len() != 2 {
                return Err(RulesError::DiceParse(format!(
                    "Invalid modifier: {}",
                    modifier_part
                )));
            }
            (
                subparts[0],
                -subparts[1].parse::<i32>().map_err(|_| {
                    RulesError::DiceParse(format!("Invalid modifier value: {}", subparts[1]))
                })?,
            )
        } else {
            (modifier_part, 0)
        };

        let sides = sides_str
            .parse::<u32>()
            .map_err(|_| RulesError::DiceParse(format!("Invalid dice sides: {}", sides_str)))?;

        Ok(DiceExpression {
            count,
            sides,
            modifier,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1d20() {
        let expr = DiceRoller::parse("1d20").unwrap();
        assert_eq!(expr.count, 1);
        assert_eq!(expr.sides, 20);
        assert_eq!(expr.modifier, 0);
    }

    #[test]
    fn test_parse_2d8_plus_3() {
        let expr = DiceRoller::parse("2d8+3").unwrap();
        assert_eq!(expr.count, 2);
        assert_eq!(expr.sides, 8);
        assert_eq!(expr.modifier, 3);
    }

    #[test]
    fn test_parse_2d8_minus_3() {
        let expr = DiceRoller::parse("2d8-3").unwrap();
        assert_eq!(expr.count, 2);
        assert_eq!(expr.sides, 8);
        assert_eq!(expr.modifier, -3);
    }

    #[test]
    fn test_roll_1d20() {
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
    fn test_roll_2d8_plus_3() {
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
    fn test_roll_deterministic_seed() {
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
    fn test_roll_advantage() {
        let mut roller = DiceRoller::with_seed(42);
        let expr = DiceExpression {
            count: 2,
            sides: 20,
            modifier: 0,
        };
        let result = roller.roll(&expr, RollMode::Advantage).unwrap();
        assert_eq!(result.total, *result.rolls.iter().max().unwrap() as i32);
    }

    #[test]
    fn test_roll_disadvantage() {
        let mut roller = DiceRoller::with_seed(42);
        let expr = DiceExpression {
            count: 2,
            sides: 20,
            modifier: 0,
        };
        let result = roller.roll(&expr, RollMode::Disadvantage).unwrap();
        assert_eq!(result.total, *result.rolls.iter().min().unwrap() as i32);
    }

    #[test]
    fn test_roll_edge_cases() {
        let mut roller = DiceRoller::new();

        // 1d1
        let expr = DiceExpression {
            count: 1,
            sides: 1,
            modifier: 0,
        };
        let result = roller.roll(&expr, RollMode::Normal).unwrap();
        assert_eq!(result.total, 1);

        // 0d20
        let expr = DiceExpression {
            count: 0,
            sides: 20,
            modifier: 5,
        };
        let result = roller.roll(&expr, RollMode::Normal).unwrap();
        assert_eq!(result.total, 5);

        // Negative modifier
        let expr = DiceExpression {
            count: 1,
            sides: 20,
            modifier: -5,
        };
        let result = roller.roll(&expr, RollMode::Normal).unwrap();
        assert!(result.total >= -4 && result.total <= 15);
    }
}
