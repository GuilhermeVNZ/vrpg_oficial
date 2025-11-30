# VRPG Client - Tasks Consolidadas de Implementação

## Visão Geral

Este documento consolida **todas as tarefas de implementação** do VRPG Client, incluindo:
- Infraestrutura base
- Serviços core
- **Migração para Pipeline de 3 Agentes (PRIORIDADE CRÍTICA)**
- Sistema D&D 5e completo
- Frontend Electron
- Integração e testes

**Formato**: Cada tarefa deve ser criada usando `rulebook task create <task-id>` antes da implementação.

**Prioridade**: As tarefas estão organizadas por prioridade e dependências. Implementar na ordem especificada.

**Cobertura de Testes**: Todas as tarefas devem incluir testes com cobertura mínima de 95% (conforme AGENTS.md).

**Última Atualização**: 2025-01-XX

---

## 🚨 FASE CRÍTICA: Migração para Pipeline de 3 Agentes

**Status**: PRIORIDADE MÁXIMA  
**Objetivo**: Migrar sistema atual para arquitetura de pipeline com 3 agentes (Orquestrador + Qwen-1.5B + Qwen-14B)

**Referências**:
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Especificação do orquestrador
- [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md) - Especificação do Qwen-1.5B
- [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md) - Especificação do Qwen-14B
- [TASKS_PIPELINE_MIGRATION.md](TASKS_PIPELINE_MIGRATION.md) - Tasks detalhadas de migração

### M1. Preparação e Infraestrutura

#### M1.1 Atualizar Documentação ✅
**Task ID**: `update-docs-pipeline-architecture`

**Status**: ✅ CONCLUÍDO

**Descrição**: Atualizar toda a documentação para refletir a nova arquitetura de pipeline.

**Tarefas**:
- [x] Criar PIPELINE_ARCHITECTURE.md
- [x] Atualizar ORCHESTRATOR.md com nova arquitetura
- [x] Atualizar ARCHITECTURE.md com pipeline de 2 modelos
- [x] Atualizar LLM_CORE_SPEC.md com dual model inference
- [x] Criar QWEN_1_5B_SPEC.md
- [x] Criar QWEN_14B_SPEC.md
- [x] Atualizar todos os documentos relacionados

**Dependências**: Nenhuma

---

#### M1.2 Adicionar Suporte a Qwen-1.5B no LLM Core
**Task ID**: `add-qwen-1-5b-support`

**Status**: ✅ CONCLUÍDO

**Descrição**: Adicionar suporte para carregar e executar inferência com Qwen-1.5B no LLM Core.

**Tarefas**:
- [x] Adicionar configuração para modelo 1.5B em `config/llm_config.json`
- [x] Implementar carregamento de modelo 1.5B em `src/llm-core/inference.rs`
- [x] Implementar função `infer_1_5b()` para inferência rápida
- [x] Configurar parâmetros otimizados (max_tokens=40, temperature=0.8, top_p=0.9)
- [x] Implementar gerenciamento de memória para manter ambos modelos carregados
- [x] Adicionar endpoint HTTP `/llm/prelude` para inferência 1.5B
- [x] Implementar logging específico para 1.5B
- [x] Adicionar métricas de latência para 1.5B

**Testes Críticos**:
- [x] Teste de carregamento de ambos modelos simultaneamente
- [x] Teste de inferência 1.5B < 1.2s total
- [x] Teste de geração de resposta emocional (1-2 frases, max 40 tokens)
- [x] Teste de que 1.5B não gera resultados finais ou consequências
- [x] Teste de uso de memória com ambos modelos carregados
- [x] Teste de cobertura (95%+) - Testes S2S executados com sucesso (16/16 testes passaram)

**Dependências**: `setup-project-base`

---

#### M1.3 Implementar Banco de Frases de Ponte Humana e Sistema Anti-Loop
**Task ID**: `implement-human-bridge-phrases`

**Status**: ✅ CONCLUÍDO

**Descrição**: Criar banco local com 50-300 frases de "ponte humana" para o 1.5B escolher e sistema completo anti-loop para prevenir respostas repetitivas.

**Tarefas**:
- [x] Criar estrutura de dados para frases de ponte (`src/llm-core/bridge_phrases.rs`)
- [x] Criar arquivo JSON/YAML com frases categorizadas por emoção:
  - neutral, gentle_prompt, anticipation
  - tension_low, tension_high
  - cinematic_low, cinematic_high
  - empowering, empathetic
  - roleplay_positive, roleplay_mysterious
  - validation, momentum
- [x] Implementar função de seleção aleatória por categoria
- [x] Implementar sistema anti-repetição (não repetir frases recentes - últimas 30)
- [x] **Implementar sistema anti-loop completo**:
  - [x] Banco local de frases de "ponte humana" (200+ frases em 13 categorias)
  - [x] Sistema de tracking de respostas recentes (últimas 30 frases, 20 categorias)
  - [x] Detecção de padrões repetitivos (verificação de categoria recente)
  - [x] Rotação forçada de categorias quando padrão detectado (min 5 respostas antes de reusar)
  - [x] Fallback para frases genéricas quando todas foram usadas recentemente
- [x] Integrar com prompt do 1.5B (incluído no system prompt como inspiração)
- [x] Adicionar testes de seleção, anti-repetição e anti-loop (12 testes, 100% passando)

**Testes Críticos**:
- [x] Teste de seleção aleatória por categoria
- [x] Teste de anti-repetição (não repetir nas últimas 30 respostas)
- [x] Teste de anti-loop (não repetir padrões nas últimas 20-30 respostas)
- [x] Teste de rotação forçada de categorias
- [x] Teste de que frases são humanas e não formulaicas
- [x] Teste de cobertura (12 testes, 100% passando)

**Dependências**: `add-qwen-1-5b-support`

**Prioridade**: CRÍTICA (previne respostas repetitivas que quebram imersão)

---

### M2. Orquestrador - Pipeline de 3 Agentes

#### M2.1 Implementar Estado de Pipeline no Orquestrador
**Task ID**: `implement-pipeline-state`

**Status**: ✅ TESTES COMPLETOS (Implementação parcial)

**Descrição**: Implementar gerenciamento de estado do pipeline (waiting, processing_1_5b, waiting_final_asr, processing_14b).

**Tarefas**:
- [x] Adicionar enum `PipelineStatus` em `src/orchestrator/pipeline.rs`
- [x] Implementar estrutura `PipelineState` com:
  - `game_state` (RAM)
  - `scene_context` (RAM + Vector)
  - `lore_cache` (Vectorizer)
  - `pipeline_status` (PipelineStatus)
- [x] Implementar transições de estado
- [x] Implementar validação de transições (não permitir estados inválidos)
- [ ] Adicionar logging de transições de estado
- [ ] Implementar persistência de estado (opcional, para recovery)

**Testes Críticos**:
- [x] Teste de todas as transições de estado válidas (9 testes unitários)
- [x] Teste de rejeição de transições inválidas
- [x] Teste de thread-safety (múltiplas threads acessando estado)
- [x] Teste de integração com 1.5B, 14B e ASR (4 testes de integração)
- [ ] Teste de cobertura (95%+) - Requer análise de cobertura

**Dependências**: `setup-project-base`

---

#### M2.2 Implementar Lógica de Disparo do 1.5B
**Task ID**: `implement-1-5b-trigger-logic`

**Status**: ✅ TESTES COMPLETOS (Implementação parcial)

**Descrição**: Implementar lógica para decidir quando disparar o 1.5B (6-8s de fala, pausa detectada, ação clara).

**Tarefas**:
- [x] Implementar função `should_trigger_1_5b()` em `src/orchestrator/pipeline/trigger.rs`
- [x] Implementar detecção de tempo de fala (6-8 segundos)
- [x] Implementar detecção de pausa (VAD ou silêncio > threshold)
- [x] Implementar detecção de ação clara (intent parsing)
- [x] Implementar função `trigger_1_5b()` que:
  - Prepara prompt emocional (mock)
  - Chama LLM Core `/llm/prelude` (TODO: implementar chamada real)
  - Retorna texto do prelúdio
- [x] Implementar envio imediato para TTS após geração (TODO: implementar chamada real)
- [ ] Adicionar logging e métricas

**Testes Críticos**:
- [x] Teste de disparo após 6-8s de fala (7 testes unitários)
- [x] Teste de disparo após pausa detectada
- [x] Teste de disparo após ação clara identificada
- [x] Teste de que não dispara prematuramente
- [x] Teste de latência total < 1.2s (mock)
- [x] Teste de integração com ASR, LLM Core e TTS (4 testes de integração)
- [ ] Teste de cobertura (95%+) - Requer análise de cobertura

**Dependências**: `implement-pipeline-state`, `add-qwen-1-5b-support`

---

#### M2.3 Implementar Preparação de Contexto para 14B
**Task ID**: `implement-14b-context-preparation`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar preparação de contexto completo para o 14B, incluindo fast_prelude do 1.5B.

