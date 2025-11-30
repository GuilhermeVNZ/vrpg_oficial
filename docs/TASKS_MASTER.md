# VRPG Client - Master Task List

## Visão Geral

Este documento lista **todas as tarefas necessárias** para implementar o VRPG Client de ponta a ponta, seguindo as diretrizes do rulebook e as especificações da documentação do projeto.

**Formato**: Cada tarefa deve ser criada usando `rulebook task create <task-id>` antes da implementação.

**Prioridade**: As tarefas estão organizadas por prioridade e dependências. Implementar na ordem especificada.

**Cobertura de Testes**: Todas as tarefas devem incluir testes com cobertura mínima de 95% (conforme AGENTS.md).

---

## Fase 0: Infraestrutura Base (Pré-requisitos)

### 0.1 Setup do Projeto Base
**Task ID**: `setup-project-base`

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
**Task ID**: `implement-rules5e-service` ✅ (já existe)

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
**Task ID**: `implement-asr-service` ✅ (já existe)

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
**Task ID**: `implement-tts-service` ✅ (estrutura existe, migrado para XTTS + SoVITS)

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

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 5.1, [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md), [VOICE_INTENTS.md](VOICE_INTENTS.md)

---

### 1.4 LLM Core
**Task ID**: `implement-llm-core` ✅ (já existe)

**Descrição**: Implementar serviço de inferência LLM local com suporte a múltiplas personas.

**Tarefas**:
- [ ] Integrar llama.cpp ou Candle para inferência
- [ ] Implementar carregamento de modelo (Qwen 2.5 14B Q4_K_M)
- [ ] Implementar otimizações (mmap, mlock, NUMA)
- [ ] Implementar KV cache para contexto
- [ ] Implementar streaming de tokens
- [ ] Implementar sistema de personas (DM, NPC, Player IA, Monster, Narrator)
- [ ] Implementar DSL de intenções (describe_scene, npc_dialogue, combat_resolution)
- [ ] Implementar integração com LessTokens (compressão de prompts)
- [ ] Implementar HTTP server (localhost:7002)
- [ ] Implementar endpoint `/health`
- [ ] Implementar endpoint `/llm` (requisição principal)
- [ ] Implementar integração com Memory Service
- [ ] Implementar integração com Rules5e Service
- [ ] Implementar logging estruturado
- [ ] Implementar métricas de performance (tokens/s, latência)

**Testes Críticos**:
- [ ] Teste de latência LLM (< 200ms para inferência)
- [ ] Teste de mudança de persona (consistência mantida)
- [ ] Teste de streaming (tokens incrementais)
- [ ] Teste de integração com Memory Service
- [ ] Teste de integração com Rules5e Service
- [ ] Teste de compressão LessTokens (quando aplicável)
- [ ] Teste de cobertura (95%+)

**Dependências**: `setup-project-base`, `implement-memory-service`

**Prioridade**: ALTA (core do sistema)

---

### 1.5 Memory Service
**Task ID**: `implement-memory-service` ✅ (já existe)

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
**Task ID**: `implement-infra-runtime` ✅ (já existe)

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

---

## Fase 2: Orquestrador e INTENT DSL

### 2.1 Orquestrador Base
**Task ID**: `implement-orchestrator`

**Descrição**: Implementar módulo Orquestrador que coordena todos os serviços e gerencia estados de cena.

**Tarefas**:
- [ ] Criar estrutura `src/orchestrator/` em Rust
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
- [ ] Teste de comunicação IPC/WebSocket
- [ ] Teste de integração com services
- [ ] Teste de persistência de sessão
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-rules5e-service`, `implement-memory-service`, `implement-asr-service`, `implement-tts-service`

**Prioridade**: CRÍTICA (base da nova arquitetura)

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 1.1

---

### 2.2 Parser de INTENT DSL
**Task ID**: `implement-intent-dsl-parser`

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

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 1.2, [INTENT_DSL.md](INTENT_DSL.md)

---

### 2.3 Executor de INTENTs
**Task ID**: `implement-intent-executor`

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

**Dependências**: `implement-intent-dsl-parser`, `implement-rules5e-service`, `implement-memory-service`

**Prioridade**: CRÍTICA (execução de ações)

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 1.3

---

### 2.4 Atualizar LLM Core para Gerar INTENT DSL
**Task ID**: `update-llm-core-intent-dsl`

**Descrição**: Modificar `llm-core` para gerar INTENT DSL ao invés de JSON.

**Tarefas**:
- [ ] Atualizar prompts do Mestre IA com exemplos de INTENT DSL
- [ ] Atualizar prompts de Jogadores IA (remover geração de INTENTs)
- [ ] Modificar processamento de resposta:
  - [ ] Extrair blocos `[INTENTS] ... [/INTENTS]`
  - [ ] Separar narração de INTENTs
  - [ ] Validar INTENTs antes de enviar ao Orquestrador
- [ ] Incorporar `DM_MINDSET.md` nos prompts
- [ ] Incorporar `CHARACTER_AGENTS.md` nos prompts
- [ ] Testes de geração de INTENTs

**Testes Críticos**:
- [ ] Teste de geração de INTENTs válidas
- [ ] Teste de separação narração/INTENTs
- [ ] Teste de fallback quando parsing falha
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-intent-dsl-parser`, `implement-llm-core`

