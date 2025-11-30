# VRPG — Turn Engine (Combate em Turnos, D&D 5e, Mestre IA Vivo)

Este documento define o **motor de turnos** do VRPG:  
como iniciativa é calculada, como turnos avançam, como INTENTs de ação são processadas,  
como as rolagens são divididas entre client (jogador) e servidor (NPC/engine),  
e como o Mestre IA se encaixa nesse ciclo **micro-narrativo por ação**.

Integra este módulo com:

- `ORCHESTRATOR.md`
- `RULES_ENGINE.md`
- `COMBAT_FLOW.md`
- `INTENT_DSL.md`
- `VOICE_INTENTS.md`
- `VISUAL_PIPELINE.md`

---

## 1. Objetivo do Turn Engine

O Turn Engine é o módulo que:

- Gerencia **iniciativa**.
- Controla **ordem de turno**.
- Garante **economia de ações** (Ação, Movimento, Bônus, Reação).
- Orquestra **processamento de INTENTs** em combate.
- Resolve a transição de estados:
  - fora de combate → combate,
  - combate → pós-combate.

Ele não:

- narra,
- decide intenções narrativas,
- gera imagens,
- toca áudio.

Ele é **puro sistema tático**.

---

## 2. Estados de Combate

### 2.1 CombatState (vista geral)

```rust
pub struct CombatState {
    pub encounter_id: String,
    pub round: u32,
    pub initiative_order: Vec<InitiativeEntry>,
    pub active_index: usize,
    pub participants: HashMap<String, CreatureCombatState>,
    pub started_at: Timestamp,
    pub ended_at: Option<Timestamp>,
}
```

### 2.2 InitiativeEntry

```rust
pub struct InitiativeEntry {
    pub creature_id: String,
    pub name: String,
    pub initiative_value: i16,
    pub is_alive: bool,
    pub is_conscious: bool,
    pub is_player_controlled: bool,
}
```

### 2.3 CreatureCombatState

```rust
pub struct CreatureCombatState {
    pub creature_id: String,
    pub base: CreatureRef,          // aponta pra ficha do personagem
    pub current_hp: i32,
    pub temp_hp: i32,
    pub conditions: Vec<ConditionState>,
    pub used_action: bool,
    pub used_bonus_action: bool,
    pub used_reaction: bool,
    pub remaining_movement_ft: i16,
    pub position: GridCoord,
}
```

---

## 3. Transição para Combate

### 3.1 Gatilhos possíveis

INTENT do Mestre IA:
- `COMBAT_START`

Ação do jogador:
- ataque explícito em cena social

Trigger de cenário:
- emboscada, armadilha, scripted event.

### 3.2 Fluxo de entrada

Orquestrador detecta `COMBAT_START`.

Consulta lista de criaturas envolvidas:
- players,
- jogadores IA,
- NPCs/monstros.

Calcula ou solicita rolagens de iniciativa:
- players: pedidos de rolagem via UI (client-side).
- NPCs: rolagens servidor (engine).

Ordena `initiative_order`.

Emite:
- `CombatUpdate` (para UI),
- `VOICE_INTENT:EVENT` (entrada dramática),
- mudança de música (pipeline de áudio).

---

## 4. Rolagens de Dados (client vs servidor)

### 4.1 Regra de ouro

Rolagens de jogadores são feitas no client.
Rolagens de NPCs e checks internos são feitas pela Engine.

### 4.2 Por quê?

Jogar dado é parte da graça.

Jogador sentir o d20 "girando" faz parte da experiência.

Mas NPC não precisa, e seria redundante.

### 4.3 Interface

#### 4.3.1 Pedido de rolagem

Engine → Orquestrador → UI:

```typescript
type RollRequest = {
  sessionId: string;
  requestId: string;
  actorId: string;       // player_x
  rollKind: "attack" | "skill" | "save" | "initiative" | "other";
  skill?: string;
  ability?: string;
  dc?: number;           // opcional visível ou não
  formulaHint?: string;  // "1d20 + CHA_MOD + PROF"
  reason: string;        // "convencer o guarda", "acertar o goblin"
};
```

#### 4.3.2 Resposta (client → servidor)

```typescript
type RollResult = {
  sessionId: string;
  requestId: string;
  actorId: string;
  total: number;
  natural: number;
  breakdown: Record<string, number>;
  clientSeed?: string;
  timestamp: number;
};
```

### 4.4 Validação

Opcionalmente, a Engine pode:

- validar se total é coerente (anti-cheat),
- logar seeds pra replay.

---

## 5. Ciclo de Turno

### 5.1 Visão de alto nível

Para cada turno:

1. Selecionar criatura ativa.
2. Resetar economia de ações para essa criatura.
3. Processar intenção:
   - do jogador humano,
   - do Jogador IA,
   - do Mestre para NPCs.
