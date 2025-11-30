# VRPG – Orquestrador

## 1. Objetivo Central

O Orquestrador é o **núcleo determinístico** do sistema.

Ele:

- **Gerencia fluxo** entre humano → IA → motor de regras
- **Protege os modelos** contra prompts incorretos, ambiguidades e hallucination
- **Mantém coerência** mecânica, espacial, temporal e narrativa
- **Garante** que o jogo opere como um RPG de mesa real

**O Orquestrador não é uma IA.**  
Ele não interpreta.  
Ele executa políticas.

---

## 2. Princípios Fundamentais

### 2.1 Fonte Única de Verdade

**Qualquer valor factual deve ser respondido pelo Orquestrador, nunca pela LLM.**

Exemplos:

- HP atual
- Dano recebido
- Slots de magia
- Classe de Armadura
- Condições de status
- Iniciativa
- Recursos: Ki / Rage / Sorcery Points
- Distância real (grid / coordenada 3D)
- Inventário
- Cooldowns

**Se existe no GameState → Orquestrador responde.**

LLM não "chuta", "estimativa", "talvez" ou "acho que".

### 2.2 Orquestrador é Motor, não Narrador

- **Quem narra** é Qwen-14B
- **Quem reage** é Qwen-1.5B
- **Quem decide factual** é Orquestrador

Orquestrador é o dado na mesa, a planilha do mestre, o log do Foundry.

### 2.3 Latência é Orquestrador

- Controle de turnos
- Controle de pausa
- Cancelamento de TTS
- Trigger de animações
- Streaming parcial

**A LLM nunca controla timing.**

---

## 3. Pipeline de 3 Agentes

O Orquestrador coordena:

1. **Qwen-1.5B** → Reação humana inicial ("prelúdio") - < 1.2s
2. **Qwen-14B** → Narrativa completa, consequências, resolução - < 6s
3. **Orquestrador** → Lógica pura, determinística - < 50ms

**Regra Central**: Nenhuma resposta final é emitida pelo 14B até que o 1.5B já tenha iniciado a resposta.

**Ver detalhes completos em [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md)**

---

## 4. Entradas do Orquestrador

### 4.1 ASR Input (Whisper)

- Recebe transcrição parcial (`asr_partial`)
- Recebe transcrição final (`asr_final`)
- Recebe marcadores de silêncio (VAD)
- Recebe duração da fala (clock)

**O Orquestrador não interpreta linguagem, apenas mede.**

### 4.2 Intent Classifier (Router LLM pequeno)

**Entrada**:
- Texto parcial/final
- Contexto de cena
- Token "turno"

**Saída** (1 string):
- `FACT_QUERY`
- `SIMPLE_RULE_QUERY`
- `META_QUERY`
- `WORLD_ACTION`
- `COMBAT_ACTION`
- `UNCERTAIN`

**Orquestrador NUNCA infere intenção por adivinhação.**  
O Router faz a classificação, não a resolução.

### 4.3 Estado Mecânico (GameState)

Estrutura fixa em memória (Rust ECS, por exemplo):

```rust
Entity(id, type=Npcs|Players|Monsters)
  - HP
  - TempHP
  - AC
  - Resistances
  - Vulnerabilities
  - Position(x,y,z)
  - MovementAvailable
  - ActionsAvailable
  - Resources: Rage, Ki, Slots, etc
```

### 4.4 Cache Episódico (Short-term Memory)

**Não é "histórico da partida".**  
É apenas contexto útil para resposta.

**Armazena**:
- Últimas 3–6 ações por entidade
- Resultado do último turno
- Interações recentes (Player ↔ NPC ↔ Ambiente)
- Eventos relevantes ("portal selou", "chão desabou")

**Jamais**:
- Descrição redundante
- Logs completos
- Texto de sessão inteira
- Conversa raw

---

## 5. Saídas do Orquestrador

### 5.1 Para Qwen-1.5B (EAR)

Somente quando há intenção narrativa ou incompleta:

- pequena janela de contexto
- sem números
- sem regras

### 5.2 Para Qwen-14B (MASTER)

Quando:

- Ação requer narração
- Consequências precisam ser aplicadas
- Pedidos de rolagem
- Aplicação de efeito/resultado

### 5.3 Para Vectorizer/Nexus/Lexum

Quando:

