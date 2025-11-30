# VRPG — Plano de Migração Completo para Nova Arquitetura

Este documento descreve o plano completo de migração do sistema VRPG atual para a nova arquitetura baseada em:

- **Orquestrador** (coordenador central em Rust)
- **INTENT DSL** (sistema de intenções estruturadas)
- **Modos de Cena** (SocialFreeFlow, Exploration, CombatTurnBased, DowntimePreparation)
- **Mindset do Mestre IA** (narrativa pura, sem cálculos)
- **Agentes Jogadores IA** (companheiros de party)
- **Fluxo de Combate** (turn-based, narração por ação)
- **Pipeline Visual** (Flux + LoRA, estilo Vox Machina)
- **Pipeline de Áudio** (local, low-latency, zero-API, Voice INTENTS)
- **Engine de Regras** (determinística, condições AUTO)
- **Turn Engine** (combate em turnos, rolagens client vs servidor)

---

## 1. Análise de Impacto

### 1.1 Componentes Afetados

#### ✅ Mantidos (com ajustes)

- `rules5e-service`: Mantido, mas agora integrado via Orquestrador
- `asr-service`: Mantido, integração com Orquestrador
- `tts-service`: Mantido, integração com Orquestrador + Voice INTENTS
- `memory-service`: Mantido, integração com Orquestrador
- `client-electron`: Mantido, mas UI adaptada para novos modos de cena

#### 🔄 Refatorados

- `game-engine`: Refatorado para trabalhar com Orquestrador
- `llm-core`: Refatorado para gerar INTENT DSL ao invés de JSON
- Comunicação entre serviços: Migrada para protocolo do Orquestrador

#### ➕ Novos Componentes

- `orchestrator`: Novo módulo central em Rust
- Parser de INTENT DSL: Novo módulo em Rust
- Sistema de modos de cena: Novo FSM no Orquestrador
- Art Daemon: Novo serviço para geração de assets (Flux + LoRA)
- Turn Engine: Sistema completo de combate em turnos

---

## 2. Fases de Migração

### Fase 1: Fundação (Orquestrador + INTENT DSL)

**Objetivo**: Criar a base do novo sistema sem quebrar o existente.  
**Duração Estimada**: 4-5 semanas

#### Task 1.1: Criar Módulo Orquestrador Base

**ID**: `migration-1-1`  
**Prioridade**: CRÍTICA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Criar `src/orchestrator/` com estrutura de projeto Rust
- [ ] Implementar `fsm.rs` com máquina de estados de cena:
  - [ ] Enum `SceneState` (SocialFreeFlow, Exploration, CombatTurnBased, DowntimePreparation)
  - [ ] Transições entre estados
  - [ ] Validação de transições
- [ ] Implementar `session.rs`:
  - [ ] Estrutura `GameSession`
  - [ ] Gerenciamento de estado de sessão
  - [ ] Persistência de sessão
- [ ] Implementar `communication.rs`:
  - [ ] Interface IPC (Electron ↔ Rust)
  - [ ] Interface WebSocket (alternativa)
  - [ ] Serialização de mensagens
- [ ] Testes unitários do FSM
- [ ] Testes de comunicação

**Critérios de Aceitação**:
- Orquestrador compila sem erros
- FSM funciona corretamente
- Comunicação IPC/WebSocket estabelecida
- Testes passam (100%)

**Dependências**: Nenhuma

---

#### Task 1.2: Implementar Parser de INTENT DSL

**ID**: `migration-1-2`  
**Prioridade**: CRÍTICA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Criar `intent_parser.rs`
- [ ] Implementar gramática simplificada:
  - [ ] Parser de blocos `[INTENTS] ... [/INTENTS]`
  - [ ] Parser de INTENTs individuais
  - [ ] Parser de campos KEY: VALUE
- [ ] Implementar enum `Intent` com todas as variantes:
  - [ ] SkillCheck
  - [ ] MeleeAttack
  - [ ] RangedAttack
  - [ ] SpellCast
  - [ ] LoreQuery
  - [ ] RuleQuery
  - [ ] GeneratePortrait
  - [ ] GenerateScene
  - [ ] GenerateBattlemap
  - [ ] CombatStart
  - [ ] CombatEnd
  - [ ] (outras conforme necessário)
- [ ] Implementar normalização:
  - [ ] Trim whitespace
  - [ ] Remover aspas redundantes
  - [ ] Inferência de valores padrão
- [ ] Implementar validação:
  - [ ] Validação de IDs
  - [ ] Validação de contexto (ex: MELEE_ATTACK só em combate)
- [ ] Tratamento de erros:
  - [ ] Erros de parsing
  - [ ] Erros de validação
  - [ ] Fallbacks
- [ ] Testes extensivos:
  - [ ] Testes de parsing de cada tipo de INTENT
  - [ ] Testes de edge cases
  - [ ] Testes de validação
  - [ ] Testes de normalização

**Critérios de Aceitação**:
- Parser funciona com 100% de precisão em casos de teste
- Todos os tipos de INTENT são suportados
- Validação funciona corretamente
- Erros são tratados graciosamente
- Testes passam (100%)

**Dependências**: Nenhuma

---

#### Task 1.3: Implementar Executor de INTENTs