4. Resolver mecânica (Engine).
5. Notificar Mestre IA para narrar.
6. Atualizar UI (HP, posições, condições, turno).
7. Avançar para próxima criatura.

### 5.2 Pseudo-código

```rust
fn run_combat_loop(state: &mut CombatState) {
    loop {
        if combat_ended(state) {
            handle_combat_end(state);
            break;
        }

        let active_id = state.initiative_order[state.active_index].creature_id.clone();
        start_turn(state, &active_id);

        process_turn_actions(state, &active_id);

        end_turn(state, &active_id);
        advance_initiative(state);
    }
}
```

---

## 6. Economia de Ações (5e simplificada)

Em cada turno:

- `used_action: bool`
- `used_bonus_action: bool`
- `used_reaction: bool` (reação pode disparar fora do turno)
- `remaining_movement_ft: i16`

### 6.1 Ligações com INTENTs

Cada INTENT de combate:

- consome 1 tipo de recurso
- a Engine valida se ainda é permitido

Exemplos:

- `MELEE_ATTACK` → consome Action
- `RANGED_ATTACK` → consome Action
- `SPELL_CAST` (spell não-cantrip) → consome Action + slot
- `DASH` → consome Action (ou Bônus, se class feature)
- `DISENGAGE` → Action (ou Bônus, se feature)
- `HELP` → Action
- `USE_ITEM` → Action ou Bônus (regra contextual)

Se a INTENT pedir algo inviável (sem ação, sem slot, sem movimento), a Engine recusa e o Orquestrador (via Mestre IA) narra a frustração.

---

## 7. Processamento de INTENTs em Turno

### 7.1 Ciclo por ação

Jogador / IA descreve em linguagem natural.

Mestre IA interpreta → gera DSL:

```
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
WEAPON: weapon_longsword_01
MOVE_REQUIRED: YES
END_INTENT
[/INTENTS]
```

Orquestrador:

- parse TR → `Intent::MeleeAttack`
- valida contexto (alvo vivo? distância? visão? economia de ações?)

Se válido:

- se precisa de rolagem de jogador → `RollRequest`
- combina resultado com ficha → chama Engine → `AttackOutcome`

Quando Engine retorna:

- atualiza `CombatState` (HP, condições)
- gera evento para IA narrar
- envia `CombatUpdate` + `Narration` para UI/áudio

### 7.2 Várias ações no mesmo turno

No modelo "por ação", um turno pode ter:

- 1 ação principal (ataque/feitiço)
- 1 movimentação
- 1 bônus (ex: "disengage")
- 0/1 reação (fora do turno, mas no round)

Cada uma gera um miniciclo INTENT → Engine → IA.

---

## 8. Linha de Visão (LoS) e Alcance

### 8.1 Representação

Battlemap é um grid 2D com obstáculos.

```rust
struct GridCell {
    coord: GridCoord,     // (x, y)
    walkable: bool,
    blocks_los: bool,
    elevation: i16,
}
```

### 8.2 Checagem de LoS

Para um ataque à distância ou magia, Engine faz:

- Bresenham entre `attacker.position` e `target.position`
- se todas as células intermediárias têm `blocks_los == false` → LoS ok
- se não → falta linha de visão

### 8.3 Alcance

`distance_ft = grid_distance * 5` (cada célula = 5 ft)

Se `distance_ft > weapon_range`:

- INTENT invalidada
- ou transformada em tentativa (desvantagem, se casa com regra)

Orquestrador pode:

- pedir ao Mestre IA para sinalizar "ele está fora do alcance",
- sugerir movimento antes do ataque (nova INTENT).

---

## 9. Áreas de Efeito (AoE)

### 9.1 Representação

AoE é definido por:

- centro (grid)
- forma (círculo, cone, quadrado)
- raio / alcance

```rust
enum AreaShape {
    Circle { radius_ft: u16 },
    Cone { length_ft: u16, angle_deg: u16 },
    Square { size_ft: u16 },
}
```

### 9.2 Fluxo

INTENT:

```
INTENT: SPELL_CAST
ACTOR: player_1
SPELL: fireball
SLOT_LEVEL: 3
AREA_CENTER: "15, 8"
END_INTENT
```

Engine calcula células atingidas.

Aplica saves e dano conforme SRD 5e.

Gera outcomes por criatura:

- `SpellOutcomeCreature` (hit, fail, success half, etc.)

Orquestrador envia `CombatUpdate` + marcadores visuais de AoE.

Mestre IA narra:

> "A esfera de fogo se expande e engole os goblins…"

---

## 10. Avanço de Iniciativa

### 10.1 Algoritmo

Depois de `end_turn`:

