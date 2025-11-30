# VRPG Client - Testes Detalhados por Task

## ⚠️ REGRA CRÍTICA: NENHUMA TASK É CONCLUÍDA SEM PASSAR EM TODOS OS TESTES

**Esta é a regra fundamental**: Uma task só pode ser marcada como concluída quando:
1. ✅ **TODOS** os testes unitários passam (100%)
2. ✅ **TODOS** os testes de integração passam (100%)
3. ✅ **TODOS** os testes de performance passam (100%)
4. ✅ **TODOS** os testes de edge cases passam (100%)
5. ✅ **TODOS** os testes de stress passam (100%)
6. ✅ Cobertura de código ≥ 95%
7. ✅ Linters passam sem warnings
8. ✅ Type checkers passam sem erros

**NÃO EXISTE EXCEÇÃO PARA ESTA REGRA.**

**Referências**:
- [TASKS.md](TASKS.md) - Tasks consolidadas de implementação
- [TESTS_MASTER.md](TESTS_MASTER.md) - Master test plan completo
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura do pipeline

---

## 🚨 FASE M: Migração para Pipeline de 3 Agentes

### Task: add-qwen-1-5b-support (M1.2)

**Módulo**: `llm-core`  
**Tipo**: Teste Unitário + Integração + Performance  
**Prioridade**: CRÍTICA

#### Testes Unitários

```rust
// tests/unit/llm_core/qwen_1_5b_test.rs

#[tokio::test]
async fn test_qwen_1_5b_model_loading() {
    // Testar carregamento do modelo Qwen-1.5B
    // Verificar que modelo é carregado corretamente
    // Verificar que memória é gerenciada (sem leaks)
    // Verificar que modelo pode ser recarregado
    // Verificar que caminho do modelo está correto
}

#[tokio::test]
async fn test_qwen_1_5b_model_loading_with_14b() {
    // Testar carregamento simultâneo de ambos modelos
    // Verificar que ambos modelos são carregados
    // Verificar que memória total é gerenciada corretamente
    // Verificar que não há conflitos de recursos
    // Verificar que ambos modelos podem inferir simultaneamente
}

#[tokio::test]
async fn test_qwen_1_5b_inference_basic() {
    // Testar inferência básica do 1.5B
    // Verificar que inferência funciona
    // Verificar que resposta é gerada
    // Verificar que max_tokens=40 é respeitado
    // Verificar que temperatura=0.8 é aplicada
    // Verificar que top_p=0.9 é aplicado
}

#[tokio::test]
async fn test_qwen_1_5b_inference_emotional_response() {
    // Testar geração de resposta emocional
    // Verificar que resposta tem 1-2 frases
    // Verificar que resposta tem 15-45 palavras
    // Verificar que resposta é emocional (não técnica)
    // Verificar que resposta não contém números
    // Verificar que resposta não resolve ações
}

#[tokio::test]
async fn test_qwen_1_5b_inference_no_final_results() {
    // Testar que 1.5B NUNCA gera resultados finais
    // Verificar que resposta não contém "você acerta"
    // Verificar que resposta não contém "você erra"
    // Verificar que resposta não contém "dano"
    // Verificar que resposta não contém "HP"
    // Verificar que resposta não contém números de resultado
}

#[tokio::test]
async fn test_qwen_1_5b_inference_no_consequences() {
    // Testar que 1.5B NUNCA gera consequências
    // Verificar que resposta não descreve resultados
    // Verificar que resposta não aplica regras
    // Verificar que resposta não resolve mecânicas
}

#[tokio::test]
async fn test_qwen_1_5b_latency_target() {
    // Testar latência do 1.5B
    // Executar 100 inferências
    // Medir p50, p95, p99
    // Verificar que p95 < 1.2s total (incluindo TTS)
    // Verificar que p99 < 1.5s
}

#[tokio::test]
async fn test_qwen_1_5b_memory_usage() {
    // Testar uso de memória com ambos modelos
    // Verificar que memória total é razoável
    // Verificar que não há memory leaks após 1000 inferências
    // Verificar que memória é liberada ao descarregar modelos
}

#[tokio::test]
async fn test_qwen_1_5b_endpoint_health() {
    // Testar endpoint /llm/prelude
    // Verificar que endpoint existe
    // Verificar que aceita requisições POST
    // Verificar que retorna JSON válido
    // Verificar que validação funciona
}

#[tokio::test]
async fn test_qwen_1_5b_endpoint_inference() {
    // Testar endpoint /llm/prelude com inferência real
    // Verificar que inferência é executada
    // Verificar que resposta é retornada
    // Verificar que latência é medida
    // Verificar que métricas são registradas
}

#[tokio::test]
async fn test_qwen_1_5b_concurrent_requests() {
    // Testar múltiplas requisições concorrentes (10+)
    // Verificar que não há race conditions
    // Verificar que cada requisição é processada corretamente
    // Verificar que latência não degrada significativamente
}

#[tokio::test]
async fn test_qwen_1_5b_error_handling() {
    // Testar tratamento de erros
    // Verificar que erros de modelo são tratados
    // Verificar que erros de inferência são tratados
    // Verificar que erros são reportados corretamente
    // Verificar que sistema continua funcionando após erro
}
```

#### Testes de Integração

```rust
// tests/integration/llm_core/qwen_1_5b_integration_test.rs

#[tokio::test]
async fn test_qwen_1_5b_with_orchestrator() {
    // Testar integração 1.5B ↔ Orquestrador
    // Verificar que Orquestrador pode chamar 1.5B
    // Verificar que resposta é recebida corretamente
    // Verificar que latência end-to-end < 1.2s
}

#[tokio::test]
async fn test_qwen_1_5b_with_tts() {
    // Testar integração 1.5B → TTS
    // Verificar que resposta do 1.5B é enviada para TTS
    // Verificar que áudio é gerado corretamente
    // Verificar que latência total < 1.2s
}

#[tokio::test]
async fn test_qwen_1_5b_pipeline_order() {
    // Testar ordem do pipeline
    // Verificar que 1.5B sempre responde antes do 14B
    // Verificar que 1.5B não espera 14B
    // Verificar que 14B recebe fast_prelude do 1.5B
}
```

#### Testes de Performance

