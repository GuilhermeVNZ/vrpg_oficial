# VRPG Client - Master Test Plan (Completo e Profundo)

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

## Goals Principais

**Objetivo Central**: Garantir gameplay consistente e determinístico com loops voz→ação de baixa latência.

### Requisitos Críticos de Performance
- **Latência TTS**: < 150ms para síntese de voz
- **Pipeline ASR**: Tempo real com chunks de 320ms
- **Latência Voz→Voz**: < 300ms (target: 250ms)
- **Game State Transitions**: Instantâneas e determinísticas

## Testes Críticos do Sistema

### 1. LLM Persona Switching
Testar mudança de persona do LLM entre: Mestre (DM), NPCs específicos, Jogadores IA, Monstros, Narrador cinematográfico.

**Requisito**: Persona deve ser consistente e mantida durante a interação.

### 2. Real-time ASR Pipeline
Testar pipeline de reconhecimento de voz em tempo real: Captura de áudio via getUserMedia, Segmentação em chunks, Transcrição incremental, Detecção de início/fim de fala (VAD).

**Requisito**: Latência < 80ms para chunks de 320ms.

### 3. TTS Latency
Testar síntese de voz com múltiplas vozes: Síntese para Mestre, NPCs, monstros, Aplicação de efeitos de áudio, Cache de frases comuns.

**Requisito**: Latência < 150ms para falas curtas.

### 4. Game State Transitions
Testar transições de estado do jogo: Progressão de turnos, Aplicação de dano e condições, Mudanças de cena, Persistência de sessão.

**Requisito**: Transições determinísticas e instantâneas.

### 5. Rules5e Calculations
Testar cálculos determinísticos das regras D&D 5e: Rolagem de dados, Ataques e dano, Testes de habilidade, Salvaguardas.

**Requisito**: Resultados 100% determinísticos e consistentes.

### 6. Memory Lookup via Vectorizer/Nexus
Testar busca de memória usando stack Hive: Busca semântica (Vectorizer), Busca textual (Lexum), Relações de grafo (Nexus).

**Requisito**: Resultados relevantes em < 100ms.

### 7. Document Ingestion via Transmutation
Testar processamento de documentos: Conversão PDF → Markdown, OCR de imagens, Transcrição de áudio/vídeo, Classificação automática.

**Requisito**: Pipeline completo funcional e otimizado para LLM.

---

## Visão Geral

Este documento detalha **TODOS os testes necessários** para cada task do VRPG Client, com profundidade máxima e cobertura completa de edge cases, stress tests e regressão.

**Cobertura Mínima**: 95% (conforme AGENTS.md)  
**Estrutura**: Testes organizados por task, tipo (unitário, integração, E2E, performance, stress) e prioridade.

---

## Testes Críticos do Sistema (Cross-Module)

### 0. Orquestrador e INTENT DSL
**Módulo**: `orchestrator`  
**Tipo**: Teste de Integração + Regressão + Performance  
**Prioridade**: CRÍTICA

**Testes Detalhados**:

#### Testes Unitários
```rust
// tests/unit/orchestrator/fsm_test.rs

#[tokio::test]
async fn test_fsm_transitions() {
    // Testar todas as transições válidas entre estados de cena
    // SocialFreeFlow → Exploration
    // SocialFreeFlow → CombatTurnBased
    // Exploration → CombatTurnBased
    // CombatTurnBased → SocialFreeFlow/Exploration
    // Qualquer estado → DowntimePreparation
}

#[tokio::test]
async fn test_fsm_invalid_transitions() {
    // Testar que transições inválidas são rejeitadas
    // Ex: DowntimePreparation → CombatTurnBased (deve ser bloqueado)
}

#[tokio::test]
async fn test_fsm_state_persistence() {
    // Testar que estado é persistido corretamente
    // Testar que estado é restaurado após restart
}
```

```rust
// tests/unit/orchestrator/intent_parser_test.rs

#[test]
fn test_parse_skill_check_intent() {
    // Testar parsing de INTENT SkillCheck
    // Verificar que campos são extraídos corretamente
}

#[test]
fn test_parse_melee_attack_intent() {
    // Testar parsing de INTENT MeleeAttack
    // Verificar que campos são extraídos corretamente
}

#[test]
fn test_parse_intent_validation() {
    // Testar validação de INTENTs
    // Verificar que INTENTs inválidas são rejeitadas
}

#[test]
fn test_parse_intent_normalization() {
    // Testar normalização de INTENTs
    // Trim whitespace, remover aspas redundantes
}

#[test]
fn test_parse_intent_edge_cases() {
    // Testar edge cases:
    // - INTENTs malformadas
    // - Campos faltando
    // - Valores inválidos
    // - Múltiplas INTENTs no mesmo bloco
}
```

```rust
// tests/unit/orchestrator/intent_executor_test.rs

#[tokio::test]
async fn test_execute_skill_check() {
    // Testar execução de SkillCheck
    // Verificar que RollRequest é gerado corretamente
}

#[tokio::test]
async fn test_execute_melee_attack() {
    // Testar execução de MeleeAttack
    // Verificar que chamada a rules5e-service é feita
}

#[tokio::test]
async fn test_execute_lore_query() {
    // Testar execução de LoreQuery
    // Verificar que chamada a memory-service é feita
}

#[tokio::test]
async fn test_execute_combat_start() {
    // Testar execução de CombatStart
    // Verificar que transição para CombatTurnBased ocorre
}
```

#### Testes de Integração
```rust
// tests/integration/orchestrator/integration_test.rs

#[tokio::test]
async fn test_orchestrator_with_llm_core() {
    // Testar integração Orquestrador ↔ LLM Core
    // Verificar que INTENTs são recebidas e executadas
}

#[tokio::test]
async fn test_orchestrator_with_rules5e_service() {
    // Testar integração Orquestrador ↔ Rules5e Service
    // Verificar que ações são resolvidas corretamente
}

#[tokio::test]
async fn test_orchestrator_with_memory_service() {
    // Testar integração Orquestrador ↔ Memory Service
    // Verificar que consultas funcionam
}

#[tokio::test]
async fn test_orchestrator_full_pipeline() {
    // Testar pipeline completo:
    // ASR → LLM → INTENT DSL → Orquestrador → Services → TTS
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Parser funciona com 100% de precisão em casos de teste
- ✅ Todas as transições de FSM funcionam corretamente
- ✅ Todas as INTENTs são executadas corretamente
- ✅ Integrações com services funcionam
- ✅ Latência de parsing < 10ms (p95)
- ✅ Latência de execução < 50ms (p95)
- ✅ Cobertura de código ≥ 95%

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Fase 1, [ORCHESTRATOR.md](ORCHESTRATOR.md), [INTENT_DSL.md](INTENT_DSL.md)

---

### 0.1 Turn Engine (Combate em Turnos)
**Módulo**: `orchestrator` (turn_engine.rs)  
**Tipo**: Teste de Integração + Performance  
**Prioridade**: CRÍTICA

**Testes Detalhados**:

#### Testes Unitários
```rust
// tests/unit/orchestrator/turn_engine_test.rs

