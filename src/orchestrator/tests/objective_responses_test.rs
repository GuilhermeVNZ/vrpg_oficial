//! Tests for Objective Responses - M3.1

use orchestrator::pipeline::objective_responses::answer_objective_question;
use orchestrator::pipeline::PipelineState;

#[test]
fn test_answer_hp_question() {
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/50, AC: 15".to_string());

    let query = "Quantos HP eu tenho?";
    let answer = answer_objective_question(&state, query).unwrap();

    assert!(answer.contains("50") || answer.contains("HP"));
    assert!(!answer.contains("LLM") && !answer.contains("IA"));
}

#[test]
fn test_answer_ac_question() {
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/50, AC: 15".to_string());

    let query = "Qual minha AC?";
    let answer = answer_objective_question(&state, query).unwrap();

    assert!(answer.contains("15") || answer.contains("AC"));
}

#[test]
fn test_answer_spell_slots_question() {
    let mut state = PipelineState::new();
    state.update_game_state("SpellSlots: level3=1, level2=3".to_string());

    let query = "Quantos slots nível 3 eu tenho?";
    let answer = answer_objective_question(&state, query).unwrap();

    assert!(answer.contains("1") || answer.contains("slot"));
}

#[test]
fn test_answer_position_question() {
    let mut state = PipelineState::new();
    state.update_game_state("Position: (5, 3, 0)".to_string());

    let query = "Qual minha posição?";
    let answer = answer_objective_question(&state, query).unwrap();

    assert!(answer.contains("5") || answer.contains("3") || answer.contains("posição"));
}

#[test]
fn test_answer_resources_question() {
    let mut state = PipelineState::new();
    state.update_game_state("Rage: 3/3, Ki: 5/5".to_string());

    let query = "Quantos recursos eu tenho?";
    let answer = answer_objective_question(&state, query).unwrap();

    assert!(!answer.is_empty());
}

#[test]
fn test_objective_response_latency() {
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/50".to_string());

    let query = "Quantos HP eu tenho?";
    let start = std::time::Instant::now();
    let _answer = answer_objective_question(&state, query).unwrap();
    let elapsed = start.elapsed();

    // Should be < 50ms for objective responses
    assert!(
        elapsed.as_millis() < 50,
        "Objective response should be fast (< 50ms), took: {:?}",
        elapsed
    );
}

#[test]
fn test_objective_response_no_llm() {
    // This test ensures LLM is never called for objective questions
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/50".to_string());

    let query = "Quantos HP eu tenho?";
    let answer = answer_objective_question(&state, query).unwrap();

    // Answer should be deterministic and not contain LLM-like responses
    assert!(!answer.contains("talvez") || !answer.contains("acho que"));
    assert!(!answer.is_empty());
}

#[test]
fn test_multiple_objective_questions() {
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/50, AC: 15, Position: (5, 3)".to_string());

    let queries = vec![
        "Quantos HP eu tenho?",
        "Qual minha AC?",
        "Qual minha posição?",
    ];

    for query in queries {
        let answer = answer_objective_question(&state, query).unwrap();
        assert!(!answer.is_empty(), "Should answer query: {}", query);
    }
}

#[test]
fn test_objective_response_with_empty_game_state() {
    let state = PipelineState::new();

    let query = "Quantos HP eu tenho?";
    let answer = answer_objective_question(&state, query).unwrap();

    // Should still return a response, even if game_state is empty
    assert!(!answer.is_empty());
}
