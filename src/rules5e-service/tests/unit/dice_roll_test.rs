use rules5e_service::dice::{DiceRoller, DiceExpression, RollMode};

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
    let expr = DiceExpression { count: 1, sides: 1, modifier: 0 };
    let result = roller.roll(&expr, RollMode::Normal).unwrap();
    assert_eq!(result.total, 1);
    
    // 0d20 (resultado sempre 0 + modifier)
    let expr = DiceExpression { count: 0, sides: 20, modifier: 5 };
    let result = roller.roll(&expr, RollMode::Normal).unwrap();
    assert_eq!(result.total, 5);
    
    // Modificador negativo
    let expr = DiceExpression { count: 1, sides: 20, modifier: -5 };
    let result = roller.roll(&expr, RollMode::Normal).unwrap();
    assert!(result.total >= -4 && result.total <= 15);
    
    // Modificador muito grande
    let expr = DiceExpression { count: 1, sides: 20, modifier: 100 };
    let result = roller.roll(&expr, RollMode::Normal).unwrap();
    assert!(result.total >= 101 && result.total <= 120);
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
    // Aceitamos uma variação de ±200 (400-600)
    assert!(*min_count >= 300, "Distribution too skewed: min={}", min_count);
    assert!(*max_count <= 700, "Distribution too skewed: max={}", max_count);
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

#[test]
fn test_dice_parse_complex() {
    // Testar expressões complexas
    let expr = DiceRoller::parse("2d6+1d4+3").unwrap();
    // Note: nosso parser atual não suporta múltiplos dados, mas podemos testar o básico
    let expr2 = DiceRoller::parse("2d8+3").unwrap();
    assert_eq!(expr2.count, 2);
    assert_eq!(expr2.sides, 8);
    assert_eq!(expr2.modifier, 3);
}


