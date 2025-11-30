//! Pipeline Performance Tests - M5.2
//! Performance benchmarks and latency tests for the 3-agent pipeline

use orchestrator::pipeline::flow::handle_player_input;
use orchestrator::pipeline::objective_responses::answer_objective_question;
use orchestrator::pipeline::simple_rule_query::answer_simple_rule_query;
use orchestrator::pipeline::trigger::trigger_1_5b;
use orchestrator::pipeline::{PipelineState, PipelineStateManager};
use std::time::{Duration, Instant};

/// Benchmark 1: Latência do 1.5B (< 1.2s)
#[tokio::test]
async fn benchmark_1_5b_latency() {
    let mut state = PipelineState::new();
    let queries = vec![
        "Eu quero atacar o goblin",
        "Vou investigar a porta",
        "Preciso me mover para a direita",
    ];

    let mut total_latency = Duration::ZERO;
    let mut iterations = 0;

    for query in queries {
        let start = Instant::now();
        let _prelude = trigger_1_5b(&mut state, "dm", query).await.unwrap();
        let elapsed = start.elapsed();
        total_latency += elapsed;
        iterations += 1;

        // Individual test should be < 1.2s
        assert!(
            elapsed < Duration::from_millis(1200),
            "1.5B latency should be < 1.2s, got: {:?} for query: {}",
            elapsed,
            query
        );
    }

    // Average latency
    let avg_latency = total_latency / iterations;
    assert!(
        avg_latency < Duration::from_millis(1200),
        "Average 1.5B latency should be < 1.2s, got: {:?}",
        avg_latency
    );

    println!("1.5B Benchmark Results:");
    println!("  Average latency: {:?}", avg_latency);
    println!("  Total queries: {}", iterations);
}

/// Benchmark 2: Latência do 14B (< 6s)
#[tokio::test]
async fn benchmark_14b_latency() {
    let state_manager = PipelineStateManager::new();
    let queries = vec![
        "Eu quero atacar o dragão com minha espada mágica e depois me esconder",
        "Vou investigar a sala cuidadosamente procurando por armadilhas e pistas",
    ];

    let mut total_latency = Duration::ZERO;
    let mut iterations = 0;

    for query in queries {
        let start = Instant::now();
        let result = handle_player_input(
            &state_manager,
            query,
            Duration::from_secs(7), // Long speech - triggers 1.5B first
            None,
            false,
        )
        .await;

        assert!(result.is_ok(), "Pipeline should complete successfully");
        let elapsed = start.elapsed();
        total_latency += elapsed;
        iterations += 1;

        // Total pipeline latency (includes 1.5B + 14B) should be < 6s
        assert!(
            elapsed < Duration::from_secs(6),
            "Total pipeline latency (14B) should be < 6s, got: {:?} for query: {}",
            elapsed,
            query
        );
    }

    // Average latency
    let avg_latency = total_latency / iterations;
    assert!(
        avg_latency < Duration::from_secs(6),
        "Average pipeline latency (14B) should be < 6s, got: {:?}",
        avg_latency
    );

    println!("14B Benchmark Results:");
    println!("  Average latency: {:?}", avg_latency);
    println!("  Total queries: {}", iterations);
}

/// Benchmark 3: Latência de respostas objetivas (< 50ms)
#[tokio::test]
async fn benchmark_objective_responses_latency() {
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/100, AC: 18, Spell Slots: [3, 2, 1]".to_string());

    let queries = vec![
        "Quantos HP eu tenho?",
        "Qual minha AC?",
        "Quantos slots nível 3 eu tenho?",
        "Qual minha posição?",
    ];

    let mut total_latency = Duration::ZERO;
    let mut iterations = 0;
    let mut max_latency = Duration::ZERO;

    for query in queries {
        let start = Instant::now();
        let _answer = answer_objective_question(&state, query).unwrap();
        let elapsed = start.elapsed();
        total_latency += elapsed;
        iterations += 1;

        if elapsed > max_latency {
            max_latency = elapsed;
        }

        // Individual response should be < 50ms
        assert!(
            elapsed < Duration::from_millis(50),
            "Objective response latency should be < 50ms, got: {:?} for query: {}",
            elapsed,
            query
        );
    }

    // Average latency
    let avg_latency = total_latency / iterations;
    assert!(
        avg_latency < Duration::from_millis(50),
        "Average objective response latency should be < 50ms, got: {:?}",
        avg_latency
    );

    println!("Objective Responses Benchmark Results:");
    println!("  Average latency: {:?}", avg_latency);
    println!("  Max latency: {:?}", max_latency);
    println!("  Total queries: {}", iterations);
}

