# Manual dos Monstros - Tasks Detalhadas

## Fase 5.5: Sistema Completo de Fichas de Monstros (Monster Manual)

### 5.5.1 Estrutura Completa de Ficha de Monstro
**Task ID**: `implement-monster-stat-block-complete`

**Descrição**: Implementar estrutura completa de ficha de monstro conforme Manual dos Monstros.

**Tarefas**:
- [ ] Implementar estrutura `MonsterStatBlock` completa:
  - **Cabeçalho**:
    - Nome (com subtítulo, se aplicável)
    - Tamanho, tipo, alinhamento
  - **Armor Class**:
    - Valor de AC
    - Fonte (armor, natural armor, shield, etc.)
    - Exemplo: "15 (leather armor)" ou "18 (natural armor)"
  - **Hit Points**:
    - HP total
    - Fórmula de hit dice (ex: "45 (7d8 + 14)")
    - Hit dice type
  - **Speed**:
    - Walking speed (base)
    - Flying speed (se aplicável)
    - Swimming speed (se aplicável)
    - Climbing speed (se aplicável)
    - Burrowing speed (se aplicável)
    - Outros tipos especiais
  - **Ability Scores**:
    - STR (score e modifier)
    - DEX (score e modifier)
    - CON (score e modifier)
    - INT (score e modifier)
    - WIS (score e modifier)
    - CHA (score e modifier)
  - **Saving Throws**:
    - Lista de saving throws proficientes
    - Bônus de saving throw (ability modifier + proficiency)
  - **Skills**:
    - Lista de skills proficientes
    - Bônus de skill (ability modifier + proficiency)
  - **Damage Resistances/Immunities/Vulnerabilities**:
    - Lista completa de tipos de dano
  - **Condition Immunities**:
    - Lista de condições imunes
  - **Senses**:
    - Passive Perception
    - Darkvision, blindsight, tremorsense, etc.
    - Alcance de cada sentido
  - **Languages**:
    - Idiomas falados
    - Telepathy (se aplicável)
  - **Challenge Rating**:
    - CR (pode ser fração: 1/8, 1/4, 1/2)
    - XP value (baseado em CR)
    - Proficiency Bonus (baseado em CR)
- [ ] Implementar cálculo automático de Proficiency Bonus por CR
- [ ] Implementar cálculo automático de XP por CR
- [ ] Implementar busca completa no Vectorizer
- [ ] Implementar endpoint `/monsters/stat-block/{monster_name}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: ALTA

---

### 5.5.2 Traits e Habilidades Especiais
**Task ID**: `implement-monster-traits`

**Descrição**: Sistema de traits e habilidades especiais de monstros.

**Tarefas**:
- [ ] Implementar estrutura `MonsterTrait`:
  - Nome
  - Descrição
  - Tipo (passive, active, reaction)
  - Efeitos
- [ ] Implementar traits comuns:
  - [ ] Damage Resistance/Immunity/Vulnerability
  - [ ] Condition Immunity
  - [ ] Magic Resistance
  - [ ] Regeneration
  - [ ] Pack Tactics
  - [ ] Keen Senses
  - [ ] Amphibious
  - [ ] Outros
- [ ] Implementar busca de traits no Vectorizer
- [ ] Implementar endpoint `/monsters/{monster_name}/traits`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monster-stat-block-complete`  
**Prioridade**: ALTA

---

### 5.5.3 Ações Detalhadas de Monstros
**Task ID**: `implement-monster-actions-detailed`

**Descrição**: Sistema detalhado de todas as ações de monstros.

**Tarefas**:
- [ ] Implementar **Multiattack**:
  - Descrição de combinações
  - Número de ataques
  - Tipos de ataques disponíveis
- [ ] Implementar **Melee Weapon Attack**:
  - Nome da arma
  - To Hit: modificador + proficiency
  - Reach: alcance em pés
  - Um alvo
  - Hit: dano + tipo
  - Efeitos adicionais (se aplicável)
- [ ] Implementar **Ranged Weapon Attack**:
  - Nome da arma
  - To Hit: modificador + proficiency
  - Range: normal/long
  - Um alvo
  - Hit: dano + tipo
- [ ] Implementar **Breath Weapon**:
  - Tipo (fire, cold, acid, etc.)
  - Recharge (5-6, etc.)
  - Área de efeito
  - Saving throw
  - Dano
- [ ] Implementar **Spellcasting**:
  - Tipo (innate, spellcaster)
  - Spellcasting ability
  - Spell save DC
  - Spell attack modifier
  - Cantrips conhecidos
  - Spell slots por nível
  - Spells conhecidos/preparados
