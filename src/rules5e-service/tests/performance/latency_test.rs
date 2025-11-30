use rules5e_service::dice::{DiceRoller, DiceExpression, RollMode};
use rules5e_service::attack::{AttackResolver, AttackRequest};
use rules5e_service::damage::{DamageResolver, DamageRequest, Damage, DamageType};
use std::time::Instant;

fn percentile(data: &mut [u64], p: f64) -> u64 {
    data.sort();
    let index = (data.len() as f64 * p) as usize;
    data[index.min(data.len() - 1)]
}

#[test]
fn test_dice_roll_latency_10000_rolls() {
    let mut roller = DiceRoller::new();
    let expr = DiceExpression {
        count: 1,
        sides: 20,
        modifier: 0,
    };
    
    let mut latencies = Vec::with_capacity(10000);
    
    for _ in 0..10000 {
        let start = Instant::now();
        let _ = roller.roll(&expr, RollMode::Normal).unwrap();
        let duration = start.elapsed();
        latencies.push(duration.as_micros() as u64);
    }
    
    let p50 = percentile(&mut latencies.clone(), 0.50);
    let p95 = percentile(&mut latencies.clone(), 0.95);
    let p99 = percentile(&mut latencies.clone(), 0.99);
    
    println!("Dice Roll Latency (10000 rolls):");
    println!("  p50: {}μs", p50);
    println!("  p95: {}μs", p95);
    println!("  p99: {}μs", p99);
    
    // p95 should be < 5ms = 5000μs
    assert!(p95 < 5000, "p95 latency {}μs exceeds 5ms", p95);
    // p99 should be < 10ms = 10000μs
    assert!(p99 < 10000, "p99 latency {}μs exceeds 10ms", p99);
}

#[test]
fn test_attack_calculation_latency_10000_attacks() {
    let resolver = AttackResolver::new();
    let request = AttackRequest {
        attack_bonus: 5,
        ac: 15,
        advantage: false,
        disadvantage: false,
        seed: None,
    };
    
    let mut latencies = Vec::with_capacity(10000);
    
    for i in 0..10000 {
        let start = Instant::now();
        let mut req = request.clone();
        req.seed = Some(i);
        let _ = resolver.resolve(&req).unwrap();
        let duration = start.elapsed();
        latencies.push(duration.as_micros() as u64);
    }
    
    let p50 = percentile(&mut latencies.clone(), 0.50);
    let p95 = percentile(&mut latencies.clone(), 0.95);
    let p99 = percentile(&mut latencies.clone(), 0.99);
    
    println!("Attack Calculation Latency (10000 attacks):");
    println!("  p50: {}μs", p50);
    println!("  p95: {}μs", p95);
    println!("  p99: {}μs", p99);
    
    // p95 should be < 5ms = 5000μs
    assert!(p95 < 5000, "p95 latency {}μs exceeds 5ms", p95);
    // p99 should be < 10ms = 10000μs
    assert!(p99 < 10000, "p99 latency {}μs exceeds 10ms", p99);
}

#[test]
fn test_damage_calculation_latency_10000_damages() {
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
    
    let mut latencies = Vec::with_capacity(10000);
    
    for _ in 0..10000 {
        let start = Instant::now();
        let _ = resolver.resolve(&request);
        let duration = start.elapsed();
        latencies.push(duration.as_micros() as u64);
    }
    
    let p50 = percentile(&mut latencies.clone(), 0.50);
    let p95 = percentile(&mut latencies.clone(), 0.95);
    let p99 = percentile(&mut latencies.clone(), 0.99);
    
    println!("Damage Calculation Latency (10000 damages):");
    println!("  p50: {}μs", p50);
    println!("  p95: {}μs", p95);
    println!("  p99: {}μs", p99);
    
    // p95 should be < 5ms = 5000μs
    assert!(p95 < 5000, "p95 latency {}μs exceeds 5ms", p95);
    // p99 should be < 10ms = 10000μs
    assert!(p99 < 10000, "p99 latency {}μs exceeds 10ms", p99);
}


