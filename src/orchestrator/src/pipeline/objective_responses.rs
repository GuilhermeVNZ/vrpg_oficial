//! Objective Responses - M3.1
//! Answers factual questions directly from GameState without calling LLM

use crate::error::Result;
use crate::pipeline::PipelineState;
use regex::Regex;
use tracing::info;

/// Answer objective question directly from GameState
pub fn answer_objective_question(state: &PipelineState, query: &str) -> Result<String> {
    info!("Answering objective question: {}", query);

    let game_state = state.game_state();
    let query_lower = query.to_lowercase();

    // Parse game_state to extract values
    let hp_regex = Regex::new(r"(?i)HP:\s*(\d+)/(\d+)").unwrap();
    let ac_regex = Regex::new(r"(?i)AC:\s*(\d+)").unwrap();
    let slots_regex = Regex::new(r"(?i)level(\d+)=(\d+)").unwrap();
    let position_regex = Regex::new(r"(?i)Position:\s*\((\d+),\s*(\d+)").unwrap();

    // Answer HP questions
    if query_lower.contains("hp") || query_lower.contains("vida") || query_lower.contains("health")
    {
        if let Some(captures) = hp_regex.captures(game_state) {
            let current = captures.get(1).map(|m| m.as_str()).unwrap_or("0");
            let max = captures.get(2).map(|m| m.as_str()).unwrap_or("0");
            return Ok(format!("Você tem {} HP de {}.", current, max));
        }
        return Ok("Não foi possível determinar seus HP.".to_string());
    }

    // Answer AC questions
    if query_lower.contains("ac")
        || query_lower.contains("armadura")
        || query_lower.contains("classe.*armadura")
    {
        if let Some(captures) = ac_regex.captures(game_state) {
            let ac = captures.get(1).map(|m| m.as_str()).unwrap_or("0");
            return Ok(format!("Sua AC é {}.", ac));
        }
        return Ok("Não foi possível determinar sua AC.".to_string());
    }

    // Answer spell slots questions
    if query_lower.contains("slot") && query_lower.contains("nível")
        || query_lower.contains("slot") && query_lower.contains("level")
    {
        if let Some(level_capture) = Regex::new(r"(?:nível|level)\s*(\d+)")
            .unwrap()
            .captures(&query_lower)
        {
            let level = level_capture.get(1).map(|m| m.as_str()).unwrap_or("");

            // Find slots for this level
            if let Some(captures) = slots_regex.captures(game_state) {
                let slot_level = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                let slot_count = captures.get(2).map(|m| m.as_str()).unwrap_or("0");

                if slot_level == level {
                    return Ok(format!(
                        "Você tem {} slot{} de nível {} restante{}.",
                        slot_count,
                        if slot_count != "1" { "s" } else { "" },
                        level,
                        if slot_count != "1" { "s" } else { "" }
                    ));
                }
            }
        }
        return Ok("Não foi possível determinar seus slots de magia.".to_string());
    }

    // Answer position questions
    if query_lower.contains("posição")
        || query_lower.contains("position")
        || query_lower.contains("posicao")
    {
        if let Some(captures) = position_regex.captures(game_state) {
            let x = captures.get(1).map(|m| m.as_str()).unwrap_or("0");
            let y = captures.get(2).map(|m| m.as_str()).unwrap_or("0");
            return Ok(format!("Você está na posição ({}, {}).", x, y));
        }
        return Ok("Não foi possível determinar sua posição.".to_string());
    }

    // Answer resources questions
    if query_lower.contains("recurso") || query_lower.contains("resource") {
        // Extract resources from game_state (Rage, Ki, etc.)
        if game_state.contains("Rage") || game_state.contains("Ki") {
            // Parse resources from game_state
            let resources: Vec<&str> = game_state
                .split(',')
                .filter(|s| s.contains("Rage") || s.contains("Ki") || s.contains("Sorcery"))
                .collect();

            if !resources.is_empty() {
                return Ok(format!("Seus recursos: {}.", resources.join(", ")));
            }
        }
        return Ok("Você não possui recursos disponíveis.".to_string());
    }

    // Default response if question type is not recognized
    Ok(format!(
        "Não foi possível responder essa pergunta objetiva. Estado do jogo: {}",
        if game_state.is_empty() {
            "vazio"
        } else {
            game_state
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_hp() {
        let mut state = PipelineState::new();
        state.update_game_state("HP: 50/50".to_string());

        let answer = answer_objective_question(&state, "Quantos HP eu tenho?").unwrap();
        assert!(answer.contains("50"));
    }

    #[test]
    fn test_answer_ac() {
        let mut state = PipelineState::new();
        state.update_game_state("AC: 15".to_string());

        let answer = answer_objective_question(&state, "Qual minha AC?").unwrap();
        assert!(answer.contains("15"));
    }
}