#[test]
fn test_initiative_calculation() {
    // Testar cálculo de iniciativa (1d20 + DEX_MOD)
    // Verificar que valores são corretos
}

#[test]
fn test_initiative_ordering() {
    // Testar ordenação de participantes por iniciativa
    // Verificar que ordem está correta
}

#[test]
fn test_action_economy() {
    // Testar economia de ações:
    // - Ação consumida corretamente
    // - Movimento consumido corretamente
    // - Reação consumida corretamente
    // - Bonus Action consumido corretamente
    // - Bloqueio quando recursos esgotados
}

#[test]
fn test_line_of_sight() {
    // Testar cálculo de Line of Sight
    // Verificar que obstáculos bloqueiam LoS
}

#[test]
fn test_range_check() {
    // Testar verificação de alcance
    // Verificar que distâncias são calculadas corretamente
}

#[test]
fn test_area_of_effect() {
    // Testar cálculo de Áreas de Efeito
    // Verificar que células atingidas são corretas
}

#[test]
fn test_initiative_advancement() {
    // Testar avanço de iniciativa
    // Verificar que próxima criatura é selecionada corretamente
    // Verificar que rounds são incrementados
}
```

#### Testes de Integração
```rust
// tests/integration/orchestrator/turn_engine_integration_test.rs

#[tokio::test]
async fn test_combat_full_round() {
    // Testar round completo de combate:
    // - Iniciativa calculada
    // - Turnos executados em ordem
    // - Ações resolvidas
    // - Condições aplicadas
}

#[tokio::test]
async fn test_roll_request_client_side() {
    // Testar que RollRequest é enviado para client
    // Testar que RollResult é recebido e validado
}

#[tokio::test]
async fn test_roll_npc_server_side() {
    // Testar que rolagens de NPCs são feitas no servidor
    // Verificar que resultados são corretos
}