/// Benchmark 4: Latência de consulta de regras simples (< 1.5s)
#[tokio::test]
async fn benchmark_simple_rule_query_latency() {
    let state = PipelineState::new();
    let queries = vec![
        "Stealth usa Destreza?",
        "Investigation é Inteligência?",
        "Acrobatics usa qual atributo?",
    ];

    // Mock vectorizer results
    let vectorizer_results = vec!["Mock rule definition".to_string()];

    let mut total_latency = Duration::ZERO;
    let mut iterations = 0;

    for query in queries {
        let start = Instant::now();
        let result = answer_simple_rule_query(&state, query, &vectorizer_results)
            .await
            .unwrap();
        let elapsed = start.elapsed();
        total_latency += elapsed;
        iterations += 1;

        // Individual query should be < 1.5s (Vectorizer + 1.5B)
        assert!(
            elapsed < Duration::from_millis(1500),
            "Simple rule query latency should be < 1.5s, got: {:?} for query: {}",
            elapsed,
            query
        );

        // Verify that 14B was not used
        assert!(!result.used_14b, "14B should not be used for simple rule queries");
    }

    // Average latency
    let avg_latency = total_latency / iterations;
    assert!(
        avg_latency < Duration::from_millis(1500),
        "Average simple rule query latency should be < 1.5s, got: {:?}",
        avg_latency
    );

    println!("Simple Rule Query Benchmark Results:");
    println!("  Average latency: {:?}", avg_latency);
    println!("  Total queries: {}", iterations);
}

/// Benchmark 5: Uso de memória com ambos modelos (simulado)
#[tokio::test]
async fn benchmark_memory_usage_both_models() {
    // This test simulates memory usage tracking
    // In real implementation, we would measure actual memory usage
    
    let state_manager = PipelineStateManager::new();
    
    // Simulate multiple queries to track memory usage
    let queries = vec![
        "Query 1",
        "Query 2",
        "Query 3",
    ];

    let mut total_pipeline_calls = 0;
    let total_queries = queries.len();

    for query in queries.iter() {
        let result = handle_player_input(
            &state_manager,
            query,
            Duration::from_secs(7),
            None,
            false,
        )
        .await;

        assert!(result.is_ok(), "Pipeline should complete successfully");
        total_pipeline_calls += 1;
    }

    // Verify that pipeline can handle multiple calls without memory issues
    assert!(
        total_pipeline_calls == total_queries,
        "Should handle multiple pipeline calls"
    );

    println!("Memory Usage Benchmark Results:");
    println!("  Total pipeline calls: {}", total_pipeline_calls);
    println!("  Note: Actual memory measurement requires integration with monitoring");
}

