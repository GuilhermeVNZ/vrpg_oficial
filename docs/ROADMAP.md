# VRPG Client - Roadmap de Implementação

## Visão Geral

Este documento apresenta o roadmap de implementação do VRPG Client, baseado no [TASKS.md](TASKS.md) e organizado por fases de desenvolvimento.

**Status Atual**: Migração para Pipeline de 3 Agentes (PRIORIDADE CRÍTICA)  
**Última Atualização**: 2025-01-XX

**Arquitetura Atual**: Sistema com LLM único (Qwen 14B)  
**Arquitetura Alvo**: Pipeline de 3 Agentes (Orquestrador + Qwen-1.5B + Qwen-14B)

---

## 🚨 FASE CRÍTICA: Migração para Pipeline de 3 Agentes

**Status**: 🔄 EM PROGRESSO  
**Prioridade**: MÁXIMA  
**Objetivo**: Migrar sistema atual para arquitetura otimizada de pipeline com 3 agentes

**Por que esta migração é crítica**:
- **Reduz latência percebida**: Resposta imediata do 1.5B (< 1.2s) enquanto 14B prepara narrativa completa
- **Melhora qualidade narrativa**: 14B recebe contexto pré-processado do 1.5B
- **Elimina silêncio cognitivo**: Jogador sempre recebe feedback imediato
- **Otimiza uso de recursos**: Modelos especializados para suas funções

**Referências**:
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa
- [TASKS.md](TASKS.md) - Tasks consolidadas (Fase M1-M6)
- [TASKS_PIPELINE_MIGRATION.md](TASKS_PIPELINE_MIGRATION.md) - Tasks detalhadas de migração

### M1: Preparação e Infraestrutura
**Status**: ✅ Documentação concluída, 🔄 Implementação pendente

#### M1.1 Atualizar Documentação ✅
- [x] Criar PIPELINE_ARCHITECTURE.md
- [x] Criar QWEN_1_5B_SPEC.md
- [x] Criar QWEN_14B_SPEC.md
- [x] Atualizar todos os documentos relacionados

**Task**: `update-docs-pipeline-architecture` ✅

#### M1.2 Adicionar Suporte a Qwen-1.5B
- [ ] Configuração para modelo 1.5B
- [ ] Carregamento de modelo 1.5B
- [ ] Função de inferência rápida
- [ ] Endpoint `/llm/prelude`
- [ ] Testes (95%+ coverage)

**Task**: `add-qwen-1-5b-support`

#### M1.3 Banco de Frases de Ponte Humana
- [ ] Estrutura de dados
- [ ] Arquivo JSON/YAML com frases categorizadas
- [ ] Sistema anti-repetição
- [ ] Integração com prompt 1.5B
- [ ] Testes (95%+ coverage)

**Task**: `implement-human-bridge-phrases`

**Dependências**: `add-qwen-1-5b-support`

---

### M2: Orquestrador - Pipeline de 3 Agentes
**Status**: 🔄 PENDENTE

#### M2.1 Estado de Pipeline
- [ ] Enum `PipelineStatus`
- [ ] Estrutura `PipelineState`
- [ ] Transições de estado
- [ ] Validação de transições
- [ ] Testes (95%+ coverage)

**Task**: `implement-pipeline-state`

#### M2.2 Lógica de Disparo do 1.5B
- [ ] Função `should_trigger_1_5b()`
- [ ] Detecção de tempo de fala (6-8s)
- [ ] Detecção de pausa (VAD)
- [ ] Detecção de ação clara
- [ ] Função `trigger_1_5b()`
- [ ] Envio imediato para TTS
- [ ] Testes (95%+ coverage)

**Task**: `implement-1-5b-trigger-logic`

**Dependências**: `implement-pipeline-state`, `add-qwen-1-5b-support`

#### M2.3 Preparação de Contexto para 14B
- [ ] Função `prepare_14b_context()`
- [ ] Inclusão de `fast_prelude` (1.5B)
- [ ] Inclusão de `asr_final`
- [ ] Inclusão de `game_state`
- [ ] Inclusão de `context_slice` (últimos 3-6 eventos)
- [ ] Inclusão de `vectorizer_results`
- [ ] Limitação de tokens (8192)
- [ ] Testes (95%+ coverage)