**Prioridade**: ALTA (necessário para nova arquitetura)

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 1.4, [DM_MINDSET.md](DM_MINDSET.md), [CHARACTER_AGENTS.md](CHARACTER_AGENTS.md)

---

## Fase 3: Game Engine (Refatorado)

### 3.1 Game Engine Core (Refatorado)
**Task ID**: `refactor-game-engine-orchestrator` ✅ (estrutura existe, precisa refatoração)

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
- [ ] Implementar modo DowntimePreparation:
  - [ ] Estado no FSM
  - [ ] Fila de jobs (GPU/CPU)
- [ ] Testes de transições entre modos

**Testes Críticos**:
- [ ] Teste de cada modo de cena
- [ ] Teste de transições entre modos
- [ ] Teste de UI adaptada por modo
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-orchestrator`

**Prioridade**: ALTA

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Fase 2

---

### 4.2 Turn Engine (Combate em Turnos)
**Task ID**: `implement-turn-engine`

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

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Task 2.3, [COMBAT_FLOW.md](COMBAT_FLOW.md)

---

## Fase 5: Client Electron (Frontend)

### 5.1 Electron Main Process
**Task ID**: `implement-client-electron` ✅ (já existe)

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

### 3.2 Electron Renderer - Componentes Base
**Task ID**: `implement-renderer-base-components`

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

### 3.3 BattleMap Component
**Task ID**: `implement-battlemap-component`

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

### 3.4 Turn Order / Talking Cards Component
**Task ID**: `implement-turn-order-component`

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

### 3.5 Action Bar Component
**Task ID**: `implement-action-bar-component`

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
- [ ] Teste de renderização (barra fixa no rodapé)
- [ ] Teste de botão TALK (ativação de voz)
- [ ] Teste de slots (drag-and-drop funciona)
- [ ] Teste de atalhos (teclado funciona)
- [ ] Teste de integração com game-engine (ações aplicadas)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-game-engine`

**Prioridade**: ALTA (essencial para interação)

---

### 3.6 Menus Retráteis Component
**Task ID**: `implement-retractable-menus-component`

**Descrição**: Implementar menus retráteis na esquerda (inventário, ficha, mapa, diário).

**Tarefas**:
- [ ] Implementar coluna vertical de botões/ícones (esquerda)
- [ ] Implementar painel de Inventário (I)
- [ ] Implementar painel de Ficha/Personagem (C)
- [ ] Implementar painel de Mapa (M)
- [ ] Implementar painel de Diário/Journal (J)
- [ ] Implementar painel de Party/Companions
- [ ] Implementar painel de Configurações
- [ ] Implementar painel de Regras
- [ ] Implementar animações de abertura/fechamento
- [ ] Implementar estilo BG3 (bordas decoradas, fundo texturizado)
- [ ] Implementar atalhos de teclado (I, C, M, J)

**Testes Críticos**:
- [ ] Teste de renderização (menus aparecem/desaparecem)
- [ ] Teste de atalhos (teclado funciona)
- [ ] Teste de animações (suaves e responsivas)
- [ ] Teste de conteúdo (dados corretos exibidos)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-game-engine`

**Prioridade**: MÉDIA (funcionalidades importantes)

---

### 3.7 History Panel Component
**Task ID**: `implement-history-panel-component`

**Descrição**: Implementar painel de histórico à direita (áudios, eventos, rolagens, imagens).

**Tarefas**:
- [ ] Implementar painel vertical à direita (recolhível)
- [ ] Implementar histórico de áudio (estilo WhatsApp in-game)
- [ ] Implementar histórico de rolagens (resultados, modificadores, tipo)
- [ ] Implementar histórico visual (imagens geradas - thumbnails clicáveis)
- [ ] Implementar indicador visual de quem falou
- [ ] Implementar reprodução de áudio (tocar novamente falas)
- [ ] Implementar scroll virtual (para listas longas)
- [ ] Implementar filtros (por tipo, por entidade)

**Testes Críticos**:
- [ ] Teste de renderização (painel aparece/desaparece)
- [ ] Teste de histórico de áudio (mensagens exibidas)
- [ ] Teste de histórico de rolagens (resultados corretos)
- [ ] Teste de histórico visual (imagens exibidas)
- [ ] Teste de reprodução de áudio (tocar novamente)
- [ ] Teste de scroll virtual (performance com muitos itens)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-game-engine`

**Prioridade**: MÉDIA (funcionalidades importantes)

---

### 3.8 Dice Rolling Component
**Task ID**: `implement-dice-rolling-component`

**Descrição**: Implementar animação de rolagem de dados no centro da tela (estilo BG3).

**Tarefas**:
- [ ] Implementar animação 3D/2D estilizada de dados
- [ ] Implementar animação de rolagem (movimento realista)
- [ ] Implementar exibição de resultado (com brilho)
- [ ] Implementar ícones de resultado (crítico, falha, normal)
- [ ] Implementar som de rolagem suave
- [ ] Implementar integração com Rules5e Service
- [ ] Implementar overlay (aparece sobre o mapa)
- [ ] Implementar animações de entrada/saída

