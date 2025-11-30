# VRPG – Engine de Regras (D&D 5e)

## 1. Objetivo

A Engine é a **fonte de verdade mecânica** do VRPG.

Ela:

- resolve **tudo que é número**:
  - to-hit,
  - AC,
  - dano,
  - DC,
  - saving throws,
  - iniciativa,
  - condições.
- aplica **efeitos 5e** sem depender do Mestre IA.
- garante **coerência absoluta** em combate e exploração.

> **O Mestre IA NUNCA calcula nada.  
> Ele apenas narra.**

---

## 2. Princípios

### 2.1 Determinística

Dado o mesmo estado e seed → sempre o mesmo resultado.

### 2.2 Stateless por chamada

Cada operação recebe contexto e retorna outcome puro.

### 2.3 Isolada da IA

Nenhum modelo LLM interfere na lógica.

### 2.4 Rastreadora de estados

A Engine mantém um "CombatState" completo,  
com condições, turnos e efeitos.

---

## 3. Controle de Condições (sua escolha)

- **AUTO** = Engine aplica, mantém e remove
- **NOTIFICAÇÃO** = Quando uma condição termina → envia evento para IA narrar

> O Mestre IA vira "voz dramática" do sistema mecânico.

---

## 4. Modelo Mental: "Planilha perfeita D&D 5e"

> Engine = Excel do mestre, só que impossível de errar.

Ela é o juiz silencioso:

- aplica regras frias,
- loga eventos,
- não improvisa.

---

## 5. Representações Centrais

### 5.1 Creature (Personagem, NPC, Monstro)

```rust
struct Creature {
    id: String,
    name: String,
    creature_type: CreatureType,       // Player | NPC | Monster
    level: u8,

    // base
    abilities: Abilities,              // STR/DEX/CON/INT/WIS/CHA
    proficiency_bonus: i8,
    armor_class: i16,

    // HP
    max_hp: i32,
    current_hp: i32,

    // Saves & skills
    saving_throws: SavingThrowProficiencies,
    skills: SkillProficiencies,

    // D&D state
    conditions: Vec<ConditionState>,   // auto gerido
    inventory: Vec<ItemRef>,
    spells: Vec<SpellRef>,
    features: Vec<FeatureRef>,
}
```

### 5.2 CombatState

```rust
struct CombatState {
    encounter_id: String,
    round: u32,
    active_index: usize,
    initiative_order: Vec<InitiativeEntry>,
    participants: HashMap<String, CreatureCombatState>,
}
```

#### 5.2.1 InitiativeEntry

```rust
struct InitiativeEntry {
    creature_id: String,
    initiative_value: i16,
    is_alive: bool,
}
```

#### 5.2.2 CreatureCombatState

Estado parcial que não existe fora de combate:

- HP no momento
- condições temporárias
- ações usadas no turno
- flags de reação

---

## 6. Condições (AUTO)

### 6.1 Representação

```rust
struct ConditionState {
    kind: ConditionKind,     // Prone, Blinded, Charmed, Grappled...
    source: Option<String>,  // efeito, criatura, item
    stacks: u8,
    duration: DurationType,  // UntilSave, UntilEndTurn, Rounds(n)...
}
```

### 6.2 DurationType

```rust
enum DurationType {
    UntilEndOfTurn(String),    // creature_id
    UntilStartOfTurn(String),
    Rounds(u32),
    Permanent,
}
```

---

## 7. Eventos Automáticos

A Engine dispara eventos internos:

- ConditionApplied { creature, kind, duration }
- ConditionEnded { creature, kind }
- Death { creature }
- Knockdown { creature }
- ConcentrationLost { caster }

Esses eventos são reenviados para o Orquestrador.

O Mestre IA recebe:

- "condição terminou"
- contexto do que aconteceu
- nunca a matemática

E narra:

> "A paralisia esvai-se lentamente dos seus tendões."

---

## 8. Rolagens determinísticas

### 8.1 RNG não é "random.local()"

Ele segue:

```rust
Rng(seed_session + seed_combat + seed_turn)
```

Por quê?

- replay determinístico
- multiplayer sem cheat
- gravação de sessões

---

## 9. Fluxos Mecânicos

### 9.1 Iniciativa

```rust
roll = 1d20 + DEX_MOD + bonus
```

Engine ordena.  
Orquestrador expõe UI.

### 9.2 Ataque Corpo-a-corpo

**Entrada**

- atacante
- arma
- alvo
- vantagem/desvantagem

**Execução**

- rolar d20
- aplicar modifiers
- comparar AC
- se hit → rolar dano
- aplicar resist/vulner
- atualizar HP
- condições adicionais
- emitir eventos

**Exemplo de retorno**

```rust
AttackOutcome {
    hit: true,
    critical: false,
    d20: 17,
    roll_total: 22,
    target_ac: 15,
    damage: 9,
    target_hp_after: 3,
    applied_conditions: [],
}
```

### 9.3 Saving throws

- d20 + ability_mod + prof_if_proficient
- falha/sucesso
- condição / dano
- eventos

### 9.4 Magias

Regras de 5e:

- Slot level ≥ spell level
- saves aplicados pelo alvo
- concentration

**Concentration Loss**

- falha save → evento

### 9.5 Derrubar (Prone)

- hit com ataque corpo a corpo > threshold
- crítico + arma pesada
- efeitos de magia
- grapples

A Engine decide, aplica e notifica.

---

## 10. Combate Turn-Based

Ciclo:

1. verificar active_creature
2. setar action economy
3. processar intenção
4. resolver mecânica
5. emitir eventos
6. avançar turno

---

## 11. Expiração de Condições

Essa é a parte que você pediu:

Engine remove automaticamente

Exemplo:

- UntilEndOfTurn(player_1)
- quando turno termina → condição sai

Orquestrador recebe evento:

```
ConditionEnded(creature_id="npc_guard_01", kind="charm")
```

Orquestrador envia ao Mestre IA:

```
<CONDITION_UPDATE>
CREATURE npc_guard_01
ENDED charm
</CONDITION_UPDATE>
```

Não narra. Só informa.

Mestre IA então narra:

> "Você vê os olhos do guarda voltarem ao normal…"

---

## 12. "Narração de Estado" NÃO é Engine

A Engine NÃO diz:

- "o goblin cambaleia"
- "a magia se dissipa com um brilho verde"
- "o troll ruge e recua"

Isso é papel do Mestre IA.

---

## 13. Carga pesada — MVP otimizado

Todas as chamadas mecânicas:

- O(1) ou O(n) trivial
- sem IA
- sem rede
- sem GPU

Isso torna:

- combate rápido
- multiplayer viável
- mobile futuramente possível

---

## 14. Multiplayer sem fraude

Jogadores não podem alterar valores.

O client só rola o dado

Engine valida pelo seed e fórmula

se mismatch → log + hack flag

---

## 15. Edge cases obrigatórios

- atingir 0 HP:
  - entrar em unconscious
- dano massivo:
  - morte instantânea
- resistências múltiplas
- vulnerabilities (+ x2)
- cover
- concentração de múltiplas magias

---

## 16. Testes

Pacote obrigatório de testes:

- 2000+ testes unitários
- testes determinísticos por seed
- "battle scenarios"
- regressão após patches

---

## 17. Filosofia final

A Engine é indiscutível.  
O Mestre é interpretação.  
O Jogador é escolha.