```rust
// tests/performance/llm_core/qwen_1_5b_performance_test.rs

#[tokio::test]
async fn test_qwen_1_5b_latency_benchmark_1000_samples() {
    // Testar latência com 1000 amostras
    // Medir p50, p95, p99
    // Verificar que p95 < 1.2s total
    // Verificar que p99 < 1.5s
    // Documentar resultados
}

#[tokio::test]
async fn test_qwen_1_5b_throughput() {
    // Testar throughput (inferências por segundo)
    // Verificar que ≥ 0.8 inferências/s podem ser processadas
    // Verificar que latência não degrada sob carga
}

#[tokio::test]
async fn test_qwen_1_5b_memory_under_load() {
    // Testar uso de memória sob carga
    // Verificar que memória não cresce indefinidamente
    // Verificar que não há memory leaks
}
```

#### Testes de Stress

```rust
// tests/stress/llm_core/qwen_1_5b_stress_test.rs

#[tokio::test]
async fn test_qwen_1_5b_continuous_1_hour() {
    // Testar inferência contínua por 1 hora
    // Verificar que não há memory leaks
    // Verificar que latência não degrada
    // Verificar que qualidade não degrada
}

#[tokio::test]
async fn test_qwen_1_5b_10000_inferences() {
    // Testar 10000 inferências consecutivas
    // Verificar que não há memory leaks
    // Verificar que performance não degrada
    // Verificar que qualidade é mantida
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Modelo 1.5B carrega corretamente (100% das vezes)
- ✅ Ambos modelos podem ser carregados simultaneamente (100% das vezes)
- ✅ Inferência 1.5B < 1.2s total (p95)
- ✅ Resposta tem 1-2 frases, 15-45 palavras (≥ 95% das vezes)
- ✅ 1.5B NUNCA gera resultados finais ou consequências (0% de violações)
- ✅ Endpoint /llm/prelude funciona corretamente (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-human-bridge-phrases (M1.3)

**Módulo**: `llm-core`  
**Tipo**: Teste Unitário + Integração  
**Prioridade**: ALTA

#### Testes Unitários

```rust
// tests/unit/llm_core/bridge_phrases_test.rs

#[test]
fn test_bridge_phrases_loading() {
    // Testar carregamento de frases de ponte
    // Verificar que arquivo JSON/YAML é carregado
    // Verificar que frases são parseadas corretamente
    // Verificar que categorias são identificadas
    // Verificar que estrutura de dados é válida
}

#[test]
fn test_bridge_phrases_categories() {
    // Testar categorias de frases
    // Verificar que todas as categorias existem:
    // - tensão
    // - surpresa
    // - aprovação
    // - curiosidade
    // - cautela
    // Verificar que cada categoria tem frases suficientes (≥ 10)
}

#[test]
fn test_bridge_phrases_selection_random() {
    // Testar seleção aleatória por categoria
    // Executar 1000 seleções
    // Verificar que distribuição é aproximadamente uniforme
    // Verificar que todas as frases podem ser selecionadas
    // Verificar que seleção é realmente aleatória
}

#[test]
fn test_bridge_phrases_anti_repetition() {
    // Testar sistema anti-repetição
    // Selecionar 20 frases consecutivas
    // Verificar que nenhuma frase é repetida nas últimas 10
    // Verificar que sistema funciona corretamente
    // Verificar que após 10+ frases, frases antigas podem ser reutilizadas
}

#[test]
fn test_bridge_phrases_human_like() {
    // Testar que frases são humanas e não formulaicas
    // Verificar que frases não são genéricas demais
    // Verificar que frases têm personalidade
    // Verificar que frases são variadas
    // Verificar que frases não são repetitivas em estrutura
}

#[test]
fn test_bridge_phrases_integration_with_prompt() {
    // Testar integração com prompt do 1.5B
    // Verificar que frases são incluídas no prompt
    // Verificar que formato está correto
    // Verificar que prompt é válido
}

#[test]
fn test_bridge_phrases_edge_cases() {
    // Testar edge cases:
    // - Categoria vazia
    // - Categoria com apenas 1 frase
    // - Seleção quando todas as frases foram usadas recentemente
    // - Seleção com categoria inexistente
}
```

#### Testes de Integração

```rust
// tests/integration/llm_core/bridge_phrases_integration_test.rs

#[tokio::test]
async fn test_bridge_phrases_with_1_5b() {
    // Testar integração com 1.5B
    // Verificar que frases são usadas no prompt
    // Verificar que 1.5B pode escolher entre frases
    // Verificar que resposta é influenciada pelas frases
}