**Testes Críticos**:
- [ ] Teste de renderização (animação aparece)
- [ ] Teste de animação (suave e responsiva)
- [ ] Teste de resultado (exibido corretamente)
- [ ] Teste de integração com Rules5e Service
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-rules5e-service`

**Prioridade**: MÉDIA (melhora imersão)

---

### 3.9 Voice Input/Output Integration
**Task ID**: `implement-voice-integration`

**Descrição**: Implementar integração completa de voz (captura, transcrição, síntese).

**Tarefas**:
- [ ] Implementar captura de áudio via `getUserMedia`
- [ ] Implementar segmentação em chunks (320ms)
- [ ] Implementar envio de chunks para ASR Service via IPC
- [ ] Implementar recebimento de transcrições
- [ ] Implementar reprodução de áudio TTS via Web Audio API
- [ ] Implementar waveform animado (visualização de áudio)
- [ ] Implementar indicador de status do microfone
- [ ] Implementar indicador de latência
- [ ] Implementar mixagem de áudio (TTS + música ambiente)
- [ ] Implementar controle de volume

**Testes Críticos**:
- [ ] Teste de captura de áudio (chunks corretos)
- [ ] Teste de pipeline completo (voz → texto → resposta → voz)
- [ ] Teste de latência (< 300ms target)
- [ ] Teste de waveform (animação correta)
- [ ] Teste de mixagem (áudio não distorce)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-renderer-base-components`, `implement-asr-service`, `implement-tts-service`

**Prioridade**: ALTA (core da experiência)

---

## Fase 5.9: Componentes React Implementados

**Nota**: Esta seção lista tasks para componentes React já criados que precisam de testes e integração completos.

### 5.9.1 Voice HUD Component
**Task ID**: `test-integrate-voice-hud`

**Localização**: `src/client-electron/src/components/VoiceHUD.tsx`

**Status**: ✅ Componente criado | ⚠️ Testes pendentes | ⚠️ Integração pendente

**Tasks Pendentes**:
- [ ] **Task 1.1**: Criar testes unitários para `VoiceHUD.tsx`
  - [ ] Testar renderização de estados (listening, processing, speaking, hidden)
  - [ ] Testar animações de visualizador de áudio
  - [ ] Testar auto-hide timer
  - [ ] Testar typewriter effect
  - [ ] Cobertura mínima: 95%
- [ ] **Task 1.2**: Criar testes de integração para `useVoiceHUD.ts`
  - [ ] Testar transições de estado
  - [ ] Testar cleanup de timers
  - [ ] Testar callbacks (onClose)
  - [ ] Cobertura mínima: 95%
- [ ] **Task 1.3**: Integrar Voice HUD com sistema de voz
  - [ ] Conectar com ASR service
  - [ ] Conectar com TTS service
  - [ ] Implementar eventos de voz
  - [ ] Testar latência e performance
- [ ] **Task 1.4**: Melhorias de acessibilidade
  - [ ] Adicionar ARIA labels
  - [ ] Suporte a `prefers-reduced-motion`
  - [ ] Navegação por teclado
  - [ ] Screen reader support

**Dependências**: `implement-voice-integration`
**Prioridade**: ALTA

---

### 5.9.2 Character Sheet Component
**Task ID**: `test-integrate-character-sheet`

**Localização**: `src/client-electron/src/components/CharacterSheet.tsx`

**Status**: ✅ Componente criado | ⚠️ Testes pendentes | ⚠️ Integração pendente

**Tasks Pendentes**:
- [ ] **Task 2.1**: Criar testes unitários para `CharacterSheet.tsx`
  - [ ] Testar renderização de todos os campos
  - [ ] Testar sistema de abas (main, spells, inventory, features)
  - [ ] Testar barra de atributos horizontal
  - [ ] Testar campos opcionais (portrait, alignment, xp, etc.)
  - [ ] Testar cálculos de modificadores
  - [ ] Cobertura mínima: 95%
- [ ] **Task 2.2**: Criar testes de integração para `useCharacterSheet.ts`
  - [ ] Testar mudança de abas
  - [ ] Testar validação de dados
  - [ ] Testar callbacks (onClose, onSave)
  - [ ] Cobertura mínima: 95%
- [ ] **Task 2.3**: Integrar Character Sheet com game engine
  - [ ] Conectar com session manager
  - [ ] Sincronizar dados do personagem
  - [ ] Implementar salvamento de alterações
  - [ ] Testar atualizações em tempo real
- [ ] **Task 2.4**: Melhorias de acessibilidade
  - [ ] Adicionar ARIA labels
  - [ ] Navegação por teclado entre abas
  - [ ] Screen reader support
  - [ ] Suporte a alto contraste

**Dependências**: `refactor-game-engine-orchestrator`
**Prioridade**: ALTA

---

### 5.9.3 Journal Component
**Task ID**: `test-integrate-journal`

**Localização**: `src/client-electron/src/components/Journal.tsx`

**Status**: ✅ Componente criado | ⚠️ Testes pendentes | ⚠️ Integração pendente

**Tasks Pendentes**:
- [ ] **Task 3.1**: Criar testes unitários para `Journal.tsx`
  - [ ] Testar renderização de lista de entradas
  - [ ] Testar busca em tempo real
  - [ ] Testar filtros por tipo (all, quest, lore, note)
  - [ ] Testar seleção de entrada
  - [ ] Testar estado vazio
  - [ ] Testar scrollbars customizadas
  - [ ] Cobertura mínima: 95%