**ID**: `migration-1-3`  
**Prioridade**: CRÍTICA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Criar `intent_executor.rs`
- [ ] Implementar execução de cada tipo de INTENT:
  - [ ] SkillCheck → RollRequest para UI
  - [ ] MeleeAttack → chamada a rules5e-service
  - [ ] RangedAttack → chamada a rules5e-service
  - [ ] SpellCast → chamada a rules5e-service
  - [ ] LoreQuery → chamada a memory-service (Vectorizer/Lexum/Nexus)
  - [ ] RuleQuery → chamada a memory-service
  - [ ] GeneratePortrait → chamada a Art Daemon
  - [ ] GenerateScene → chamada a Art Daemon
  - [ ] GenerateBattlemap → chamada a Art Daemon
  - [ ] CombatStart → transição para CombatTurnBased
  - [ ] CombatEnd → transição para SocialFreeFlow/Exploration
- [ ] Integração com `rules5e-service`:
  - [ ] Cliente HTTP para rules5e-service
  - [ ] Tratamento de respostas
  - [ ] Tratamento de erros
- [ ] Integração com `memory-service`:
  - [ ] Cliente para Hive stack
  - [ ] Tratamento de respostas
  - [ ] Cache de consultas
- [ ] Integração com Art Daemon (futuro):
  - [ ] Interface para geração de assets
  - [ ] Fila de geração
- [ ] Testes de integração:
  - [ ] Testes com services mockados
  - [ ] Testes de execução de cada INTENT

**Critérios de Aceitação**:
- Todas as INTENTs são executadas corretamente
- Integrações com services funcionam
- Erros são tratados graciosamente
- Testes passam (100%)

**Dependências**: Task 1.2, `rules5e-service`, `memory-service`

---

#### Task 1.4: Atualizar LLM Core para Gerar INTENT DSL

**ID**: `migration-1-4`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Atualizar prompts do Mestre IA:
  - [ ] Adicionar exemplos de INTENT DSL
  - [ ] Instruções sobre quando gerar INTENTs
  - [ ] Formato esperado
- [ ] Atualizar prompts de Jogadores IA:
  - [ ] Remover geração de INTENTs (jogadores não geram)
  - [ ] Foco em roleplay puro
- [ ] Modificar processamento de resposta:
  - [ ] Extrair blocos `[INTENTS] ... [/INTENTS]`
  - [ ] Separar narração de INTENTs
  - [ ] Validar INTENTs antes de enviar ao Orquestrador
- [ ] Testes:
  - [ ] Testes de geração de INTENTs
  - [ ] Testes de validação
  - [ ] Testes de fallback quando parsing falha

**Critérios de Aceitação**:
- LLM gera INTENTs válidas em formato DSL
- Narração e INTENTs são separadas corretamente
- Fallbacks funcionam quando parsing falha
- Testes passam (100%)

**Dependências**: Task 1.2

---

### Fase 2: Modos de Cena e Fluxos

**Objetivo**: Implementar os 4 modos de cena e seus fluxos específicos.  
**Duração Estimada**: 6-8 semanas

#### Task 2.1: Implementar Modo SocialFreeFlow

**ID**: `migration-2-1`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Implementar estado SocialFreeFlow no FSM
- [ ] UI adaptada:
  - [ ] Remover grid do battlemap
  - [ ] Focar em retratos/ambiente
  - [ ] Cards de participantes (topo)
  - [ ] Histórico de diálogo (direita)
- [ ] Fluxo de diálogo:
  - [ ] Jogador fala → ASR → LLM → narração + INTENTs (se necessário)
  - [ ] INTENTs apenas para SKILL_CHECK, LORE_QUERY, etc.
  - [ ] Sem INTENTs de combate
- [ ] Testes:
  - [ ] Testes de fluxo social
  - [ ] Testes de geração de INTENTs em contexto social
  - [ ] Testes de UI

**Critérios de Aceitação**:
- Modo SocialFreeFlow funciona corretamente
- UI adaptada para modo social
- INTENTs são geradas apenas quando necessário
- Testes passam (100%)

**Dependências**: Task 1.1, Task 1.2, Task 1.3

---

#### Task 2.2: Implementar Modo Exploration

**ID**: `migration-2-2`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Implementar estado Exploration no FSM
- [ ] Sistema de movimento livre:
  - [ ] Movimento sem grid
  - [ ] Detecção de áreas/interações
- [ ] Perception checks automáticos:
  - [ ] Checks passivos
  - [ ] Checks ativos (quando jogador investiga)
- [ ] Triggers de emboscada:
  - [ ] Detecção de encontros
  - [ ] Transição para combate
- [ ] Testes:
  - [ ] Testes de exploração
  - [ ] Testes de perception checks
  - [ ] Testes de triggers

**Critérios de Aceitação**:
- Modo Exploration funciona corretamente
- Movimento livre implementado
- Perception checks funcionam
- Triggers de combate funcionam
- Testes passam (100%)

**Dependências**: Task 1.1, Task 2.1

---

#### Task 2.3: Implementar Modo CombatTurnBased (Turn Engine)