**Task**: `implement-14b-context-preparation`

**Dependências**: `implement-pipeline-state`, `add-qwen-1-5b-support`

#### M2.4 Fluxo Completo do Pipeline
- [ ] Função `handle_player_input()`
- [ ] Recepção de `asr_partial`
- [ ] Parsing de intent
- [ ] Disparo automático do 1.5B
- [ ] Espera por `asr_final`
- [ ] Preparação de contexto para 14B
- [ ] Chamada ao 14B
- [ ] Envio para TTS
- [ ] Tratamento de erros
- [ ] Testes (95%+ coverage)

**Task**: `implement-complete-pipeline-flow`

**Dependências**: `implement-1-5b-trigger-logic`, `implement-14b-context-preparation`

---

### M3: Orquestrador - Respostas Objetivas
**Status**: 🔄 PENDENTE

#### M3.1 Respostas Objetivas sem LLM
- [ ] Função `is_objective_question()`
- [ ] Detecção de perguntas factuais
- [ ] Função `answer_objective_question()`
- [ ] Consulta direta ao game_state
- [ ] Testes (95%+ coverage)

**Task**: `implement-objective-responses`

**Dependências**: `implement-pipeline-state`

#### M3.2 Consulta de Regras Simples (Vectorizer + 1.5B)
- [ ] Função `is_simple_rule_question()`
- [ ] Detecção de perguntas de regra simples
- [ ] Consulta ao Vectorizer
- [ ] Conversão via 1.5B (não 14B)
- [ ] Testes (95%+ coverage)

**Task**: `implement-simple-rule-query`

**Dependências**: `implement-objective-responses`, `add-qwen-1-5b-support`

---

### M4: Cache e Estado
**Status**: 🔄 PENDENTE

#### M4.1 Cache de Estado do Jogo (RAM)
- [ ] Estrutura `GameStateCache`
- [ ] Armazenamento (HP, AC, recursos, status, posição, iniciativa)
- [ ] Atualização de cache
- [ ] Consulta rápida
- [ ] Invalidação
- [ ] Testes (95%+ coverage)

**Task**: `implement-game-state-cache`

**Dependências**: `implement-pipeline-state`

#### M4.2 Cache de Contexto da Cena (RAM + Vector)
- [ ] Estrutura `SceneContextCache`
- [ ] Armazenamento (últimas 3-6 ações, rolagens, NPCs ativos)
- [ ] Limite de histórico (máximo 6 eventos)
- [ ] Integração com Vectorizer
- [ ] Preparação de context_slice para 14B
- [ ] Testes (95%+ coverage)

**Task**: `implement-scene-context-cache`

**Dependências**: `implement-game-state-cache`

#### M4.3 Cache de Lore (Vectorizer)
- [ ] Estrutura `LoreCache`
- [ ] Integração com Vectorizer
- [ ] Cache de queries frequentes (TTL: 5 minutos)
- [ ] Preparação de lore_context para 14B
- [ ] Testes (95%+ coverage)

**Task**: `implement-lore-cache`

**Dependências**: `implement-scene-context-cache`

---

### M5: Validação e Testes
**Status**: 🔄 PENDENTE

#### M5.1 Testes de Integração do Pipeline
- [ ] Teste end-to-end: ASR → 1.5B → 14B → TTS
- [ ] Teste de que 1.5B sempre dispara antes do 14B
- [ ] Teste de latência total < 6s
- [ ] Teste de que 1.5B não gera resultados finais
- [ ] Teste de que 14B recebe fast_prelude
- [ ] Teste de respostas objetivas sem LLM
- [ ] Teste de consulta de regras simples
- [ ] Teste de tratamento de erros
- [ ] Teste de cache

**Task**: `test-pipeline-integration`

**Dependências**: `implement-complete-pipeline-flow`, `implement-objective-responses`, `implement-simple-rule-query`, `implement-lore-cache`

#### M5.2 Testes de Performance
- [ ] Benchmark latência 1.5B (< 1.2s)
- [ ] Benchmark latência 14B (< 6s)
- [ ] Benchmark respostas objetivas (< 50ms)
- [ ] Benchmark consulta de regras simples (< 1.5s)
- [ ] Benchmark uso de memória (ambos modelos)
- [ ] Benchmark throughput (interações/minuto)