- [ ] **Task 3.2**: Criar testes de integração para `useJournal.ts`
  - [ ] Testar abertura/fechamento do modal
  - [ ] Testar busca e filtros
  - [ ] Testar seleção de entrada
  - [ ] Testar cleanup ao fechar
  - [ ] Cobertura mínima: 95%
- [ ] **Task 3.3**: Integrar Journal com memory service
  - [ ] Conectar com Vectorizer para busca semântica
  - [ ] Conectar com Lexum para busca textual
  - [ ] Implementar criação de novas entradas
  - [ ] Implementar edição de entradas
  - [ ] Testar performance com muitas entradas
- [ ] **Task 3.4**: Melhorias de acessibilidade
  - [ ] Adicionar ARIA labels
  - [ ] Navegação por teclado
  - [ ] Screen reader support
  - [ ] Suporte a alto contraste

**Dependências**: `implement-memory-service`
**Prioridade**: ALTA

---

### 5.9.4 Gameplay Interface Component
**Task ID**: `test-integrate-gameplay-interface`

**Localização**: `src/client-electron/src/components/GameplayInterface.tsx`

**Status**: ✅ Componente criado | ⚠️ Testes pendentes | ⚠️ Integração pendente

**Tasks Pendentes**:
- [ ] **Task 4.1**: Criar testes unitários para `GameplayInterface.tsx`
  - [ ] Testar renderização de todas as áreas (top-left, sidebar, top-right, footer)
  - [ ] Testar toggle UI (screenshot mode)
  - [ ] Testar party frame com retratos e HP bars
  - [ ] Testar action bar com slots
  - [ ] Testar chat panel com mensagens e cards
  - [ ] Testar push-to-talk container
  - [ ] Cobertura mínima: 95%
- [ ] **Task 4.2**: Criar testes de integração
  - [ ] Testar interação entre componentes
  - [ ] Testar hotkeys (H para toggle UI)
  - [ ] Testar atualizações de estado em tempo real
  - [ ] Testar performance com muitos elementos
  - [ ] Cobertura mínima: 95%
- [ ] **Task 4.3**: Integrar Gameplay Interface com serviços
  - [ ] Conectar com Orchestrator para updates de cena
  - [ ] Conectar com game engine para party data
  - [ ] Conectar com chat service para mensagens
  - [ ] Conectar com voice service para push-to-talk
  - [ ] Implementar atualizações de latência/FPS
  - [ ] Testar performance e latência
- [ ] **Task 4.4**: Melhorias de acessibilidade
  - [ ] Adicionar ARIA labels
  - [ ] Navegação por teclado completa
  - [ ] Screen reader support
  - [ ] Suporte a alto contraste
  - [ ] Suporte a `prefers-reduced-motion`

**Dependências**: `implement-orchestrator`, `refactor-game-engine-orchestrator`
**Prioridade**: ALTA

---

### 5.9.5 Tasks de Integração Geral de Componentes

- [ ] **Task 5.1**: Criar sistema de roteamento de componentes
  - [ ] Implementar navegação entre componentes
  - [ ] Gerenciar estado global de UI
  - [ ] Implementar hotkeys globais
  - [ ] Testar transições entre componentes
- [ ] **Task 5.2**: Criar sistema de temas e design tokens
  - [ ] Implementar sistema de temas
  - [ ] Garantir consistência de design tokens
  - [ ] Testar dark/light mode (se aplicável)
  - [ ] Testar responsividade
- [ ] **Task 5.3**: Criar sistema de notificações
  - [ ] Implementar toast notifications
  - [ ] Integrar com eventos do sistema
  - [ ] Testar diferentes tipos de notificações

**Dependências**: `implement-renderer-base-components`
**Prioridade**: MÉDIA

---

## Fase 4: Integração e Pipeline

### 4.1 IPC and API Contracts
**Task ID**: `implement-ipc-contracts`

**Descrição**: Implementar contratos de comunicação entre todos os módulos.

**Tarefas**:
- [ ] Definir tipos TypeScript compartilhados (shared/)
- [ ] Definir tipos Rust para contratos HTTP
- [ ] Implementar validação de mensagens IPC
- [ ] Implementar validação de payloads HTTP
- [ ] Implementar versionamento de contratos (api_version)
- [ ] Implementar serialização/deserialização (JSON)
- [ ] Documentar todos os contratos (OpenAPI/Swagger)
- [ ] Implementar testes de contratos (contract testing)

**Testes Críticos**:
- [ ] Teste de validação (mensagens inválidas rejeitadas)
- [ ] Teste de serialização (JSON correto)
- [ ] Teste de versionamento (compatibilidade)
- [ ] Teste de contratos (todos os endpoints)
- [ ] Teste de cobertura (95%+)

**Dependências**: Todos os módulos core

**Prioridade**: ALTA (necessário para comunicação)

---

### 4.2 Pipeline Voz → Voz Completo
**Task ID**: `implement-voice-pipeline`

**Descrição**: Implementar pipeline completo voz→voz com otimizações de latência.

**Tarefas**:
- [ ] Integrar todos os componentes (ASR → Game Engine → LLM → TTS)
- [ ] Implementar otimizações de latência (streaming, cache)
- [ ] Implementar tratamento de erros (fallbacks)
- [ ] Implementar métricas de latência (monitoramento)
- [ ] Implementar degradação elegante (se serviços falharem)
- [ ] Implementar testes de latência end-to-end
- [ ] Otimizar pipeline para < 300ms (target: 250ms)