**ID**: `migration-2-3`  
**Prioridade**: CRÍTICA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Implementar estado CombatTurnBased no FSM
- [ ] Sistema de iniciativa:
  - [ ] Cálculo de iniciativa (1d20 + DEX_MOD)
  - [ ] Ordenação de participantes
  - [ ] UI de ordem de turno (cards BG3-like)
- [ ] Sistema de rolagens (client vs servidor):
  - [ ] RollRequest para jogadores (client-side)
  - [ ] RollResult de jogadores (validação opcional)
  - [ ] Rolagens de NPCs (servidor/engine)
- [ ] UI de combate:
  - [ ] Battlemap com grid
  - [ ] Tokens no mapa
  - [ ] Barra de ações (inferior)
  - [ ] Log de combate (direita)
  - [ ] Destaque do turno ativo
- [ ] Economia de ações:
  - [ ] Ação (1 por turno)
  - [ ] Movimento (1 por turno)
  - [ ] Reação (condicional)
  - [ ] Bonus Action (se aplicável)
  - [ ] Tracking de uso
- [ ] Narração por ação:
  - [ ] Cada ação gera narração separada
  - [ ] Não narra "turno completo"
  - [ ] Narração após resolução mecânica
- [ ] Integração com Engine:
  - [ ] Resolução de ataques
  - [ ] Resolução de magias
  - [ ] Aplicação de condições
  - [ ] Cálculo de dano
  - [ ] Line of Sight (LoS) e alcance
  - [ ] Áreas de Efeito (AoE)
- [ ] Avanço de iniciativa:
  - [ ] Algoritmo de avanço
  - [ ] Detecção de fim de combate
  - [ ] Notificações de novo turno/round
- [ ] Testes:
  - [ ] Testes de iniciativa
  - [ ] Testes de rolagens (client vs servidor)
  - [ ] Testes de economia de ações
  - [ ] Testes de resolução de combate
  - [ ] Testes de narração por ação
  - [ ] Testes de LoS e alcance
  - [ ] Testes de AoE

**Critérios de Aceitação**:
- Modo CombatTurnBased funciona corretamente
- Iniciativa calculada e ordenada
- Rolagens client vs servidor funcionam
- Economia de ações respeitada
- Narração por ação implementada
- UI de combate completa
- LoS e alcance funcionam
- AoE funciona corretamente
- Testes passam (100%)

**Dependências**: Task 1.1, Task 1.3, `rules5e-service`, `COMBAT_FLOW.md`

---

#### Task 2.4: Implementar Modo DowntimePreparation

**ID**: `migration-2-4`  
**Prioridade**: MÉDIA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Implementar estado DowntimePreparation no FSM
- [ ] Fila de jobs:
  - [ ] Jobs para GPU (geração de imagens)
  - [ ] Jobs para CPU (treino de LoRA, processamento)
  - [ ] Priorização de jobs
- [ ] Geração de assets pesados:
  - [ ] Battlemaps complexos
  - [ ] Retratos completos
  - [ ] Cenas chave
- [ ] Treino de LoRAs:
  - [ ] Identificação de personagens recorrentes
  - [ ] Criação de datasets
  - [ ] Treino de LoRAs
- [ ] Atualização de memória:
  - [ ] Indexação de eventos da sessão
  - [ ] Atualização de Hive
- [ ] Testes:
  - [ ] Testes de fila de jobs
  - [ ] Testes de geração de assets
  - [ ] Testes de treino de LoRA

**Critérios de Aceitação**:
- Modo DowntimePreparation funciona corretamente
- Fila de jobs implementada
- Assets são gerados corretamente
- LoRAs são treinadas corretamente
- Memória é atualizada
- Testes passam (100%)

**Dependências**: Task 1.1, Task 4.1 (Pipeline Visual)

---

### Fase 3: Mindset do Mestre IA e Agentes

**Objetivo**: Implementar o comportamento correto do Mestre IA e Jogadores IA.  
**Duração Estimada**: 4-5 semanas

#### Task 3.1: Atualizar Prompts do Mestre IA

**ID**: `migration-3-1`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Incorporar `DM_MINDSET.md` nos prompts:
  - [ ] Identidade do Mestre
  - [ ] Filosofia central
  - [ ] Estilo narrativo
  - [ ] Os três modos do VRPG
  - [ ] INTENTs (palavra-chave sagrada)
  - [ ] Limites (nunca calcular, nunca explicar mecânica)
- [ ] Remover qualquer lógica de cálculo:
  - [ ] Remover prompts que ensinam regras
  - [ ] Remover prompts que ensinam cálculos
  - [ ] Focar apenas em narração
- [ ] Ensinar geração de INTENTs:
  - [ ] Quando gerar INTENTs
  - [ ] Como estruturar INTENTs
  - [ ] Exemplos de INTENTs corretas
- [ ] Testes:
  - [ ] Testes de narração (sem números)
  - [ ] Testes de geração de INTENTs
  - [ ] Testes de consulta a Hive

**Critérios de Aceitação**:
- Mestre IA nunca calcula regras
- Mestre IA apenas narra
- INTENTs são geradas corretamente
- Consultas a Hive funcionam
- Testes passam (100%)

**Dependências**: Task 1.4, `DM_MINDSET.md`

---

#### Task 3.2: Implementar Sistema de Agentes Jogadores IA