#[tokio::test]
async fn test_bridge_phrases_anti_loop() {
    // Testar que sistema previne loops
    // Executar 100 inferências consecutivas
    // Verificar que respostas não são idênticas
    // Verificar que variação é mantida
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Frases são carregadas corretamente (100% das vezes)
- ✅ Seleção aleatória funciona (distribuição uniforme)
- ✅ Anti-repetição funciona (0% de repetições nas últimas 10)
- ✅ Frases são humanas e não formulaicas (avaliação subjetiva ≥ 90%)
- ✅ Integração com 1.5B funciona (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-pipeline-state (M2.1)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração  
**Prioridade**: CRÍTICA

#### Testes Unitários

```rust
// tests/unit/orchestrator/pipeline_state_test.rs

#[test]
fn test_pipeline_status_enum() {
    // Testar enum PipelineStatus
    // Verificar que todos os estados existem:
    // - WaitingForInput
    // - Processing1_5B
    // - WaitingForFinalASR
    // - Processing14B
    // - ReadyForTTS
    // Verificar que enum é serializável
}

#[test]
fn test_pipeline_state_creation() {
    // Testar criação de PipelineState
    // Verificar que estrutura é criada corretamente
    // Verificar que campos são inicializados
    // Verificar que estado inicial é WaitingForInput
}

#[test]
fn test_pipeline_state_transitions_valid() {
    // Testar todas as transições válidas:
    // WaitingForInput → Processing1_5B
    // Processing1_5B → WaitingForFinalASR
    // WaitingForFinalASR → Processing14B
    // Processing14B → ReadyForTTS
    // ReadyForTTS → WaitingForInput
    // Verificar que cada transição funciona
}

#[test]
fn test_pipeline_state_transitions_invalid() {
    // Testar rejeição de transições inválidas:
    // WaitingForInput → Processing14B (deve ser bloqueado)
    // Processing1_5B → ReadyForTTS (deve ser bloqueado)
    // Processing14B → Processing1_5B (deve ser bloqueado)
    // Verificar que transições inválidas são rejeitadas
    // Verificar que erro é reportado
}

#[test]
fn test_pipeline_state_thread_safety() {
    // Testar thread-safety
    // Criar múltiplas threads acessando estado
    // Verificar que não há race conditions
    // Verificar que transições são atômicas
    // Verificar que estado é consistente
}

#[test]
fn test_pipeline_state_persistence() {
    // Testar persistência de estado
    // Salvar estado
    // Carregar estado
    // Verificar que estado é restaurado corretamente
    // Verificar que todos os campos são preservados
}

#[test]
fn test_pipeline_state_game_state() {
    // Testar campo game_state
    // Verificar que game_state pode ser atualizado
    // Verificar que game_state é consultado corretamente
    // Verificar que game_state é mantido em RAM
}

#[test]
fn test_pipeline_state_scene_context() {
    // Testar campo scene_context
    // Verificar que scene_context pode ser atualizado
    // Verificar que scene_context é consultado corretamente
    // Verificar que integração com Vectorizer funciona
}

#[test]
fn test_pipeline_state_lore_cache() {
    // Testar campo lore_cache
    // Verificar que lore_cache pode ser atualizado
    // Verificar que lore_cache é consultado corretamente
    // Verificar que integração com Vectorizer funciona
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/pipeline_state_integration_test.rs

#[tokio::test]
async fn test_pipeline_state_with_1_5b() {
    // Testar integração com 1.5B
    // Verificar que estado é atualizado quando 1.5B inicia
    // Verificar que estado é atualizado quando 1.5B termina
    // Verificar que transições são corretas
}

#[tokio::test]
async fn test_pipeline_state_with_14b() {
    // Testar integração com 14B
    // Verificar que estado é atualizado quando 14B inicia
    // Verificar que estado é atualizado quando 14B termina
    // Verificar que transições são corretas
}

#[tokio::test]
async fn test_pipeline_state_with_asr() {
    // Testar integração com ASR
    // Verificar que estado é atualizado quando ASR parcial chega
    // Verificar que estado é atualizado quando ASR final chega
    // Verificar que transições são corretas
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Todas as transições válidas funcionam (100% das vezes)
- ✅ Transições inválidas são rejeitadas (100% das vezes)
- ✅ Thread-safety garantido (0% de race conditions)
- ✅ Persistência funciona corretamente (100% das vezes)
- ✅ Integração com componentes funciona (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-1-5b-trigger-logic (M2.2)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração + Performance  
**Prioridade**: CRÍTICA

#### Testes Unitários

```rust
// tests/unit/orchestrator/trigger_1_5b_test.rs

#[test]
fn test_should_trigger_1_5b_time_based() {
    // Testar detecção baseada em tempo (6-8s)
    // Simular 6s de fala → deve disparar
    // Simular 7s de fala → deve disparar
    // Simular 8s de fala → deve disparar
    // Simular 5s de fala → não deve disparar
    // Simular 9s de fala → deve disparar (já passou do threshold)
}

#[test]
fn test_should_trigger_1_5b_pause_based() {
    // Testar detecção baseada em pausa
    // Simular pausa > threshold → deve disparar
    // Simular pausa < threshold → não deve disparar
    // Simular VAD detectando fim → deve disparar
}

#[test]
fn test_should_trigger_1_5b_action_based() {
    // Testar detecção baseada em ação clara
    // Simular intent parsing detectando ação → deve disparar
    // Simular intent parsing sem ação clara → não deve disparar
    // Verificar que diferentes tipos de ação são detectados
}

#[test]
fn test_should_trigger_1_5b_combined() {
    // Testar combinação de critérios
    // Verificar que qualquer critério pode disparar
    // Verificar que múltiplos critérios não causam múltiplos disparos
}

#[test]
fn test_should_trigger_1_5b_no_premature() {
    // Testar que não dispara prematuramente
    // Simular 1s de fala → não deve disparar
    // Simular 2s de fala → não deve disparar
    // Simular 3s de fala → não deve disparar
    // Simular 4s de fala → não deve disparar
    // Simular 5s de fala → não deve disparar
}

#[tokio::test]
async fn test_trigger_1_5b_function() {
    // Testar função trigger_1_5b()
    // Verificar que prompt emocional é preparado
    // Verificar que chamada a LLM Core /llm/prelude é feita
    // Verificar que texto do prelúdio é retornado
    // Verificar que latência < 1.2s
}

#[tokio::test]
async fn test_trigger_1_5b_immediate_tts() {
    // Testar envio imediato para TTS
    // Verificar que resposta do 1.5B é enviada para TTS imediatamente
    // Verificar que não espera 14B
    // Verificar que latência total < 1.2s
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/trigger_1_5b_integration_test.rs

#[tokio::test]
async fn test_trigger_1_5b_with_asr() {
    // Testar integração com ASR
    // Simular ASR parcial chegando
    // Verificar que trigger é avaliado
    // Verificar que 1.5B é disparado quando apropriado
}

#[tokio::test]
async fn test_trigger_1_5b_with_llm_core() {
    // Testar integração com LLM Core
    // Verificar que chamada a /llm/prelude é feita
    // Verificar que resposta é recebida
    // Verificar que latência é medida
}

#[tokio::test]
async fn test_trigger_1_5b_with_tts() {
    // Testar integração com TTS
    // Verificar que resposta é enviada para TTS
    // Verificar que áudio é gerado
    // Verificar que latência total < 1.2s
}
```

#### Testes de Performance

```rust
// tests/performance/orchestrator/trigger_1_5b_performance_test.rs

#[tokio::test]
async fn test_trigger_1_5b_latency_benchmark() {
    // Testar latência do trigger
    // Executar 100 triggers
    // Medir p50, p95, p99
    // Verificar que p95 < 1.2s total
    // Verificar que p99 < 1.5s
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Dispara após 6-8s de fala (≥ 95% das vezes)
- ✅ Dispara após pausa detectada (≥ 95% das vezes)
- ✅ Dispara após ação clara identificada (≥ 95% das vezes)
- ✅ Não dispara prematuramente (0% de disparos antes de 6s)
- ✅ Latência total < 1.2s (p95)
- ✅ Integração com componentes funciona (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-14b-context-preparation (M2.3)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração  
**Prioridade**: CRÍTICA

#### Testes Unitários

```rust
// tests/unit/orchestrator/context_14b_test.rs

#[test]
fn test_prepare_14b_context_fast_prelude() {
    // Testar inclusão de fast_prelude
    // Verificar que fast_prelude (texto do 1.5B) está sempre incluído
    // Verificar que formato está correto
    // Verificar que não está vazio
}

#[test]
fn test_prepare_14b_context_asr_final() {
    // Testar inclusão de asr_final
    // Verificar que asr_final (transcrição completa) está incluído
    // Verificar que formato está correto
}

#[test]
fn test_prepare_14b_context_game_state() {
    // Testar inclusão de game_state
    // Verificar que game_state (estado atual do jogo) está incluído
    // Verificar que formato está correto
    // Verificar que dados relevantes estão presentes
}

#[test]
fn test_prepare_14b_context_context_slice() {
    // Testar inclusão de context_slice
    // Verificar que últimos 3-6 eventos estão incluídos
    // Verificar que eventos recentes vêm primeiro
    // Verificar que limite de 6 eventos é respeitado
}

#[test]
fn test_prepare_14b_context_vectorizer_results() {
    // Testar inclusão de vectorizer_results
    // Verificar que resultados são incluídos quando relevante
    // Verificar que resultados não são incluídos quando não relevante
    // Verificar que formato está correto
}

#[test]
fn test_prepare_14b_context_scene_link() {
    // Testar ligação com a cena atual
    // Verificar que contexto da cena está incluído
    // Verificar que dados da cena estão corretos
}

#[test]
fn test_prepare_14b_context_token_limit() {
    // Testar limitação de tokens (8192)
    // Verificar que contexto não excede 8192 tokens
    // Verificar que priorização funciona (recente > antigo)
    // Verificar que dados importantes não são cortados
}

#[test]
fn test_prepare_14b_context_prioritization() {
    // Testar priorização de contexto
    // Verificar que eventos recentes vêm primeiro
    // Verificar que dados importantes são mantidos
    // Verificar que dados antigos são removidos primeiro
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/context_14b_integration_test.rs

#[tokio::test]
async fn test_prepare_14b_context_with_1_5b() {
    // Testar integração com 1.5B
    // Verificar que fast_prelude do 1.5B está incluído
    // Verificar que formato está correto
}

#[tokio::test]
async fn test_prepare_14b_context_with_vectorizer() {
    // Testar integração com Vectorizer
    // Verificar que resultados do Vectorizer são incluídos quando relevante
    // Verificar que consultas são feitas corretamente
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ fast_prelude está sempre incluído (100% das vezes)
- ✅ Contexto não excede limite de tokens (100% das vezes)
- ✅ Priorização funciona corretamente (eventos recentes primeiro)
- ✅ vectorizer_results são incluídos quando relevante (≥ 90% das vezes)
- ✅ Integração com componentes funciona (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-complete-pipeline-flow (M2.4)

**Módulo**: `orchestrator`  
**Tipo**: Teste Integração + E2E + Performance  
**Prioridade**: CRÍTICA

#### Testes de Integração

```rust
// tests/integration/orchestrator/pipeline_flow_test.rs

#[tokio::test]
async fn test_pipeline_flow_end_to_end() {
    // Testar fluxo completo end-to-end:
    // ASR → Intent Router → 1.5B → Wait Final ASR → 14B → TTS
    // Verificar que cada etapa funciona
    // Verificar que ordem é respeitada
    // Verificar que latência total < 6s
}

#[tokio::test]
async fn test_pipeline_flow_1_5b_before_14b() {
    // Testar que 1.5B sempre dispara antes do 14B
    // Executar 100 fluxos completos
    // Verificar que em 100% dos casos, 1.5B dispara antes do 14B
    // Verificar que ordem é mantida mesmo sob carga
}

#[tokio::test]
async fn test_pipeline_flow_asr_partial() {
    // Testar recepção de asr_partial
    // Verificar que asr_partial é recebido corretamente
    // Verificar que trigger do 1.5B é avaliado
    // Verificar que estado é atualizado
}

#[tokio::test]
async fn test_pipeline_flow_intent_parsing() {
    // Testar parsing de intent
    // Verificar que intent é parseado corretamente
    // Verificar que intent router funciona
    // Verificar que routing é correto
}

#[tokio::test]
async fn test_pipeline_flow_wait_final_asr() {
    // Testar espera por asr_final
    // Verificar que sistema espera asr_final
    // Verificar que timeout é respeitado
    // Verificar que estado é atualizado quando asr_final chega
}

#[tokio::test]
async fn test_pipeline_flow_14b_context() {
    // Testar preparação de contexto para 14B
    // Verificar que contexto é preparado corretamente
    // Verificar que fast_prelude está incluído
    // Verificar que contexto não excede limite de tokens
}

#[tokio::test]
async fn test_pipeline_flow_14b_call() {
    // Testar chamada ao 14B
    // Verificar que 14B é chamado com contexto completo
    // Verificar que resposta é recebida
    // Verificar que latência < 6s
}

#[tokio::test]
async fn test_pipeline_flow_tts_send() {
    // Testar envio para TTS
    // Verificar que narrativa é enviada para TTS
    // Verificar que áudio é gerado
    // Verificar que latência total < 6s
}

#[tokio::test]
async fn test_pipeline_flow_state_updates() {
    // Testar atualização de estado
    // Verificar que estado é atualizado em cada etapa
    // Verificar que transições são corretas
    // Verificar que estado final é WaitingForInput
}
```

#### Testes de Tratamento de Erros

```rust
// tests/integration/orchestrator/pipeline_flow_error_test.rs

#[tokio::test]
async fn test_pipeline_flow_asr_failure() {
    // Testar falha do ASR
    // Verificar que erro é tratado graciosamente
    // Verificar que sistema continua funcionando
    // Verificar que erro é reportado
}

#[tokio::test]
async fn test_pipeline_flow_llm_failure() {
    // Testar falha do LLM (1.5B ou 14B)
    // Verificar que erro é tratado graciosamente
    // Verificar que sistema continua funcionando
    // Verificar que fallback é aplicado se disponível
}

#[tokio::test]
async fn test_pipeline_flow_tts_failure() {
    // Testar falha do TTS
    // Verificar que erro é tratado graciosamente
    // Verificar que sistema continua funcionando
    // Verificar que erro é reportado
}
```

#### Testes de Performance

```rust
// tests/performance/orchestrator/pipeline_flow_performance_test.rs

#[tokio::test]
async fn test_pipeline_flow_latency_benchmark() {
    // Testar latência do fluxo completo
    // Executar 100 fluxos completos
    // Medir p50, p95, p99
    // Verificar que p95 < 6s total
    // Verificar que p99 < 8s
    // Documentar resultados
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Fluxo completo end-to-end funciona (100% das vezes)
- ✅ 1.5B sempre dispara antes do 14B (100% das vezes)
- ✅ Latência total < 6s (p95)
- ✅ Tratamento de erros funciona (100% dos casos de erro tratados)
- ✅ Estado é atualizado corretamente (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-objective-responses (M3.1)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração + Performance  
**Prioridade**: ALTA

#### Testes Unitários

```rust
// tests/unit/orchestrator/objective_responses_test.rs

#[test]
fn test_is_objective_question_detection() {
    // Testar detecção de perguntas objetivas
    // "Quantos HP eu tenho?" → deve detectar
    // "Quantos slots nível X eu tenho?" → deve detectar
    // "Qual minha AC?" → deve detectar
    // "Qual minha posição?" → deve detectar
    // "O que acontece se eu atacar?" → não deve detectar (narrativa)
    // Verificar que detecção é precisa (≥ 95%)
}

#[test]
fn test_answer_objective_question_hp() {
    // Testar resposta para "Quantos HP eu tenho?"
    // Verificar que game_state é consultado
    // Verificar que resposta é retornada sem chamar LLM
    // Verificar que resposta está correta
    // Verificar que latência < 50ms
}

#[test]
fn test_answer_objective_question_slots() {
    // Testar resposta para "Quantos slots nível X eu tenho?"
    // Verificar que game_state é consultado
    // Verificar que resposta é retornada sem chamar LLM
    // Verificar que resposta está correta
    // Verificar que latência < 50ms
}

#[test]
fn test_answer_objective_question_ac() {
    // Testar resposta para "Qual minha AC?"
    // Verificar que game_state é consultado
    // Verificar que resposta é retornada sem chamar LLM
    // Verificar que resposta está correta
    // Verificar que latência < 50ms
}

#[test]
fn test_answer_objective_question_position() {
    // Testar resposta para "Qual minha posição?"
    // Verificar que game_state é consultado
    // Verificar que resposta é retornada sem chamar LLM
    // Verificar que resposta está correta
    // Verificar que latência < 50ms
}

#[test]
fn test_objective_responses_no_llm_call() {
    // Testar que LLM não é chamado para perguntas objetivas
    // Executar 100 perguntas objetivas
    // Verificar que LLM não é chamado em nenhum caso
    // Verificar que todas as respostas são retornadas diretamente
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/objective_responses_integration_test.rs

#[tokio::test]
async fn test_objective_responses_with_game_state() {
    // Testar integração com game_state
    // Verificar que game_state é consultado corretamente
    // Verificar que respostas são baseadas em dados reais
    // Verificar que atualizações de game_state são refletidas
}
```

#### Testes de Performance

```rust
// tests/performance/orchestrator/objective_responses_performance_test.rs

#[tokio::test]
async fn test_objective_responses_latency_benchmark() {
    // Testar latência de respostas objetivas
    // Executar 1000 perguntas objetivas
    // Medir p50, p95, p99
    // Verificar que p95 < 50ms
    // Verificar que p99 < 100ms
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Detecção de perguntas objetivas funciona (≥ 95% precisão)
- ✅ Respostas são corretas (100% das vezes)
- ✅ LLM não é chamado para perguntas objetivas (0% de chamadas)
- ✅ Latência < 50ms (p95)
- ✅ Integração com game_state funciona (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-simple-rule-query (M3.2)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração + Performance  
**Prioridade**: ALTA

#### Testes Unitários

```rust
// tests/unit/orchestrator/simple_rule_query_test.rs

#[test]
fn test_is_simple_rule_question_detection() {
    // Testar detecção de perguntas de regra simples
    // "Stealth usa Destreza?" → deve detectar
    // "Investigation é Inteligência?" → deve detectar
    // "Como funciona o sistema de magias?" → não deve detectar (narrativa)
    // Verificar que detecção é precisa (≥ 95%)
}

#[tokio::test]
async fn test_simple_rule_query_vectorizer() {
    // Testar consulta ao Vectorizer
    // Verificar que consulta é feita corretamente
    // Verificar que resultado é recebido
    // Verificar que resultado é relevante
}

#[tokio::test]
async fn test_simple_rule_query_1_5b_conversion() {
    // Testar conversão via 1.5B
    // Verificar que resultado do Vectorizer é enviado para 1.5B
    // Verificar que 1.5B converte em resposta humana
    // Verificar que resposta é natural e não técnica
    // Verificar que 14B não é chamado
}

#[tokio::test]
async fn test_simple_rule_query_no_14b() {
    // Testar que 14B não é chamado para regras simples
    // Executar 100 perguntas de regra simples
    // Verificar que 14B não é chamado em nenhum caso
    // Verificar que apenas 1.5B é usado
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/simple_rule_query_integration_test.rs

#[tokio::test]
async fn test_simple_rule_query_with_vectorizer() {
    // Testar integração com Vectorizer
    // Verificar que consultas são feitas corretamente
    // Verificar que resultados são recebidos
    // Verificar que latência < 1.5s total
}

#[tokio::test]
async fn test_simple_rule_query_with_1_5b() {
    // Testar integração com 1.5B
    // Verificar que conversão funciona corretamente
    // Verificar que resposta é humana
    // Verificar que latência < 1.5s total
}
```

#### Testes de Performance

```rust
// tests/performance/orchestrator/simple_rule_query_performance_test.rs

#[tokio::test]
async fn test_simple_rule_query_latency_benchmark() {
    // Testar latência de consulta de regras simples
    // Executar 100 perguntas
    // Medir p50, p95, p99
    // Verificar que p95 < 1.5s total
    // Verificar que p99 < 2s
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Detecção de perguntas de regra simples funciona (≥ 95% precisão)
- ✅ Consulta ao Vectorizer funciona (100% das vezes)
- ✅ Conversão via 1.5B funciona (100% das vezes)
- ✅ 14B não é chamado para regras simples (0% de chamadas)
- ✅ Latência < 1.5s total (p95)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-game-state-cache (M4.1)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração + Performance  
**Prioridade**: ALTA

#### Testes Unitários

```rust
// tests/unit/orchestrator/game_state_cache_test.rs

#[test]
fn test_game_state_cache_creation() {
    // Testar criação de GameStateCache
    // Verificar que estrutura é criada corretamente
    // Verificar que campos são inicializados
}

#[test]
fn test_game_state_cache_storage_hp() {
    // Testar armazenamento de HP
    // Verificar que HP por entidade é armazenado
    // Verificar que HP pode ser recuperado
    // Verificar que HP pode ser atualizado
}

#[test]
fn test_game_state_cache_storage_ac() {
    // Testar armazenamento de AC
    // Verificar que AC por entidade é armazenado
    // Verificar que AC pode ser recuperado
    // Verificar que AC pode ser atualizado
}

#[test]
fn test_game_state_cache_storage_resources() {
    // Testar armazenamento de recursos
    // Verificar que recursos (rage, slots, smites, ki) são armazenados
    // Verificar que recursos podem ser recuperados
    // Verificar que recursos podem ser atualizados
}

#[test]
fn test_game_state_cache_storage_status() {
    // Testar armazenamento de status
    // Verificar que status (poisoned, stealth, prone, etc) são armazenados
    // Verificar que status podem ser recuperados
    // Verificar que status podem ser atualizados
}

#[test]
fn test_game_state_cache_storage_position() {
    // Testar armazenamento de posição
    // Verificar que posição (grid 2D/3D) é armazenada
    // Verificar que posição pode ser recuperada
    // Verificar que posição pode ser atualizada
}

#[test]
fn test_game_state_cache_storage_initiative() {
    // Testar armazenamento de iniciativa
    // Verificar que iniciativa é armazenada
    // Verificar que iniciativa pode ser recuperada
    // Verificar que iniciativa pode ser atualizada
}

#[test]
fn test_game_state_cache_update() {
    // Testar atualização de cache
    // Verificar que cache é atualizado quando estado muda
    // Verificar que atualizações são atômicas
    // Verificar que consistência é mantida
}

#[test]
fn test_game_state_cache_query() {
    // Testar consulta rápida de cache
    // Verificar que consultas são rápidas (< 10ms)
    // Verificar que resultados são corretos
    // Verificar que consultas não bloqueiam
}

#[test]
fn test_game_state_cache_invalidation() {
    // Testar invalidação de cache
    // Verificar que cache é invalidado quando necessário
    // Verificar que invalidação é completa
    // Verificar que cache pode ser reconstruído
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/game_state_cache_integration_test.rs

#[tokio::test]
async fn test_game_state_cache_with_game_engine() {
    // Testar integração com game-engine
    // Verificar que cache é atualizado quando game-engine muda estado
    // Verificar que consultas refletem estado atual
}
```

#### Testes de Performance

```rust
// tests/performance/orchestrator/game_state_cache_performance_test.rs

#[tokio::test]
async fn test_game_state_cache_latency_benchmark() {
    // Testar latência de consultas
    // Executar 10000 consultas
    // Medir p50, p95, p99
    // Verificar que p95 < 10ms
    // Verificar que p99 < 20ms
}

#[tokio::test]
async fn test_game_state_cache_hit_miss_metrics() {
    // Testar métricas de hit/miss
    // Verificar que métricas são coletadas
    // Verificar que hit rate é alto (≥ 90%)
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Armazenamento e recuperação funcionam (100% das vezes)
- ✅ Atualização de cache funciona (100% das vezes)
- ✅ Invalidação de cache funciona (100% das vezes)
- ✅ Latência < 10ms para consultas (p95)
- ✅ Hit rate ≥ 90%
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-scene-context-cache (M4.2)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração  
**Prioridade**: ALTA

#### Testes Unitários

```rust
// tests/unit/orchestrator/scene_context_cache_test.rs

#[test]
fn test_scene_context_cache_creation() {
    // Testar criação de SceneContextCache
    // Verificar que estrutura é criada corretamente
}

#[test]
fn test_scene_context_cache_storage_actions() {
    // Testar armazenamento de últimas 3-6 ações
    // Verificar que ações são armazenadas
    // Verificar que limite de 6 eventos é respeitado
    // Verificar que ações antigas são removidas
}

#[test]
fn test_scene_context_cache_storage_rolls() {
    // Testar armazenamento de resultados de rolagens
    // Verificar que rolagens são armazenadas
    // Verificar que rolagens podem ser recuperadas
}

#[test]
fn test_scene_context_cache_storage_npcs() {
    // Testar armazenamento de NPCs ativos
    // Verificar que NPCs são armazenados
    // Verificar que NPCs podem ser recuperados
}

#[test]
fn test_scene_context_cache_storage_interactions() {
    // Testar armazenamento de "quem interagiu com quem"
    // Verificar que interações são armazenadas
    // Verificar que interações podem ser recuperadas
}

#[test]
fn test_scene_context_cache_limit() {
    // Testar limite de histórico (máximo 6 eventos)
    // Verificar que não armazena mais que 6 eventos
    // Verificar que eventos antigos são removidos
    // Verificar que eventos recentes são mantidos
}

#[test]
fn test_scene_context_cache_context_slice() {
    // Testar preparação de context_slice para 14B
    // Verificar que context_slice é preparado corretamente
    // Verificar que eventos recentes vêm primeiro
    // Verificar que formato está correto
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/scene_context_cache_integration_test.rs

#[tokio::test]
async fn test_scene_context_cache_with_vectorizer() {
    // Testar integração com Vectorizer
    // Verificar que busca semântica funciona
    // Verificar que resultados são relevantes
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Armazenamento de eventos recentes funciona (100% das vezes)
- ✅ Limite de histórico é respeitado (100% das vezes)
- ✅ Preparação de context_slice funciona (100% das vezes)
- ✅ Integração com Vectorizer funciona (100% das vezes)
- ✅ Cobertura de código ≥ 95%

---

### Task: implement-lore-cache (M4.3)

**Módulo**: `orchestrator`  
**Tipo**: Teste Unitário + Integração + Performance  
**Prioridade**: ALTA

#### Testes Unitários

```rust
// tests/unit/orchestrator/lore_cache_test.rs

#[test]
fn test_lore_cache_creation() {
    // Testar criação de LoreCache
    // Verificar que estrutura é criada corretamente
}

#[tokio::test]
async fn test_lore_cache_vectorizer_races() {
    // Testar consulta de descrição de raças
    // Verificar que consulta ao Vectorizer é feita
    // Verificar que resultado é recebido
    // Verificar que resultado é relevante
}

#[tokio::test]
async fn test_lore_cache_vectorizer_cities() {
    // Testar consulta de cidades/regiões/dungeons
    // Verificar que consulta funciona
    // Verificar que resultados são relevantes
}

#[tokio::test]
async fn test_lore_cache_vectorizer_npcs() {
    // Testar consulta de NPCs recorrentes
    // Verificar que consulta funciona
    // Verificar que resultados são relevantes
}

#[tokio::test]
async fn test_lore_cache_vectorizer_history() {
    // Testar consulta de história da campanha
    // Verificar que consulta funciona
    // Verificar que resultados são relevantes
}

#[tokio::test]
async fn test_lore_cache_vectorizer_areas() {
    // Testar consulta de áreas, facções, crenças
    // Verificar que consulta funciona
    // Verificar que resultados são relevantes
}

#[tokio::test]
async fn test_lore_cache_query_cache() {
    // Testar cache de queries frequentes (TTL: 5 minutos)
    // Verificar que queries frequentes são cacheadas
    // Verificar que TTL é respeitado
    // Verificar que cache é invalidado após TTL
}

#[test]
fn test_lore_cache_lore_context_preparation() {
    // Testar preparação de lore_context para 14B
    // Verificar que lore_context é preparado corretamente
    // Verificar que formato está correto
    // Verificar que dados relevantes estão presentes
}
```

#### Testes de Integração

```rust
// tests/integration/orchestrator/lore_cache_integration_test.rs

#[tokio::test]
async fn test_lore_cache_with_vectorizer() {
    // Testar integração com Vectorizer
    // Verificar que consultas são feitas corretamente
    // Verificar que resultados são recebidos
    // Verificar que latência < 100ms para consultas cacheadas
}
```

#### Testes de Performance

```rust
// tests/performance/orchestrator/lore_cache_performance_test.rs

#[tokio::test]
async fn test_lore_cache_latency_benchmark() {
    // Testar latência de consultas
    // Executar 100 consultas (50 cacheadas, 50 não cacheadas)
    // Medir p50, p95, p99
    // Verificar que p95 < 100ms para consultas cacheadas
    // Verificar que cache reduz latência em ≥ 50%
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Consulta ao Vectorizer funciona (100% das vezes)
- ✅ Cache de queries frequentes funciona (100% das vezes)
- ✅ Preparação de lore_context funciona (100% das vezes)
- ✅ Latência < 100ms para consultas cacheadas (p95)
- ✅ Cache reduz latência em ≥ 50%
- ✅ Cobertura de código ≥ 95%

---

### Task: test-pipeline-integration (M5.1)

**Módulo**: `orchestrator` + `llm-core` + `asr-service` + `tts-service`  
**Tipo**: Teste E2E + Integração  
**Prioridade**: CRÍTICA

#### Testes E2E

```rust
// tests/e2e/pipeline/pipeline_integration_test.rs

#[tokio::test]
async fn test_pipeline_end_to_end_asr_to_tts() {
    // Testar fluxo completo end-to-end:
    // ASR → 1.5B → 14B → TTS
    // Verificar que cada etapa funciona
    // Verificar que ordem é respeitada
    // Verificar que latência total < 6s
}

#[tokio::test]
async fn test_pipeline_1_5b_always_before_14b() {
    // Testar que 1.5B sempre dispara antes do 14B
    // Executar 100 fluxos completos
    // Verificar que em 100% dos casos, 1.5B dispara antes do 14B
    // Verificar que ordem é mantida mesmo sob carga
}

#[tokio::test]
async fn test_pipeline_1_5b_no_final_results() {
    // Testar que 1.5B não gera resultados finais
    // Executar 100 fluxos completos
    // Verificar que em 0% dos casos, 1.5B gera resultados finais
    // Verificar que 1.5B apenas gera prelúdio emocional
}

#[tokio::test]
async fn test_pipeline_14b_receives_fast_prelude() {
    // Testar que 14B recebe fast_prelude
    // Executar 100 fluxos completos
    // Verificar que em 100% dos casos, 14B recebe fast_prelude do 1.5B
    // Verificar que fast_prelude está no contexto
}

#[tokio::test]
async fn test_pipeline_objective_responses_no_llm() {
    // Testar respostas objetivas sem LLM
    // Executar 100 perguntas objetivas
    // Verificar que em 100% dos casos, LLM não é chamado
    // Verificar que respostas são retornadas diretamente
}

#[tokio::test]
async fn test_pipeline_simple_rule_query_1_5b_only() {
    // Testar consulta de regras simples (Vectorizer + 1.5B)
    // Executar 100 perguntas de regra simples
    // Verificar que em 100% dos casos, apenas 1.5B é usado
    // Verificar que 14B não é chamado
}

#[tokio::test]
async fn test_pipeline_narrative_rule_query_14b() {
    // Testar consulta de regras narrativas (14B)
    // Executar 100 perguntas de regra narrativa
    // Verificar que em 100% dos casos, 14B é usado
    // Verificar que contexto completo é preparado
}

#[tokio::test]
async fn test_pipeline_error_handling_asr_failure() {
    // Testar tratamento de erro quando ASR falha
    // Simular falha do ASR
    // Verificar que erro é tratado graciosamente
    // Verificar que sistema continua funcionando
}

#[tokio::test]
async fn test_pipeline_error_handling_llm_failure() {
    // Testar tratamento de erro quando LLM falha
    // Simular falha do 1.5B ou 14B
    // Verificar que erro é tratado graciosamente
    // Verificar que fallback é aplicado se disponível
}

#[tokio::test]
async fn test_pipeline_error_handling_tts_failure() {
    // Testar tratamento de erro quando TTS falha
    // Simular falha do TTS
    // Verificar que erro é tratado graciosamente
    // Verificar que sistema continua funcionando
}

#[tokio::test]
async fn test_pipeline_cache_game_state() {
    // Testar cache de game_state
    // Verificar que game_state é armazenado
    // Verificar que game_state é consultado corretamente
    // Verificar que atualizações são refletidas
}

#[tokio::test]
async fn test_pipeline_cache_scene_context() {
    // Testar cache de scene_context
    // Verificar que scene_context é armazenado
    // Verificar que scene_context é consultado corretamente
    // Verificar que limite de 6 eventos é respeitado
}

#[tokio::test]
async fn test_pipeline_cache_lore_cache() {
    // Testar cache de lore_cache
    // Verificar que lore_cache é armazenado
    // Verificar que lore_cache é consultado corretamente
    // Verificar que TTL é respeitado
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Todos os testes de integração passam (100%)
- ✅ Cobertura de testes > 95%
- ✅ Latência medida e dentro dos targets
- ✅ 1.5B sempre dispara antes do 14B (100% das vezes)
- ✅ 1.5B não gera resultados finais (0% de violações)
- ✅ 14B sempre recebe fast_prelude (100% das vezes)
- ✅ Respostas objetivas não chamam LLM (0% de chamadas)
- ✅ Regras simples usam apenas 1.5B (0% de chamadas ao 14B)
- ✅ Tratamento de erros funciona (100% dos casos)

---

### Task: test-pipeline-performance (M5.2)

**Módulo**: `orchestrator` + `llm-core`  
**Tipo**: Teste Performance + Benchmark  
**Prioridade**: ALTA

#### Testes de Performance

```rust
// tests/performance/pipeline/pipeline_performance_test.rs

#[tokio::test]
async fn test_pipeline_1_5b_latency_benchmark() {
    // Testar latência do 1.5B
    // Executar 1000 inferências
    // Medir p50, p95, p99
    // Verificar que p95 < 1.2s
    // Verificar que p99 < 1.5s
    // Documentar resultados
}

#[tokio::test]
async fn test_pipeline_14b_latency_benchmark() {
    // Testar latência do 14B
    // Executar 100 inferências
    // Medir p50, p95, p99
    // Verificar que p95 < 6s
    // Verificar que p99 < 8s
    // Documentar resultados
}

#[tokio::test]
async fn test_pipeline_objective_responses_latency_benchmark() {
    // Testar latência de respostas objetivas
    // Executar 10000 perguntas objetivas
    // Medir p50, p95, p99
    // Verificar que p95 < 50ms
    // Verificar que p99 < 100ms
    // Documentar resultados
}

#[tokio::test]
async fn test_pipeline_simple_rule_query_latency_benchmark() {
    // Testar latência de consulta de regras simples
    // Executar 100 perguntas
    // Medir p50, p95, p99
    // Verificar que p95 < 1.5s
    // Verificar que p99 < 2s
    // Documentar resultados
}

#[tokio::test]
async fn test_pipeline_memory_usage_both_models() {
    // Testar uso de memória com ambos modelos
    // Carregar ambos modelos
    // Executar 1000 inferências
    // Medir uso de memória
    // Verificar que não há memory leaks
    // Documentar resultados
}

#[tokio::test]
async fn test_pipeline_throughput() {
    // Testar throughput (interações/minuto)
    // Executar pipeline por 1 minuto
    // Contar interações processadas
    // Verificar que throughput é razoável (≥ 10 interações/min)
    // Documentar resultados
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Todos os benchmarks dentro dos targets
- ✅ Documentação de resultados completa
- ✅ Métricas são coletadas e reportadas
- ✅ Comparação com targets é feita

---

### Task: test-pipeline-regression (M5.3)

**Módulo**: Todos os módulos  
**Tipo**: Teste Regressão  
**Prioridade**: ALTA

#### Testes de Regressão

```rust
// tests/regression/pipeline/pipeline_regression_test.rs

#[tokio::test]
async fn test_regression_combat_still_works() {
    // Testar que combate ainda funciona após migração
    // Executar fluxo completo de combate
    // Verificar que todas as funcionalidades funcionam
    // Verificar que não há regressões
}

#[tokio::test]
async fn test_regression_dialogs_still_work() {
    // Testar que diálogos ainda funcionam após migração
    // Executar fluxo completo de diálogo
    // Verificar que todas as funcionalidades funcionam
    // Verificar que não há regressões
}

#[tokio::test]
async fn test_regression_rolls_still_work() {
    // Testar que rolagens ainda funcionam após migração
    // Executar vários tipos de rolagem
    // Verificar que todas as funcionalidades funcionam
    // Verificar que não há regressões
}

#[tokio::test]
async fn test_regression_memory_still_works() {
    // Testar que memória ainda funciona após migração
    // Executar consultas de memória
    // Verificar que todas as funcionalidades funcionam
    // Verificar que não há regressões
}

#[tokio::test]
async fn test_regression_ui_still_works() {
    // Testar que UI ainda funciona após migração
    // Executar interações com UI
    // Verificar que todas as funcionalidades funcionam
    // Verificar que não há regressões
}

#[tokio::test]
async fn test_regression_all_existing_tests_pass() {
    // Executar todos os testes existentes do sistema
    // Verificar que todos os testes passam
    // Verificar que não há novos testes falhando
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Todos os testes existentes passam (100%)
- ✅ Nenhuma regressão identificada
- ✅ Todas as funcionalidades existentes ainda funcionam

---

## Outras Tasks Principais

### Task: implement-rules5e-service

**Ver**: [TESTS_MASTER.md](TESTS_MASTER.md) - Seção "Task: implement-rules5e-service" (linhas 1286-1679)

---

### Task: implement-asr-service

**Ver**: [TESTS_MASTER.md](TESTS_MASTER.md) - Seção "2. Real-time ASR Pipeline" (linhas 578-771)

---

### Task: implement-tts-service

**Ver**: [TESTS_MASTER.md](TESTS_MASTER.md) - Seção "3. TTS Latency" (linhas 773-972)

---

### Task: implement-llm-core

**Ver**: [TESTS_MASTER.md](TESTS_MASTER.md) - Seção "1. LLM Persona Switching e Geração de INTENT DSL" (linhas 399-574)

**Nota**: Testes devem ser atualizados para incluir pipeline dual (1.5B + 14B)

---

### Task: implement-orchestrator

**Ver**: [TESTS_MASTER.md](TESTS_MASTER.md) - Seção "0. Orquestrador e INTENT DSL" (linhas 77-214)

**Nota**: Testes devem ser atualizados para incluir pipeline de 3 agentes

---

## Resumo de Critérios de Sucesso

### Pipeline de 3 Agentes (Fase M)
- ✅ Latência do 1.5B < 1.2s (p95)
- ✅ Latência do 14B < 6s (p95)
- ✅ 1.5B sempre dispara antes do 14B (100% das vezes)
- ✅ 1.5B nunca gera resultados finais (0% de violações)
- ✅ 14B sempre recebe fast_prelude (100% das vezes)
- ✅ Respostas objetivas < 50ms (p95)
- ✅ Regras simples < 1.5s (p95)
- ✅ Cobertura de código ≥ 95% para todas as tasks

### Geral
- ✅ Todos os testes passam (100%)
- ✅ Cobertura ≥ 95%
- ✅ Linters passam sem warnings
- ✅ Type checkers passam sem erros
- ✅ Nenhuma regressão identificada

---

**Última Atualização**: 2025-01-XX

**Referências**:
- [TASKS.md](TASKS.md) - Tasks consolidadas
- [TESTS_MASTER.md](TESTS_MASTER.md) - Master test plan
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura do pipeline