```rust
fn advance_initiative(state: &mut CombatState) {
    let len = state.initiative_order.len();
    for i in 1..=len {
        let next_idx = (state.active_index + i) % len;
        let entry = &state.initiative_order[next_idx];
        if entry.is_alive && entry.is_conscious {
            state.active_index = next_idx;
            state.round += if next_idx == 0 { 1 } else { 0 };
            return;
        }
    }

    // ninguém vivo => combate encerra
}
```

### 10.2 Notificação

`CombatUpdate` com:

- `round`,
- `activeCreatureId`

VOICE:

- Mestre pode anunciar "novo turno" em momentos chave

---

## 11. Condições e Turn Engine

Condições com duração por turno são integradas ao turn engine:

- `UntilEndOfTurn(actor_id)`
- `UntilStartOfTurn(actor_id)`
- `Rounds(n)`

### 11.1 Ao iniciar turno

Engine:

- aplica efeitos de "start of turn"
- ex:
  - dano por sangramento
  - dano por fogo/ácido
- checa se a condição mata
- dispara `ConditionTicked` se necessário

### 11.2 Ao terminar turno

Engine:

- reduz `Rounds(n)` para `n-1`
- se chegar a 0 → remove condição
- gera `ConditionEnded` event

Mestre IA recebe:

```
[CONDITION_UPDATE]
CREATURE: npc_ogre
ENDED: frightened
[/CONDITION_UPDATE]
```

Narração:

> "O terror cego nos olhos do ogro se dissipa. Agora ele olha pra vocês com ódio lúcido."

---

## 12. Encerramento de Combate

### 12.1 Detecção

Combate termina quando:

- todos os hostis têm:
  - `is_alive == false` ou
  - `is_conscious == false` e sem intenção de continuar luta (capturados, rendidos).

### 12.2 Fluxo

Engine marca:

- `ended_at = now()`

Orquestrador:

- muda state global → `SceneState::SocialFreeFlow`
- emite `CombatUpdate` com `inCombat: false`
- dispara VOICE:
  - Mestre narra aftermath
- música:
  - muda pra "pós-combate"
- UI:
  - oculta barra de iniciativa
  - transiciona HUD para modo social/descanso
  - exibe resumos de loot, XP, condições persistentes

---

## 13. Logs e Replays

Cada ação em combate gera log:

```rust
struct CombatLogEntry {
    timestamp: Timestamp,
    actor_id: String,
    intent: Intent,          // DSL parseada
    outcome: Outcome,        // AttackOutcome, SpellOutcome, etc.
    narration_id: Option<String>,
}
```

Uso:

- repro de sessão
- debugger
- dataset para refinar Mestre IA
- replay cinematográfico futuro

---

## 14. Comportamento do Mestre IA dentro do Turn Engine

O Mestre IA não controla o Turn Engine.
Ele reage a eventos do Turn Engine:

- `NEW_ROUND`
- `TURN_START`
- `TURN_END`
- `DAMAGE_DEALT`
- `CREATURE_DOWNED`
- `CONDITION_APPLIED`
- `CONDITION_ENDED`
- `COMBAT_START`
- `COMBAT_END`

E transforma isso em VOZ + descrição:

> "O goblin cai."

> "O fogo lambe, mas não consome."

> "Seu braço perde a força."

INTENTs do Mestre em combate se limitam a:

- decidir ações de NPCs
- invocar AoE / magias
- ativar efeitos de cenário

---

## 15. Integração com UI (Baldur's Gate 3 / Solasta-like)

### 15.1 Topo — Iniciativa

- cards horizontais com portraits
- highlight no personagem ativo
- HP em barra fina

### 15.2 Centro — Battlemap

- grid 5ft
- tokens
- AoE highlighting
- line of sight preview (quando o user mira)

### 15.3 Bottom — Action Bar

ações possíveis para o ativo:

- Attack
- Cast
- Item
- Dash
- Disengage
- Help
- etc.

botões ficam desativados se economia de ações bloqueia

### 15.4 Direita — Combat Log

- rolagens (opcional com dados visuais)
- resultados
- condições aplicadas
- narrações curtas textuais

---

## 16. Monoplayer / Multiplayer

### 16.1 Single player (você + IA)

- todos os "players" reais = 1 humano
- os demais slots da party = Jogadores IA
- Turn Engine continua igual.

### 16.2 Multi-humano (futuro)

- cada `player_id` mapeia para um client
- INTENT do Mestre IA e Engine permanecem iguais
- Turn Engine apenas aceita ações de clientes diferentes
- rolagens são validadas por origem

---

## 17. Resumo Final

O Turn Engine do VRPG:

- garante D&D 5e honesto e determinístico em combate;
- separa completamente:
  - decisão narrativa (Mestre IA),
  - tática do jogador (humano/IA),
  - matemática das regras (Engine);