**ID**: `migration-3-2`  
**Prioridade**: MÉDIA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Estrutura para múltiplos agentes:
  - [ ] Enum de tipos de agente
  - [ ] Gerenciamento de agentes ativos
  - [ ] Switching entre agentes
- [ ] Personalidades persistentes:
  - [ ] Estrutura de personalidade
  - [ ] Persistência de personalidade
  - [ ] Aplicação de personalidade nos prompts
- [ ] Sistema de memória por agente:
  - [ ] Memória de eventos importantes
  - [ ] Memória de relacionamentos
  - [ ] Memória de decisões
- [ ] Integração com prompts:
  - [ ] Incorporar `CHARACTER_AGENTS.md`
  - [ ] Prompts específicos por arquétipo
  - [ ] Remover geração de INTENTs (jogadores não geram)
- [ ] Testes:
  - [ ] Testes de agentes
  - [ ] Testes de personalidades
  - [ ] Testes de memória

**Critérios de Aceitação**:
- Sistema de agentes funciona
- Personalidades são persistentes
- Memória por agente funciona
- Agentes se comportam como companheiros de mesa
- Testes passam (100%)

**Dependências**: Task 1.4, `CHARACTER_AGENTS.md`

---

#### Task 3.3: Integração com Hive para Lore/Regras

**ID**: `migration-3-3`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] INTENT LORE_QUERY implementada:
  - [ ] Parsing da INTENT
  - [ ] Execução (chamada a Hive)
  - [ ] Retorno de resultados
  - [ ] Integração com prompts do Mestre
- [ ] INTENT RULE_QUERY implementada:
  - [ ] Parsing da INTENT
  - [ ] Execução (chamada a Hive)
  - [ ] Retorno de resultados
  - [ ] Integração com prompts do Mestre
- [ ] Pré-inject de lore:
  - [ ] Detecção de cenas importantes
  - [ ] Busca prévia de lore relevante
  - [ ] Injeção no contexto do Mestre
- [ ] Testes:
  - [ ] Testes de LORE_QUERY
  - [ ] Testes de RULE_QUERY
  - [ ] Testes de pré-inject

**Critérios de Aceitação**:
- LORE_QUERY funciona corretamente
- RULE_QUERY funciona corretamente
- Pré-inject funciona em cenas importantes
- Testes passam (100%)

**Dependências**: Task 1.3, `memory-service`

---

### Fase 4: Pipeline Visual (Flux + LoRA)

**Objetivo**: Implementar sistema completo de geração visual.  
**Duração Estimada**: 6-8 semanas

#### Task 4.1: Setup Art Daemon

**ID**: `migration-4-1`  
**Prioridade**: MÉDIA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Criar serviço Art Daemon:
  - [ ] Estrutura base em Rust ou Python
  - [ ] Interface HTTP/GRPC
  - [ ] Integração com ComfyUI (headless)
- [ ] Integração com Flux:
  - [ ] Carregamento de modelo Flux
  - [ ] Configuração de geração
  - [ ] Otimizações de performance
- [ ] Sistema de cache:
  - [ ] Cache de assets gerados
  - [ ] Lookup rápido
  - [ ] Invalidação de cache
- [ ] API para geração:
  - [ ] `generate_portrait(character, emotion)`
  - [ ] `generate_scene(description, style)`
  - [ ] `generate_battlemap(layout, style)`
- [ ] Testes:
  - [ ] Testes básicos de geração
  - [ ] Testes de cache
  - [ ] Testes de performance

**Critérios de Aceitação**:
- Art Daemon funciona
- Integração com Flux funciona
- Cache funciona corretamente
- API exposta corretamente
- Testes passam (100%)

**Dependências**: Nenhuma (pode ser desenvolvido em paralelo)

---

#### Task 4.2: Implementar Sistema de LoRA

**ID**: `migration-4-2`  
**Prioridade**: MÉDIA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Estrutura de datasets:
  - [ ] Estrutura de pastas (personagem, estilo)
  - [ ] Curadoria de imagens
  - [ ] Anotações (meta/notes.md)
- [ ] Pipeline de treino:
  - [ ] Integração com ComfyUI
  - [ ] Configuração de treino (rank, alpha, epochs)
  - [ ] Treino de LoRA de estilo (global)
  - [ ] Treino de LoRA de personagem
- [ ] Sistema de carregamento:
  - [ ] Carregamento de LoRAs no runtime
  - [ ] Combinação de LoRAs (estilo + personagem)
  - [ ] Limite de 3 LoRAs por prompt
- [ ] Testes:
  - [ ] Testes de treino de LoRA
  - [ ] Testes de carregamento
  - [ ] Testes de consistência visual

**Critérios de Aceitação**:
- Estrutura de datasets implementada
- Pipeline de treino funciona
- LoRAs são carregadas corretamente
- Consistência visual mantida
- Testes passam (100%)

**Dependências**: Task 4.1, `LORA_GUIDELINES.md`

---

#### Task 4.3: Geração de Assets

**ID**: `migration-4-3`  
**Prioridade**: MÉDIA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Retratos:
  - [ ] Geração com LoRA de personagem
  - [ ] Múltiplas emoções (neutral, angry, determined, etc.)
  - [ ] Consistência visual
