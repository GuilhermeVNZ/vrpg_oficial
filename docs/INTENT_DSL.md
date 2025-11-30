# VRPG – Esquema de INTENTS (DSL)

## 1. Propósito

A INTENT **não é narração** e **não é comando genérico**.  
Ela é uma *declaração semântica de intenção*, enviada pelo Mestre IA para que o Orquestrador execute mecânicas concretas (Engine D&D, consulta a banco vetorial, geração de assets).

A INTENT deve ser:

- **Determinística** (sem ambiguidade).
- **Formal** (parser 100% previsível).
- **Não-mecânica** (IA descreve intenção, nunca resolve regra).
- **Independente de UI** (não "clique no botão X").

---

## 2. Fundamentos do Design

### 2.1 Linguagem DSL

Você escolheu **DSL (Modelo B)** e isso é **altamente correto** para VRPG.

Motivos:

- **Legível para humanos** (debug, logs).
- **Treinável ao LLM** (muito mais robusto que JSON).
- **Minimiza formações inválidas**.
- **Permite expressividade narrativa**.

O DSL segue princípios:

> 1 Intent = 1 bloco bem definido  
> Campos = linhas `KEY: VALUE`  
> Fechamento explícito = `END_INTENT`

**Sem vírgulas, sem colchetes, sem JSON deep nesting.**

---

## 3. Forma Base

```
[INTENTS]
INTENT: <TYPE>
<KEY>: <VALUE>
...
END_INTENT
[/INTENTS]
```

- `[INTENTS]` abre o bloco
- `INTENT:` inicia um objeto
- `END_INTENT` finaliza
- `[/INTENTS]` fecha o agrupamento

Qualquer texto antes/depois do bloco é ignorado pelo parser.

---

## 4. Convenções da DSL

### 4.1 Case

- **TYPE** e **KEY**: Upper snake case ou Upper camel → `MELEE_ATTACK`, `SpellCast`
- **VALUES** podem ser strings livres → `"convencer o guarda"`

### 4.2 Booleanos

`YES | NO`

### 4.3 Identificadores

- `player_1`
- `npc_guard_01`
- `weapon_longsword_01`

IDs são **chaves do runtime**, não nomes livres.

---

## 5. Categorias de INTENT

As INTENTs são organizadas em "famílias".

### FAMÍLIA A — Social/Roleplay

- `SKILL_CHECK`
- `LORE_QUERY`
- `RULE_QUERY`
- `NPC_DIALOGUE`
- `SCENE_EVENT`

### FAMÍLIA B — Exploração

- `INVESTIGATE_AREA`
- `SEARCH_ITEM`
- `INTERACT_OBJECT`

### FAMÍLIA C — Combate

- `MELEE_ATTACK`
- `RANGED_ATTACK`
- `SPELL_CAST`
- `USE_ITEM`
- `READY_ACTION`
- `DASH`
- `DISENGAGE`
- `HELP`
- `COMBAT_START`
- `COMBAT_END`

### FAMÍLIA D — Assets

- `GENERATE_PORTRAIT`
- `GENERATE_SCENE`
- `GENERATE_BATTLEMAP`

> **Observação**: IA não escolhe ferramenta diretamente → *orquestrador decide o executor correto*.

---

## 6. Exemplos Formais

### 6.1 Social: Skill Check

```
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda a liberar a entrada"
SUGGEST_DC: YES
END_INTENT
[/INTENTS]
```

#### Semântica:

- A IA quer que **o jogador role** Persuasão.
- DC não está definida → Engine decide (consultando RAG caso necessário).

### 6.2 Social: Lore Query

```
[INTENTS]
INTENT: LORE_QUERY
QUERY: "história do necromante de Waterdeep"
SCOPE: region
END_INTENT
[/INTENTS]
```

- Orquestrador → Hive (Vectorizer/Lexum/Nexus)
- Anexa resultados numa nova chamada para Qwen.

### 6.3 Combate: Ataque Corpo-a-Corpo