**Task**: `test-pipeline-performance`

**Dependências**: `test-pipeline-integration`

#### M5.3 Testes de Regressão
- [ ] Executar todos os testes existentes
- [ ] Verificar combate ainda funciona
- [ ] Verificar diálogos ainda funcionam
- [ ] Verificar rolagens ainda funcionam
- [ ] Verificar memória ainda funciona
- [ ] Verificar UI ainda funciona
- [ ] Corrigir regressões

**Task**: `test-pipeline-regression`

**Dependências**: `test-pipeline-integration`

---

### M6: Documentação e Deploy
**Status**: 🔄 PENDENTE

#### M6.1 Atualizar Documentação de Usuário
- [ ] Atualizar README.md
- [ ] Criar guia de configuração (1.5B e 14B)
- [ ] Criar guia de troubleshooting
- [ ] Atualizar CHANGELOG.md
- [ ] Criar guia de migração

**Task**: `update-user-documentation`

**Dependências**: `test-pipeline-regression`

#### M6.2 Preparar Deploy
- [ ] Atualizar scripts de build (incluir modelo 1.5B)
- [ ] Atualizar configurações padrão
- [ ] Criar migração de dados se necessário
- [ ] Atualizar documentação de instalação
- [ ] Preparar release notes

**Task**: `prepare-pipeline-deploy`

**Dependências**: `update-user-documentation`

---

## Fase 0: Infraestrutura Base (Pré-requisitos)

**Status**: Em Planejamento  
**Prioridade**: CRÍTICA (deve ser feito antes ou em paralelo com M1)

### 0.1 Setup do Projeto Base
- [ ] Estrutura de diretórios
- [ ] Configuração Rust workspace
- [ ] Configuração TypeScript/Electron
- [ ] Ferramentas de qualidade de código
- [ ] Scripts de desenvolvimento

**Task**: `setup-project-base`

### 0.2 Configuração de CI/CD
- [ ] Workflows GitHub Actions
- [ ] Testes automatizados
- [ ] Linting automatizado
- [ ] Builds multi-plataforma
- [ ] Releases automatizados

**Task**: `setup-cicd`

**Dependências**: `setup-project-base`

---

## Fase 1: Serviços Core (Rust)

**Status**: Estruturas criadas, implementação em progresso  
**Prioridade**: ALTA (necessários para pipeline)

### 1.1 Rules5e Service
- [ ] Parser de expressões de dados
- [ ] Cálculos determinísticos
- [ ] HTTP server
- [ ] Testes (95%+ coverage)

**Task**: `implement-rules5e-service` ✅ (estrutura criada)

### 1.2 ASR Service
- [ ] Integração Whisper
- [ ] VAD (Voice Activity Detection)
- [ ] Processamento de chunks
- [ ] HTTP server
- [ ] Testes (95%+ coverage)

**Task**: `implement-asr-service` ✅ (estrutura criada)

### 1.3 TTS Service (XTTS + SoVITS + Voice INTENTS)
- [x] Pipeline de 3 camadas (XTTS + SoVITS)
- [x] Sistema de perfis vocais
- [x] Suporte multi-voz
- [x] Voice INTENTS
- [x] HTTP server
- [ ] Integração com Orquestrador (pendente)

**Task**: `implement-tts-service` ✅ (estrutura existe, migrado para XTTS + SoVITS)

**Ver**: [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md)

### 1.4 LLM Core (Atualizar para Pipeline Dual)
- [ ] **ATUALIZAR**: Carregamento de ambos modelos (1.5B + 14B)
- [ ] **ATUALIZAR**: Endpoints separados (`/llm/prelude` e `/llm/narration`)
- [ ] Sistema de personas
- [ ] Streaming de tokens
- [ ] Integração Memory Service
- [ ] HTTP server
- [ ] Testes (95%+ coverage)

**Task**: `implement-llm-core` ✅ (estrutura criada, **precisa atualização para pipeline dual**)

**Dependências**: `add-qwen-1-5b-support` (Fase M1.2)