- [ ] Cenas sociais:
  - [ ] Geração sem grid
  - [ ] Estilo Vox Machina
  - [ ] Atmosfera narrativa
- [ ] Battlemaps:
  - [ ] Perspectiva isométrica
  - [ ] Grid 5ft sutil
  - [ ] Legibilidade tática
  - [ ] Props e obstáculos
- [ ] Eventos/sprites:
  - [ ] Slash effects
  - [ ] Fire effects
  - [ ] Ice effects
  - [ ] Transparent background
- [ ] Integração com Orquestrador:
  - [ ] INTENT GENERATE_PORTRAIT
  - [ ] INTENT GENERATE_SCENE
  - [ ] INTENT GENERATE_BATTLEMAP
  - [ ] Fila de geração (downtime vs runtime)
- [ ] Testes:
  - [ ] Testes de geração de cada tipo
  - [ ] Testes de consistência
  - [ ] Testes de integração

**Critérios de Aceitação**:
- Todos os tipos de assets são gerados
- Consistência visual mantida
- Battlemaps são taticamente legíveis
- Integração com Orquestrador funciona
- Testes passam (100%)

**Dependências**: Task 4.1, Task 4.2, Task 1.3, `PROMPTS_LIBRARY.md`

---

### Fase 5: Pipeline de Áudio

**Objetivo**: Implementar sistema completo de áudio local com Voice INTENTS.  
**Duração Estimada**: 6-7 semanas

#### Task 5.1: Atualizar TTS Service (StyleTTS2 + Voice INTENTS)

**ID**: `migration-5-1`  
**Prioridade**: ALTA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Migrar para StyleTTS2 local:
  - [ ] Integração com StyleTTS2
  - [ ] Configuração de modelo
  - [ ] Otimizações de performance
- [ ] Sistema de perfis vocais:
  - [ ] Estrutura de perfis (mestre, NPCs, jogadores IA)
  - [ ] Carregamento de perfis no boot
  - [ ] Switching entre perfis sem recarregar modelos
- [ ] Suporte multi-voz:
  - [ ] Mestre (narração neutra)
  - [ ] NPCs (guarda, taverneiro, ladina, etc.)
  - [ ] Jogadores IA (personalidades diferentes)
  - [ ] Monstros (efeitos especiais)
- [ ] Integração com RVC (opcional):
  - [ ] Treino de timbres base
  - [ ] Aplicação de timbres
- [ ] Implementar Voice INTENTS:
  - [ ] Parser de `[VOICE_INTENT:...]`
  - [ ] Suporte a todos os tipos (NARRATE, NPC_DIALOGUE, PLAYER_DIALOGUE, EVENT, CONDITION_EXPIRE, SYSTEM)
  - [ ] Integração com Orquestrador
  - [ ] Priorização de vozes
- [ ] Testes:
  - [ ] Testes de latência (< 350ms por sentença)
  - [ ] Testes de qualidade
  - [ ] Testes de multi-voz
  - [ ] Testes de Voice INTENTS

**Critérios de Aceitação**:
- StyleTTS2 funciona localmente
- Latência < 350ms por sentença
- Multi-voz funciona sem recarregar modelos
- Perfis vocais são aplicados corretamente
- Voice INTENTS funcionam corretamente
- Testes passam (100%)

**Dependências**: `AUDIO_PIPELINE.md`, `VOICE_INTENTS.md`

---

#### Task 5.2: Implementar Sistema de Música Procedural

**ID**: `migration-5-2`  
**Prioridade**: BAIXA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Camadas de música:
  - [ ] Base pad
  - [ ] Percussão
  - [ ] Cordas
  - [ ] Brass
  - [ ] Subgrave
- [ ] Sistema de transições:
  - [ ] Crossfade entre camadas
  - [ ] Transições suaves (400-900ms)
  - [ ] Sem cortes abruptos
- [ ] Integração com modos de cena:
  - [ ] Exploração (base + percussão suave + cordas)
  - [ ] Social (base + cordas mornas)
  - [ ] Tensão (ativa ritmo)
  - [ ] Combate (ativa brass + subgrave)
  - [ ] Vitória (corta ritmo, mantém cordas)
  - [ ] Morte/derrota (remove paleta alta, reverb)
- [ ] Testes:
  - [ ] Testes de camadas
  - [ ] Testes de transições
  - [ ] Testes de integração

**Critérios de Aceitação**:
- Camadas de música funcionam
- Transições são suaves
- Integração com modos de cena funciona
- Testes passam (100%)

**Dependências**: Task 2.1, Task 2.2, Task 2.3

---

#### Task 5.3: Implementar Sistema de Sound FX

**ID**: `migration-5-3`  
**Prioridade**: MÉDIA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Categorias de FX:
  - [ ] Ambiente (vento, chuva, taverna, floresta, dungeon)
  - [ ] Ações (abrir porta, pegar item, passos)
  - [ ] Combate (espada, flecha, magia, impacto crítico)
- [ ] Sistema de prioridades:
  - [ ] Voz sempre vence música
  - [ ] FX não interrompem fala
  - [ ] Priorização de FX importantes
- [ ] Envelopes ADSR:
  - [ ] Attack (rápido)
  - [ ] Sustain (curto)
  - [ ] Release (programado)