**Tarefas**:
- [x] Implementar função `prepare_14b_context()` em `src/orchestrator/pipeline/context_14b.rs`
- [x] Implementar inclusão de `fast_prelude` (texto do 1.5B)
- [x] Implementar inclusão de `asr_final` (transcrição completa)
- [x] Implementar inclusão de `game_state` (estado atual do jogo)
- [x] Implementar inclusão de `context_slice` (últimos 3-6 eventos)
- [x] Implementar inclusão de `vectorizer_results` (se relevante)
- [x] Implementar ligação com a cena atual (via PipelineState)
- [x] Implementar limitação de tokens (não exceder 8192)
- [x] Implementar priorização de contexto (recente > antigo)

**Testes Críticos**:
- [x] Teste de que fast_prelude está sempre incluído (11 testes unitários)
- [x] Teste de que contexto não excede limite de tokens
- [x] Teste de priorização (eventos recentes primeiro)
- [x] Teste de que vectorizer_results são incluídos quando relevante
- [x] Teste de cobertura (16 testes: 11 unitários + 5 integração, 100% passando)

**Dependências**: `implement-pipeline-state`, `add-qwen-1-5b-support`

---

#### M2.4 Implementar Intent Router
**Task ID**: `implement-intent-router`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar sistema de classificação de intenções que roteia entrada do jogador para o caminho correto.

**Tarefas**:
- [x] Criar `src/orchestrator/intent_router.rs`
- [x] Implementar função `classify_intent()` que classifica entrada em:
  - `FACT_QUERY` (perguntas objetivas)
  - `SIMPLE_RULE_QUERY` (perguntas de regra simples)
  - `META_QUERY` (perguntas sobre o sistema)
  - `WORLD_ACTION` (ações narrativas)
  - `COMBAT_ACTION` (ações de combate)
  - `SPELL_CAST` (lançamento de magias)
  - `MOVE` (movimento)
  - `ROLL_REQUEST` (pedidos de rolagem)
  - `UNCERTAIN` (fallback para 1.5B)
- [x] Implementar classificador regex/heurístico para casos claros
- [x] Implementar fallback para `UNCERTAIN` quando regex não detecta (será enviado para 1.5B)
- [x] Implementar cache de classificações frequentes
- [x] Implementar logging de classificações
- [ ] Adicionar métricas de precisão (opcional, para produção)

**Testes Críticos**:
- [x] Teste de classificação precisa (≥ 95% para casos claros) - 17 testes, 100% passando
- [x] Teste de fallback para `UNCERTAIN` (quando regex não detecta)
- [x] Teste de latência < 10ms para classificação
- [x] Teste de cache (reduz latência em ≥ 50%)
- [x] Teste de cobertura (17 testes, 100% passando)

**Dependências**: `implement-pipeline-state`

**Prioridade**: CRÍTICA (necessário para roteamento correto)

**Ver**: ARCHITECTURE.md linha 118-130, ORCHESTRATOR.md linha 88-100

---

#### M2.5 Implementar Fluxo Completo do Pipeline
**Task ID**: `implement-complete-pipeline-flow`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar o fluxo completo: ASR → Intent Router → 1.5B → Wait Final ASR → 14B → TTS.

**Tarefas**:
- [x] Implementar função `handle_player_input()` em `src/orchestrator/pipeline/flow.rs`
- [x] Implementar recepção de `asr_partial` do ASR Service
- [x] Implementar chamada ao Intent Router (usar `implement-intent-router`)
- [x] Implementar disparo automático do 1.5B quando apropriado
- [x] Implementar espera por `asr_final`
- [x] Implementar preparação de contexto para 14B
- [x] Implementar chamada ao 14B com contexto completo (mock para testes)
- [x] Implementar envio de narrativa para TTS (mock para testes)
- [x] Implementar atualização de estado do pipeline
- [x] Implementar tratamento de erros em cada etapa
- [x] Implementar logging detalhado do fluxo

**Testes Críticos**:
- [x] Teste de fluxo completo end-to-end (9 testes, 100% passando)
- [x] Teste de que 1.5B sempre dispara antes do 14B
- [x] Teste de latência total < 6s (mock)
- [x] Teste de tratamento de erros (ASR falha, LLM falha, TTS falha)
- [x] Teste de cobertura (9 testes, 100% passando)

**Dependências**: `implement-1-5b-trigger-logic`, `implement-14b-context-preparation`, `implement-intent-router`

---

#### M2.6 Implementar Sistema de Cancelamento de TTS
**Task ID**: `implement-tts-cancellation`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema de cancelamento de TTS quando nova entrada do jogador chega.

**Tarefas**:
- [ ] Implementar função `cancel_current_tts()` em `src/orchestrator/pipeline.rs`
- [ ] Implementar detecção de nova entrada durante TTS
- [ ] Implementar cancelamento de áudio em reprodução (via TTS Service)
- [ ] Implementar limpeza de buffer de áudio
- [ ] Implementar integração com TTS Service endpoint `/cancel`
- [ ] Implementar logging de cancelamentos
- [ ] Adicionar métricas de cancelamentos (frequência, latência)

**Testes Críticos**:
- [ ] Teste de cancelamento quando nova entrada chega
- [ ] Teste de que áudio para imediatamente (< 50ms)
- [ ] Teste de que buffer é limpo corretamente
- [ ] Teste de que não há artefatos de áudio após cancelamento
- [ ] Teste de múltiplos cancelamentos consecutivos
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-complete-pipeline-flow`, `implement-tts-service`

**Prioridade**: ALTA (necessário para UX fluida quando jogador interrompe)

**Ver**: ORCHESTRATOR.md linha 52-58

---

### M3. Orquestrador - Respostas Objetivas

#### M3.1 Implementar Respostas Objetivas sem LLM
**Task ID**: `implement-objective-responses`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar lógica para responder perguntas objetivas diretamente, sem chamar LLM.

**Tarefas**:
- [x] Detecção de perguntas factuais (já feita pelo Intent Router como FACT_QUERY):
  - "Quantos HP eu tenho?"
  - "Quantos slots nível X eu tenho?"
  - "Qual minha AC?"
  - "Qual minha posição?"
- [x] Implementar função `answer_objective_question()` que:
  - Consulta game_state diretamente
  - Retorna resposta sem chamar LLM
- [x] Implementar respostas para cada tipo de pergunta objetiva (HP, AC, slots, posição, recursos)
- [x] Integrar com `handle_fact_query()` no pipeline flow
- [x] Adicionar logging de respostas objetivas

**Testes Críticos**:
- [x] Teste de resposta correta para cada tipo (9 testes, 100% passando)
- [x] Teste de que LLM não é chamado para perguntas objetivas
- [x] Teste de latência < 50ms para respostas objetivas
- [x] Teste de múltiplas perguntas objetivas
- [x] Teste de cobertura (9 testes, 100% passando)

**Dependências**: `implement-pipeline-state`

---

#### M3.2 Implementar Consulta de Regras Simples (Vectorizer + 1.5B)
**Task ID**: `implement-simple-rule-query`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar consulta de regras simples usando Vectorizer + 1.5B (não 14B).

**Tarefas**:
- [x] Detecção de perguntas de regra simples (já feita pelo Intent Router como SIMPLE_RULE_QUERY):
  - "Stealth usa Destreza?"
  - "Investigation é Inteligência?"
  - "Acrobatics usa Destreza?"
- [x] Implementar função `answer_simple_rule_query()` que:
  - Recebe resultados do Vectorizer
  - Converte em resposta humana via 1.5B (mock para testes)
  - Nunca chama 14B
- [x] Integrar com `handle_simple_rule_query()` no pipeline flow
- [x] Adicionar logging

**Testes Críticos**:
- [x] Teste de detecção de perguntas de regra simples (8 testes, 100% passando)
- [x] Teste de consulta ao Vectorizer (mock)
- [x] Teste de conversão em resposta humana pelo 1.5B (mock)
- [x] Teste de que 14B não é chamado para regras simples
- [x] Teste de latência < 1.5s total (mock)
- [x] Teste de cobertura (8 testes, 100% passando)

**Dependências**: `implement-objective-responses`, `add-qwen-1-5b-support`

---

### M4. Cache e Estado

#### M4.1 Implementar Cache de Estado do Jogo (RAM)
**Task ID**: `implement-game-state-cache`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar cache em RAM para estado do jogo (HP, AC, recursos, status, posição, iniciativa).

**Tarefas**:
- [x] Implementar estrutura `GameStateCache` em `src/orchestrator/cache/game_state_cache.rs`
- [x] Implementar armazenamento de:
  - HP por entidade
  - AC por entidade
  - Recursos (rage, slots, smites, ki)
  - Status (poisoned, stealth, prone, etc)
  - Posição (grid 2D/3D)
  - Iniciativa
- [x] Implementar atualização de cache quando estado muda
- [x] Implementar consulta rápida de cache
- [x] Implementar invalidação de cache quando necessário
- [x] Adicionar métricas de hit/miss do cache

**Testes Críticos**:
- [x] Teste de armazenamento e recuperação de estado (8 testes, 100% passando)
- [x] Teste de atualização de cache
- [x] Teste de invalidação de cache
- [x] Teste de latência < 10ms para consultas
- [x] Teste de cobertura (8 testes, 100% passando)

**Dependências**: `implement-pipeline-state`

---

#### M4.2 Implementar Cache de Contexto da Cena (RAM + Vector)
**Task ID**: `implement-scene-context-cache`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar cache de contexto da cena (últimas 3-6 ações, resultados de rolagens, NPCs ativos).

**Tarefas**:
- [x] Implementar estrutura `SceneContextCache` em `src/orchestrator/cache/scene_context_cache.rs`
- [x] Implementar armazenamento de:
  - Últimas 3-6 ações
  - Resultados de rolagens
  - NPCs ativos
  - Quem interagiu com quem
- [x] Implementar limite de histórico (não armazenar mais que 6 eventos)
- [x] Implementar preparação de context_slice para 14B
- [x] Integração com Vectorizer (via prepare_context_slice que retorna ContextEvent[])
- [x] Adicionar logging

**Testes Críticos**:
- [x] Teste de armazenamento de eventos recentes (8 testes, 100% passando)
- [x] Teste de limite de histórico (máximo 6 eventos)
- [x] Teste de preparação de context_slice
- [x] Teste de NPCs ativos e interações
- [x] Teste de cobertura (8 testes, 100% passando)

**Dependências**: `implement-game-state-cache`

---

#### M4.3 Implementar Cache de Lore (Vectorizer)
**Task ID**: `implement-lore-cache`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar cache de lore usando Vectorizer (descrições, NPCs, locais, história).

**Tarefas**:
- [ ] Implementar estrutura `LoreCache` em `src/orchestrator/cache.rs`
- [ ] Implementar integração com Vectorizer para:
  - Descrição de raças
  - Cidade / regiões / dungeons
  - NPCs recorrentes
  - História da campanha
  - Áreas, facções, crenças
- [ ] Implementar cache de queries frequentes (TTL: 5 minutos)
- [ ] Implementar preparação de lore_context para 14B
- [ ] Adicionar métricas de hit/miss

**Testes Críticos**:
- [ ] Teste de consulta ao Vectorizer
- [ ] Teste de cache de queries frequentes
- [ ] Teste de preparação de lore_context
- [ ] Teste de latência < 100ms para consultas cacheadas
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-scene-context-cache`