### 1.5 Memory Service
- [ ] Integração Vectorizer
- [ ] Integração Nexus
- [ ] Integração Lexum
- [ ] Integração Transmutation
- [ ] Integração Classify
- [ ] HTTP server
- [ ] Testes (95%+ coverage)

**Task**: `implement-memory-service` ✅ (estrutura criada)

### 1.6 Infra Runtime
- [ ] Inicialização de serviços
- [ ] Health-check periódico
- [ ] Retry/backoff
- [ ] Graceful shutdown
- [ ] Observabilidade
- [ ] Testes (95%+ coverage)

**Task**: `implement-infra-runtime` ✅ (estrutura criada)

---

## Fase 2: Orquestrador e INTENT DSL

**Status**: Estrutura existe, precisa migração para pipeline de 3 agentes  
**Prioridade**: CRÍTICA (base da nova arquitetura)

### 2.1 Orquestrador Base
- [ ] **INTEGRAR**: Pipeline de 3 agentes (ver Fase M2)
- [ ] Máquina de estados de cena (FSM)
- [ ] Comunicação IPC/WebSocket
- [ ] Integração com services
- [ ] Testes (95%+ coverage)

**Task**: `implement-orchestrator`

**Dependências**: `implement-complete-pipeline-flow` (Fase M2.4), todos os serviços core

**Ver**: [ORCHESTRATOR.md](ORCHESTRATOR.md), [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md)

### 2.2 Parser de INTENT DSL
- [ ] Parser determinístico
- [ ] Enum Intent com todas as variantes
- [ ] Normalização e validação
- [ ] Tratamento de erros
- [ ] Testes (95%+ coverage)

**Task**: `implement-intent-dsl-parser`

**Ver**: [INTENT_DSL.md](INTENT_DSL.md)

### 2.3 Executor de INTENTs
- [ ] Execução de cada tipo de INTENT
- [ ] Integração com rules5e-service
- [ ] Integração com memory-service
- [ ] Testes (95%+ coverage)

**Task**: `implement-intent-executor`

**Dependências**: `implement-intent-dsl-parser`

### 2.4 Atualizar LLM Core para INTENT DSL
- [ ] Atualizar prompts do 14B com exemplos de INTENT DSL
- [ ] **ATUALIZAR**: Garantir que 1.5B NÃO gera INTENTs
- [ ] Incorporar DM_MINDSET.md (14B)
- [ ] Incorporar QWEN_1_5B_SPEC.md (1.5B)
- [ ] Incorporar CHARACTER_AGENTS.md
- [ ] Testes (95%+ coverage)

**Task**: `update-llm-core-intent-dsl`

**Dependências**: `implement-intent-dsl-parser`, `add-qwen-1-5b-support`

---

## Fase 3: Game Engine (Refatorado)

**Status**: Estrutura existe, precisa refatoração  
**Prioridade**: ALTA

### 3.1 Game Engine Core (Refatorado)
- [ ] Refatorar para trabalhar com Orquestrador
- [ ] Remover lógica de coordenação
- [ ] Manter apenas estado de jogo
- [ ] Estrutura GameSession
- [ ] Sistema de cenas
- [ ] Sistema de atores
- [ ] Sistema de efeitos
- [ ] Integração via Orquestrador
- [ ] Testes (95%+ coverage)

**Task**: `refactor-game-engine-orchestrator` ✅ (estrutura existe, precisa refatoração)

**Dependências**: `implement-orchestrator`

---

## Fase 4: Modos de Cena e Turn Engine

**Status**: Em Planejamento  
**Prioridade**: CRÍTICA

### 4.1 Modos de Cena (FSM)
- [ ] Modo SocialFreeFlow
- [ ] Modo Exploration
- [ ] Modo CombatTurnBased
- [ ] Modo DowntimePreparation
- [ ] Testes (95%+ coverage)

**Task**: `implement-scene-modes`

**Dependências**: `implement-orchestrator`

### 4.2 Turn Engine (Combate em Turnos)
- [ ] Sistema de iniciativa
- [ ] Rolagens client vs servidor
- [ ] Economia de ações
- [ ] Line of Sight (LoS) e alcance
- [ ] Áreas de Efeito (AoE)
- [ ] Avanço de iniciativa
- [ ] Narração por ação
- [ ] Testes (95%+ coverage)