- entrega ciclo de turno micro-narrativo, onde cada ação:
  - nasce da fala do jogador,
  - vira INTENT estruturada,
  - é resolvida pela Engine,
  - volta ao jogador como:
    - voz,
    - imagem (battlemap + efeito),
    - UI clara.

Ele é a ponte entre a história que o Qwen conta
e o jogo que D&D exige.

---

## 18. Filosofia do Combate

O combate é:
- **determinístico**
- **cinematográfico**
- **controlado por turnos**
- **com narração momento a momento**

O Mestre IA não decide regras. Ao invés disso:
- **explica motivações dos NPCs, descreve os resultados**
- **e anuncia pedidos de rolagem para o jogador**

A Engine é a autoridade mecânica. O Mestre é a voz dramática.

### Narração por Ação

> **Cada ação gera sua resposta narrativa separada.**

Fluxo base:
1. Jogador declara → `"Eu ataco o goblin!"`
2. IA converte → `[INTENTS] MELEE_ATTACK ...`
3. Orquestrador valida → engine resolve rolagem
4. Engine calcula → outcome
5. IA narra **apenas essa ação**
6. UI atualiza mapa, HP, log
7. próximo passo do turno

### Ações que pedem rolagem do Jogador

O **Mestre pede**, não a UI.

Exemplo:
> "Para tentar abrir o portão sob ataque, faça um teste de Atletismo."

Então:
- Orquestrador dispara → `RollRequest`
- UI mostra → dado animado (BG3-like)
- Jogador rola
- Engine resolve
- IA narra o resultado

### INTENTS válidas em Combate

**Ataques**: MELEE_ATTACK, RANGED_ATTACK

**Magias**: SPELL_CAST, CANTRIP_CAST, SPELL_SHAPE (ex área)

**Economia**: DASH, DISENGAGE, DODGE, HELP

**Interação**: USE_ITEM, SHOVE, GRAPPLE, STABILIZE

### Condições — AUTO

A Engine aplica sem IA:
- Prone, Blinded, Paralyzed, Concentration, Grappled

E remove **automaticamente** quando expiram.

**Mestre notificado quando expira**:
```
[CONDITION_UPDATE]
CREATURE: npc_warrior_01
ENDED: paralyzed
[/CONDITION_UPDATE]
```

O Mestre narra:
> "Você vê o guerreiro recuperar os sentidos…"

### Encerrando Turno

**Ação concluída ou jogador fala "fim"**:
- Orquestrador finaliza economia
- Triggers: end-of-turn effects, concentration checks
- Avança iniciativa → próxima criatura

### Turno de NPC

IA escolhe as ações com INTENTs. Engine resolve tudo.

Ex:
```
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: npc_goblin_02
TARGET: player_1
WEAPON: weapon_dagger
END_INTENT
[/INTENTS]
```

Engine → aplica dano  
IA narra o golpe:
> "O goblin avança e finca a lâmina em seu flanco…"

### Mortes

Engine resolve morte RAW:
- se chegar a -maxHP → morte instantânea
- senão → bleedout (death saves)

IA narra tragédia.

### Ao sair do combate

**Condição**: nenhum hostile vivo ou consciente.

Orquestrador:
- `COMBAT_END`
- calcula XP/loot
- Engine limpa estados temporários
- IA narra aftermath
- retorna para SocialFreeFlow

### Performance

Como você escolheu **narração por ação**, a IA é consultada a cada microevento.

**Para isso funcionar sem latência**, é obrigatório:
- **Pipeline de 2 modelos**: Qwen 2.5 1.5B (reação rápida) + Qwen 2.5 14B (narrativa completa)
- Qwen 1.5B Q4_K_M local (< 1.2s para prelúdio)
- Qwen 14B Q4_K_M local (< 6s para narrativa completa)
- Whisper local
- TTS local (XTTS + SoVITS)
- Engine Rust
- sem API

**Regra de Ouro**: O 1.5B sempre responde antes do 14B para evitar silêncio cognitivo.

Isso imita mesa real com resposta imediata e qualidade narrativa completa.

**Ver especificações:**
- [QWEN_1_5B_SPEC.md](../QWEN_1_5B_SPEC.md)
- [QWEN_14B_SPEC.md](../QWEN_14B_SPEC.md)
- [PIPELINE_ARCHITECTURE.md](../PIPELINE_ARCHITECTURE.md)

### Logs

Todas INTENTS, resultados e narrações são gravados:
- replay
- clipping
- dataset de treinamento
- robustez futura

### UX Cinematográfica

- rolagem animada no centro (BG3)
- efeito sonoro por tipo de arma
- flash de dano
- partículas de magia
- HP shake
- fade to black em mortes

### Falhas

**Se parsing falha**:
- reprompt interno:
> "O Mestre não entendeu sua ação. Você atacou quem?"

**Se ainda falhar**:
- fallback mecânico básico:
  - ação ignorada
  - IA narra confusão