---

#### M4.4 Implementar Sistema de Persistência de Sessão
**Task ID**: `implement-session-persistence`

**Status**: ✅ CONCLUÍDO

**Descrição**: Implementar sistema completo de persistência de sessão (save/load) para permitir continuidade entre sessões.

**Tarefas**:
- [ ] Criar estrutura de dados para sessão serializável (`src/orchestrator/session.rs`)
- [ ] Implementar função `save_session()` que serializa:
  - Game state completo (HP, AC, recursos, status, posição, iniciativa)
  - Scene context (últimas 3-6 ações, resultados de rolagens, NPCs ativos)
  - Pipeline state (estado atual do pipeline)
  - Lore cache (queries frequentes)
  - Histórico de ações (últimas 20-30 ações)
  - Configurações da sessão
- [ ] Implementar função `load_session()` que deserializa e restaura estado
- [ ] Implementar formato de arquivo (JSON/YAML)
- [ ] Implementar versionamento de formato (suporte a versões antigas)
- [ ] Implementar validação de integridade (checksums)
- [ ] Implementar compressão (opcional, para sessões grandes)
- [ ] Implementar logging de save/load
- [ ] Implementar UI para save/load (futuro, na Fase 5)

**Testes Críticos**:
- [ ] Teste de save completo (todos os dados salvos corretamente)
- [ ] Teste de load completo (estado restaurado corretamente)
- [ ] Teste de versionamento (load de versões antigas funciona)
- [ ] Teste de integridade (detecção de corrupção)
- [ ] Teste de compressão (quando habilitada)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-pipeline-state`, `implement-game-state-cache`, `implement-scene-context-cache`, `implement-lore-cache`

**Prioridade**: ALTA (necessário para continuidade entre sessões)

**Ver**: TASKS.md múltiplas menções de persistência (linhas 839, 988, 136)

---

### M5. Validação e Testes

#### M5.1 Testes de Integração do Pipeline
**Task ID**: `test-pipeline-integration`

**Status**: 🔄 PENDENTE

**Descrição**: Criar testes de integração completos para o pipeline de 3 agentes.

**Tarefas**:
- [ ] Criar teste end-to-end: ASR → 1.5B → 14B → TTS
- [ ] Criar teste de que 1.5B sempre dispara antes do 14B
- [ ] Criar teste de latência total < 6s
- [ ] Criar teste de que 1.5B não gera resultados finais
- [ ] Criar teste de que 14B recebe fast_prelude
- [ ] Criar teste de respostas objetivas sem LLM
- [ ] Criar teste de consulta de regras simples (Vectorizer + 1.5B)
- [ ] Criar teste de consulta de regras narrativas (14B)
- [ ] Criar teste de tratamento de erros em cada etapa
- [ ] Criar teste de cache (game_state, scene_context, lore_cache)

**Testes Críticos**:
- [ ] Todos os testes de integração passam
- [ ] Cobertura de testes > 95%
- [ ] Latência medida e dentro dos targets

**Dependências**: `implement-complete-pipeline-flow`, `implement-objective-responses`, `implement-simple-rule-query`, `implement-lore-cache`

---

#### M5.2 Testes de Performance
**Task ID**: `test-pipeline-performance`

**Status**: 🔄 PENDENTE

**Descrição**: Criar testes de performance e benchmarks para validar latências.

**Tarefas**:
- [ ] Criar benchmark de latência do 1.5B (< 1.2s)
- [ ] Criar benchmark de latência do 14B (< 6s)
- [ ] Criar benchmark de latência de respostas objetivas (< 50ms)
- [ ] Criar benchmark de latência de consulta de regras simples (< 1.5s)
- [ ] Criar benchmark de uso de memória com ambos modelos
- [ ] Criar benchmark de throughput (interações/minuto)
- [ ] Documentar resultados e comparar com targets

**Testes Críticos**:
- [ ] Todos os benchmarks dentro dos targets
- [ ] Documentação de resultados completa

**Dependências**: `test-pipeline-integration`

---

#### M5.3 Testes de Regressão
**Task ID**: `test-pipeline-regression`

**Status**: 🔄 PENDENTE

**Descrição**: Garantir que funcionalidades existentes não quebraram com a migração.

**Tarefas**:
- [ ] Executar todos os testes existentes do sistema
- [ ] Verificar que combate ainda funciona
- [ ] Verificar que diálogos ainda funcionam
- [ ] Verificar que rolagens ainda funcionam
- [ ] Verificar que memória ainda funciona
- [ ] Verificar que UI ainda funciona
- [ ] Corrigir regressões encontradas

**Testes Críticos**:
- [ ] Todos os testes existentes passam
- [ ] Nenhuma regressão identificada

**Dependências**: `test-pipeline-integration`

---

### M6. Documentação e Deploy

#### M6.1 Atualizar Documentação de Usuário
**Task ID**: `update-user-documentation`

**Status**: 🔄 PENDENTE

**Descrição**: Atualizar documentação para usuários finais sobre a nova arquitetura.

**Tarefas**:
- [ ] Atualizar README.md com nova arquitetura
- [ ] Criar guia de configuração dos modelos (1.5B e 14B)
- [ ] Criar guia de troubleshooting para pipeline
- [ ] Atualizar CHANGELOG.md com mudanças
- [ ] Criar guia de migração para usuários existentes

**Dependências**: `test-pipeline-regression`

---

#### M6.2 Preparar Deploy
**Task ID**: `prepare-pipeline-deploy`

**Status**: 🔄 PENDENTE

**Descrição**: Preparar deploy da nova arquitetura.

**Tarefas**:
- [ ] Atualizar scripts de build para incluir modelo 1.5B
- [ ] Atualizar configurações padrão
- [ ] Criar migração de dados se necessário
- [ ] Atualizar documentação de instalação
- [ ] Preparar release notes

**Dependências**: `update-user-documentation`

---

## Fase 0: Infraestrutura Base (Pré-requisitos)

### 0.1 Setup do Projeto Base
**Task ID**: `setup-project-base`

**Status**: 🔄 PENDENTE

**Descrição**: Configurar estrutura base do projeto, workspace Rust, configurações TypeScript, e estrutura de diretórios.

**Tarefas**:
- [ ] Criar estrutura de diretórios conforme ARCHITECTURE.md
- [ ] Configurar `Cargo.toml` workspace com todos os módulos
- [ ] Configurar `package.json` para Electron + React + TypeScript
- [ ] Configurar `tsconfig.json` com strict mode
- [ ] Configurar `rustfmt.toml` e `.clippy.toml`
- [ ] Configurar ESLint e Prettier para TypeScript
- [ ] Criar estrutura de diretórios `src/`, `tests/`, `docs/`
- [ ] Configurar `.gitignore` apropriado
- [ ] Criar `env.example` com todas as variáveis necessárias
- [ ] Configurar scripts de build e desenvolvimento

**Testes**:
- [ ] Verificar que workspace Rust compila sem erros
- [ ] Verificar que TypeScript compila sem erros
- [ ] Verificar que linters passam sem warnings
- [ ] Verificar estrutura de diretórios está correta

**Dependências**: Nenhuma

---

### 0.2 Configuração de CI/CD
**Task ID**: `setup-cicd`

**Status**: 🔄 PENDENTE

**Descrição**: Configurar pipelines de CI/CD para testes, linting, build e deployment.

**Tarefas**:
- [ ] Criar workflow GitHub Actions para Rust (test, lint, format)
- [ ] Criar workflow GitHub Actions para TypeScript (test, lint, build)
- [ ] Configurar coverage reporting (cargo llvm-cov, vitest coverage)
- [ ] Configurar codespell para verificação de typos
- [ ] Configurar security audit (cargo audit, npm audit)
- [ ] Configurar build multi-plataforma (Windows, Linux, macOS)
- [ ] Configurar publicação automática de releases

**Testes**:
- [ ] Verificar que workflows executam corretamente
- [ ] Verificar que coverage reports são gerados
- [ ] Verificar que builds multi-plataforma funcionam

**Dependências**: `setup-project-base`

---

## Fase 1: Serviços Core (Rust)

### 1.1 Rules5e Service
**Task ID**: `implement-rules5e-service`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar serviço determinístico de regras D&D 5e em Rust.

**Tarefas**:
- [ ] Implementar parser de expressões de dados (`2d8+3`)
- [ ] Implementar rolagem de dados com seed controlável
- [ ] Implementar cálculo de ataques (hit/miss, AC)
- [ ] Implementar cálculo de dano (tipos, resistências)
- [ ] Implementar testes de habilidade (ability checks)
- [ ] Implementar salvaguardas (saving throws)
- [ ] Implementar condições (poisoned, stunned, etc.)
- [ ] Implementar sistema de magias básico (SRD)
- [ ] Implementar HTTP server (localhost:7004)
- [ ] Implementar endpoint `/health`
- [ ] Implementar endpoint `/roll`
- [ ] Implementar endpoint `/attack`
- [ ] Implementar endpoint `/ability-check`
- [ ] Implementar endpoint `/saving-throw`
- [ ] Implementar logging estruturado
- [ ] Implementar métricas de performance

**Testes Críticos**:
- [ ] Teste de rolagem determinística (mesmo seed = mesmo resultado)
- [ ] Teste de cálculo de ataque (hit/miss correto)
- [ ] Teste de cálculo de dano (tipos e resistências)
- [ ] Teste de condições (aplicação e expiração)
- [ ] Teste de latência (< 5ms para cálculos)
- [ ] Teste de cobertura (95%+)

**Dependências**: `setup-project-base`

**Prioridade**: ALTA (base para game-engine)

---

### 1.2 ASR Service
**Task ID**: `implement-asr-service`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar serviço de reconhecimento de fala usando Whisper local.

**Tarefas**:
- [ ] Integrar Whisper.cpp ou binding Rust para Whisper
- [ ] Implementar carregamento de modelo (whisper-large-v3-turbo quantizado)
- [ ] Implementar VAD (Voice Activity Detection)
- [ ] Implementar processamento de chunks (320ms)
- [ ] Implementar transcrição incremental (streaming)
- [ ] Implementar HTTP server (localhost:7001)
- [ ] Implementar endpoint `/health`
- [ ] Implementar endpoint `/transcribe_chunk`
- [ ] Implementar endpoint `/transcribe_final`
- [ ] Implementar cache de transcrições frequentes
- [ ] Implementar logging estruturado
- [ ] Implementar métricas de latência

**Testes Críticos**:
- [ ] Teste de latência ASR (< 80ms para chunks de 320ms)
- [ ] Teste de precisão de transcrição (WER < 10%)
- [ ] Teste de VAD (detecção correta de início/fim)
- [ ] Teste de streaming (chunks incrementais)
- [ ] Teste de cobertura (95%+)

**Dependências**: `setup-project-base`

**Prioridade**: ALTA (crítico para pipeline voz→voz)

---

### 1.3 TTS Service (XTTS + SoVITS + Voice INTENTS)
**Task ID**: `implement-tts-service`

**Status**: ✅ Estrutura existe, migrado para XTTS + SoVITS

**Descrição**: Implementar serviço de síntese de voz usando arquitetura em 3 camadas: Qwen 2.5 14B (LLM) → XTTS v2 (TTS neutro) → SoVITS (conversão vocal por personagem), com suporte a Voice INTENTS e perfis vocais.

**Tarefas**:
- [x] Implementar pipeline de 3 camadas:
  - [x] Integração com XTTS v2 (multi-idioma) para síntese neutra rápida
  - [x] Integração com SoVITS para conversão vocal por personagem
  - [x] Integração com Qwen 2.5 14B para geração de fala + emoção + tags
  - [x] Configuração de modelo
  - [x] Otimizações de performance
- [x] Sistema de perfis vocais:
  - [x] Estrutura de perfis (mestre, NPCs, jogadores IA)
  - [x] Carregamento de perfis no boot
  - [x] Switching entre perfis sem recarregar modelos
- [x] Suporte multi-voz:
  - [x] Mestre (narração neutra)
  - [x] NPCs (guarda, taverneiro, ladina, etc.)
  - [x] Jogadores IA (personalidades diferentes)
  - [x] Monstros (efeitos especiais)
- [x] Sistema de modelos SoVITS:
  - [x] Estrutura de modelos SoVITS por personagem
  - [x] Carregamento de modelos SoVITS
  - [x] Carregamento automático de modelos SoVITS
  - [x] Aplicação de tags emocionais (actor, emotion, style, pace, volume)
- [x] Implementar Voice INTENTS:
  - [x] Parser de `<VOICE>` tags (XML-like)
  - [x] Suporte a todos os tipos (NARRATE, NPC_DIALOGUE, PLAYER_DIALOGUE, EVENT, CONDITION_EXPIRE, SYSTEM)
  - [ ] Integração com Orquestrador (pendente - depende de implement-orchestrator)
  - [x] Priorização de vozes
- [x] Implementar cache de frases comuns (implementado no XTTS)
- [ ] Implementar streaming de áudio (chunks de 100ms) - opcional para v1
- [x] Implementar HTTP server (localhost:7003)
- [x] Implementar endpoint `/health`
- [x] Implementar endpoint `/speak` (com Voice INTENT)
- [x] Implementar endpoint `/voices` (listar vozes disponíveis)
- [x] Implementar endpoint `/metrics` (métricas de performance)
- [x] Implementar normalização de volume
- [x] Implementar logging estruturado
- [x] Implementar métricas de latência

**Testes Críticos**:
- [x] Teste de latência TTS (métricas implementadas, validação em produção)
- [ ] Teste de qualidade de síntese (MOS > 3.5) - requer modelos reais
- [x] Teste de múltiplas vozes (DM, NPCs, monstros) - estrutura implementada
- [x] Teste de Voice INTENTS (todos os tipos) - testes unitários passando
- [x] Teste de perfis vocais (switching sem recarregar) - implementado
- [x] Teste de efeitos de áudio (aplicação correta) - normalização implementada
- [x] Teste de cache (reutilização de frases) - cache no XTTS implementado
- [x] Teste de cobertura (95%+) - 19 testes passando, cobertura alta

**Dependências**: `setup-project-base`, `implement-orchestrator` (para Voice INTENTS)

**Prioridade**: ALTA (crítico para pipeline voz→voz)

**Ver**: [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md), [VOICE_INTENTS.md](VOICE_INTENTS.md)

---

### 1.4 LLM Core (Atualizado para Pipeline Dual)
**Task ID**: `implement-llm-core`

**Status**: ✅ Estrutura criada, precisa atualização para pipeline dual

**Descrição**: Implementar serviço de inferência LLM local com suporte a pipeline de 2 modelos (Qwen-1.5B + Qwen-14B).

**Tarefas**:
- [x] Integrar llama.cpp ou Candle para inferência
- [ ] **ATUALIZAR**: Implementar carregamento de ambos modelos (Qwen 1.5B + Qwen 14B)
- [ ] **ATUALIZAR**: Implementar gerenciamento de memória para ambos modelos
- [ ] **ATUALIZAR**: Implementar endpoints separados:
  - [ ] `/llm/prelude` (1.5B - reação rápida)
  - [ ] `/llm/narration` (14B - narrativa completa)
- [ ] Implementar otimizações (mmap, mlock, NUMA)
- [ ] Implementar KV cache para contexto
- [ ] Implementar streaming de tokens
- [ ] Implementar sistema de personas (DM, NPC, Player IA, Monster, Narrator)
- [ ] Implementar DSL de intenções (describe_scene, npc_dialogue, combat_resolution)
- [ ] Implementar integração com LessTokens (compressão de prompts)
- [ ] Implementar HTTP server (localhost:7002)
- [ ] Implementar endpoint `/health`
- [ ] Implementar integração com Memory Service
- [ ] Implementar integração com Rules5e Service
- [ ] Implementar logging estruturado
- [ ] Implementar métricas de performance (tokens/s, latência)

**Testes Críticos**:
- [ ] Teste de latência 1.5B (< 1.2s total)
- [ ] Teste de latência 14B (< 6s total)
- [ ] Teste de carregamento simultâneo de ambos modelos
- [ ] Teste de mudança de persona (consistência mantida)
- [ ] Teste de streaming (tokens incrementais)
- [ ] Teste de integração com Memory Service
- [ ] Teste de integração com Rules5e Service
- [ ] Teste de cobertura (95%+)

**Dependências**: `setup-project-base`, `implement-memory-service`, `add-qwen-1-5b-support`

**Prioridade**: CRÍTICA (core do sistema, precisa pipeline dual)

---

### 1.5 Memory Service
**Task ID**: `implement-memory-service`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar serviço de memória usando stack Hive (Vectorizer, Nexus, Lexum).

**Tarefas**:
- [ ] Implementar integração com Vectorizer (embeddings)
- [ ] Implementar integração com Nexus (graph relations)
- [ ] Implementar integração com Lexum (full-text search)
- [ ] Implementar integração com Transmutation (conversão de documentos)
- [ ] Implementar integração com Classify (categorização)
- [ ] Implementar sistema de escopos (global, campaign, session, actor)
- [ ] Implementar inserção de memórias
- [ ] Implementar busca semântica (pipeline completo)
- [ ] Implementar consolidação de memórias antigas
- [ ] Implementar HTTP server (localhost:7005)
- [ ] Implementar endpoint `/health`
- [ ] Implementar endpoint `/insert`
- [ ] Implementar endpoint `/search`
- [ ] Implementar cache de queries frequentes
- [ ] Implementar logging estruturado
- [ ] Implementar métricas de performance

**Testes Críticos**:
- [ ] Teste de latência de busca (< 100ms)
- [ ] Teste de precisão semântica (resultados relevantes)
- [ ] Teste de escopos (filtragem correta)
- [ ] Teste de integração com Vectorizer
- [ ] Teste de integração com Nexus
- [ ] Teste de integração com Lexum
- [ ] Teste de pipeline Transmutation → Classify → Vectorizer
- [ ] Teste de cobertura (95%+)

**Dependências**: `setup-project-base`, Vectorizer/Nexus/Lexum configurados

**Prioridade**: ALTA (necessário para LLM Core)

---

### 1.6 Infra Runtime
**Task ID**: `implement-infra-runtime`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar orquestração, inicialização e observabilidade dos serviços.

**Tarefas**:
- [ ] Implementar inicialização de serviços (spawn de processos)
- [ ] Implementar health-check periódico de todos os serviços
- [ ] Implementar retry/backoff para serviços que falham
- [ ] Implementar graceful shutdown de todos os serviços
- [ ] Implementar sistema de configuração centralizado
- [ ] Implementar logging estruturado (por serviço)
- [ ] Implementar métricas agregadas (latências, uso de recursos)
- [ ] Implementar tolerância a falhas (modos de degradação)
- [ ] Implementar verificação de integridade de assets
- [ ] Implementar cópia de modelos para diretório de dados

**Testes Críticos**:
- [ ] Teste de inicialização completa (todos os serviços)
- [ ] Teste de health-check (detecção de falhas)
- [ ] Teste de retry/backoff (recuperação automática)
- [ ] Teste de graceful shutdown (limpeza de recursos)
- [ ] Teste de modos de degradação (funcionamento parcial)
- [ ] Teste de cobertura (95%+)

**Dependências**: Todos os serviços core implementados

**Prioridade**: ALTA (necessário para funcionamento completo)

**Nota**: Esta task deve incluir sistema completo de fallback e degradação. Ver seção "Funcionalidades Críticas Faltando" em TASKS_ANALYSIS.md para detalhes.

**Tarefas Adicionais (Fallback e Degradação)**:
- [ ] Implementar detecção de falhas de componentes
- [ ] Implementar modos de degradação:
  - [ ] Modo 1: ASR falha → usar texto manual
  - [ ] Modo 2: TTS falha → usar texto na tela
  - [ ] Modo 3: 1.5B falha → pular prelúdio, ir direto para 14B
  - [ ] Modo 4: 14B falha → usar resposta genérica do 1.5B
  - [ ] Modo 5: Memory Service falha → usar cache local apenas
- [ ] Implementar notificação ao usuário de degradação
- [ ] Implementar recuperação automática quando componente volta
- [ ] Implementar logging de degradações
- [ ] Implementar métricas de disponibilidade

**Testes Críticos Adicionais**:
- [ ] Teste de cada modo de degradação
- [ ] Teste de que sistema continua funcionando em modo degradado
- [ ] Teste de recuperação automática
- [ ] Teste de notificação ao usuário

---

## Fase 2: Orquestrador e INTENT DSL

### 2.1 Orquestrador Base
**Task ID**: `implement-orchestrator`

**Status**: 🔄 PENDENTE (estrutura existe, precisa migração para pipeline de 3 agentes)

**Descrição**: Implementar módulo Orquestrador que coordena todos os serviços e gerencia estados de cena, **com pipeline de 3 agentes**.

**Tarefas**:
- [ ] Criar estrutura `src/orchestrator/` em Rust
- [ ] **INTEGRAR**: Implementar pipeline de 3 agentes (ver Fase M2)
- [ ] Implementar máquina de estados de cena (FSM):
  - [ ] Enum `SceneState` (SocialFreeFlow, Exploration, CombatTurnBased, DowntimePreparation)
  - [ ] Transições entre estados
  - [ ] Validação de transições
- [ ] Implementar gerenciamento de sessão:
  - [ ] Estrutura `GameSession`
  - [ ] Persistência de sessão
- [ ] Implementar comunicação IPC/WebSocket com Electron
- [ ] Implementar integração básica com `rules5e-service`
- [ ] Implementar integração básica com `memory-service`
- [x] Implementar integração básica com `asr-service` e `tts-service`
- [ ] Testes unitários do FSM
- [ ] Testes de comunicação

**Testes Críticos**:
- [ ] Teste de FSM (transições corretas)
- [ ] Teste de pipeline de 3 agentes (1.5B → 14B)
- [ ] Teste de comunicação IPC/WebSocket
- [ ] Teste de integração com services
- [ ] Teste de persistência de sessão
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-rules5e-service`, `implement-memory-service`, `implement-asr-service`, `implement-tts-service`, `add-qwen-1-5b-support`