**Task**: `implement-turn-engine`

**Dependências**: `implement-orchestrator`, `implement-rules5e-service`

**Ver**: [COMBAT_FLOW.md](COMBAT_FLOW.md)

---

## Fase 5: Client Electron (Frontend)

**Status**: Estrutura criada  
**Prioridade**: ALTA

### 5.1 Electron Main Process
- [ ] Configuração Electron
- [ ] Spawn de serviços (incluindo Orquestrador)
- [ ] Handlers IPC
- [ ] Health-check
- [ ] Testes (95%+ coverage)

**Task**: `implement-client-electron` ✅ (estrutura criada)

### 5.2 Componentes Base
- [ ] Layout principal
- [ ] Sistema de temas
- [ ] Componentes UI base
- [ ] Testes (95%+ coverage)

**Task**: `implement-renderer-base-components`

### 5.3 BattleMap Component
- [ ] Renderização isométrica
- [ ] Sistema de tokens
- [ ] Interações
- [ ] Testes (95%+ coverage)

**Task**: `implement-battlemap-component`

### 5.4 Turn Order / Talking Cards
- [ ] Turn Order (combate)
- [ ] Talking Cards (fora de combate)
- [ ] Indicadores visuais
- [ ] Testes (95%+ coverage)

**Task**: `implement-turn-order-component`

### 5.5 Action Bar
- [ ] Barra de ações
- [ ] Botão TALK
- [ ] Slots de ações
- [ ] Testes (95%+ coverage)

**Task**: `implement-action-bar-component`

### 5.6 Menus Retráteis
- [ ] Inventário
- [ ] Ficha
- [ ] Mapa
- [ ] Diário
- [ ] Testes (95%+ coverage)

**Task**: `implement-retractable-menus-component`

### 5.7 History Panel
- [ ] Histórico de áudio
- [ ] Histórico de rolagens
- [ ] Histórico visual
- [ ] Testes (95%+ coverage)

**Task**: `implement-history-panel-component`

### 5.8 Dice Rolling
- [ ] Animação 3D/2D
- [ ] Integração Rules5e
- [ ] Efeitos visuais
- [ ] Testes (95%+ coverage)

**Task**: `implement-dice-rolling-component`

### 5.9 Voice Integration
- [ ] Captura de áudio
- [ ] Pipeline ASR
- [ ] Pipeline TTS
- [ ] Waveform
- [ ] Testes (95%+ coverage)

**Task**: `implement-voice-integration`

---

## Fase 6: Integração e Pipeline

**Status**: Em Planejamento  
**Prioridade**: ALTA

### 6.1 IPC and API Contracts
- [ ] Tipos compartilhados
- [ ] Validação de mensagens
- [ ] Versionamento
- [ ] Documentação
- [ ] Testes (95%+ coverage)

**Task**: `implement-ipc-contracts`

### 6.2 Pipeline Voz → Voz
- [ ] **ATUALIZAR**: Integração completa com pipeline de 3 agentes
- [ ] Otimizações de latência
- [ ] Tratamento de erros
- [ ] Métricas
- [ ] Testes (95%+ coverage)

**Task**: `implement-voice-pipeline`

**Dependências**: `implement-complete-pipeline-flow` (Fase M2.4)

---

## Fase 7: Sistema D&D 5e Completo

**Status**: Em Planejamento  
**Prioridade**: ALTA

### 7.1 Sistema de Personagem
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

**Task**: `implement-character-system`

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 1

### 7.2 Sistema de Combate
- [ ] Turn Engine (já listado em Fase 4.2)
- [ ] Sistema de Ataques
- [ ] Sistema de Dano
- [ ] Sistema de Condições
- [ ] Sistema de Movimento
- [ ] Sistema de Ações
- [ ] Sistema de Iniciativa
- [ ] Sistema de Death Saves

**Task**: `implement-combat-system`

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 2

### 7.3 Sistema de Magias
- [ ] Spell Database (SRD completo)
- [ ] Spell Slots Management
- [ ] Spell Casting
- [ ] Spell Components
- [ ] Spell Concentration
- [ ] Spell Duration
- [ ] Spell Areas of Effect
- [ ] Spell Saving Throws

**Task**: `implement-spell-system`

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 3