**Testes Críticos**:
- [ ] Teste de latência end-to-end (< 300ms)
- [ ] Teste de pipeline completo (funcionamento correto)
- [ ] Teste de tratamento de erros (fallbacks funcionam)
- [ ] Teste de degradação (funcionamento parcial)
- [ ] Teste de cobertura (95%+)

**Dependências**: Todos os módulos core e frontend

**Prioridade**: ALTA (core da experiência)

---

## Fase 5: Assets e Geração

### 5.1 Image Generation Pipeline
**Task ID**: `implement-image-generation-pipeline`

**Descrição**: Implementar pipeline de geração de imagens (NPCs, cenas, battlemaps).

**Tarefas**:
- [ ] Integrar Flux.1 (variante rápida para sessão)
- [ ] Integrar Flux.1 (variante qualidade para preparação)
- [ ] Implementar LoRA global de estilo VRPG
- [ ] Implementar sistema de embeddings de personagens
- [ ] Implementar geração de retratos (portraits)
- [ ] Implementar geração de cenas narrativas (keyframes)
- [ ] Implementar geração de battlemaps (com grid)
- [ ] Implementar cache de imagens (LRU)
- [ ] Implementar sistema de metadados (character_id, emotion, etc.)
- [ ] Implementar integração com Memory Service (indexação)

**Testes Críticos**:
- [ ] Teste de geração de retratos (latência < 2s em sessão)
- [ ] Teste de geração de battlemaps (grid correto)
- [ ] Teste de cache (reutilização de imagens)
- [ ] Teste de consistência visual (mesmo personagem)
- [ ] Teste de integração com Memory Service
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-memory-service`

**Prioridade**: MÉDIA (melhora imersão)

---

### 5.2 LoRA Training Pipeline
**Task ID**: `implement-lora-training-pipeline`

**Descrição**: Implementar pipeline de treinamento de LoRAs para personagens.

**Tarefas**:
- [ ] Implementar coleta de imagens para dataset
- [ ] Implementar pré-processamento (recorte, normalização)
- [ ] Implementar geração de metadados (emoção, ângulo, contexto)
- [ ] Implementar treinamento de embeddings (Textual Inversion)
- [ ] Implementar treinamento de LoRAs (Low-Rank Adaptation)
- [ ] Implementar validação de LoRAs (qualidade, consistência)
- [ ] Implementar integração com pipeline de geração
- [ ] Implementar modo preparação (execução offline)

**Testes Críticos**:
- [ ] Teste de coleta de dataset (imagens corretas)
- [ ] Teste de pré-processamento (normalização correta)
- [ ] Teste de treinamento de embedding (resultado consistente)
- [ ] Teste de treinamento de LoRA (resultado de qualidade)
- [ ] Teste de validação (qualidade verificada)
- [ ] Teste de integração (uso em geração)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-image-generation-pipeline`

**Prioridade**: BAIXA (funcionalidade avançada)

---

### 5.3 Battlemap Generation with Grid
**Task ID**: `implement-battlemap-generation`

**Descrição**: Implementar geração de battlemaps com integração grid lógico ↔ imagem.

**Tarefas**:
- [ ] Implementar representação lógica de grid (matriz de células)
- [ ] Implementar geração de layout mask (walkable/blocked)
- [ ] Implementar integração com Flux (ControlNet ou similar)
- [ ] Implementar geração de imagem respeitando layout
- [ ] Implementar validação (imagem corresponde ao grid)
- [ ] Implementar sistema de camadas (background, grid overlay, tokens)
- [ ] Implementar reutilização de mapas (variações de clima/horário)
- [ ] Implementar integração com game-engine (grid lógico)

**Testes Críticos**:
- [ ] Teste de grid lógico (células corretas)
- [ ] Teste de geração de imagem (respeita layout)
- [ ] Teste de validação (correspondência grid ↔ imagem)
- [ ] Teste de integração com game-engine
- [ ] Teste de reutilização (variações funcionam)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-image-generation-pipeline`, `implement-game-engine`

**Prioridade**: MÉDIA (funcionalidade importante)

---

## Fase 6: Modo Preparação

### 6.1 Preparation Mode System
**Task ID**: `implement-preparation-mode`

**Descrição**: Implementar sistema de modo preparação (pós-sessão) para geração pesada.

**Tarefas**:
- [ ] Implementar detecção de fim de sessão
- [ ] Implementar geração de lista de jobs (TrainEmbedding, TrainLoRA, GenerateBattlemap)
- [ ] Implementar sistema de priorização (assets certos, prováveis, futuros)
- [ ] Implementar execução de jobs (LoRA training, geração de lotes)
- [ ] Implementar monitoramento de progresso
- [ ] Implementar integração com Memory Service (atualizações)
- [ ] Implementar UI de modo preparação (progresso, logs)

**Testes Críticos**:
- [ ] Teste de detecção de fim de sessão
- [ ] Teste de geração de jobs (lista correta)
- [ ] Teste de priorização (ordem correta)
- [ ] Teste de execução (jobs completam)
- [ ] Teste de integração com Memory Service
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-image-generation-pipeline`, `implement-lora-training-pipeline`