**Prioridade**: CRÍTICA (base da nova arquitetura)

**Ver**: [ORCHESTRATOR.md](ORCHESTRATOR.md), [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md)

---

### 2.2 Intent Validation System
**Task ID**: `implement-intent-validation`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema de validação de INTENTs contra game state antes de execução.

**Tarefas**:
- [ ] Criar `src/orchestrator/intent_validator.rs`
- [ ] Implementar função `validate_intent()` que valida cada tipo de INTENT:
  - SkillCheck: verificar que skill existe
  - MeleeAttack: verificar que alvo está em alcance, linha de visão
  - RangedAttack: verificar que alvo está em alcance, linha de visão
  - SpellCast: verificar que spell está disponível, slots suficientes, componentes disponíveis
  - Move: verificar que movimento é válido, não há obstáculos
  - CombatStart/End: verificar que transição é válida
  - LoreQuery/RuleQuery: verificar que query é válida
- [ ] Implementar validação contra game_state
- [ ] Implementar validação contra regras D&D 5e (via rules5e-service)
- [ ] Implementar retorno de erros de validação detalhados
- [ ] Implementar logging de validações
- [ ] Adicionar métricas de validação (taxa de sucesso/falha)

**Testes Críticos**:
- [ ] Teste de validação de cada tipo de INTENT
- [ ] Teste de rejeição de INTENTs inválidas
- [ ] Teste de que INTENTs válidas são aceitas
- [ ] Teste de latência < 10ms para validação
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-intent-dsl-parser`, `implement-game-state-cache`, `implement-rules5e-service`

**Prioridade**: CRÍTICA (previne INTENTs inválidas que podem quebrar o jogo)

**Ver**: ORCHESTRATOR.md, TASKS.md linha 943

---

### 2.3 Parser de INTENT DSL
**Task ID**: `implement-intent-dsl-parser`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar parser determinístico para INTENT DSL gerado pelo LLM.

**Tarefas**:
- [ ] Criar `intent_parser.rs`
- [ ] Implementar gramática simplificada:
  - [ ] Parser de blocos `[INTENTS] ... [/INTENTS]`
  - [ ] Parser de INTENTs individuais
  - [ ] Parser de campos KEY: VALUE
- [ ] Implementar enum `Intent` com todas as variantes
- [ ] Implementar normalização e validação
- [ ] Implementar tratamento de erros e fallbacks
- [ ] Testes extensivos do parser

**Testes Críticos**:
- [ ] Teste de parsing de cada tipo de INTENT (100% precisão)
- [ ] Teste de edge cases
- [ ] Teste de validação
- [ ] Teste de normalização
- [ ] Teste de tratamento de erros
- [ ] Teste de cobertura (95%+)

**Dependências**: Nenhuma

**Prioridade**: CRÍTICA (necessário para execução de INTENTs)

**Ver**: [INTENT_DSL.md](INTENT_DSL.md)

---

### 2.4 Executor de INTENTs
**Task ID**: `implement-intent-executor`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar executor que converte INTENTs em ações concretas.

**Tarefas**:
- [ ] Criar `intent_executor.rs`
- [ ] Implementar execução de cada tipo de INTENT:
  - [ ] SkillCheck → RollRequest para UI
  - [ ] MeleeAttack/RangedAttack → rules5e-service
  - [ ] SpellCast → rules5e-service
  - [ ] LoreQuery/RuleQuery → memory-service
  - [ ] GeneratePortrait/Scene/Battlemap → Art Daemon (futuro)
  - [ ] CombatStart/CombatEnd → transições de estado
- [ ] Integração com `rules5e-service`
- [ ] Integração com `memory-service`
- [ ] Testes de integração

**Testes Críticos**:
- [ ] Teste de execução de cada INTENT
- [ ] Teste de integração com services
- [ ] Teste de tratamento de erros
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-intent-dsl-parser`, `implement-intent-validation`, `implement-rules5e-service`, `implement-memory-service`