- [ ] Integração com eventos:
  - [ ] Eventos do Engine
  - [ ] Eventos de combate
  - [ ] Eventos de exploração
- [ ] Testes:
  - [ ] Testes de FX
  - [ ] Testes de prioridades
  - [ ] Testes de integração

**Critérios de Aceitação**:
- Todas as categorias de FX funcionam
- Prioridades são respeitadas
- Envelopes ADSR funcionam
- Integração com eventos funciona
- Testes passam (100%)

**Dependências**: Task 2.3, `rules5e-service`

---

#### Task 5.4: Integração com Orquestrador (Áudio)

**ID**: `migration-5-4`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Callbacks narrativos:
  - [ ] CONDITION_UPDATE → narração
  - [ ] EVENT callbacks → narração
  - [ ] Integração com prompts do Mestre
- [ ] Identificador de fala:
  - [ ] Tracking de speaker (mestre, NPC, jogador)
  - [ ] UI highlight no card correspondente
  - [ ] Animação de onda minimalista
- [ ] Integração com turnos:
  - [ ] EVENT: initiative_rolled → música sobe layer "ritmo"
  - [ ] EVENT: END_TURN → SFX "soft pass" (se ninguém falar)
- [ ] Testes:
  - [ ] Testes de callbacks
  - [ ] Testes de identificador
  - [ ] Testes de integração com turnos

**Critérios de Aceitação**:
- Callbacks narrativos funcionam
- Identificador de fala funciona
- Integração com turnos funciona
- Testes passam (100%)

**Dependências**: Task 1.3, Task 5.1, Task 5.2, Task 5.3

---

### Fase 6: Engine de Regras (Condições AUTO)

**Objetivo**: Refatorar engine para controle automático de condições.  
**Duração Estimada**: 2-3 semanas

#### Task 6.1: Implementar Sistema de Condições AUTO

**ID**: `migration-6-1`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Estrutura de ConditionState:
  - [ ] Enum ConditionKind (Prone, Blinded, Charmed, Grappled, etc.)
  - [ ] Source (efeito, criatura, item)
  - [ ] Stacks
  - [ ] DurationType (UntilEndTurn, Rounds, Permanent)
- [ ] Aplicação automática:
  - [ ] Aplicação de condições em eventos
  - [ ] Tracking de condições por criatura
  - [ ] Validação de condições
- [ ] Remoção automática:
  - [ ] Verificação de expiração
  - [ ] Remoção quando expira
  - [ ] Eventos de expiração
- [ ] Integração com Turn Engine:
  - [ ] Aplicação de efeitos "start of turn"
  - [ ] Redução de duração "end of turn"
  - [ ] Notificações de expiração
- [ ] Testes:
  - [ ] Testes de aplicação
  - [ ] Testes de remoção
  - [ ] Testes de duração
  - [ ] Testes de integração com turnos

**Critérios de Aceitação**:
- Condições são aplicadas automaticamente
- Condições são removidas quando expiram
- Eventos de expiração são gerados
- Integração com Turn Engine funciona
- Testes passam (100%)

**Dependências**: `rules5e-service`, `RULES_ENGINE.md`, `COMBAT_FLOW.md`

---

#### Task 6.2: Sistema de Eventos Automáticos

**ID**: `migration-6-2`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Eventos automáticos:
  - [ ] ConditionApplied
  - [ ] ConditionEnded
  - [ ] Death
  - [ ] Knockdown
  - [ ] ConcentrationLost
- [ ] Integração com Orquestrador:
  - [ ] Envio de eventos para Orquestrador
  - [ ] Formato de eventos
  - [ ] Serialização
- [ ] Notificação ao Mestre IA:
  - [ ] Formato CONDITION_UPDATE
  - [ ] Integração com prompts
  - [ ] Narração de eventos
- [ ] Testes:
  - [ ] Testes de cada tipo de evento
  - [ ] Testes de integração
  - [ ] Testes de notificação

**Critérios de Aceitação**:
- Todos os eventos são gerados corretamente
- Eventos são enviados ao Orquestrador
- Mestre IA recebe notificações
- Testes passam (100%)

**Dependências**: Task 6.1, Task 1.3

---

### Fase 7: Integração e Testes

**Objetivo**: Integrar tudo e garantir que funciona end-to-end.  
**Duração Estimada**: 5-6 semanas

#### Task 7.1: Testes de Integração

**ID**: `migration-7-1`  
**Prioridade**: CRÍTICA  
**Estimativa**: 2 semanas

**Subtasks**:
- [ ] Teste completo: SocialFreeFlow → Exploration → Combat:
  - [ ] Fluxo completo de uma sessão
  - [ ] Transições entre modos
  - [ ] Geração de INTENTs em cada modo
  - [ ] Resolução de ações
- [ ] Teste de geração de INTENTs:
  - [ ] INTENTs em modo social
  - [ ] INTENTs em modo exploração
  - [ ] INTENTs em modo combate
  - [ ] Validação de INTENTs
- [ ] Teste de consulta a Hive:
  - [ ] LORE_QUERY
  - [ ] RULE_QUERY
  - [ ] Pré-inject de lore
- [ ] Teste de geração de assets:
  - [ ] Retratos
  - [ ] Cenas
  - [ ] Battlemaps
  - [ ] Cache