**Prioridade**: BAIXA (funcionalidade avançada)

---

## Fase 7: Testes e Qualidade

### 7.1 Test Suite Completo
**Task ID**: `implement-testing-suite` ✅ (já existe)

**Descrição**: Implementar suite completa de testes (unitários, integração, E2E).

**Tarefas**:
- [ ] Implementar testes unitários para Rules5e Service (95%+ coverage)
- [ ] Implementar testes unitários para ASR Service (95%+ coverage)
- [ ] Implementar testes unitários para TTS Service (95%+ coverage)
- [ ] Implementar testes unitários para LLM Core (95%+ coverage)
- [ ] Implementar testes unitários para Memory Service (95%+ coverage)
- [ ] Implementar testes unitários para Game Engine (95%+ coverage)
- [ ] Implementar testes unitários para componentes React (95%+ coverage)
- [ ] Implementar testes de integração (ASR → LLM → TTS)
- [ ] Implementar testes de integração (Game Engine → Rules5e)
- [ ] Implementar testes de integração (Memory Service → Vectorizer/Nexus/Lexum)
- [ ] Implementar testes E2E (pipeline completo voz→voz)
- [ ] Implementar testes de performance (latências, throughput)
- [ ] Implementar testes de carga (stress testing)
- [ ] Configurar coverage reporting (cargo llvm-cov, vitest coverage)

**Testes Críticos**:
- [ ] Teste de cobertura total (95%+ em todos os módulos)
- [ ] Teste de execução completa (100% dos testes passam)
- [ ] Teste de performance (latências dentro dos targets)
- [ ] Teste de carga (sistema não quebra sob stress)

**Dependências**: Todos os módulos implementados

**Prioridade**: ALTA (garantia de qualidade)

---

### 7.2 Integration Tests
**Task ID**: `implement-integration-tests` ✅ (já existe)

**Descrição**: Implementar testes de integração entre módulos.

**Tarefas**:
- [ ] Implementar testes de integração ASR → Game Engine → LLM
- [ ] Implementar testes de integração LLM → Rules5e → Game Engine
- [ ] Implementar testes de integração LLM → Memory Service
- [ ] Implementar testes de integração Game Engine → Client Electron
- [ ] Implementar testes de integração Client Electron → Serviços
- [ ] Implementar testes de integração Memory Service → Vectorizer/Nexus/Lexum
- [ ] Implementar testes de integração Image Generation → Memory Service
- [ ] Implementar mocks para serviços externos
- [ ] Implementar fixtures de teste (dados de exemplo)

**Testes Críticos**:
- [ ] Teste de pipeline completo (funcionamento end-to-end)
- [ ] Teste de integração entre módulos (comunicação correta)
- [ ] Teste de mocks (isolamento correto)
- [ ] Teste de cobertura (95%+)

**Dependências**: Todos os módulos implementados

**Prioridade**: ALTA (garantia de integração)

---

## Fase 8: Otimização e Performance

### 8.1 Performance Optimization
**Task ID**: `implement-performance-optimizations`

**Descrição**: Implementar otimizações de performance conforme PERFORMANCE.md.

**Tarefas**:
- [ ] Otimizar LLM Core (mmap, mlock, NUMA, KV cache)
- [ ] Otimizar ASR Service (VAD, chunk overlap, incremental decoding)
- [ ] Otimizar TTS Service (streaming, voice cache, ONNX optimization)
- [ ] Otimizar Game Engine (state diffing, event batching, lazy evaluation)
- [ ] Otimizar Memory Service (query caching, batch queries, lazy loading)
- [ ] Otimizar Frontend (virtual scrolling, canvas optimization, RAF)
- [ ] Implementar monitoramento de métricas (latências, throughput)
- [ ] Implementar profiling (identificação de gargalos)
- [ ] Implementar cache agressivo (imagens, queries, cálculos)

**Testes Críticos**:
- [ ] Teste de latência voz→voz (< 300ms, target: 250ms)
- [ ] Teste de latência ASR (< 80ms)
- [ ] Teste de latência LLM (< 200ms)
- [ ] Teste de latência TTS (< 150ms)
- [ ] Teste de FPS frontend (60 FPS constante)
- [ ] Teste de throughput (tokens/s, queries/s)
- [ ] Teste de uso de recursos (CPU, RAM, GPU)

**Dependências**: Todos os módulos implementados

**Prioridade**: ALTA (garantia de performance)

---

### 8.2 Caching System
**Task ID**: `implement-caching-system`

**Descrição**: Implementar sistema de cache agressivo para assets e dados.

**Tarefas**:
- [ ] Implementar cache de imagens geradas (LRU)
- [ ] Implementar cache de queries Memory Service (TTL: 5 minutos)
- [ ] Implementar cache de cálculos Rules5e (dice rolls, combat)
- [ ] Implementar cache de vozes TTS (frases comuns)
- [ ] Implementar cache de modelos (carregamento uma vez)
- [ ] Implementar cache de embeddings (voice embeddings)
- [ ] Implementar invalidação de cache (versionamento)
- [ ] Implementar métricas de cache (hit rate, miss rate)