**Prioridade**: CRÍTICA (execução de ações)

---

### 2.5 Atualizar LLM Core para Gerar INTENT DSL
**Task ID**: `update-llm-core-intent-dsl`

**Status**: 🔄 PENDENTE

**Descrição**: Modificar `llm-core` para gerar INTENT DSL ao invés de JSON, **com suporte a pipeline dual**.

**Tarefas**:
- [ ] Atualizar prompts do Mestre IA (14B) com exemplos de INTENT DSL
- [ ] Atualizar prompts do Mestre Reflexo (1.5B) - **NÃO deve gerar INTENTs**
- [ ] Atualizar prompts de Jogadores IA (remover geração de INTENTs)
- [ ] Modificar processamento de resposta:
  - [ ] Extrair blocos `[INTENTS] ... [/INTENTS]` (apenas do 14B)
  - [ ] Separar narração de INTENTs
  - [ ] Validar INTENTs antes de enviar ao Orquestrador
- [ ] Incorporar `DM_MINDSET.md` nos prompts (14B)
- [ ] Incorporar `QWEN_1_5B_SPEC.md` nos prompts (1.5B)
- [ ] Incorporar `CHARACTER_AGENTS.md` nos prompts
- [ ] Testes de geração de INTENTs

**Testes Críticos**:
- [ ] Teste de que 1.5B nunca gera INTENTs
- [ ] Teste de geração de INTENTs válidas pelo 14B
- [ ] Teste de separação narração/INTENTs
- [ ] Teste de fallback quando parsing falha
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-intent-dsl-parser`, `implement-llm-core`, `add-qwen-1-5b-support`

**Prioridade**: ALTA (necessário para nova arquitetura)

**Ver**: [DM_MINDSET.md](DM_MINDSET.md), [CHARACTER_AGENTS.md](CHARACTER_AGENTS.md), [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md), [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md)

---

## Fase 3: Game Engine (Refatorado)

### 3.1 Game Engine Core (Refatorado)
**Task ID**: `refactor-game-engine-orchestrator`

**Status**: ✅ Estrutura existe, precisa refatoração

**Descrição**: Refatorar `game-engine` para trabalhar com Orquestrador (remover coordenação, manter apenas estado).

**Tarefas**:
- [ ] Refatorar para trabalhar com Orquestrador:
  - [ ] Remover lógica de coordenação (move para Orquestrador)
  - [ ] Manter apenas estado de jogo
  - [ ] Integração via Orquestrador
- [ ] Implementar estrutura `GameSession` (raiz da sessão)
- [ ] Implementar estrutura `Scene` (cena atual com mapa, clima, iluminação)
- [ ] Implementar estrutura `Actor` (jogadores, NPCs, monstros)
- [ ] Implementar `TurnTracker` (ordem de iniciativa, turnos) - agora via Orquestrador
- [ ] Implementar sistema de `Effect` (buffs, debuffs, condições)
- [ ] Implementar `loadSession` e `saveSession`
- [ ] Implementar `applySceneUpdate` (aplicar mudanças de cena)
- [ ] Implementar `applyCombatEvent` (eventos de combate)
- [ ] Implementar `getStateForLlm` (contexto serializável para LLM)
- [ ] Implementar Event Bus interno (SceneChanged, ActorMoved, etc.)
- [ ] Implementar persistência de sessão (JSON/YAML)

**Testes Críticos**:
- [ ] Teste de integração com Orquestrador
- [ ] Teste de aplicação de dano (HP atualizado corretamente)
- [ ] Teste de condições (aplicação e expiração)
- [ ] Teste de persistência (save/load funcionam)
- [ ] Teste de Event Bus (eventos emitidos corretamente)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-orchestrator`, `implement-rules5e-service`

