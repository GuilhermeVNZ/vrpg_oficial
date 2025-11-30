# Fases Pendentes da Implementação E2E (Backend)

## Resumo Executivo

**Pipeline de 3 Agentes**: ✅ **CONCLUÍDO** (M1-M6)
**Progresso Geral Backend**: ~30% (Pipeline completo, serviços core parciais)

---

## ✅ Fases Concluídas

### Pipeline de 3 Agentes (M1-M6)
- ✅ M1: Preparação e Infraestrutura
- ✅ M2: Orquestrador - Pipeline de 3 Agentes
- ✅ M3: Orquestrador - Respostas Objetivas
- ✅ M4: Cache e Estado (incluindo Persistência de Sessão)
- ✅ M5: Testes de Integração, Performance e Regressão
- ✅ M6: Documentação e Deploy

---

## 🔄 Fases Pendentes (Backend)

### Fase 0: Infraestrutura Base

#### 0.1 Setup CI/CD e Qualidade
**Status**: 🔄 PENDENTE  
**Prioridade**: MÉDIA

**Tarefas Principais**:
- [ ] Configurar GitHub Actions workflows
- [ ] Configurar codespell para verificação de typos
- [ ] Configurar security audit (cargo audit, npm audit)
- [ ] Configurar build multi-plataforma (Windows, Linux, macOS)
- [ ] Configurar publicação automática de releases

---

### Fase 1: Serviços Core (Rust)

#### 1.1 Rules5e Service
**Status**: ✅ Estrutura criada, 🔄 Implementação pendente  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Parser de expressões de dados (`2d8+3`)
- [ ] Rolagem de dados com seed controlável
- [ ] Cálculo de ataques (hit/miss, AC)
- [ ] Cálculo de dano (tipos, resistências)
- [ ] Testes de habilidade (ability checks)
- [ ] Salvaguardas (saving throws)
- [ ] Condições (poisoned, stunned, etc.)
- [ ] Sistema de magias básico (SRD)
- [ ] HTTP server (localhost:7004)
- [ ] Endpoints: `/health`, `/roll`, `/attack`, `/ability-check`, `/saving-throw`

**Impacto**: Base para game-engine e combate

---

#### 1.2 ASR Service
**Status**: ✅ Estrutura criada (Whisper via Python bridge)  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Integração completa com Whisper (atualmente via Python bridge)
- [ ] VAD (Voice Activity Detection) robusto
- [ ] Processamento de chunks (320ms)
- [ ] Transcrição incremental (streaming)
- [ ] HTTP server completo (localhost:7001)
- [ ] Endpoints: `/health`, `/transcribe_chunk`, `/transcribe_final`
- [ ] Cache de transcrições frequentes
- [ ] Migração para usar Synap completamente (opcional)

**Impacto**: Crítico para pipeline voz→voz

---

#### 1.3 TTS Service
**Status**: ✅ Estrutura existe (XTTS + SoVITS)  
**Prioridade**: ALTA (já implementado, precisa integração completa)

**Tarefas Principais**:
- [ ] Integração completa com pipeline de 3 agentes
- [ ] Endpoints HTTP completos
- [ ] Testes de latência e qualidade
- [ ] Migração para usar Synap completamente (opcional)

**Impacto**: Crítico para pipeline voz→voz

---

#### 1.4 LLM Core
**Status**: ✅ Pipeline de 2 modelos implementado (via Synap)  
**Prioridade**: ALTA (implementado, precisa otimizações)

**Tarefas Principais**:
- [ ] Otimizações de latência
- [ ] Gerenciamento de memória avançado
- [ ] KV cache para contexto
- [ ] Streaming de tokens
- [ ] Integração completa com Memory Service

**Impacto**: Core do sistema

---

#### 1.5 Memory Service
**Status**: ✅ Estrutura criada, 🔄 Implementação pendente  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Integração com Vectorizer (embeddings)
- [ ] Integração com Nexus (graph relations)
- [ ] Integração com Lexum (full-text search)
- [ ] Integração com Transmutation (conversão de documentos)
- [ ] Integração com Classify (categorização)
- [ ] Sistema de escopos (global, campaign, session, actor)
- [ ] Inserção de memórias
- [ ] Busca semântica (pipeline completo)
- [ ] Consolidação de memórias antigas
- [ ] HTTP server (localhost:7005)
- [ ] Endpoints: `/health`, `/insert`, `/search`
- [ ] Cache de queries frequentes

**Impacto**: Necessário para LLM Core e narrativa contextual

---