- [ ] Teste de pipeline de áudio completo:
  - [ ] Voz→voz completo
  - [ ] Voice INTENTS
  - [ ] Música procedural
  - [ ] Sound FX
  - [ ] Callbacks narrativos
- [ ] Teste de Turn Engine completo:
  - [ ] Iniciativa
  - [ ] Rolagens client vs servidor
  - [ ] Economia de ações
  - [ ] LoS e alcance
  - [ ] AoE
  - [ ] Condições AUTO

**Critérios de Aceitação**:
- Todos os testes de integração passam
- Fluxos completos funcionam
- Performance dentro dos targets
- Testes passam (100%)

**Dependências**: Todas as fases anteriores

---

#### Task 7.2: Testes de Performance

**ID**: `migration-7-2`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Latência voz→voz:
  - [ ] Medição completa do pipeline
  - [ ] Identificação de gargalos
  - [ ] Otimizações
  - [ ] Target: < 600ms
- [ ] Latência de geração de INTENTs:
  - [ ] Medição de parsing
  - [ ] Medição de execução
  - [ ] Otimizações
- [ ] Performance do Orquestrador:
  - [ ] Throughput de INTENTs
  - [ ] Uso de memória
  - [ ] Uso de CPU
  - [ ] Otimizações
- [ ] Performance de geração de assets:
  - [ ] Tempo de geração (downtime)
  - [ ] Tempo de lookup (runtime)
  - [ ] Uso de GPU
  - [ ] Otimizações

**Critérios de Aceitação**:
- Latência voz→voz < 600ms
- Performance do Orquestrador aceitável
- Geração de assets não bloqueia runtime
- Testes passam (100%)

**Dependências**: Todas as fases anteriores

---

#### Task 7.3: Testes de Robustez

**ID**: `migration-7-3`  
**Prioridade**: ALTA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Tratamento de erros de parsing:
  - [ ] INTENTs malformadas
  - [ ] INTENTs inválidas
  - [ ] Fallbacks
  - [ ] Recuperação
- [ ] Fallbacks quando Hive está offline:
  - [ ] LORE_QUERY sem Hive
  - [ ] RULE_QUERY sem Hive
  - [ ] Mestre IA improvisa
  - [ ] Marcação para revisão
- [ ] Fallbacks quando Art Daemon cai:
  - [ ] Uso de cache
  - [ ] Placeholders
  - [ ] Degradação graciosa
- [ ] Recuperação de estado:
  - [ ] Após falhas
  - [ ] Após restart
  - [ ] Persistência de sessão

**Critérios de Aceitação**:
- Erros são tratados graciosamente
- Fallbacks funcionam
- Sistema se recupera de falhas
- Testes passam (100%)

**Dependências**: Todas as fases anteriores

---

#### Task 7.4: Documentação Final

**ID**: `migration-7-4`  
**Prioridade**: MÉDIA  
**Estimativa**: 1 semana

**Subtasks**:
- [ ] Atualizar todos os documentos:
  - [ ] ARCHITECTURE.md
  - [ ] INDEX.md
  - [ ] README.md
  - [ ] Documentos de cada módulo
- [ ] Criar guias de uso:
  - [ ] Guia do Mestre IA
  - [ ] Guia de desenvolvimento
  - [ ] Guia de assets
  - [ ] Guia de áudio
- [ ] Documentar APIs:
  - [ ] API do Orquestrador
  - [ ] API de INTENTs
  - [ ] API de services
- [ ] Exemplos de uso:
  - [ ] Exemplos de INTENTs
  - [ ] Exemplos de prompts
  - [ ] Exemplos de integração

**Critérios de Aceitação**:
- Documentação completa e atualizada
- Guias são claros e úteis
- APIs estão documentadas
- Exemplos funcionam
- Testes passam (100%)

**Dependências**: Todas as fases anteriores

---

## 3. Estrutura de Código Proposta

### 3.1 Novo Módulo: `orchestrator`

```
src/orchestrator/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── fsm.rs              # Máquina de estados de cena
│   ├── intent_parser.rs    # Parser de INTENT DSL
│   ├── intent_executor.rs  # Executor de INTENTs
│   ├── session.rs          # Gerenciamento de sessão
│   ├── combat.rs           # Lógica de combate (coordenação)
│   ├── turn_engine.rs      # Turn Engine (combate em turnos)
│   ├── communication.rs    # IPC/WebSocket com Electron
│   └── error.rs
└── tests/
    ├── fsm_test.rs
    ├── intent_parser_test.rs
    ├── turn_engine_test.rs
    └── integration_test.rs
```

### 3.2 Atualizações em Módulos Existentes

#### `llm-core`
- Adicionar exemplos de INTENT DSL nos prompts
- Modificar saída para gerar DSL ao invés de JSON
- Adicionar prompts de `DM_MINDSET.md` e `CHARACTER_AGENTS.md`
- Integrar Voice INTENTS na geração

#### `game-engine`
- Refatorar para trabalhar com Orquestrador
- Remover lógica de coordenação (move para Orquestrador)
- Manter apenas estado de jogo

#### `rules5e-service`
- Adicionar sistema de condições AUTO
- Adicionar sistema de eventos automáticos
- Manter interface HTTP, mas agora chamada via Orquestrador