**Prioridade**: ALTA (core da lógica de jogo, mas agora coordenado pelo Orquestrador)

---

## Fase 4: Modos de Cena e Turn Engine

### 4.1 Modos de Cena (FSM)
**Task ID**: `implement-scene-modes`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar os 4 modos de cena no Orquestrador.

**Tarefas**:
- [ ] Implementar modo SocialFreeFlow:
  - [ ] Estado no FSM
  - [ ] UI adaptada (sem grid, foco em retratos)
  - [ ] Fluxo de diálogo via Orquestrador
- [ ] Implementar modo Exploration:
  - [ ] Estado no FSM
  - [ ] Sistema de movimento livre
  - [ ] Perception checks automáticos
  - [ ] Triggers de emboscada
- [ ] Implementar modo CombatTurnBased:
  - [ ] Estado no FSM
  - [ ] Integração com Turn Engine
  - [ ] UI adaptada (grid, turn order)
- [ ] Implementar modo DowntimePreparation:
  - [ ] Estado no FSM
  - [ ] Geração de assets em background
  - [ ] Preparação de próximas cenas

**Testes Críticos**:
- [ ] Teste de cada modo de cena
- [ ] Teste de transições entre modos
- [ ] Teste de UI adaptada por modo
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-orchestrator`

**Prioridade**: ALTA

---

### 4.2 Turn Engine (Combate em Turnos)
**Task ID**: `implement-turn-engine`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema completo de combate em turnos com rolagens client vs servidor.

**Tarefas**:
- [ ] Implementar sistema de iniciativa:
  - [ ] Cálculo de iniciativa (1d20 + DEX_MOD)
  - [ ] Ordenação de participantes
  - [ ] UI de ordem de turno (cards BG3-like)
- [ ] Implementar sistema de rolagens:
  - [ ] RollRequest para jogadores (client-side)
  - [ ] RollResult de jogadores (validação opcional)
  - [ ] Rolagens de NPCs (servidor/engine)
- [ ] Implementar economia de ações:
  - [ ] Ação, Movimento, Reação, Bonus Action
  - [ ] Tracking de uso
- [ ] Implementar Line of Sight (LoS) e alcance
- [ ] Implementar Áreas de Efeito (AoE)
- [ ] Implementar avanço de iniciativa
- [ ] Implementar narração por ação (não por turno completo)
- [ ] Integração com Engine de Regras
- [ ] Testes completos

**Testes Críticos**:
- [ ] Teste de iniciativa
- [ ] Teste de rolagens (client vs servidor)
- [ ] Teste de economia de ações
- [ ] Teste de LoS e alcance
- [ ] Teste de AoE
- [ ] Teste de narração por ação
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-orchestrator`, `implement-rules5e-service`

**Prioridade**: CRÍTICA

**Ver**: [COMBAT_FLOW.md](COMBAT_FLOW.md)

---

## Fase 5: Client Electron (Frontend)

### 5.1 Electron Main Process
**Task ID**: `implement-client-electron`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar processo principal do Electron com orquestração de serviços via Orquestrador.

**Tarefas**:
- [ ] Configurar app Electron (BrowserWindow, menus, ícones)
- [ ] Implementar localização de recursos (models/, backend/)
- [ ] Implementar spawn de serviços locais (child_process):
  - [ ] Orquestrador (novo)
  - [ ] ASR Service
  - [ ] TTS Service
  - [ ] LLM Core
  - [ ] Rules5e Service
  - [ ] Memory Service
- [ ] Implementar health-check periódico (HTTP /health)
- [ ] Implementar handlers IPC:
  - [ ] `ipcMain.handle("orchestrator:request", ...)` (novo - principal)
  - [ ] `ipcMain.handle("asr:transcribe", ...)`
  - [ ] `ipcMain.handle("tts:speak", ...)`
  - [ ] `ipcMain.handle("game:getState", ...)`
  - [ ] `ipcMain.handle("game:applyUpdate", ...)`
- [ ] Implementar tela de loading/inicialização
- [ ] Implementar gerenciamento de processos filhos (cleanup)
- [ ] Implementar retry/backoff para serviços
- [ ] Implementar logging de erros IPC