- Consulta de regra oficial
- Lore
- Detalhes canônicos
- Backstory

### 5.4 Para XTTS Streaming Pipeline

Quando:

- Texto do Qwen 1.5B ou 14B está pronto
- Narrativa completa gerada
- Mensagem sistêmica curta

**Pipeline de Streaming:**
1. Semantic Chunker divide texto (3-7s chunks)
2. XTTS Worker gera chunks (Thread C, adaptive GPU)
3. AudioBuffer FIFO armazena (Float32 → int16)
4. Pre-Buffer Manager mantém 1-2 chunks à frente
5. Audio Output toca (Thread D, zero-gap)

---

## 6. Roteamento de Fluxo (Single Source)

**Fluxo de alto nível**:

```
ASR → Router → Orquestrador → (1.5B / Vectorizer / 14B / GameState)
```

**Nunca existe "Jogador → LLM diretamente".**

O jogador conversa com o mundo → não com a IA.

### 6.1 Exemplo: Pergunta factual

```
"Quantos slots de nível 3 ainda tenho?"

Router → FACT_QUERY
Orquestrador → lê state.player.spellslots[3]
Resposta: "Você tem 1 slot de nível 3 restante."

Sem Qwen.
Sem Vectorizer.
Sem IA criativa.
```

### 6.2 Exemplo: Regra simples

```
"Ataque furtivo usa Destreza?"

Router → SIMPLE_RULE_QUERY
Orquestrador:
  1. Vectorizer busca regra real
  2. 1.5B sintetiza como fala humana
  3. "Furtividade usa Destreza."
```

### 6.3 Exemplo: Ação narrativa

```
"Corro pela ponte e corto a garganta do gnoll."

Router → WORLD_ACTION
Orquestrador:
  1. 1.5B dispara uma reação emocional de abertura
  2. 14B recebe o restante e decide
```

---

## 7. Trigger Conditions (Timing REAL)

### 7.1 Qwen 1.5B dispara quando:

- Jogador fala > 6–8s E ainda não terminou
- pausa VAD > 0.7–1.3s
- intenção clara detectada antes do final

Isso dá sensação de "DM humano".

### 7.2 Qwen 14B dispara quando:

- `asr_final` recebido
- 1.5B já forneceu prelúdio
- contexto mínimo consolidado

---

## 8. Cancelamento e Interrupção (Hard Rules)

### 8.1 Jogador interrompe narrativa

- XTTS Streaming é cancelado **HARD**
- AudioBuffer FIFO é limpo
- Orquestrador insere flag `interrupt`
- Qwen14B descarta geração corrente
- XTTS Worker cancela chunks em geração
- 1.5B responde: "Ok, estou ouvindo."

### 8.2 Dois jogadores falam juntos

- diarization identifica speaker
- 1.5B pausa
- Orquestrador pede reenvio
- "Preciso que um jogador fale por vez."

Sem drama.  
Sem LLM.

---

## 9. Modos de Cena (FSM de Alto Nível)

O Orquestrador trabalha com uma **máquina de estados por sessão**.

```text
SceneState =
  - SocialFreeFlow      (diálogo, roleplay, sem grid)
  - Exploration         (investigação, movimento livre, sem combate ativo)
  - CombatTurnBased     (modo combate, grid ativo)
  - DowntimePreparation (entre sessões, preparação do Mestre IA)
```

### 9.1 SocialFreeFlow

- Qwen se comporta como Mestre "quase humano"
- resposta rápida, sem excesso de INTENT técnica
- INTENTs aparecem apenas quando necessário (teste de habilidade, consulta de lore/regra)
- UI foca em cena / retratos / ambiente, sem grade nem HUD de combate

### 9.2 Exploration

- Sem combate, mas com foco em movimentação, perception checks, investigação
- Pode já preparar potencial de combate (spawn de criaturas invisíveis, triggers de emboscada)

### 9.3 CombatTurnBased

- Entramos quando IA declara hostilidade ou Orquestrador detecta `INTENT combat_start`
- Engine de regras D&D assume todas as decisões mecânicas
- Qwen continua narrando, mas não define matemática
- UI: battlemap com grid, ordem de iniciativa no topo, barra de ações no rodapé

### 9.4 DowntimePreparation

- Usado entre sessões
- Geração de imagens pesadas, treino de LoRA/embeddings, atualização de memória (Hive)
- Orquestrador enfileira jobs para GPU/CPU sem preocupação de latência