#### `tts-service`
- Integrar Voice INTENTS
- Suporte a perfis vocais
- Integração com StyleTTS2

---

## 4. Cronograma Estimado

| Fase | Duração | Dependências |
|------|---------|--------------|
| Fase 1: Fundação | 4-5 semanas | Nenhuma |
| Fase 2: Modos de Cena | 6-8 semanas | Fase 1 |
| Fase 3: Mindset e Agentes | 4-5 semanas | Fase 1, Fase 2 |
| Fase 4: Pipeline Visual | 6-8 semanas | Fase 1 |
| Fase 5: Pipeline de Áudio | 6-7 semanas | Fase 1 |
| Fase 6: Engine de Regras | 2-3 semanas | Fase 1 |
| Fase 7: Integração | 5-6 semanas | Todas as anteriores |

**Total Estimado**: 33-42 semanas (8-10 meses)

---

## 5. Resumo de Tasks

**Total de Tasks**: 25 tasks principais  
**Total de Subtasks**: ~200 subtasks  
**Duração Estimada Total**: 33-42 semanas (8-10 meses)

### Distribuição por Fase

- **Fase 1**: 4 tasks (4-5 semanas)
- **Fase 2**: 4 tasks (6-8 semanas)
- **Fase 3**: 3 tasks (4-5 semanas)
- **Fase 4**: 3 tasks (6-8 semanas)
- **Fase 5**: 4 tasks (6-7 semanas)
- **Fase 6**: 2 tasks (2-3 semanas)
- **Fase 7**: 4 tasks (5-6 semanas)

### Prioridades

- **CRÍTICA**: 8 tasks
- **ALTA**: 12 tasks
- **MÉDIA**: 5 tasks
- **BAIXA**: 1 task

---

## 6. Riscos e Mitigações

### Risco 1: Quebra de Funcionalidades Existentes

**Mitigação**:
- Manter código antigo funcionando durante migração
- Migração incremental (fase por fase)
- Testes de regressão a cada fase

### Risco 2: Performance Degradada

**Mitigação**:
- Benchmarks antes e depois
- Otimização do Orquestrador (async, pools)
- Cache agressivo

### Risco 3: Complexidade do INTENT DSL

**Mitigação**:
- Parser robusto com fallbacks
- Validação rigorosa
- Testes extensivos

### Risco 4: Latência de Áudio

**Mitigação**:
- StyleTTS2 local (não API)
- Otimização de pipeline
- Benchmarks contínuos

---

## 7. Checklist de Migração

### Pré-Migração

- [ ] Backup completo do código atual
- [ ] Documentação da arquitetura atual
- [ ] Lista de funcionalidades existentes
- [ ] Testes atuais passando (baseline)

### Durante Migração

- [ ] Criar branch `feature/orchestrator-migration`
- [ ] Implementar Orquestrador (Fase 1)
- [ ] Implementar modos de cena (Fase 2)
- [ ] Atualizar Mestre IA (Fase 3)
- [ ] Implementar pipeline visual (Fase 4)
- [ ] Implementar pipeline de áudio (Fase 5)
- [ ] Refatorar engine de regras (Fase 6)
- [ ] Integração e testes (Fase 7)

### Pós-Migração

- [ ] Todos os testes passando
- [ ] Performance dentro dos targets
- [ ] Documentação atualizada
- [ ] Código antigo removido (se aplicável)
- [ ] Merge para main

---

## 8. Próximos Passos Imediatos

1. **Revisar este plano** com a equipe
2. **Priorizar tasks** baseado em dependências
3. **Criar issues** no sistema de tracking
4. **Iniciar Fase 1** imediatamente
5. **Setup de CI/CD** para testes contínuos

---

## 9. Notas Importantes

- **Migração Incremental**: Não quebrar funcionalidades existentes durante migração
- **Testes Contínuos**: Testes a cada fase, não apenas no final
- **Documentação Viva**: Manter documentação atualizada durante migração
- **Performance First**: Monitorar performance continuamente
- **Robustez**: Sistema deve ser robusto a falhas desde o início

---

## 10. Referências

- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Arquitetura do Orquestrador
- [INTENT_DSL.md](INTENT_DSL.md) - Especificação da DSL
- [DM_MINDSET.md](DM_MINDSET.md) - Mindset do Mestre IA
- [CHARACTER_AGENTS.md](CHARACTER_AGENTS.md) - Agentes Jogadores IA
- [COMBAT_FLOW.md](COMBAT_FLOW.md) - Fluxo de combate e Turn Engine
- [VISUAL_PIPELINE.md](VISUAL_PIPELINE.md) - Pipeline visual
- [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md) - Pipeline de áudio
- [VOICE_INTENTS.md](VOICE_INTENTS.md) - Voice INTENTS
- [RULES_ENGINE.md](RULES_ENGINE.md) - Engine de regras
- [LORA_GUIDELINES.md](LORA_GUIDELINES.md) - Guidelines de LoRA
- [PROMPTS_LIBRARY.md](PROMPTS_LIBRARY.md) - Biblioteca de prompts
- [TRAINING_PIPELINE.md](TRAINING_PIPELINE.md) - Pipeline de treinamento