#### 1.6 Infra Runtime
**Status**: ✅ Estrutura criada, 🔄 Implementação pendente  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Inicialização de serviços (spawn de processos)
- [ ] Health-check periódico de todos os serviços
- [ ] Retry/backoff para serviços que falham
- [ ] Graceful shutdown de todos os serviços
- [ ] Sistema de configuração centralizado
- [ ] Logging estruturado (por serviço)
- [ ] Métricas agregadas (latências, uso de recursos)
- [ ] Tolerância a falhas (modos de degradação):
  - Modo 1: ASR falha → usar texto manual
  - Modo 2: TTS falha → usar texto na tela
  - Modo 3: 1.5B falha → pular prelúdio, ir direto para 14B
  - Modo 4: 14B falha → usar resposta genérica do 1.5B
  - Modo 5: Memory Service falha → usar cache local apenas
- [ ] Notificação ao usuário de degradação
- [ ] Recuperação automática quando componente volta
- [ ] Verificação de integridade de assets

**Impacto**: Necessário para funcionamento completo e robusto

---

### Fase 2: Orquestrador e INTENT DSL

#### 2.1 Orquestrador Base (Integração Completa)
**Status**: 🔄 PENDENTE (pipeline feito, falta integração completa)  
**Prioridade**: CRÍTICA

**Tarefas Principais**:
- [x] Pipeline de 3 agentes (CONCLUÍDO)
- [ ] Integração completa com Rules5e Service
- [ ] Integração completa com Memory Service
- [ ] Comunicação IPC/WebSocket com Electron
- [ ] Testes de integração completos

**Impacto**: Base da nova arquitetura

---

#### 2.2 Intent Validation System
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Sistema de validação de INTENTs contra game state
- [ ] Validação de cada tipo de INTENT
- [ ] Mensagens de erro amigáveis
- [ ] Testes completos

**Impacto**: Evita ações inválidas durante jogo

---

#### 2.3 Intent Router (Melhorias)
**Status**: ✅ Implementado, 🔄 Melhorias pendentes  
**Prioridade**: MÉDIA

**Tarefas Principais**:
- [ ] Melhorar precisão de classificação
- [ ] Adicionar mais padrões de regex
- [ ] Treinamento de modelo ML (opcional)
- [ ] Cache de classificações frequentes (já implementado)

**Impacto**: Melhora roteamento de intenções

---

### Fase 3: Game Engine

#### 3.1 Game Engine Base
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Refatorar para trabalhar com Orquestrador
- [ ] Estrutura `GameSession`
- [ ] Estrutura `Scene`
- [ ] Estrutura `Actor`
- [ ] Sistema de aplicação de dano
- [ ] Sistema de condições
- [ ] Event Bus interno
- [ ] Integração com Orquestrador

**Impacto**: Core da lógica de jogo

---

### Fase 4: Modos de Cena e Turn Engine

#### 4.1 Modos de Cena (FSM)
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Modo SocialFreeFlow (diálogo livre)
- [ ] Modo Exploration (exploração livre)
- [ ] Modo CombatTurnBased (combate em turnos)
- [ ] Modo DowntimePreparation (preparação)
- [ ] Transições entre modos
- [ ] UI adaptada por modo (backend informa tipo de UI)

**Impacto**: Define modos de jogo

---

#### 4.2 Turn Engine (Combate em Turnos)
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Sistema de iniciativa
- [ ] Ordem de turnos
- [ ] Ações de turno (Action, Bonus Action, Reaction)
- [ ] Sistema de movimento
- [ ] Integração com Rules5e Service
- [ ] Integração com Game Engine

**Impacto**: Core do combate em turnos

---

### Fase 5-6: Sistema D&D 5e Completo

#### 5.1 Sistema de Personagens
**Status**: 🔄 PENDENTE  
**Prioridade**: CRÍTICA

**Tarefas Principais**:
- [ ] Ficha de personagem completa
- [ ] Atributos e modificadores
- [ ] Habilidades (Skills)
- [ ] Perícias (Proficiencies)
- [ ] Equipamentos e itens
- [ ] Sistema de níveis e XP
- [ ] Classes e raças (SRD)

**Impacto**: Base para jogo D&D 5e

---

#### 5.2 Sistema de Combate
**Status**: 🔄 PENDENTE  
**Prioridade**: CRÍTICA

**Tarefas Principais**:
- [ ] Sistema de HP e dano
- [ ] Sistema de AC e armadura
- [ ] Sistema de movimento
- [ ] Sistema de ações (Action, Bonus Action, Reaction)
- [ ] Sistema de iniciativa
- [ ] Sistema de Death Saves

**Impacto**: Core do combate

---

#### 5.3 Sistema de Magias
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Spell Database (SRD completo)
- [ ] Spell Slots Management
- [ ] Spell Casting
- [ ] Spell Components (V, S, M)
- [ ] Spell Concentration
- [ ] Spell Duration
- [ ] Spell Areas of Effect
- [ ] Spell Saving Throws

**Impacto**: Sistema de magias completo