---

## 10. Integração com Hive Infra

### 10.1 Vectorizer

- Busca semântica em livros/lores
- Retorno factual
- Chunk responsivo 512–4096 tokens
- Zero inferência

**Vectorizer nunca "fala".**  
Ele entrega blocos brutos.

### 10.2 Lexum

- Textos longos (lore extenso)
- Interpretação posterior pelo 14B
- **Cuidado**: nunca expor raw em diálogo

### 10.3 Nexus

- Grafos de relações
- NPC ↔ jogador ↔ locação
- Quests
- Consequência social

14B usa para coerência, não como fonte mecânica.

### 10.4 Synap

- Modelo ↔ Memória sem prompt gigante
- Estado parcial
- Slots
- Condições
- Histórico imediato

Evita prompt com 3k tokens toda vez.

---

## 11. Preservação da Experiência (Anti-MMORPG)

**Erro tradicional de VTT**:

```
Jogador clica ação
Sistema responde com números
```

**VRPG (design real)**:

```
Jogador descreve
Mestre imagina
Mundo reage
```

**Orquestrador garante**:

A IA nunca vira menu de botões.

---

## 12. Edge Cases e Mitigações

### 12.1 Jogador tenta "resolver" regra

```
"Então se eu fizer X tenho vantagem, certo?"

Router → INTENT
14B decide no contexto, não citação textual.
```

### 12.2 Jogador pede tutorial

```
"Como funciona iniciativa?"

Router → META_QUERY
Orquestrador decide:
  - resposta curta sistêmica
  ou
  - 14B contextualiza narrativamente

Jamais: "Iniciativa = D20 + Dex + proficiency…"
Isso vira "chatGPT".
```

---

## 13. Segurança e Anti-Hallucination

### 13.1 Bloqueio de mecânicas

- **14B NUNCA inventa regra**
- **1.5B NUNCA aplica mecânica**
- **Vectorizer SEMPRE fonte real**

### 13.2 Mecanismo de recuperação

Se 14B gerar algo impossível:

- Orquestrador corrige via log de ação
- Sem explicar ao jogador
- Apenas re-narra.

---

## 14. Arquitetura Hybrid – Rust + Electron

### 14.1 Núcleo em Rust

**Responsável por**:

- Orquestração de estados
- Engine D&D
- Integração com:
  - runtime de IA (Qwen local)
  - Hive (via cliente HTTP/GRPC)
  - Art Daemon
- Gerenciamento de sessão:
  - threads
  - timers
  - filas de eventos

**Interface externa**:

- exposta via:
  - IPC local (Electron <-> Rust) ou
  - WebSocket/HTTP (localhost, porta interna)

### 14.2 Client Electron/TS

**Responsável por**:

- UI BG3-like (layout, animações, componentes)
- Mixagem de áudio (playback de XTTS streaming, efeitos sonoros, música ambiente)
- Controle de streaming (cancelamento, pausa, resume)
- Gerenciamento de AudioBuffer FIFO
- Coordenação com GPU adaptive control
- Captura de voz (microfone, streaming de áudio), cliques e hotkeys, rolagens "físicas"

**Comunicação**:

- Envia para Rust: `PlayerAction`, `RollResult`, eventos de UI
- Recebe de Rust: `SceneUpdate`, `CombatUpdate`, `RollRequest`, `Narration`, referência de imagens

---

## 15. Contrato de Comunicação UI ↔ Orquestrador

### 15.1 Mensagens UI → Orquestrador (TS → Rust)

#### PlayerAction

```typescript
type PlayerAction = {
  sessionId: string;
  playerId: string;
  kind: "voice" | "ui";
  text?: string;           // resultado de ASR
  uiIntent?: string;       // ex: "click_attack_button"
  targetId?: string;       // se clicou num token
  metadata?: Record<string, any>;
};
```

#### RollResult

```typescript
type RollResult = {
  sessionId: string;
  requestId: string;  // correlaciona com RollRequest
  actorId: string;
  total: number;
  natural: number;
  breakdown: Record<string, number>;
  clientSeed?: string;
  timestamp: number;
};
```

### 15.2 Mensagens Orquestrador → UI (Rust → TS)

#### SceneUpdate