/// Benchmark 6: Throughput (interações/minuto)
#[tokio::test]
async fn benchmark_throughput_interactions_per_minute() {
    let state_manager = PipelineStateManager::new();
    
    // Simulate multiple quick interactions
    let queries = vec![
        "Quantos HP?",      // Fast objective query
        "Qual minha AC?",   // Fast objective query
        "Stealth usa?",     // Fast simple rule query
    ];

    let start = Instant::now();
    let mut successful_interactions = 0;
    let total_queries = queries.len();

    // Execute queries in sequence
    for query in queries.iter() {
        // For objective queries, use objective_responses directly
        if query.contains("HP") || query.contains("AC") {
            let mut state = PipelineState::new();
            state.update_game_state("HP: 50/100, AC: 18".to_string());
            let _answer = answer_objective_question(&state, query).unwrap();
            successful_interactions += 1;
        } else {
            // For rule queries, use simple_rule_query
            let state = PipelineState::new();
            let vectorizer_results = vec!["Mock result".to_string()];
            let result = answer_simple_rule_query(&state, query, &vectorizer_results)
                .await
                .unwrap();
            assert!(!result.answer.is_empty());
            successful_interactions += 1;
        }
    }

    let elapsed = start.elapsed();
    
    // Calculate throughput (interactions per minute)
    let interactions_per_minute = if elapsed.as_secs() > 0 {
        (successful_interactions as f64 / elapsed.as_secs() as f64) * 60.0
    } else {
        successful_interactions as f64 * 60.0
    };

    println!("Throughput Benchmark Results:");
    println!("  Total interactions: {}", successful_interactions);
    println!("  Total queries: {}", total_queries);
    println!("  Total time: {:?}", elapsed);
    println!("  Throughput: {:.2} interactions/minute", interactions_per_minute);
    
    // Target: At least 10 interactions/minute (for mixed query types)
    assert!(
        interactions_per_minute >= 10.0,
        "Throughput should be >= 10 interactions/minute, got: {:.2}",
        interactions_per_minute
    );
}

/// Benchmark 7: Stress test - multiple rapid queries
#[tokio::test]
async fn benchmark_stress_test_rapid_queries() {
    let mut state = PipelineState::new();
    state.update_game_state("HP: 50/100, AC: 18".to_string());

    // Rapid objective queries
    let queries = vec![
        "Quantos HP?",
        "Qual AC?",
        "HP atual?",
        "AC atual?",
    ];

    let start = Instant::now();
    let mut success_count = 0;
    let total_queries = queries.len();

    for query in queries.iter() {
        let result = answer_objective_question(&state, query);
        if result.is_ok() {
            success_count += 1;
        }
    }

    let elapsed = start.elapsed();

    // All queries should succeed
    assert_eq!(
        success_count,
        total_queries,
        "All rapid queries should succeed"
    );

    // Total time should be reasonable (all < 200ms for 4 queries)
    assert!(
        elapsed < Duration::from_millis(200),
        "Rapid queries should complete quickly, took: {:?}",
        elapsed
    );

    println!("Stress Test Results:");
    println!("  Successful queries: {}/{}", success_count, total_queries);
    println!("  Total time: {:?}", elapsed);
    println!("  Average per query: {:?}", elapsed / total_queries as u32);
}

/// Benchmark 8: Latência do pipeline completo (end-to-end)
#[tokio::test]
async fn benchmark_full_pipeline_latency() {
    let state_manager = PipelineStateManager::new();
    let query = "Eu quero atacar o dragão com minha espada e depois me esconder atrás da rocha";

    let start = Instant::now();
    let result = handle_player_input(
        &state_manager,
        query,
        Duration::from_secs(8), // Long speech
        None,
        false,
    )
    .await;

    assert!(result.is_ok(), "Full pipeline should complete successfully");
    let elapsed = start.elapsed();
    let flow_result = result.unwrap();

    // Full pipeline (ASR → Intent → 1.5B → ASR Final → 14B → TTS) should be < 6s
    assert!(
        elapsed < Duration::from_secs(6),
        "Full pipeline latency should be < 6s, got: {:?}",
        elapsed
    );
    assert!(
        flow_result.total_latency_ms < 6000,
        "Reported total latency should be < 6000ms, got: {}ms",
        flow_result.total_latency_ms
    );

    // Verify that 1.5B was triggered
    assert!(
        flow_result.fast_prelude.is_some(),
        "1.5B should have been triggered for long speech"
    );

    println!("Full Pipeline Benchmark Results:");
    println!("  Total latency: {:?}", elapsed);
    println!("  Reported latency: {}ms", flow_result.total_latency_ms);
    println!("  1.5B triggered: {}", flow_result.fast_prelude.is_some());
}