---

#### 5.4 Sistema de Monstros
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Monster Database (SRD completo)
- [ ] Monster Stat Blocks
- [ ] Monster Abilities
- [ ] Monster Actions
- [ ] Monster Legendary Actions
- [ ] Monster Lair Actions

**Impacto**: NPCs e inimigos no jogo

---

### Fase 7: Integração e Pipeline

#### 7.1 IPC and API Contracts
**Status**: 🔄 PENDENTE  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Tipos compartilhados (TypeScript ↔ Rust)
- [ ] Validação de mensagens
- [ ] Versionamento de API
- [ ] Documentação de contratos
- [ ] Testes de contratos

**Impacto**: Comunicação frontend-backend

---

#### 7.2 Pipeline Voz → Voz (Integração Completa)
**Status**: 🔄 PENDENTE (pipeline base feito, falta integração E2E)  
**Prioridade**: CRÍTICA

**Tarefas Principais**:
- [ ] Integração completa ASR → Orquestrador → 1.5B → 14B → TTS
- [ ] Otimizações de latência end-to-end
- [ ] Tratamento de erros robusto
- [ ] Métricas de pipeline completas
- [ ] Testes end-to-end reais
- [ ] Teste de latência total < 6s

**Impacto**: Pipeline completo de voz

---

### Fase 8: Assets e Geração

#### 8.1 Image Generation Pipeline
**Status**: 🔄 PENDENTE  
**Prioridade**: MÉDIA

**Tarefas Principais**:
- [ ] Integração Flux.1
- [ ] Geração de retratos
- [ ] Geração de cenas
- [ ] Geração de battlemaps
- [ ] Cache de imagens

**Impacto**: Assets visuais para jogo

---

#### 8.2 LoRA Training Pipeline
**Status**: 🔄 PENDENTE  
**Prioridade**: BAIXA

**Tarefas Principais**:
- [ ] Coleta de datasets
- [ ] Treinamento de embeddings
- [ ] Treinamento de LoRAs
- [ ] Validação

**Impacto**: Personalização de modelos

---

### Fase 9: Testes e Qualidade

#### 9.1 Test Suite Completo
**Status**: 🔄 PENDENTE (testes do pipeline feitos)  
**Prioridade**: ALTA

**Tarefas Principais**:
- [ ] Testes unitários completos (95%+ coverage)
- [ ] Testes de integração entre serviços
- [ ] Testes E2E completos
- [ ] Testes de performance
- [ ] Testes de carga
- [ ] Testes de stress

**Impacto**: Qualidade e confiabilidade

---

## Priorização Recomendada

### Crítica (Fazer Agora)
1. **Rules5e Service** (1.1) - Base para tudo
2. **Memory Service** (1.5) - Necessário para LLM
3. **Game Engine Base** (3.1) - Core do jogo
4. **Sistema de Personagens** (5.1) - Base D&D 5e
5. **Pipeline Voz → Voz E2E** (7.2) - Integração completa

### Alta (Próximos)
1. **Infra Runtime** (1.6) - Robustez e degradação
2. **ASR Service** (1.2) - Completar integração
3. **Modos de Cena** (4.1) - Definição de modos
4. **Turn Engine** (4.2) - Combate em turnos
5. **Sistema de Combate** (5.2) - Core combate

### Média (Depois)
1. **Intent Validation** (2.2) - Validação
2. **Sistema de Magias** (5.3) - Magias
3. **Sistema de Monstros** (5.4) - NPCs
4. **IPC Contracts** (7.1) - Frontend integration
5. **Image Generation** (8.1) - Assets

### Baixa (Futuro)
1. **CI/CD** (0.1) - Automação
2. **LoRA Training** (8.2) - Personalização

---

## Estatísticas

### Concluído
- ✅ Pipeline de 3 Agentes: 100%
- ✅ Orquestrador Base (pipeline): 100%
- ✅ Caches e Estado: 100%
- ✅ Testes do Pipeline: 100%

### Em Progresso
- 🔄 Services Core: ~40%
- 🔄 Game Engine: ~20%
- 🔄 Sistema D&D 5e: ~10%

### Pendente
- ⏳ Integração E2E: 0%
- ⏳ Modos de Cena: 0%
- ⏳ Sistema Completo: 0%

---

## Próximos Passos Recomendados

1. **Completar Rules5e Service** - Base para tudo
2. **Completar Memory Service** - Necessário para LLM
3. **Refatorar Game Engine** - Core do jogo
4. **Implementar Sistema de Personagens** - Base D&D 5e
5. **Integrar Pipeline Voz → Voz E2E** - Testar tudo junto

---

**Última Atualização**: 2025-01-XX  
**Pipeline de 3 Agentes**: ✅ **100% CONCLUÍDO**