```typescript
type SceneUpdate = {
  sessionId: string;
  sceneState: "SocialFreeFlow" | "Exploration" | "CombatTurnBased" | "DowntimePreparation";
  summary: string;
  activeSpeakerId?: string;
  participants: Array<{
    id: string;
    name: string;
    portraitUrl?: string;
    isNpc: boolean;
  }>;
};
```

#### CombatUpdate

```typescript
type CombatUpdate = {
  sessionId: string;
  inCombat: boolean;
  round: number;
  initiativeOrder: Array<{
    creatureId: string;
    name: string;
    currentHp: number;
    maxHp: number;
    isActive: boolean;
  }>;
  activeCreatureId?: string;
};
```

#### RollRequest

```typescript
type RollRequest = {
  sessionId: string;
  requestId: string;
  actorId: string;
  rollKind: "attack" | "skill" | "save" | "initiative" | "other";
  skill?: string;
  ability?: string;
  dc?: number;
  formulaHint?: string;
  reason: string;
};
```

#### Narration

```typescript
type Narration = {
  sessionId: string;
  speakerId: string;
  text: string;
  emotion?: string;
  taggedForTts: boolean;
};
```

---

## 16. INTENT DSL

Formato inspirado em "comandos" legíveis, mas ainda estruturáveis.

**Exemplo**:

```
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda a deixar o grupo entrar"
END_INTENT
[/INTENTS]
```

**Parsing**:

Orquestrador (Rust) recebe o texto bruto do Qwen, extrai o bloco `[INTENTS] ... [/INTENTS]`, faz parsing linha a linha.

**Ver detalhes completos em [INTENT_DSL.md](INTENT_DSL.md)**

---

## 17. Implementação Real (Rust)

### 17.1 ECS (Entity Component System)

- **Entities**: players, NPCs, monsters
- **Components**: HP, AC, Position, Resources
- **Systems**: combat, effects, turn

**Orquestrador é um System independente.**

### 17.2 Pipe (Streaming Real-Time)

```
audio_in → whisper → router
router_out → orchestrator
orchestrator → { qwen_1.5b, qwen_14b, vectorizer, gamestate }
    ↓
qwen_1.5b → semantic_chunker → xtts_streaming_worker (Thread C)
qwen_14b → semantic_chunker → xtts_streaming_worker (Thread C)
    ↓
xtts_streaming_worker → audiobuffer_fifo → audio_output (Thread D)
    ↓
audio_out (WASAPI/ASIO/CoreAudio, zero-gap playback)
```

**Thread Architecture:**
- **Thread A**: Qwen 1.5B → Prelude
- **Thread B**: Qwen 14B → Narrative
- **Thread C**: XTTS Worker (adaptive parallel/sequential, GPU control)
- **Thread D**: Audio Consumer (dedicated I/O, não bloqueia geração)

---

## 18. Concorrência e Threads

### 18.1 Rust Core

- **Thread principal**: loop de eventos da sessão
- **Pool de workers**: inferência de IA (se Qwen estiver em server separado), chamadas Hive (rede)
- **Fila de jobs**: geração de imagem (modo preparação ou assíncrono em sessão)

### 18.2 Garantias

- **Evitar travar UI**: toda comunicação UI ↔ Rust é não-bloqueante (async/await)
- **Engine D&D rápida**: chamadas de microssegundos a poucos ms
- **IA é gargalo natural**: orquestrador deve minimizar o número de chamadas por turno

---

## 19. Tratamento de Erros e Fallbacks

### 19.1 Parsing de INTENT falha

- loga erro
- pode pedir nova tentativa ao modelo com contexto de erro
- ou cair em fallback de ação genérica

### 19.2 Hive offline

- Qwen é instruído a "improvisar" sem referência exata de regra
- cena é marcada para revisão posterior

### 19.3 Art Daemon cai

- UI usa imagens já cacheadas
- placeholders para novas

---

## 20. Regra Final

**Se não precisa de imaginação → Orquestrador.**  
**Se precisa de reação humana → 1.5B.**  
**Se precisa de autor → 14B.**

---

## Referências

- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa do pipeline de 3 agentes
- [INTENT_DSL.md](INTENT_DSL.md) - Especificação da DSL de Intenções
- [specs/ORCHESTRATOR_SPEC.md](specs/ORCHESTRATOR_SPEC.md) - Especificação técnica detalhada