#[tokio::test]
async fn test_combat_end_detection() {
    // Testar detecção de fim de combate
    // Verificar que transição para SocialFreeFlow/Exploration ocorre
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Iniciativa calculada e ordenada corretamente
- ✅ Rolagens client vs servidor funcionam
- ✅ Economia de ações respeitada
- ✅ LoS e alcance funcionam corretamente
- ✅ AoE funciona corretamente
- ✅ Avanço de iniciativa funciona
- ✅ Cobertura de código ≥ 95%

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 2.3, [COMBAT_FLOW.md](COMBAT_FLOW.md)

---

### 0.2 Voice INTENTS
**Módulo**: `tts-service` + `orchestrator`  
**Tipo**: Teste de Integração + Performance  
**Prioridade**: ALTA

**Testes Detalhados**:

#### Testes Unitários
```rust
// tests/unit/tts_service/voice_intent_test.rs

#[test]
fn test_parse_voice_intent_narrate() {
    // Testar parsing de VOICE_INTENT:NARRATE
}

#[test]
fn test_parse_voice_intent_npc_dialogue() {
    // Testar parsing de VOICE_INTENT:NPC_DIALOGUE
}

#[test]
fn test_parse_voice_intent_player_dialogue() {
    // Testar parsing de VOICE_INTENT:PLAYER_DIALOGUE
}

#[test]
fn test_parse_voice_intent_event() {
    // Testar parsing de VOICE_INTENT:EVENT
}

#[test]
fn test_parse_voice_intent_condition_expire() {
    // Testar parsing de VOICE_INTENT:CONDITION_EXPIRE
}

#[test]
fn test_voice_intent_prioritization() {
    // Testar priorização de vozes:
    // NARRATE > PLAYER_DIALOGUE > NPC_DIALOGUE > EVENT > FX > MUSIC
}
```

#### Testes de Integração
```rust
// tests/integration/tts_service/voice_intent_integration_test.rs

#[tokio::test]
async fn test_voice_intent_with_piper_sovits() {
    // Testar que Voice INTENTs são processadas pelo pipeline Piper → SoVITS
    // Verificar que perfis vocais são aplicados
}

#[tokio::test]
async fn test_voice_intent_with_orchestrator() {
    // Testar integração Voice INTENTs ↔ Orquestrador
    // Verificar que callbacks narrativos funcionam
}

#[tokio::test]
async fn test_voice_intent_multi_voice() {
    // Testar múltiplas vozes simultâneas
    // Verificar que priorização funciona
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Todos os tipos de Voice INTENT são parseados corretamente
- ✅ Perfis vocais são aplicados corretamente
- ✅ Priorização funciona
- ✅ Latência < 800ms total (Qwen + Piper + SoVITS)
- ✅ Cobertura de código ≥ 95%

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 5.1, [VOICE_INTENTS.md](VOICE_INTENTS.md)

---

### 1. LLM Persona Switching e Geração de INTENT DSL
**Módulo**: `llm-core`  
**Tipo**: Teste de Integração + Regressão  
**Prioridade**: CRÍTICA

**Testes Detalhados**:

#### Testes Unitários
```rust
// tests/unit/llm_core/persona_test.rs

#[tokio::test]
async fn test_dm_persona_consistency_10_interactions() {
    // Testar que persona DM é mantida durante 10+ interações consecutivas
    // Verificar que respostas seguem estilo narrativo de DM
    // Verificar que não há "bleeding" entre personas
    // Verificar que contexto é preservado entre interações
    // Verificar que sistema de prompts mantém persona
}

#[tokio::test]
async fn test_dm_persona_under_stress() {
    // Testar persona DM sob stress (múltiplas requisições rápidas)
    // Verificar que persona não degrada sob carga
    // Verificar que latência não afeta consistência
}

#[tokio::test]
async fn test_npc_persona_switching_rapid() {
    // Testar mudança rápida entre diferentes NPCs (5+ em sequência)
    // Verificar que cada NPC mantém sua personalidade
    // Verificar que contexto é preservado
    // Verificar que não há contaminação entre NPCs
}

#[tokio::test]
async fn test_npc_persona_consistency_long_conversation() {
    // Testar conversa longa com NPC (20+ turnos)
    // Verificar que personalidade é mantida
    // Verificar que memória de contexto funciona
}

#[tokio::test]
async fn test_player_ia_persona_decision_making() {
    // Testar persona de jogador IA em decisões
    // Verificar que decisões seguem personalidade do personagem
    // Verificar que não há interferência do DM
    // Verificar que alinhamento é respeitado
}

#[tokio::test]
async fn test_monster_persona_combat_behavior() {
    // Testar persona de monstro em combate
    // Verificar que ações seguem comportamento do monstro
    // Verificar que não há "humanização" excessiva
    // Verificar que inteligência do monstro é respeitada
}

#[tokio::test]
async fn test_narrator_cinematic_persona_style() {
    // Testar persona de narrador cinematográfico
    // Verificar que descrições são cinematográficas
    // Verificar que estilo é diferente de DM normal
    // Verificar que transições são suaves
}

#[tokio::test]
async fn test_persona_switching_edge_cases() {
    // Testar edge cases de mudança de persona:
    // - Mudança durante streaming de tokens
    // - Mudança durante requisição em andamento
    // - Mudança com cache KV ativo
    // - Mudança com múltiplas requisições concorrentes
}

#[tokio::test]
async fn test_persona_bleeding_detection() {
    // Testar detecção de "bleeding" entre personas
    // Verificar que respostas não contêm elementos de outras personas
    // Verificar que estilo é consistente
    // Verificar que vocabulário é apropriado
}
```

#### Testes de Integração
```rust
// tests/integration/llm_core/persona_integration_test.rs

#[tokio::test]
async fn test_persona_with_memory_service() {
    // Testar persona com integração Memory Service
    // Verificar que contexto de memória não afeta persona
    // Verificar que busca de memória é apropriada para persona
}

#[tokio::test]
async fn test_persona_with_rules5e_service() {
    // Testar persona com integração Rules5e Service
    // Verificar que pedidos de rolagem são apropriados para persona
    // Verificar que cálculos não afetam persona
}

#[tokio::test]
async fn test_persona_with_orchestrator() {
    // Testar persona com integração Orquestrador
    // Verificar que INTENTs geradas são apropriadas para persona
    // Verificar que narração é separada de INTENTs
}

#[tokio::test]
async fn test_intent_dsl_generation() {
    // Testar geração de INTENT DSL pelo LLM
    // Verificar que formato está correto
    // Verificar que INTENTs são válidas
    // Verificar que narração não contém INTENTs
}

#[tokio::test]
async fn test_intent_dsl_fallback() {
    // Testar fallback quando parsing de INTENT falha
    // Verificar que sistema continua funcionando
    // Verificar que erro é tratado graciosamente
}
```

#### Testes de Performance
```rust
// tests/performance/llm_core/persona_performance_test.rs

#[tokio::test]
async fn test_persona_switching_latency() {
    // Testar latência de mudança de persona
    // Verificar que < 50ms para mudança
    // Verificar que não há degradação de performance
}

#[tokio::test]
async fn test_persona_consistency_under_load() {
    // Testar consistência de persona sob carga (100+ requisições/min)
    // Verificar que persona não degrada
    // Verificar que latência permanece aceitável
}
```

#### Testes de Stress
```rust
// tests/stress/llm_core/persona_stress_test.rs

#[tokio::test]
async fn test_persona_1000_switches() {
    // Testar 1000 mudanças de persona consecutivas
    // Verificar que não há memory leaks
    // Verificar que performance não degrada
    // Verificar que consistência é mantida
}

#[tokio::test]
async fn test_persona_concurrent_requests() {
    // Testar múltiplas requisições concorrentes com diferentes personas
    // Verificar que não há race conditions
    // Verificar que cada requisição mantém sua persona
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Persona mantida por ≥ 10 interações consecutivas (100% das vezes)
- ✅ Respostas seguem estilo da persona (verificação heurística: ≥ 95% de match)
- ✅ Sem "bleeding" entre personas (0% de contaminação detectada)
- ✅ INTENT DSL gerada com formato correto (≥ 95% das vezes)
- ✅ Narração separada de INTENTs corretamente (100% das vezes)
- ✅ Latência de mudança < 50ms (p95)
- ✅ 1000 mudanças consecutivas sem memory leaks
- ✅ Múltiplas requisições concorrentes sem race conditions
- ✅ Cobertura de código ≥ 95%

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 1.4, [DM_MINDSET.md](DM_MINDSET.md), [INTENT_DSL.md](INTENT_DSL.md)

---

### 2. Real-time ASR Pipeline
**Módulo**: `asr-service`  
**Tipo**: Teste de Integração + Performance + Stress  
**Prioridade**: CRÍTICA

**Testes Detalhados**:

#### Testes Unitários
```rust
// tests/unit/asr_service/whisper_test.rs

#[tokio::test]
async fn test_whisper_model_loading() {
    // Testar carregamento de modelo Whisper
    // Verificar que modelo é carregado corretamente
    // Verificar que memória é gerenciada (sem leaks)
    // Verificar que modelo pode ser recarregado
}

#[tokio::test]
async fn test_whisper_model_unloading() {
    // Testar descarregamento de modelo
    // Verificar que memória é liberada
    // Verificar que modelo pode ser recarregado após unload
}

#[tokio::test]
async fn test_vad_detection_start_silence() {
    // Testar detecção de início de fala após silêncio
    // Verificar que VAD detecta corretamente (≥ 95% precisão)
    // Verificar que não há falsos positivos (< 5%)
    // Verificar que latência de detecção < 50ms
}

#[tokio::test]
async fn test_vad_detection_end_silence() {
    // Testar detecção de fim de fala (silêncio)
    // Verificar que VAD detecta corretamente (≥ 95% precisão)
    // Verificar que não há falsos positivos (< 5%)
    // Verificar que timeout é respeitado
}

#[tokio::test]
async fn test_vad_detection_edge_cases() {
    // Testar edge cases de VAD:
    // - Fala muito curta (< 100ms)
    // - Fala muito longa (> 30s)
    // - Ruído de fundo constante
    // - Ruído intermitente
    // - Múltiplas vozes simultâneas
    // - Sussurros
    // - Gritos
}

#[tokio::test]
async fn test_chunk_processing_320ms() {
    // Testar processamento de chunks de exatamente 320ms
    // Verificar que chunks são processados incrementalmente
    // Verificar que latência < 80ms por chunk
    // Verificar que chunks são ordenados corretamente
}

#[tokio::test]
async fn test_chunk_processing_variable_size() {
    // Testar processamento de chunks de tamanhos variáveis
    // Verificar que sistema lida com chunks menores que 320ms
    // Verificar que sistema lida com chunks maiores que 320ms
    // Verificar que latência permanece aceitável
}

#[tokio::test]
async fn test_incremental_transcription() {
    // Testar transcrição incremental
    // Verificar que texto é atualizado progressivamente
    // Verificar que texto final é correto
    // Verificar que histórico de chunks é mantido
}

#[tokio::test]
async fn test_transcription_accuracy_wer() {
    // Testar precisão de transcrição (WER < 10%)
    // Usar dataset de teste com áudio conhecido (100+ amostras)
    // Verificar que palavras-chave são reconhecidas (≥ 90%)
    // Verificar que pontuação é preservada quando possível
}

#[tokio::test]
async fn test_transcription_accuracy_edge_cases() {
    // Testar precisão em edge cases:
    // - Sotaques diferentes
    // - Velocidade de fala variável
    // - Qualidade de áudio baixa
    // - Ruído de fundo
    // - Múltiplas vozes
    // - Termos técnicos D&D
    // - Nomes próprios
}

#[tokio::test]
async fn test_audio_format_conversion() {
    // Testar conversão de formatos de áudio
    // Verificar que diferentes sample rates funcionam
    // Verificar que diferentes bit depths funcionam
    // Verificar que conversão não degrada qualidade
}

#[tokio::test]
async fn test_audio_buffer_overflow() {
    // Testar buffer overflow
    // Verificar que sistema lida com buffers muito grandes
    // Verificar que não há memory leaks
    // Verificar que sistema recupera de overflow
}
```

#### Testes de Integração
```rust
// tests/integration/asr_service/pipeline_test.rs

#[tokio::test]
async fn test_asr_pipeline_end_to_end() {
    // Testar pipeline completo (áudio → texto)
    // Verificar que áudio → texto funciona
    // Verificar que latência < 80ms (p95)
    // Verificar que precisão é mantida
}

#[tokio::test]
async fn test_asr_with_client_electron() {
    // Testar integração com client-electron
    // Verificar que chunks são recebidos corretamente
    // Verificar que transcrições são enviadas corretamente
    // Verificar que latência end-to-end < 100ms
}
```

#### Testes de Performance
```rust
// tests/performance/asr_service/latency_test.rs

#[tokio::test]
async fn test_asr_latency_benchmark_100_samples() {
    // Testar latência com 100 amostras de áudio
    // Medir p50, p95, p99
    // Verificar que p95 < 80ms
    // Verificar que p99 < 100ms
}

#[tokio::test]
async fn test_asr_throughput() {
    // Testar throughput (chunks processados por segundo)
    // Verificar que ≥ 3 chunks/s podem ser processados
    // Verificar que latência não degrada sob carga
}
```

#### Testes de Stress
```rust
// tests/stress/asr_service/stress_test.rs

#[tokio::test]
async fn test_asr_continuous_1_hour() {
    // Testar processamento contínuo por 1 hora
    // Verificar que não há memory leaks
    // Verificar que latência não degrada
    // Verificar que precisão não degrada
}

#[tokio::test]
async fn test_asr_concurrent_streams() {
    // Testar múltiplos streams concorrentes (5+)
    // Verificar que não há race conditions
    // Verificar que cada stream é processado corretamente
    // Verificar que latência não degrada significativamente
}

#[tokio::test]
async fn test_asr_high_frequency_chunks() {
    // Testar chunks em alta frequência (10+ chunks/s)
    // Verificar que sistema não quebra
    // Verificar que latência permanece aceitável
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Latência < 80ms para chunks de 320ms (p95)
- ✅ WER < 10% em dataset de teste (100+ amostras)
- ✅ VAD detecta início/fim corretamente (≥ 95% precisão, < 5% falsos positivos)
- ✅ Transcrição incremental funciona corretamente (100% das vezes)
- ✅ Processamento contínuo por 1 hora sem memory leaks
- ✅ Múltiplos streams concorrentes sem race conditions
- ✅ Cobertura de código ≥ 95%

---

### 3. TTS Latency
**Módulo**: `tts-service`  
**Tipo**: Teste de Performance + Integração  
**Prioridade**: CRÍTICA

**Testes Detalhados**:

#### Testes Unitários
```rust
// tests/unit/tts_service/piper_sovits_test.rs

#[tokio::test]
async fn test_piper_model_loading() {
    // Testar carregamento de modelo Piper TTS (PT/EN)
    // Verificar que modelo é carregado corretamente
    // Verificar que memória é gerenciada
    // Verificar que modelo pode ser recarregado
}

#[tokio::test]
async fn test_sovits_model_loading() {
    // Testar carregamento de modelo SoVITS por personagem
    // Verificar que modelos são carregados corretamente
    // Verificar que memória é gerenciada
    // Verificar que modelos podem ser recarregados
}

#[tokio::test]
async fn test_voice_loading_all_voices() {
    // Testar carregamento de todas as vozes disponíveis
    // Verificar que vozes são carregadas corretamente
    // Verificar que vozes são distintas
    // Verificar que metadados são corretos
}

#[tokio::test]
async fn test_voice_switching() {
    // Testar mudança rápida entre vozes
    // Verificar que mudança é instantânea (< 10ms)
    // Verificar que qualidade não degrada
}

#[tokio::test]
async fn test_audio_synthesis_short_phrase() {
    // Testar síntese de frase curta (< 10 palavras)
    // Verificar que áudio é gerado corretamente
    // Verificar que qualidade é aceitável (MOS > 3.5)
    // Verificar que latência < 150ms
}

#[tokio::test]
async fn test_audio_synthesis_long_phrase() {
    // Testar síntese de frase longa (> 50 palavras)
    // Verificar que áudio é gerado corretamente
    // Verificar que qualidade é mantida
    // Verificar que streaming funciona
}

#[tokio::test]
async fn test_audio_synthesis_edge_cases() {
    // Testar edge cases de síntese:
    // - Texto vazio
    // - Texto muito longo (> 1000 palavras)
    // - Caracteres especiais
    // - Emojis
    // - Números
    // - Abreviações
    // - Termos técnicos D&D
}

#[tokio::test]
async fn test_audio_effects_pitch() {
    // Testar aplicação de efeito de pitch
    // Verificar que pitch é aplicado corretamente
    // Verificar que qualidade não degrada significativamente
}

#[tokio::test]
async fn test_audio_effects_reverb() {
    // Testar aplicação de efeito de reverb
    // Verificar que reverb é aplicado corretamente
    // Verificar que qualidade não degrada significativamente
}

#[tokio::test]
async fn test_audio_effects_filters() {
    // Testar aplicação de filtros de áudio
    // Verificar que filtros são aplicados corretamente
    // Verificar que qualidade não degrada significativamente
}

#[tokio::test]
async fn test_audio_effects_combination() {
    // Testar combinação de múltiplos efeitos
    // Verificar que efeitos não interferem entre si
    // Verificar que qualidade é mantida
}

#[tokio::test]
async fn test_cache_hit() {
    // Testar cache hit (frase já sintetizada)
    // Verificar que cache é usado
    // Verificar que latência é reduzida (≥ 50%)
    // Verificar que qualidade é idêntica
}

#[tokio::test]
async fn test_cache_miss() {
    // Testar cache miss (frase nova)
    // Verificar que síntese é executada
    // Verificar que cache é atualizado
}

#[tokio::test]
async fn test_cache_eviction() {
    // Testar eviction de cache (LRU)
    // Verificar que cache não cresce indefinidamente
    // Verificar que eviction funciona corretamente
}

#[tokio::test]
async fn test_streaming_chunks_100ms() {
    // Testar streaming de chunks de 100ms
    // Verificar que chunks são entregues progressivamente
    // Verificar que latência do primeiro chunk < 150ms
    // Verificar que chunks são ordenados corretamente
}
```

#### Testes de Integração
```rust
// tests/integration/tts_service/pipeline_test.rs

#[tokio::test]
async fn test_tts_pipeline_end_to_end() {
    // Testar pipeline completo (texto → áudio)
    // Verificar que texto → áudio funciona
    // Verificar que latência < 150ms (p95)
    // Verificar que qualidade é mantida
}

#[tokio::test]
async fn test_tts_with_client_electron() {
    // Testar integração com client-electron
    // Verificar que áudio é recebido corretamente
    // Verificar que reprodução funciona
    // Verificar que latência end-to-end < 200ms
}
```

#### Testes de Performance
```rust
// tests/performance/tts_service/latency_test.rs

#[tokio::test]
async fn test_tts_latency_benchmark_100_phrases() {
    // Testar latência com 100 frases de teste
    // Medir p50, p95, p99
    // Verificar que p95 < 150ms para frases curtas
    // Verificar que p99 < 200ms
}

#[tokio::test]
async fn test_tts_throughput() {
    // Testar throughput (frases processadas por segundo)
    // Verificar que ≥ 5 frases/s podem ser processadas
    // Verificar que latência não degrada sob carga
}
```

#### Testes de Stress
```rust
// tests/stress/tts_service/stress_test.rs

#[tokio::test]
async fn test_tts_continuous_1_hour() {
    // Testar síntese contínua por 1 hora
    // Verificar que não há memory leaks
    // Verificar que latência não degrada
    // Verificar que qualidade não degrada
}

#[tokio::test]
async fn test_tts_concurrent_requests() {
    // Testar múltiplas requisições concorrentes (10+)
    // Verificar que não há race conditions
    // Verificar que cada requisição é processada corretamente
    // Verificar que latência não degrada significativamente
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Latência < 150ms para falas curtas (p95)
- ✅ Qualidade de síntese (MOS > 3.5) - avaliação subjetiva
- ✅ Cache reduz latência em ≥ 50% para frases comuns
- ✅ Streaming funciona corretamente (100% das vezes)
- ✅ Síntese contínua por 1 hora sem memory leaks
- ✅ Múltiplas requisições concorrentes sem race conditions
- ✅ Cobertura de código ≥ 95%

---

## Testes por Task (Detalhados)

### Task: setup-project-base

**Testes Obrigatórios (TODOS DEVEM PASSAR)**:

#### 1. Testes de Estrutura
```bash
# tests/integration/setup/project_structure_test.sh

# Verificar estrutura de diretórios
test -d src/ || exit 1
test -d tests/ || exit 1
test -d docs/ || exit 1
test -d config/ || exit 1
test -d src/client-electron/ || exit 1
test -d src/game-engine/ || exit 1
test -d src/llm-core/ || exit 1
test -d src/asr-service/ || exit 1
test -d src/tts-service/ || exit 1
test -d src/rules5e-service/ || exit 1
test -d src/memory-service/ || exit 1
test -d src/infra-runtime/ || exit 1
```

#### 2. Testes de Compilação Rust
```rust
// tests/integration/setup/rust_compilation_test.rs

#[test]
fn test_workspace_compiles() {
    // Executar: cargo check --workspace
    // Verificar que compila sem erros
    // Verificar que todos os membros do workspace compilam
}

#[test]
fn test_workspace_tests_compile() {
    // Executar: cargo test --workspace --no-run
    // Verificar que todos os testes compilam
}

#[test]
fn test_workspace_examples_compile() {
    // Executar: cargo build --workspace --examples
    // Verificar que exemplos compilam
}
```

#### 3. Testes de Formatação Rust
```rust
// tests/integration/setup/rust_formatting_test.rs

#[test]
fn test_rustfmt_check() {
    // Executar: cargo fmt --check --all
    // Verificar que formatação está correta
    // Verificar que não há diferenças
}
```

#### 4. Testes de Linting Rust
```rust
// tests/integration/setup/rust_linting_test.rs

#[test]
fn test_clippy_no_warnings() {
    // Executar: cargo clippy --workspace -- -D warnings
    // Verificar que não há warnings
    // Verificar que não há erros
}
```

#### 5. Testes de Compilação TypeScript
```typescript
// tests/integration/setup/typescript_compilation_test.ts

describe('TypeScript Compilation', () => {
  it('compiles without errors', () => {
    // Executar: npm run type-check
    // Verificar que TypeScript compila sem erros
  });

  it('compiles with strict mode', () => {
    // Verificar que tsconfig.json tem strict: true
    // Verificar que todos os erros de tipo são resolvidos
  });
});
```

#### 6. Testes de Linting TypeScript
```typescript
// tests/integration/setup/typescript_linting_test.ts

describe('TypeScript Linting', () => {
  it('passes ESLint without warnings', () => {
    // Executar: npm run lint
    // Verificar que não há warnings
    // Verificar que não há erros
  });

  it('passes Prettier formatting check', () => {
    // Executar: npm run format:check
    // Verificar que formatação está correta
  });
});
```

#### 7. Testes de Configuração
```rust
// tests/integration/setup/configuration_test.rs

#[test]
fn test_cargo_toml_valid() {
    // Verificar que Cargo.toml é válido
    // Verificar que workspace está configurado corretamente
    // Verificar que dependências estão definidas
}

#[test]
fn test_package_json_valid() {
    // Verificar que package.json é válido
    // Verificar que scripts estão definidos
    // Verificar que dependências estão definidas
}

#[test]
fn test_env_example_exists() {
    // Verificar que env.example existe
    // Verificar que todas as variáveis necessárias estão documentadas
}
```

#### 8. Testes de Scripts
```bash
# tests/integration/setup/scripts_test.sh

# Testar cada script npm
npm run dev --dry-run || exit 1
npm run build --dry-run || exit 1
npm run test --dry-run || exit 1
npm run lint --dry-run || exit 1
npm run format --dry-run || exit 1
npm run type-check --dry-run || exit 1
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Estrutura de diretórios está correta (100% dos diretórios existem)
- ✅ Workspace Rust compila sem erros (cargo check --workspace)
- ✅ Todos os testes Rust compilam (cargo test --workspace --no-run)
- ✅ Formatação Rust está correta (cargo fmt --check --all)
- ✅ Linting Rust passa sem warnings (cargo clippy --workspace -- -D warnings)
- ✅ TypeScript compila sem erros (npm run type-check)
- ✅ Linting TypeScript passa sem warnings (npm run lint)
- ✅ Formatação TypeScript está correta (npm run format:check)
- ✅ env.example existe e está completo
- ✅ Todos os scripts npm funcionam

---

### Task: setup-cicd

**Testes Obrigatórios (TODOS DEVEM PASSAR)**:

#### 1. Testes de Workflows GitHub Actions
```yaml
# .github/workflows/test_workflows.yml (workflow de teste)

name: Test CI/CD Workflows

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test-rust-workflow:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Test Rust Workflow
        run: |
          # Verificar que rust-test.yml existe
          test -f .github/workflows/rust-test.yml || exit 1
          # Verificar que rust-lint.yml existe
          test -f .github/workflows/rust-lint.yml || exit 1
          # Verificar que workflows são válidos YAML
          yamllint .github/workflows/rust-test.yml
          yamllint .github/workflows/rust-lint.yml

  test-typescript-workflow:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Test TypeScript Workflow
        run: |
          # Verificar que typescript-test.yml existe
          test -f .github/workflows/typescript-test.yml || exit 1
          # Verificar que typescript-lint.yml existe
          test -f .github/workflows/typescript-lint.yml || exit 1
          # Verificar que workflows são válidos YAML
          yamllint .github/workflows/typescript-test.yml
          yamllint .github/workflows/typescript-lint.yml

  test-build-workflow:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Test Build Workflow
        run: |
          # Verificar que build.yml existe
          test -f .github/workflows/build.yml || exit 1
          # Verificar que workflow é válido YAML
          yamllint .github/workflows/build.yml
```

#### 2. Testes de Execução de Workflows
```rust
// tests/integration/cicd/workflow_execution_test.rs

#[tokio::test]
async fn test_rust_workflow_executes() {
    // Simular execução do workflow Rust
    // Verificar que cargo test é executado
    // Verificar que cargo clippy é executado
    // Verificar que cargo fmt é executado
    // Verificar que coverage é gerado
}

#[tokio::test]
async fn test_typescript_workflow_executes() {
    // Simular execução do workflow TypeScript
    // Verificar que npm test é executado
    // Verificar que npm run lint é executado
    // Verificar que npm run type-check é executado
    // Verificar que coverage é gerado
}

#[tokio::test]
async fn test_build_workflow_executes() {
    // Simular execução do workflow de build
    // Verificar que builds são executados para todas as plataformas
    // Verificar que artefatos são gerados
}
```

#### 3. Testes de Coverage Reporting
```rust
// tests/integration/cicd/coverage_test.rs

#[tokio::test]
async fn test_rust_coverage_generated() {
    // Executar: cargo llvm-cov --all
    // Verificar que relatório de coverage é gerado
    // Verificar que coverage é ≥ 95%
}

#[tokio::test]
async fn test_typescript_coverage_generated() {
    // Executar: npm run test:coverage
    // Verificar que relatório de coverage é gerado
    // Verificar que coverage é ≥ 95%
}
```

#### 4. Testes de Codespell
```rust
// tests/integration/cicd/codespell_test.rs

#[tokio::test]
async fn test_codespell_executes() {
    // Executar: codespell
    // Verificar que codespell é executado
    // Verificar que typos são detectados
    // Verificar que workflow falha em typos
}
```

#### 5. Testes de Security Audits
```rust
// tests/integration/cicd/security_audit_test.rs

#[tokio::test]
async fn test_cargo_audit_executes() {
    // Executar: cargo audit
    // Verificar que audit é executado
    // Verificar que vulnerabilidades são detectadas
    // Verificar que workflow falha em vulnerabilidades críticas
}

#[tokio::test]
async fn test_npm_audit_executes() {
    // Executar: npm audit
    // Verificar que audit é executado
    // Verificar que vulnerabilidades são detectadas
    // Verificar que workflow falha em vulnerabilidades críticas
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Todos os workflows GitHub Actions existem e são válidos
- ✅ Workflows executam corretamente (testados localmente ou via act)
- ✅ Coverage reports são gerados (Rust e TypeScript)
- ✅ Codespell é executado e detecta typos
- ✅ Security audits são executados (cargo audit, npm audit)
- ✅ Builds multi-plataforma são configurados
- ✅ Release workflow está configurado

---

### Task: implement-rules5e-service

**Testes Obrigatórios (TODOS DEVEM PASSAR)**:

#### 1. Testes Unitários - Dice Rolling
```rust
// tests/unit/rules5e_service/dice_roll_test.rs

#[test]
fn test_dice_roll_1d20() {
    // Testar rolagem 1d20
    // Verificar que resultado está entre 1 e 20
    // Verificar que distribuição é uniforme (teste estatístico)
    // Verificar que latência < 1ms
}

#[test]
fn test_dice_roll_2d8_plus_3() {
    // Testar rolagem 2d8+3
    // Verificar que resultado está entre 5 e 19
    // Verificar que modificador é aplicado
    // Verificar que latência < 1ms
}

#[test]
fn test_dice_roll_deterministic_seed() {
    // Testar rolagem determinística com seed
    // Verificar que mesmo seed = mesmo resultado
    // Verificar que diferentes seeds = diferentes resultados
    // Verificar que seed pode ser especificado
}

#[test]
fn test_dice_roll_edge_cases() {
    // Testar edge cases:
    // - 1d1 (resultado sempre 1)
    // - 0d20 (resultado sempre 0)
    // - 100d100 (muitos dados)
    // - Modificador negativo
    // - Modificador muito grande
    // - Expressões complexas (2d6+1d4+3)
}

#[test]
fn test_dice_roll_distribution() {
    // Testar distribuição de resultados (10000 rolagens)
    // Verificar que distribuição é aproximadamente uniforme
    // Verificar que não há viés
    // Verificar que valores extremos ocorrem
}

#[test]
fn test_dice_roll_advantage() {
    // Testar rolagem com vantagem
    // Verificar que maior valor é retornado
    // Verificar que ambos os valores são gerados
}

#[test]
fn test_dice_roll_disadvantage() {
    // Testar rolagem com desvantagem
    // Verificar que menor valor é retornado
    // Verificar que ambos os valores são gerados
}
```

#### 2. Testes Unitários - Attack Calculation
```rust
// tests/unit/rules5e_service/attack_test.rs

#[test]
fn test_attack_hit_calculation() {
    // Testar cálculo de ataque
    // Verificar que hit/miss é calculado corretamente
    // Verificar que AC é respeitado
    // Verificar que modificadores são aplicados
    // Verificar que latência < 1ms
}

#[test]
fn test_attack_critical_hit() {
    // Testar crítico (natural 20)
    // Verificar que crítico é detectado
    // Verificar que dano é dobrado
    // Verificar que modificadores são aplicados corretamente
}

#[test]
fn test_attack_critical_miss() {
    // Testar falha automática (natural 1)
    // Verificar que falha automática é detectada
    // Verificar que ataque sempre falha
}

#[test]
fn test_attack_with_advantage() {
    // Testar ataque com vantagem
    // Verificar que maior rolagem é usada
    // Verificar que crítico funciona corretamente
}

#[test]
fn test_attack_with_disadvantage() {
    // Testar ataque com desvantagem
    // Verificar que menor rolagem é usada
    // Verificar que falha automática funciona corretamente
}

#[test]
fn test_attack_edge_cases() {
    // Testar edge cases:
    // - AC muito alto (30+)
    // - AC muito baixo (0 ou negativo)
    // - Modificador muito alto
    // - Modificador muito baixo
    // - Bônus mágico
    // - Múltiplos modificadores
}
```

#### 3. Testes Unitários - Damage Calculation
```rust
// tests/unit/rules5e_service/damage_test.rs

#[test]
fn test_damage_calculation() {
    // Testar cálculo de dano
    // Verificar que tipos de dano são aplicados
    // Verificar que resistências são respeitadas
    // Verificar que vulnerabilidades são respeitadas
    // Verificar que latência < 1ms
}

#[test]
fn test_damage_resistance() {
    // Testar resistência a dano
    // Verificar que dano é reduzido pela metade
    // Verificar que arredondamento é para baixo
    // Verificar que múltiplas resistências funcionam
}

#[test]
fn test_damage_vulnerability() {
    // Testar vulnerabilidade a dano
    // Verificar que dano é dobrado
    // Verificar que múltiplas vulnerabilidades funcionam
}

#[test]
fn test_damage_immunity() {
    // Testar imunidade a dano
    // Verificar que dano é reduzido a 0
    // Verificar que tipos específicos são respeitados
}

#[test]
fn test_damage_multiple_types() {
    // Testar dano de múltiplos tipos
    // Verificar que cada tipo é calculado separadamente
    // Verificar que resistências são aplicadas por tipo
}

#[test]
fn test_damage_edge_cases() {
    // Testar edge cases:
    // - Dano 0
    // - Dano muito alto
    // - Múltiplas resistências ao mesmo tipo
    // - Resistência + vulnerabilidade (cancelam)
}
```

#### 4. Testes Unitários - Ability Checks
```rust
// tests/unit/rules5e_service/ability_check_test.rs

#[test]
fn test_ability_check() {
    // Testar teste de habilidade
    // Verificar que modificadores são aplicados
    // Verificar que DC é respeitado
    // Verificar que vantagem/desvantagem funcionam
    // Verificar que latência < 1ms
}

#[test]
fn test_ability_check_with_proficiency() {
    // Testar teste de habilidade com proficiência
    // Verificar que bônus de proficiência é aplicado
    // Verificar que nível de proficiência é respeitado
}

#[test]
fn test_ability_check_with_expertise() {
    // Testar teste de habilidade com expertise
    // Verificar que bônus de proficiência é dobrado
}

#[test]
fn test_ability_check_edge_cases() {
    // Testar edge cases:
    // - DC muito alto (30+)
    // - DC muito baixo (0 ou negativo)
    // - Modificador muito alto
    // - Modificador muito baixo
    // - Vantagem + desvantagem (cancelam)
}
```

#### 5. Testes Unitários - Saving Throws
```rust
// tests/unit/rules5e_service/saving_throw_test.rs

#[test]
fn test_saving_throw() {
    // Testar salvaguarda
    // Verificar que modificadores são aplicados
    // Verificar que DC é respeitado
    // Verificar que vantagem/desvantagem funcionam
    // Verificar que latência < 1ms
}

#[test]
fn test_saving_throw_with_proficiency() {
    // Testar salvaguarda com proficiência
    // Verificar que bônus de proficiência é aplicado
}

#[test]
fn test_saving_throw_edge_cases() {
    // Testar edge cases (similar a ability checks)
}
```

#### 6. Testes Unitários - Conditions
```rust
// tests/unit/rules5e_service/condition_test.rs

#[test]
fn test_condition_poisoned() {
    // Testar condição envenenado
    // Verificar que condição é aplicada
    // Verificar que dano é aplicado corretamente
    // Verificar que condição expira
}

#[test]
fn test_condition_stunned() {
    // Testar condição atordoado
    // Verificar que ações são bloqueadas
    // Verificar que condição expira
}

#[test]
fn test_condition_multiple() {
    // Testar múltiplas condições simultâneas
    // Verificar que todas são aplicadas
    // Verificar que não há conflitos
}

#[test]
fn test_condition_expiration() {
    // Testar expiração de condições
    // Verificar que condições expiram no momento correto
    // Verificar que condições permanentes não expiram
}

#[test]
fn test_condition_edge_cases() {
    // Testar edge cases:
    // - Condição aplicada duas vezes
    // - Condição que cancela outra
    // - Condição permanente
    // - Condição com duração variável
}
```

#### 7. Testes de Integração - HTTP Server
```rust
// tests/integration/rules5e_service/http_test.rs

#[tokio::test]
async fn test_health_endpoint() {
    // Testar endpoint /health
    // Verificar que retorna 200 OK
    // Verificar que resposta é JSON válido
}

#[tokio::test]
async fn test_roll_endpoint() {
    // Testar endpoint /roll
    // Verificar que aceita requisições POST
    // Verificar que retorna resultado correto
    // Verificar que validação funciona
}

#[tokio::test]
async fn test_attack_endpoint() {
    // Testar endpoint /attack
    // Verificar que aceita requisições POST
    // Verificar que retorna resultado correto
    // Verificar que validação funciona
}

#[tokio::test]
async fn test_ability_check_endpoint() {
    // Testar endpoint /ability-check
    // Verificar que aceita requisições POST
    // Verificar que retorna resultado correto
    // Verificar que validação funciona
}

#[tokio::test]
async fn test_saving_throw_endpoint() {
    // Testar endpoint /saving-throw
    // Verificar que aceita requisições POST
    // Verificar que retorna resultado correto
    // Verificar que validação funciona
}

#[tokio::test]
async fn test_endpoint_error_handling() {
    // Testar tratamento de erros
    // Verificar que requisições inválidas retornam 400
    // Verificar que erros são reportados corretamente
}

#[tokio::test]
async fn test_endpoint_concurrent_requests() {
    // Testar requisições concorrentes (100+)
    // Verificar que não há race conditions
    // Verificar que latência não degrada
}
```

#### 8. Testes de Performance
```rust
// tests/performance/rules5e_service/latency_test.rs

#[test]
fn test_dice_roll_latency_10000_rolls() {
    // Testar latência de 10000 rolagens
    // Medir p50, p95, p99
    // Verificar que p95 < 5ms
    // Verificar que p99 < 10ms
}

#[test]
fn test_attack_calculation_latency_10000_attacks() {
    // Testar latência de 10000 cálculos de ataque
    // Medir p50, p95, p99
    // Verificar que p95 < 5ms
    // Verificar que p99 < 10ms
}

#[test]
fn test_damage_calculation_latency_10000_damages() {
    // Testar latência de 10000 cálculos de dano
    // Medir p50, p95, p99
    // Verificar que p95 < 5ms
    // Verificar que p99 < 10ms
}
```

#### 9. Testes de Stress
```rust
// tests/stress/rules5e_service/stress_test.rs

#[tokio::test]
async fn test_continuous_1_hour() {
    // Testar processamento contínuo por 1 hora
    // Verificar que não há memory leaks
    // Verificar que latência não degrada
}

#[tokio::test]
async fn test_concurrent_requests_1000() {
    // Testar 1000 requisições concorrentes
    // Verificar que não há race conditions
    // Verificar que todas as requisições são processadas
    // Verificar que latência não degrada significativamente
}
```

**Critérios de Sucesso (TODOS DEVEM PASSAR)**:
- ✅ Cálculos são 100% determinísticos (mesmo input = mesmo output)
- ✅ Resultados seguem regras D&D 5e SRD (100% de conformidade)
- ✅ Latência < 5ms para cálculos (p95)
- ✅ Cobertura de casos edge (100% dos casos testados)
- ✅ HTTP server funciona corretamente (todos os endpoints)
- ✅ Requisições concorrentes sem race conditions
- ✅ Processamento contínuo por 1 hora sem memory leaks
- ✅ Cobertura de código ≥ 95%

---

**CONTINUAÇÃO**: Este documento continua com testes detalhados para todas as outras tasks (ASR Service, TTS Service, LLM Core, Memory Service, Game Engine, Client Electron, etc.). Cada task terá a mesma profundidade de testes mostrada acima.

**IMPORTANTE**: Este documento será expandido continuamente conforme novas tasks são adicionadas. Cada nova task DEVE ter sua seção completa de testes antes de ser marcada como concluída.

---

## Regras Finais

1. **NENHUMA TASK É CONCLUÍDA SEM PASSAR EM TODOS OS TESTES**
2. **Cobertura mínima**: 95% (não negociável)
3. **Todos os testes devem ser automatizados**
4. **Testes devem ser executados em CI/CD**
5. **Testes de regressão devem ser executados antes de cada release**
6. **Documentação de testes deve ser mantida atualizada**

---

**Este documento serve como master test plan completo e profundo para garantir qualidade máxima do VRPG Client.**

---

## Testes de Componentes React Implementados

**Nota**: Esta seção detalha todos os testes necessários para os componentes React já implementados (Voice HUD, Character Sheet, Journal, Gameplay Interface).

**Cobertura Mínima**: 95% (conforme AGENTS.md)  
**Framework**: Vitest + React Testing Library  
**Estrutura**: Testes organizados por componente e tipo (unitário, integração, E2E)

### Voice HUD Component

#### Testes Unitários

**VoiceHUD.tsx**:
- Testar renderização de estados (listening, processing, speaking, hidden)
- Testar animações de visualizador de áudio
- Testar auto-hide timer
- Testar typewriter effect
- Testar interações (botão de fechar)
- Testar acessibilidade (ARIA labels, prefers-reduced-motion)

**useVoiceHUD.ts**:
- Testar estados iniciais
- Testar transições de estado
- Testar auto-hide
- Testar cleanup de timers

#### Testes de Integração
- Integração componente + hook
- Integração com ASR/TTS services

**Ver**: Código completo de testes na seção "Testes de Componentes React Implementados" acima

---

### Character Sheet Component

#### Testes Unitários

**CharacterSheet.tsx**:
- Testar renderização de todos os campos
- Testar sistema de abas (main, spells, inventory, features)
- Testar barra de atributos horizontal
- Testar campos opcionais (portrait, alignment, xp, etc.)
- Testar cálculos de modificadores

**useCharacterSheet.ts**:
- Testar mudança de abas
- Testar validação de dados
- Testar callbacks (onClose, onSave)

#### Testes de Integração
- Integração com game engine
- Sincronização de dados do personagem

**Ver**: Código completo de testes na seção "Testes de Componentes React Implementados" acima

---

### Journal Component

#### Testes Unitários

**Journal.tsx**:
- Testar renderização de lista de entradas
- Testar busca em tempo real
- Testar filtros por tipo (all, quest, lore, note)
- Testar seleção de entrada
- Testar estado vazio
- Testar scrollbars customizadas

**useJournal.ts**:
- Testar abertura/fechamento do modal
- Testar busca e filtros
- Testar seleção de entrada
- Testar cleanup ao fechar

#### Testes de Integração
- Integração com Memory Service (Vectorizer, Lexum)
- Criação e edição de entradas

**Ver**: Código completo de testes na seção "Testes de Componentes React Implementados" acima

---

### Gameplay Interface Component

#### Testes Unitários

**GameplayInterface.tsx**:
- Testar renderização de todas as áreas (top-left, sidebar, top-right, footer)
- Testar toggle UI (screenshot mode)
- Testar party frame com retratos e HP bars
- Testar action bar com slots
- Testar chat panel com mensagens e cards
- Testar push-to-talk container

#### Testes de Integração
- Integração com Orchestrator
- Integração com game engine
- Integração com chat service
- Integração com voice service
- Testar hotkeys (H para toggle UI)
- Testar atualizações de estado em tempo real

**Ver**: Código completo de testes na seção "Testes de Componentes React Implementados" acima

---

### Testes E2E para Componentes

- Fluxo completo de interação com Voice HUD
- Fluxo completo de Character Sheet
- Fluxo completo de Journal
- Fluxo completo de Gameplay Interface
- Interação entre componentes

**Ferramentas**: Playwright ou Cypress

**Ver**: Código completo de testes E2E na seção "Testes de Componentes React Implementados" acima

---

### Metas de Cobertura para Componentes

- **Unitários**: ≥ 95% por componente
- **Integração**: ≥ 90% por fluxo
- **E2E**: Cobertura de fluxos críticos

**Ferramentas**:
- **Vitest**: Framework de testes
- **React Testing Library**: Testes de componentes
- **Playwright**: Testes E2E
- **@testing-library/jest-dom**: Matchers adicionais