**Testes Críticos**:
- [ ] Teste de cache de imagens (reutilização correta)
- [ ] Teste de cache de queries (hit rate > 80%)
- [ ] Teste de cache de cálculos (resultados corretos)
- [ ] Teste de invalidação (cache atualizado)
- [ ] Teste de cobertura (95%+)

**Dependências**: Todos os módulos implementados

**Prioridade**: MÉDIA (melhora performance)

---

## Fase 9: Documentação e Deployment

### 9.1 Documentação Completa
**Task ID**: `implement-complete-documentation`

**Descrição**: Completar toda a documentação do projeto.

**Tarefas**:
- [ ] Atualizar README.md com instruções completas
- [ ] Completar ARCHITECTURE.md com detalhes de implementação
- [ ] Completar DESIGN_SYSTEM.md com especificações de UI
- [ ] Completar CONFIGURATION.md com todas as opções
- [ ] Completar TESTING.md com exemplos de testes
- [ ] Completar PERFORMANCE.md com benchmarks
- [ ] Completar DEPLOYMENT.md com instruções de build
- [ ] Criar guias de desenvolvimento (DEVELOPMENT.md)
- [ ] Criar guias de contribuição (CONTRIBUTING.md)
- [ ] Criar CHANGELOG.md com histórico de versões
- [ ] Criar documentação de API (OpenAPI/Swagger)

**Testes Críticos**:
- [ ] Verificar que toda documentação está atualizada
- [ ] Verificar que exemplos de código funcionam
- [ ] Verificar que links estão corretos
- [ ] Verificar que documentação segue padrões do rulebook

**Dependências**: Todas as implementações

**Prioridade**: MÉDIA (importante para manutenção)

---

### 9.2 Build e Deployment
**Task ID**: `implement-build-deployment`

**Descrição**: Implementar sistema de build e deployment do aplicativo Electron.

**Tarefas**:
- [ ] Configurar build do Electron (electron-builder ou similar)
- [ ] Implementar empacotamento de modelos e assets
- [ ] Implementar build multi-plataforma (Windows, Linux, macOS)
- [ ] Implementar geração de installers (MSI, DMG, AppImage)
- [ ] Implementar code signing (Windows, macOS)
- [ ] Implementar auto-updater (opcional)
- [ ] Implementar verificação de integridade (hashes)
- [ ] Implementar distribuição (GitHub Releases, etc.)

**Testes Críticos**:
- [ ] Teste de build (sucesso em todas as plataformas)
- [ ] Teste de empacotamento (todos os assets incluídos)
- [ ] Teste de instalação (installers funcionam)
- [ ] Teste de execução (app funciona após instalação)
- [ ] Teste de integridade (hashes corretos)

**Dependências**: Todas as implementações

**Prioridade**: MÉDIA (necessário para distribuição)

---

## Fase 10: Funcionalidades Avançadas

### 10.1 Adventure Structure System
**Task ID**: `implement-adventure-structure`

**Descrição**: Implementar sistema de estruturação de aventuras (one-shot, mini-campanha, campanha longa).

**Tarefas**:
- [ ] Implementar estrutura de aventura (adventure_id, chapters, scenes)
- [ ] Implementar organização de assets por aventura
- [ ] Implementar sistema de capítulos/atos
- [ ] Implementar sistema de cenas (social, exploração, combate)
- [ ] Implementar integração com Memory Service (indexação)
- [ ] Implementar UI para gerenciamento de aventuras
- [ ] Implementar exportação/importação de aventuras