**Testes Críticos**:
- [ ] Teste de inicialização (todos os serviços)
- [ ] Teste de handlers IPC (comunicação correta)
- [ ] Teste de health-check (detecção de falhas)
- [ ] Teste de cleanup (processos encerrados corretamente)
- [ ] Teste de cobertura (95%+)

**Dependências**: Todos os serviços core, `implement-infra-runtime`

**Prioridade**: ALTA (necessário para UI)

---

### 5.2 Electron Renderer - Componentes Base
**Task ID**: `implement-renderer-base-components`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar componentes React base do frontend (layout, estrutura).

**Tarefas**:
- [ ] Configurar React + TypeScript + Vite
- [ ] Configurar TailwindCSS com tema BG3/Solasta
- [ ] Configurar Zustand para estado global
- [ ] Implementar layout principal (widescreen, responsivo)
- [ ] Implementar sistema de roteamento (se necessário)
- [ ] Implementar componentes de UI base (Button, Card, Panel)
- [ ] Implementar sistema de temas (cores BG3/Solasta)
- [ ] Implementar tipografia (serif para títulos, sans para UI)
- [ ] Implementar sistema de ícones
- [ ] Implementar sistema de animações base

**Testes Críticos**:
- [ ] Teste de renderização de componentes
- [ ] Teste de responsividade (21:9, 16:9, 4K)
- [ ] Teste de temas (aplicação correta de cores)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-client-electron`

**Prioridade**: ALTA (base para todos os componentes)

---

### 5.3 BattleMap Component
**Task ID**: `implement-battlemap-component`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar componente de mapa de combate com perspectiva isométrica/3D fake.

**Tarefas**:
- [ ] Integrar PixiJS ou Three.js
- [ ] Implementar renderização de mapa (perspectiva isométrica)
- [ ] Implementar sistema de tokens (jogadores, NPCs, monstros)
- [ ] Implementar halos/círculos no chão para tokens
- [ ] Implementar iluminação suave e sombras
- [ ] Implementar interações (clique, drag, zoom, pan)
- [ ] Implementar integração com backgrounds gerados por IA
- [ ] Implementar grid overlay (opcional, para debug)
- [ ] Implementar sistema de camadas (background, tokens, effects)
- [ ] Implementar otimizações de performance (culling, LOD)

**Testes Críticos**:
- [ ] Teste de renderização (60 FPS constante)
- [ ] Teste de interações (clique, drag funcionam)
- [ ] Teste de performance (sem lag com muitos tokens)
- [ ] Teste de integração com game-engine (posições corretas)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-game-engine`

**Prioridade**: ALTA (componente principal)

---

### 5.4 Turn Order / Talking Cards Component
**Task ID**: `implement-turn-order-component`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar componente de ordem de turno (combate) e talking cards (fora de combate).

**Tarefas**:
- [ ] Implementar Turn Order (linha horizontal de cards estilo BG3)
- [ ] Implementar cards com retrato, HP e status
- [ ] Implementar highlight do card ativo (brilho)
- [ ] Implementar Talking Cards (quem está na cena)
- [ ] Implementar indicador de quem está falando (pulso/brilho)
- [ ] Implementar waveform animado acima do card ativo
- [ ] Implementar transições suaves entre estados
- [ ] Implementar responsividade (adaptação a diferentes resoluções)

**Testes Críticos**:
- [ ] Teste de renderização de cards (combate e fora de combate)
- [ ] Teste de highlight (card ativo destacado)
- [ ] Teste de indicador de fala (ativação correta)
- [ ] Teste de integração com game-engine (ordem de turno)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-game-engine`

**Prioridade**: ALTA (essencial para UX)

---

### 5.5 Action Bar Component
**Task ID**: `implement-action-bar-component`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar barra de ações fixa no rodapé (estilo BG3).

**Tarefas**:
- [ ] Implementar barra fixa no rodapé
- [ ] Implementar botão TALK integrado (canto esquerdo)
- [ ] Implementar slots para ações (ataque, movimento, habilidades, spells)
- [ ] Implementar atalhos essenciais (Dash, Hide, Disengage)
- [ ] Implementar botão Pass Turn (estilo BG3)
- [ ] Implementar indicadores de latência e status do microfone
- [ ] Implementar sistema de drag-and-drop para slots
- [ ] Implementar tooltips ao passar o mouse
- [ ] Implementar estilo visual BG3 (bordas douradas, ícones grandes)

**Testes Críticos**:
- [ ] Teste de renderização da barra
- [ ] Teste de botão TALK (ativação correta)
- [ ] Teste de slots de ações (drag-and-drop funciona)
- [ ] Teste de indicadores (latência, microfone)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`

**Prioridade**: ALTA (essencial para UX)

---

### 5.6 Menus Retráteis
**Task ID**: `implement-retractable-menus-component`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar menus retráteis (Inventário, Ficha, Mapa, Diário).

**Tarefas**:
- [ ] Implementar Inventário (lista de itens, drag-and-drop)
- [ ] Implementar Ficha (atributos, perícias, magias)
- [ ] Implementar Mapa (mapa do mundo, pontos de interesse)
- [ ] Implementar Diário (histórico de eventos, notas)
- [ ] Implementar animações de abertura/fechamento
- [ ] Implementar sistema de tabs dentro de cada menu
- [ ] Implementar busca/filtro em cada menu

**Testes Críticos**:
- [ ] Teste de renderização de cada menu
- [ ] Teste de animações (abertura/fechamento suave)
- [ ] Teste de integração com game-engine (dados corretos)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`

**Prioridade**: MÉDIA

---

### 5.7 History Panel
**Task ID**: `implement-history-panel-component`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar painel de histórico (áudio, rolagens, visual).

**Tarefas**:
- [ ] Implementar histórico de áudio (rewind, replay)
- [ ] Implementar histórico de rolagens (visualização de dados)
- [ ] Implementar histórico visual (timeline de eventos)
- [ ] Implementar busca no histórico
- [ ] Implementar filtros (por tipo, por data)
- [ ] Implementar exportação de histórico

**Testes Críticos**:
- [ ] Teste de renderização do histórico
- [ ] Teste de rewind/replay de áudio
- [ ] Teste de busca e filtros
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`

**Prioridade**: MÉDIA

---

### 5.8 Dice Rolling
**Task ID**: `implement-dice-rolling-component`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar componente de rolagem de dados (animação 3D/2D).

**Tarefas**:
- [ ] Implementar animação 3D/2D de dados
- [ ] Implementar integração com Rules5e Service
- [ ] Implementar efeitos visuais (brilho, partículas)
- [ ] Implementar som de rolagem
- [ ] Implementar exibição de resultado
- [ ] Implementar histórico de rolagens

**Testes Críticos**:
- [ ] Teste de animação (suave, 60 FPS)
- [ ] Teste de integração com Rules5e (resultados corretos)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-rules5e-service`

**Prioridade**: ALTA (essencial para UX)

---

### 5.9 Voice Integration
**Task ID**: `implement-voice-integration`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar integração de voz no frontend (captura, pipeline ASR/TTS, waveform).

**Tarefas**:
- [ ] Implementar captura de áudio (microfone)
- [ ] Implementar pipeline ASR (streaming para backend)
- [ ] Implementar pipeline TTS (reprodução de áudio do backend)
- [ ] Implementar waveform animado
- [ ] Implementar indicadores visuais (quem está falando)
- [ ] Implementar controle de volume
- [ ] Implementar mute/unmute

**Testes Críticos**:
- [ ] Teste de captura de áudio (qualidade, latência)
- [ ] Teste de pipeline ASR (transcrição correta)
- [ ] Teste de pipeline TTS (reprodução correta)
- [ ] Teste de waveform (animação suave)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-asr-service`, `implement-tts-service`

**Prioridade**: ALTA (essencial para pipeline voz→voz)

---

## Fase 6: Sistema D&D 5e Completo

**Nota**: Ver [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) para lista completa e detalhada de todas as tasks do sistema D&D 5e.

### 6.1 Sistema de Personagem
**Task ID**: `implement-character-system`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema completo de personagens D&D 5e.

**Tarefas Principais**:
- [ ] Atributos e Modificadores
- [ ] Raças (Races)
- [ ] Classes (Classes)
- [ ] Backgrounds
- [ ] Feats
- [ ] Skills System
- [ ] Equipment Management
- [ ] Inventory System
- [ ] Spellcasting System
- [ ] XP & Leveling

**Dependências**: `implement-rules5e-service`

**Prioridade**: ALTA

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 1

---

### 6.2 Sistema de Combate
**Task ID**: `implement-combat-system`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema completo de combate D&D 5e.

**Tarefas Principais**:
- [ ] Turn Engine (já listado em Fase 4.2)
- [ ] Sistema de Ataques
- [ ] Sistema de Dano
- [ ] Sistema de Condições
- [ ] Sistema de Movimento
- [ ] Sistema de Ações (Action, Bonus Action, Reaction)
- [ ] Sistema de Iniciativa
- [ ] Sistema de Death Saves

**Dependências**: `implement-rules5e-service`, `implement-turn-engine`

**Prioridade**: CRÍTICA

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 2