```
[INTENTS]
INTENT: MELEE_ATTACK
ACTOR: player_1
TARGET: npc_goblin_02
WEAPON: weapon_shortsword_01
MOVE_REQUIRED: YES
END_INTENT
[/INTENTS]
```

#### Semântica:

- Player_1 quer atacar com espada curta
- Precisa mover até o alvo → orquestrador calcula alcance e path
- Engine resolve ataque e dano

### 6.4 Combate: Magia de Área

```
[INTENTS]
INTENT: SPELL_CAST
ACTOR: player_1
SPELL: fireball
SLOT_LEVEL: 3
AREA_CENTER: "15,8"
TARGETS: npc_troll_01, npc_goblin_03, npc_goblin_04
END_INTENT
[/INTENTS]
```

---

## 7. Parsing (Rust)

> **Não use regex lixo.  
> Use um parser determinístico linha a linha.**

### 7.1 Gramática simplificada

```
Block := "[INTENTS]" { Intent } "[/INTENTS]"
Intent := "INTENT:" TYPE { KeyVal } "END_INTENT"
KeyVal := KEY ":" VALUE
```

Sem nesting, sem loops.

---

## 8. Tipagem Interna (Rust)

Conversão DSL → Enum:

```rust
pub enum Intent {
    SkillCheck {
        actor: String,
        skill: String,
        target: Option<String>,
        context: Option<String>,
        suggest_dc: bool,
    },
    MeleeAttack {
        actor: String,
        target: String,
        weapon: Option<String>,
        move_required: bool,
    },
    SpellCast {
        actor: String,
        spell: String,
        slot_level: u8,
        area_center: Option<(i32, i32)>,
        targets: Vec<String>,
    },
    LoreQuery {
        query: String,
        scope: Option<String>,
    },
    RuleQuery {
        query: String,
        context: Option<String>,
    },
    GenerateBattlemap { id: String },
    CombatStart { reason: Option<String> },
    CombatEnd { reason: Option<String> },
}
```

---

## 9. Normalização

### 9.1 Sanitização

- trim whitespace
- remover aspas redundantes
- lower camel para SKILL
- garantir IDs válidos no runtime

### 9.2 Inferência

Se IA omite campo não crítico:

- MOVE_REQUIRED ausente → NO
- SUGGEST_DC ausente → YES
- TARGET ausente (skill social genérica) → None

Nunca inventar valores mecânicos.

---

## 10. Validações

### 10.1 Fora de combate

- MELEE_ATTACK → bloquear
- SPELL_CAST ofensivo → bloquear
- DISENGAGE etc. → bloquear

### 10.2 Em combate

- NPC_DIALOGUE → permitido
- INVESTIGATE_AREA → permitir só se ação usa turno inteiro ou regra custom

---

## 11. Edge Cases

### 11.1 Target morto

Orquestrador recusa INTENT

IA recebe:

"O alvo está inconsciente/morto."

### 11.2 Score de skill impossíveis

- Rola normal
- Usa DC padrão
- Mestre narra falha com elegância

### 11.3 Omissão deliberada

IA pode gerar INTENT parcial

Exemplo: ASK_FOR_INFO

Orquestrador → Hive → IA → INTENT concreta

---

## 12. Performance & Robustez

- DSL permite prompt mais curto
- 1–3 INTENTs por call
- parsing O(n)
- zero custo estrutural

---

## 13. Anti-Hallucination

Quanto mais o DSL limitar estrutura:

- Menos chance da IA inventar
- Treinamento: "role play first, DSL second"
- Criança aprende "peça ação", não "resolva ação"

---

## 14. Logging

Salvar blocos brutos do Qwen antes do parsing:

- Debug de comportamento
- Replay determinístico
- Treinamento futuro

---

## 15. Segurança

Proibir no DSL:

- toolcall,
- nomes de API,
- linguagem técnica.

IA nunca deve dizer:

"chame a função resolveAttack()"

Ela só diz:

```
INTENT: MELEE_ATTACK
```

---

## 16. Futuro

INTENTS podem receber versões:

```
INTENT: SPELL_CAST_V2
...
END_INTENT
```

Sem quebrar parsing legado.

