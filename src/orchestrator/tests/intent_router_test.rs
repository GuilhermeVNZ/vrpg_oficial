//! Tests for Intent Router - M2.4

use orchestrator::intent_router::{IntentClassification, IntentRouter};
use orchestrator::pipeline::PipelineState;

#[test]
fn test_classify_fact_query_hp() {
    let router = IntentRouter::new();
    let input = "Quantos HP eu tenho?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::FactQuery);
    assert!(classification.confidence >= 0.95);
}

#[test]
fn test_classify_fact_query_ac() {
    let router = IntentRouter::new();
    let input = "Qual minha AC?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::FactQuery);
}

#[test]
fn test_classify_fact_query_spell_slots() {
    let router = IntentRouter::new();
    let input = "Quantos slots nível 3 eu tenho?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::FactQuery);
}

#[test]
fn test_classify_fact_query_position() {
    let router = IntentRouter::new();
    let input = "Qual minha posição?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::FactQuery);
}

#[test]
fn test_classify_simple_rule_query() {
    let router = IntentRouter::new();
    let input = "Stealth usa Destreza?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(
        classification.intent_type,
        IntentClassification::SimpleRuleQuery
    );
}

#[test]
fn test_classify_simple_rule_query_investigation() {
    let router = IntentRouter::new();
    let input = "Investigation é Inteligência?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(
        classification.intent_type,
        IntentClassification::SimpleRuleQuery
    );
}

#[test]
fn test_classify_meta_query() {
    let router = IntentRouter::new();
    let input = "Como funciona o sistema?";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::MetaQuery);
}

#[test]
fn test_classify_world_action() {
    let router = IntentRouter::new();
    let input = "Eu quero abrir a porta";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(
        classification.intent_type,
        IntentClassification::WorldAction
    );
}

#[test]
fn test_classify_combat_action_attack() {
    let router = IntentRouter::new();
    let input = "Eu ataco o goblin";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(
        classification.intent_type,
        IntentClassification::CombatAction
    );
}

#[test]
fn test_classify_spell_cast() {
    let router = IntentRouter::new();
    let input = "Eu lanço Fireball no goblin";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::SpellCast);
}

#[test]
fn test_classify_move() {
    let router = IntentRouter::new();
    let input = "Eu me movo para a direita";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(classification.intent_type, IntentClassification::Move);
}

#[test]
fn test_classify_roll_request() {
    let router = IntentRouter::new();
    let input = "Eu rolo um d20";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    assert_eq!(
        classification.intent_type,
        IntentClassification::RollRequest
    );
}

#[test]
fn test_classify_latency() {
    let router = IntentRouter::new();
    let input = "Quantos HP eu tenho?";
    let start = std::time::Instant::now();
    let _classification = router.classify(input, &PipelineState::new()).unwrap();
    let elapsed = start.elapsed();

    // Should be < 10ms for regex-based classification
    assert!(
        elapsed.as_millis() < 10,
        "Classification should be fast (< 10ms), took: {:?}",
        elapsed
    );
}

#[test]
fn test_classify_cache_reduces_latency() {
    let router = IntentRouter::new();
    let input = "Quantos HP eu tenho?";

    // First classification (no cache)
    let start1 = std::time::Instant::now();
    let _classification1 = router.classify(input, &PipelineState::new()).unwrap();
    let elapsed1 = start1.elapsed();

    // Second classification (should use cache)
    let start2 = std::time::Instant::now();
    let _classification2 = router.classify(input, &PipelineState::new()).unwrap();
    let elapsed2 = start2.elapsed();

    // Cached should be at least 50% faster
    assert!(
        elapsed2.as_nanos() < elapsed1.as_nanos() / 2,
        "Cached classification should be faster. First: {:?}, Second: {:?}",
        elapsed1,
        elapsed2
    );
}

#[test]
fn test_classify_confidence_scores() {
    let router = IntentRouter::new();

    // Clear cases should have high confidence
    let clear_cases = vec![
        ("Quantos HP eu tenho?", IntentClassification::FactQuery),
        (
            "Stealth usa Destreza?",
            IntentClassification::SimpleRuleQuery,
        ),
        ("Eu ataco o goblin", IntentClassification::CombatAction),
    ];

    for (input, expected) in clear_cases {
        let classification = router.classify(input, &PipelineState::new()).unwrap();
        assert_eq!(classification.intent_type, expected, "Input: {}", input);
        assert!(
            classification.confidence >= 0.95,
            "Clear cases should have high confidence (>= 0.95), got: {}",
            classification.confidence
        );
    }
}

#[test]
fn test_classify_uncertain_fallback() {
    let router = IntentRouter::new();
    // Ambiguous input that regex can't classify
    let input = "Talvez eu devesse fazer algo interessante";
    let classification = router.classify(input, &PipelineState::new()).unwrap();

    // Should fall back to Uncertain, which will trigger 1.5B
    assert_eq!(classification.intent_type, IntentClassification::Uncertain);
    assert!(
        classification.confidence < 0.95,
        "Uncertain cases should have lower confidence"
    );
}

#[test]
fn test_classify_multiple_sequential_requests() {
    let router = IntentRouter::new();
    let inputs = vec![
        "Quantos HP eu tenho?",
        "Qual minha AC?",
        "Eu ataco o goblin",
        "Stealth usa Destreza?",
    ];

    for input in inputs {
        let classification = router.classify(input, &PipelineState::new()).unwrap();
        assert!(
            !matches!(classification.intent_type, IntentClassification::Uncertain),
            "Should classify clear input: {}",
            input
        );
    }
}