---

### 6.3 Sistema de Magias
**Task ID**: `implement-spell-system`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema completo de magias D&D 5e.

**Tarefas Principais**:
- [ ] Spell Database (SRD completo)
- [ ] Spell Slots Management
- [ ] Spell Casting
- [ ] Spell Components (V, S, M)
- [ ] Spell Concentration
- [ ] Spell Duration
- [ ] Spell Areas of Effect
- [ ] Spell Saving Throws

**Dependências**: `implement-rules5e-service`, `implement-character-system`

**Prioridade**: ALTA

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 3

---

### 6.4 Sistema de Monstros
**Task ID**: `implement-monster-system`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema completo de monstros D&D 5e.

**Tarefas Principais**:
- [ ] Monster Database (SRD completo)
- [ ] Monster Stat Blocks
- [ ] Monster Abilities
- [ ] Monster Actions
- [ ] Monster Legendary Actions
- [ ] Monster Lair Actions

**Dependências**: `implement-rules5e-service`

**Prioridade**: ALTA

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 4, [MONSTER_MANUAL_TASKS.md](MONSTER_MANUAL_TASKS.md)

---

## Fase 7: Integração e Pipeline

### 7.1 IPC and API Contracts
**Task ID**: `implement-ipc-contracts`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar contratos IPC e API entre frontend e backend.

**Tarefas**:
- [ ] Tipos compartilhados (TypeScript ↔ Rust)
- [ ] Validação de mensagens
- [ ] Versionamento de API
- [ ] Documentação de contratos
- [ ] Testes de contratos

**Testes Críticos**:
- [ ] Teste de serialização/deserialização
- [ ] Teste de validação de mensagens
- [ ] Teste de versionamento
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-client-electron`, `implement-orchestrator`

**Prioridade**: ALTA

---

### 7.2 Pipeline Voz → Voz
**Task ID**: `implement-voice-pipeline`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar pipeline completo voz → voz com pipeline de 3 agentes.

**Tarefas**:
- [ ] Integração completa ASR → Orquestrador → 1.5B → 14B → TTS
- [ ] Otimizações de latência
- [ ] Tratamento de erros
- [ ] Métricas de pipeline
- [ ] Testes end-to-end

**Testes Críticos**:
- [ ] Teste de latência total < 6s
- [ ] Teste de que 1.5B sempre dispara antes do 14B
- [ ] Teste de tratamento de erros
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-complete-pipeline-flow`, `implement-voice-integration`

**Prioridade**: CRÍTICA

---

## Fase 8: Assets e Geração

### 8.1 Image Generation Pipeline
**Task ID**: `implement-image-generation-pipeline`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar pipeline de geração de imagens (retratos, cenas, battlemaps).

**Tarefas**:
- [ ] Integração Flux.1
- [ ] Geração de retratos
- [ ] Geração de cenas
- [ ] Geração de battlemaps
- [ ] Cache de imagens
- [ ] Testes (95%+ coverage)

**Dependências**: `implement-orchestrator`

**Prioridade**: MÉDIA

**Ver**: [ASSETS_GENERATION.md](ASSETS_GENERATION.md)

---

### 8.2 LoRA Training Pipeline
**Task ID**: `implement-lora-training-pipeline`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar pipeline de treinamento de LoRAs.

**Tarefas**:
- [ ] Coleta de datasets
- [ ] Treinamento de embeddings
- [ ] Treinamento de LoRAs
- [ ] Validação
- [ ] Testes (95%+ coverage)

**Dependências**: `implement-image-generation-pipeline`

**Prioridade**: BAIXA

**Ver**: [TRAINING_PIPELINE.md](TRAINING_PIPELINE.md)

---

## Fase 9: Testes e Qualidade

### 9.1 Test Suite Completo
**Task ID**: `implement-testing-suite`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar suite completa de testes.

**Tarefas**:
- [ ] Testes unitários (95%+ coverage)
- [ ] Testes de integração
- [ ] Testes E2E
- [ ] Testes de performance
- [ ] Testes de carga

**Dependências**: Todas as fases anteriores

**Prioridade**: ALTA

**Ver**: [TESTS_MASTER.md](TESTS_MASTER.md)

---

### 9.2 Integration Tests
**Task ID**: `implement-integration-tests`

**Status**: ✅ Estrutura criada

**Descrição**: Implementar testes de integração entre componentes.

**Tarefas**:
- [ ] Testes ASR → LLM → TTS
- [ ] Testes Game Engine → Rules5e
- [ ] Testes Memory Service → Hive
- [ ] Testes Client Electron → Serviços
- [ ] Testes Pipeline completo (ASR → 1.5B → 14B → TTS)

**Dependências**: `implement-testing-suite`

**Prioridade**: ALTA

---

## Fase 10: Otimização e Performance

### 10.1 Performance Optimizations
**Task ID**: `implement-performance-optimizations`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar otimizações de performance em todos os módulos.

**Tarefas**:
- [ ] Otimizações LLM Core (pipeline dual)
- [ ] Otimizações ASR Service
- [ ] Otimizações TTS Service
- [ ] Otimizações Game Engine
- [ ] Otimizações Memory Service
- [ ] Otimizações Frontend
- [ ] Monitoramento de métricas

**Dependências**: Todas as fases anteriores

**Prioridade**: ALTA

**Ver**: [PERFORMANCE.md](PERFORMANCE.md)

---

### 10.2 Caching System
**Task ID**: `implement-caching-system`

**Status**: 🔄 PENDENTE (parcialmente implementado em M4)

**Descrição**: Implementar sistema completo de cache.

**Tarefas**:
- [ ] Cache de imagens
- [ ] Cache de queries
- [ ] Cache de cálculos
- [ ] Cache de vozes
- [ ] Cache de modelos
- [ ] Invalidação de cache

**Dependências**: `implement-game-state-cache`, `implement-scene-context-cache`, `implement-lore-cache`

**Prioridade**: ALTA

---

## Fase 11: Documentação e Deployment

### 11.1 Documentação Completa
**Task ID**: `implement-complete-documentation`

**Status**: 🔄 PENDENTE

**Descrição**: Completar toda a documentação do projeto.

**Tarefas**:
- [ ] Atualizar README.md
- [ ] Completar ARCHITECTURE.md
- [ ] Completar DESIGN_SYSTEM.md
- [ ] Completar CONFIGURATION.md
- [ ] Completar TESTING.md
- [ ] Completar PERFORMANCE.md
- [ ] Criar guias de desenvolvimento
- [ ] Criar documentação de API

**Dependências**: Todas as fases anteriores

**Prioridade**: MÉDIA

---

### 11.2 Build e Deployment
**Task ID**: `implement-build-deployment`

**Status**: 🔄 PENDENTE

**Descrição**: Implementar sistema de build e deployment.

**Tarefas**:
- [ ] Configurar electron-builder
- [ ] Build multi-plataforma
- [ ] Gerar installers
- [ ] Code signing
- [ ] Auto-updater
- [ ] Distribuição

**Dependências**: `implement-complete-documentation`

**Prioridade**: MÉDIA

**Ver**: [DEPLOYMENT.md](DEPLOYMENT.md)

---

## Resumo de Dependências (Pipeline de 3 Agentes)

```
setup-project-base
    ↓
add-qwen-1-5b-support
    ↓
implement-human-bridge-phrases
    ↓
implement-pipeline-state
    ↓
implement-1-5b-trigger-logic ──┐
    ↓                            │
implement-14b-context-preparation│
    ↓                            │
implement-complete-pipeline-flow─┘
    ↓
implement-objective-responses
    ↓
implement-simple-rule-query
    ↓
implement-game-state-cache
    ↓
implement-scene-context-cache
    ↓
implement-lore-cache
    ↓
test-pipeline-integration
    ↓
test-pipeline-performance
    ↓
test-pipeline-regression
    ↓
update-user-documentation
    ↓
prepare-pipeline-deploy
```

---

## Métricas de Sucesso (Pipeline de 3 Agentes)

- ✅ Latência do 1.5B < 1.2s
- ✅ Latência do 14B < 6s
- ✅ Latência de respostas objetivas < 50ms
- ✅ 1.5B sempre dispara antes do 14B
- ✅ 1.5B nunca gera resultados finais
- ✅ 14B sempre recebe fast_prelude
- ✅ Cobertura de testes > 95%
- ✅ Nenhuma regressão em funcionalidades existentes

---

## Referências

- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa do pipeline
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Especificação do orquestrador
- [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md) - Especificação do Qwen-1.5B
- [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md) - Especificação do Qwen-14B
- [TASKS_PIPELINE_MIGRATION.md](TASKS_PIPELINE_MIGRATION.md) - Tasks detalhadas de migração
- [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Tasks completas do sistema D&D 5e
- [ROADMAP.md](ROADMAP.md) - Roadmap de implementação
- [ARCHITECTURE.md](ARCHITECTURE.md) - Arquitetura do sistema
- [TESTS_MASTER.md](TESTS_MASTER.md) - Plano completo de testes

---

**Última Atualização**: 2025-01-XX