- [ ] Implementar busca de ações no Vectorizer
- [ ] Implementar endpoint `/monsters/{monster_name}/actions/detailed`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monster-stat-block-complete`  
**Prioridade**: ALTA

---

### 5.5.4 Legendary e Lair Actions Detalhadas
**Task ID**: `implement-legendary-lair-actions`

**Descrição**: Sistema completo de Legendary e Lair Actions.

**Tarefas**:
- [ ] Implementar **Legendary Actions**:
  - Número de ações por turno (geralmente 3)
  - Ações disponíveis com custo:
    - [ ] Custo 1: ações menores
    - [ ] Custo 2: ações médias
    - [ ] Custo 3: ações poderosas
  - Timing: no final do turno de outra criatura
  - Resolução de ações
- [ ] Implementar **Lair Actions**:
  - Condição: monstro deve estar em seu covil
  - Timing: iniciativa 20 (perde empates)
  - Ações disponíveis (lista completa)
  - Efeitos regionais (se aplicável)
- [ ] Implementar **Regional Effects**:
  - Efeitos quando monstro está na região
  - Efeitos após morte do monstro
- [ ] Implementar busca no Vectorizer
- [ ] Implementar endpoint `/monsters/{monster_name}/legendary-actions`
- [ ] Implementar endpoint `/monsters/{monster_name}/lair-actions`
- [ ] Implementar endpoint `/monsters/{monster_name}/regional-effects`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monster-stat-block-complete`  
**Prioridade**: ALTA

---

### 5.5.5 Tabela Completa de XP por CR
**Task ID**: `implement-complete-xp-table`

**Descrição**: Tabela completa de XP por Challenge Rating do Manual dos Monstros.

**Tarefas**:
- [ ] Implementar tabela completa CR → XP:
  ```rust
  CR 0 = 0 XP
  CR 1/8 = 25 XP
  CR 1/4 = 50 XP
  CR 1/2 = 100 XP
  CR 1 = 200 XP
  CR 2 = 450 XP
  CR 3 = 700 XP
  CR 4 = 1,100 XP
  CR 5 = 1,800 XP
  CR 6 = 2,300 XP
  CR 7 = 2,900 XP
  CR 8 = 3,900 XP
  CR 9 = 5,000 XP
  CR 10 = 5,900 XP
  CR 11 = 7,200 XP
  CR 12 = 8,400 XP
  CR 13 = 10,000 XP
  CR 14 = 11,500 XP
  CR 15 = 13,000 XP
  CR 16 = 15,000 XP
  CR 17 = 18,000 XP
  CR 18 = 20,000 XP
  CR 19 = 22,000 XP
  CR 20 = 25,000 XP
  CR 21 = 33,000 XP
  CR 22 = 41,000 XP
  CR 23 = 50,000 XP
  CR 24 = 62,000 XP
  CR 25 = 75,000 XP
  CR 26 = 90,000 XP
  CR 27 = 105,000 XP
  CR 28 = 120,000 XP
  CR 29 = 135,000 XP
  CR 30 = 155,000 XP
  ```
- [ ] Implementar função de conversão CR → XP
- [ ] Implementar função de conversão XP → CR (aproximado)
- [ ] Implementar validação de CR válido
- [ ] Implementar endpoint `/monsters/xp-table`
- [ ] Implementar endpoint `/monsters/cr-to-xp/{cr}`
- [ ] Implementar endpoint `/monsters/xp-to-cr/{xp}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: Nenhuma  
**Prioridade**: ALTA

---

### 5.5.6 Tabela de Proficiency Bonus por CR
**Task ID**: `implement-proficiency-bonus-table`

**Descrição**: Tabela de Proficiency Bonus baseado em CR.

**Tarefas**:
- [ ] Implementar tabela CR → Proficiency Bonus:
  ```rust
  CR 0-1/4 = +2
  CR 1/2-1 = +2
  CR 2-4 = +2
  CR 5-8 = +3
  CR 9-12 = +4
  CR 13-16 = +5
  CR 17-20 = +6
  CR 21-24 = +7
  CR 25-28 = +8
  CR 29-30 = +8
  ```
- [ ] Implementar função de conversão CR → Proficiency Bonus
- [ ] Implementar cálculo automático em stat blocks
- [ ] Implementar endpoint `/monsters/proficiency-bonus/{cr}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: Nenhuma  
**Prioridade**: ALTA

---

## Integração com Sistema Existente

Todas essas tasks devem integrar-se com:
- `game-engine` para uso em combate
- `memory-service` para busca no Vectorizer
- `rules5e-service` para cálculos de dano/ataque
- Sistema de encontros para balanceamento