### 7.4 Sistema de Monstros
- [ ] Monster Database (SRD completo)
- [ ] Monster Stat Blocks
- [ ] Monster Abilities
- [ ] Monster Actions
- [ ] Monster Legendary Actions
- [ ] Monster Lair Actions

**Task**: `implement-monster-system`

**Ver**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md) - Fase 4, [MONSTER_MANUAL_TASKS.md](MONSTER_MANUAL_TASKS.md)

---

## Fase 8: Assets e Geração

**Status**: Em Planejamento  
**Prioridade**: MÉDIA

### 8.1 Image Generation Pipeline
- [ ] Integração Flux.1
- [ ] Geração de retratos
- [ ] Geração de cenas
- [ ] Geração de battlemaps
- [ ] Cache de imagens
- [ ] Testes (95%+ coverage)

**Task**: `implement-image-generation-pipeline`

**Ver**: [ASSETS_GENERATION.md](ASSETS_GENERATION.md)

### 8.2 LoRA Training Pipeline
- [ ] Coleta de datasets
- [ ] Treinamento de embeddings
- [ ] Treinamento de LoRAs
- [ ] Validação
- [ ] Testes (95%+ coverage)

**Task**: `implement-lora-training-pipeline`

**Ver**: [TRAINING_PIPELINE.md](TRAINING_PIPELINE.md)

---

## Fase 9: Modo Preparação

**Status**: Em Planejamento  
**Prioridade**: BAIXA

### 9.1 Preparation Mode System
- [ ] Detecção de fim de sessão
- [ ] Geração de jobs
- [ ] Priorização
- [ ] Execução de jobs
- [ ] UI de progresso
- [ ] Testes (95%+ coverage)

**Task**: `implement-preparation-mode`

---

## Fase 10: Testes e Qualidade

**Status**: Estrutura criada  
**Prioridade**: ALTA

### 10.1 Test Suite Completo
- [ ] Testes unitários (95%+ coverage)
- [ ] Testes de integração
- [ ] Testes E2E
- [ ] Testes de performance
- [ ] Testes de carga

**Task**: `implement-testing-suite` ✅ (estrutura criada)

### 10.2 Integration Tests
- [ ] Testes ASR → LLM → TTS
- [ ] Testes Game Engine → Rules5e
- [ ] Testes Memory Service → Hive
- [ ] Testes Client Electron → Serviços
- [ ] **NOVO**: Testes Pipeline completo (ASR → 1.5B → 14B → TTS)

**Task**: `implement-integration-tests` ✅ (estrutura criada)

---

## Fase 11: Otimização e Performance

**Status**: Em Planejamento  
**Prioridade**: ALTA

### 11.1 Performance Optimizations
- [ ] Otimizações LLM Core (pipeline dual)
- [ ] Otimizações ASR Service
- [ ] Otimizações TTS Service
- [ ] Otimizações Game Engine
- [ ] Otimizações Memory Service
- [ ] Otimizações Frontend
- [ ] Monitoramento de métricas

**Task**: `implement-performance-optimizations`

**Ver**: [PERFORMANCE.md](PERFORMANCE.md)

### 11.2 Caching System
- [ ] Cache de imagens
- [ ] Cache de queries
- [ ] Cache de cálculos
- [ ] Cache de vozes
- [ ] Cache de modelos
- [ ] Invalidação de cache

**Task**: `implement-caching-system`

**Nota**: Cache de estado já implementado em Fase M4

---

## Fase 12: Documentação e Deployment

**Status**: Em Planejamento  
**Prioridade**: MÉDIA

### 12.1 Documentação Completa
- [ ] Atualizar README.md
- [ ] Completar ARCHITECTURE.md
- [ ] Completar DESIGN_SYSTEM.md
- [ ] Completar CONFIGURATION.md
- [ ] Completar TESTING.md
- [ ] Completar PERFORMANCE.md
- [ ] Criar guias de desenvolvimento
- [ ] Criar documentação de API

**Task**: `implement-complete-documentation`

### 12.2 Build e Deployment
- [ ] Configurar electron-builder
- [ ] Build multi-plataforma
- [ ] Gerar installers
- [ ] Code signing
- [ ] Auto-updater
- [ ] Distribuição