**Testes Críticos**:
- [ ] Teste de estrutura (organização correta)
- [ ] Teste de assets (associação correta)
- [ ] Teste de integração com Memory Service
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-memory-service`, `implement-image-generation-pipeline`

**Prioridade**: BAIXA (funcionalidade avançada)

---

### 10.2 Synap Integration (Opcional)
**Task ID**: `implement-synap-integration`

**Descrição**: Implementar integração opcional com Synap para conversação multi-modelo.

**Tarefas**:
- [ ] Implementar cliente Synap (Rust)
- [ ] Implementar orquestração de diálogos multi-modelo
- [ ] Implementar integração com LLM Core
- [ ] Implementar configuração opcional (enabled/disabled)
- [ ] Implementar fallback (se Synap não disponível)

**Testes Críticos**:
- [ ] Teste de orquestração (diálogos funcionam)
- [ ] Teste de fallback (funcionamento sem Synap)
- [ ] Teste de cobertura (95%+)

**Dependências**: `implement-llm-core`, Synap configurado

**Prioridade**: BAIXA (opcional)

---

## Resumo de Prioridades

### Prioridade ALTA (Implementar Primeiro)
1. `setup-project-base` - Infraestrutura base
2. `implement-rules5e-service` - Base para game-engine
3. `implement-asr-service` - Crítico para voz
4. `implement-tts-service` - Crítico para voz
5. `implement-memory-service` - Necessário para LLM
6. `implement-llm-core` - Core do sistema
7. `implement-game-engine` - Lógica de jogo
8. `implement-infra-runtime` - Orquestração
9. `implement-client-electron` - Frontend base
10. `implement-renderer-base-components` - Componentes base
11. `implement-battlemap-component` - Componente principal
12. `implement-turn-order-component` - UX essencial
13. `implement-action-bar-component` - Interação essencial
14. `implement-voice-integration` - Pipeline completo
15. `implement-ipc-contracts` - Comunicação
16. `implement-voice-pipeline` - Pipeline completo
17. `implement-testing-suite` - Qualidade
18. `implement-integration-tests` - Integração
19. `implement-performance-optimizations` - Performance

### Prioridade MÉDIA (Implementar Depois)
1. `implement-retractable-menus-component` - Funcionalidades importantes
2. `implement-history-panel-component` - Funcionalidades importantes
3. `implement-dice-rolling-component` - Melhora imersão
4. `implement-image-generation-pipeline` - Melhora imersão
5. `implement-battlemap-generation` - Funcionalidade importante
6. `implement-caching-system` - Melhora performance
7. `implement-complete-documentation` - Manutenção
8. `implement-build-deployment` - Distribuição

### Prioridade BAIXA (Funcionalidades Avançadas)
1. `implement-lora-training-pipeline` - Funcionalidade avançada
2. `implement-preparation-mode` - Funcionalidade avançada
3. `implement-adventure-structure` - Funcionalidade avançada
4. `implement-synap-integration` - Opcional

---

## Critérios de Conclusão

Cada tarefa deve atender aos seguintes critérios antes de ser marcada como completa:

1. **Implementação Completa**:
   - [ ] Todas as subtarefas da lista implementadas
   - [ ] Código segue padrões do AGENTS.md
   - [ ] Código formatado e sem warnings de linter

2. **Testes**:
   - [ ] Cobertura de testes ≥ 95%
   - [ ] Todos os testes críticos implementados e passando
   - [ ] 100% dos testes passam

3. **Documentação**:
   - [ ] Documentação atualizada
   - [ ] Exemplos de código funcionam
   - [ ] CHANGELOG.md atualizado

4. **Quality Checks**:
   - [ ] Type check passa (TypeScript/Rust)
   - [ ] Linter passa sem warnings
   - [ ] Formatter aplicado
   - [ ] Build passa sem erros

5. **Validação**:
   - [ ] `rulebook task validate <task-id>` passa
   - [ ] Especificações atendidas
   - [ ] Performance dentro dos targets

---

## Notas Importantes

- **NUNCA** implementar sem criar task no rulebook primeiro
- **SEMPRE** seguir o workflow do rulebook (proposal.md → tasks.md → spec.md → validate)
- **SEMPRE** executar quality checks antes de commitar
- **SEMPRE** manter cobertura de testes ≥ 95%
- **SEMPRE** atualizar documentação após implementação
- **SEMPRE** seguir as diretrizes do AGENTS.md

---

Este documento serve como **master checklist** para implementação completa do VRPG Client. Cada tarefa deve ser criada no rulebook antes da implementação, seguindo o formato OpenSpec-compatible.

---

## Fase 11: Sistema D&D 5e Completo

**Nota**: Esta seção referencia o documento completo de tasks do sistema D&D 5e.

**Data de Criação**: 2025-11-23  
**Baseado em**: Regras oficiais D&D 5e (Livro do Jogador, Guia do Mestre, Manual dos Monstros)  
**Status Atual**: Estrutura base implementada, expandindo para cobertura completa

### Status de Implementação Atual

#### ✅ Implementado
- **Dice Rolling**: Rolagem de dados com advantage/disadvantage
- **Attack Resolution**: Resolução de ataques com críticos
- **Damage Calculation**: Cálculo de dano com resistências/vulnerabilidades
- **Ability Checks**: Testes de habilidade com proficiência
- **Saving Throws**: Salvaguardas
- **Conditions**: Sistema básico de condições

#### ⚠️ Parcialmente Implementado
- **Game Engine**: Estrutura básica (sessões, cenas, atores, turnos) - **Nota**: Será refatorado para trabalhar com Orquestrador
- **Memory Service**: Integração com Vectorizer/Nexus/Lexum
- **Orquestrador**: Módulo central de coordenação (em planejamento)
- **INTENT DSL**: Sistema de intenções estruturadas (em planejamento)
- **Turn Engine**: Sistema completo de combate em turnos (em planejamento)
- **Voice INTENTS**: Sistema de intenções de voz (em planejamento)

#### ❌ Não Implementado
- **Character Creation**: Criação completa de personagens
- **Weapons & Equipment**: Tabelas de armas e equipamentos
- **Races & Classes**: Raças e classes completas
- **Spells System**: Sistema completo de magias
- **Monsters**: Sistema completo de monstros
- **XP & Leveling**: Sistema de experiência e níveis
- **Combat System**: Sistema completo de combate
- **Skills System**: Sistema completo de perícias
- **Feats**: Talentos e melhorias
- **Backgrounds**: Antecedentes
- **Equipment Management**: Gerenciamento de equipamentos
- **Inventory System**: Sistema de inventário
- **Spellcasting**: Sistema completo de lançamento de magias
- **Rest & Recovery**: Descanso e recuperação
- **Travel & Exploration**: Viagem e exploração
- **Social Encounters**: Encontros sociais
- **Environmental Effects**: Efeitos ambientais

**Para ver a lista completa e detalhada de todas as tasks do sistema D&D 5e (20 fases, 80+ tasks), consulte**: [TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md)

**Resumo**: O documento completo contém 20 fases com 80+ tasks, 150+ endpoints planejados, e 800+ testes necessários, cobrindo desde sistema de personagem até design de dungeons e aventuras.