**Task**: `implement-build-deployment`

**Ver**: [DEPLOYMENT.md](DEPLOYMENT.md)

---

## Ordem de Implementação Recomendada

### Sprint 1: Preparação (2-3 semanas)
1. ✅ Atualizar documentação (M1.1) - CONCLUÍDO
2. Setup do projeto base (Fase 0.1)
3. Adicionar suporte a Qwen-1.5B (M1.2)
4. Banco de frases de ponte humana (M1.3)

### Sprint 2: Pipeline Core (3-4 semanas)
1. Estado de pipeline (M2.1)
2. Lógica de disparo do 1.5B (M2.2)
3. Preparação de contexto para 14B (M2.3)
4. Fluxo completo do pipeline (M2.4)

### Sprint 3: Otimizações (2-3 semanas)
1. Respostas objetivas sem LLM (M3.1)
2. Consulta de regras simples (M3.2)
3. Cache de estado do jogo (M4.1)
4. Cache de contexto da cena (M4.2)
5. Cache de lore (M4.3)

### Sprint 4: Validação (2 semanas)
1. Testes de integração (M5.1)
2. Testes de performance (M5.2)
3. Testes de regressão (M5.3)

### Sprint 5: Deploy (1 semana)
1. Atualizar documentação de usuário (M6.1)
2. Preparar deploy (M6.2)

### Sprint 6+: Funcionalidades Adicionais
- Orquestrador completo (Fase 2)
- INTENT DSL (Fase 2)
- Game Engine refatorado (Fase 3)
- Modos de cena (Fase 4)
- Turn Engine (Fase 4)
- Frontend (Fase 5)
- Sistema D&D 5e completo (Fase 7)

---

## Métricas de Progresso

### Por Fase
- **Fase M (Migração)**: 5% (1/20 tasks completas - documentação)
- **Fase 0**: 0% (0/2 tasks completas)
- **Fase 1**: 0% (0/6 tasks completas - estruturas criadas)
- **Fase 2**: 0% (0/4 tasks completas)
- **Fase 3**: 0% (0/1 tasks completas)
- **Fase 4**: 0% (0/2 tasks completas)
- **Fase 5**: 0% (0/9 tasks completas)
- **Fase 6**: 0% (0/2 tasks completas)
- **Fase 7**: 0% (0/4 tasks completas)
- **Fase 8**: 0% (0/2 tasks completas)
- **Fase 9**: 0% (0/1 tasks completas)
- **Fase 10**: 0% (0/2 tasks completas)
- **Fase 11**: 0% (0/2 tasks completas)
- **Fase 12**: 0% (0/2 tasks completas)

### Geral
- **Total de Tasks**: 60+ (incluindo migração)
- **Tasks Completas**: 1 (documentação)
- **Tasks em Progresso**: 0
- **Tasks Pendentes**: 59+
- **Progresso Geral**: ~2%

---

## Próximos Passos Imediatos

1. **Completar Fase M1** (Preparação)
   - Adicionar suporte a Qwen-1.5B no LLM Core
   - Implementar banco de frases de ponte humana

2. **Iniciar Fase M2** (Pipeline de 3 Agentes)
   - Implementar estado de pipeline
   - Implementar lógica de disparo do 1.5B
   - Implementar preparação de contexto para 14B
   - Implementar fluxo completo

3. **Completar Fase 0** (Infraestrutura Base)
   - Setup do projeto base
   - Configuração de CI/CD

4. **Atualizar Fase 1** (Serviços Core)
   - Atualizar LLM Core para pipeline dual
   - Completar implementação dos serviços

---

## Referências

- [TASKS.md](TASKS.md) - Tasks consolidadas de implementação
- [TASKS_PIPELINE_MIGRATION.md](TASKS_PIPELINE_MIGRATION.md) - Tasks detalhadas de migração
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa do pipeline
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Especificação do orquestrador
- [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md) - Especificação do Qwen-1.5B
- [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md) - Especificação do Qwen-14B
- [ARCHITECTURE.md](ARCHITECTURE.md) - Arquitetura do sistema
- [TESTS_MASTER.md](TESTS_MASTER.md) - Plano completo de testes

---

**Última Atualização**: 2025-01-XX
