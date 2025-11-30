# VRPG Client - Tasks Completas D&D 5e
## Plano Completo para Mesa de RPG

Este documento lista **TODAS as tasks necessárias** para implementar um sistema completo de mesa de RPG D&D 5e, baseado nas regras consultadas via Vectorizer + Lexum + Nexus.

**Data de Criação**: 2025-11-23  
**Baseado em**: Regras oficiais D&D 5e (Livro do Jogador, Guia do Mestre, Manual dos Monstros)  
**Status Atual**: Estrutura base implementada, expandindo para cobertura completa

---

## 📊 Status de Implementação Atual

### ✅ Implementado
- **Dice Rolling**: Rolagem de dados com advantage/disadvantage
- **Attack Resolution**: Resolução de ataques com críticos
- **Damage Calculation**: Cálculo de dano com resistências/vulnerabilidades
- **Ability Checks**: Testes de habilidade com proficiência
- **Saving Throws**: Salvaguardas
- **Conditions**: Sistema básico de condições

### ⚠️ Parcialmente Implementado
- **Game Engine**: Estrutura básica (sessões, cenas, atores, turnos) - **Nota**: Será refatorado para trabalhar com Orquestrador
- **Memory Service**: Integração com Vectorizer/Nexus/Lexum
- **Orquestrador**: **NOVO** - Módulo central de coordenação (em planejamento)
- **INTENT DSL**: **NOVO** - Sistema de intenções estruturadas (em planejamento)
- **Turn Engine**: **NOVO** - Sistema completo de combate em turnos (em planejamento)
- **Voice INTENTS**: **NOVO** - Sistema de intenções de voz (em planejamento)

### ❌ Não Implementado
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

---

## 🎯 Fase 1: Sistema de Personagem (Character System)

### 1.1 Atributos e Modificadores
**Task ID**: `implement-ability-scores`

**Descrição**: Sistema completo de atributos (Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma) e seus modificadores.

**Tarefas**:
- [ ] Implementar estrutura `AbilityScores` com os 6 atributos
- [ ] Implementar cálculo de modificadores (score - 10) / 2, arredondado para baixo
- [ ] Implementar geração de atributos (point buy, standard array, rolling)
- [ ] Implementar aumentos de atributo (raças, classes, níveis)
- [ ] Implementar limite máximo de atributos (20, 30 com magia)
- [ ] Implementar endpoint `/ability-scores/calculate-modifier`
- [ ] Implementar endpoint `/ability-scores/generate`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `setup-project-base`  
**Prioridade**: ALTA (base para tudo)

---

### 1.2 Raças (Races)
**Task ID**: `implement-races`

**Descrição**: Sistema completo de raças D&D 5e com traits raciais.

**Tarefas**:
- [ ] Implementar estrutura `Race` com:
  - Nome, tamanho, velocidade
  - Aumentos de atributo
  - Traits raciais (darkvision, resistências, etc.)
  - Idiomas
  - Sub-raças (se aplicável)
- [ ] Implementar todas as raças do PHB:
  - [ ] Humano (Human)
  - [ ] Elfo (Elf) - High, Wood, Dark
  - [ ] Anão (Dwarf) - Hill, Mountain
  - [ ] Halfling - Lightfoot, Stout
  - [ ] Draconato (Dragonborn)
  - [ ] Gnomo (Gnome) - Forest, Rock
  - [ ] Meio-Elfo (Half-Elf)
  - [ ] Meio-Orc (Half-Orc)
  - [ ] Tiefling
- [ ] Implementar busca de raças no Vectorizer
- [ ] Implementar endpoint `/races/list`
- [ ] Implementar endpoint `/races/get/{race_name}`
- [ ] Implementar aplicação de traits raciais ao personagem
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-ability-scores`, `memory-service`  
**Prioridade**: ALTA

---

### 1.3 Classes (Classes)
**Task ID**: `implement-classes`

**Descrição**: Sistema completo de classes D&D 5e com features por nível.

**Tarefas**:
- [ ] Implementar estrutura `Class` com:
  - Nome, hit dice, proficiências
  - Saving throw proficiencies
  - Skill proficiencies
  - Equipment proficiencies
  - Features por nível
  - Spellcasting (se aplicável)
- [ ] Implementar todas as classes do PHB:
  - [ ] Bárbaro (Barbarian)
  - [ ] Bardo (Bard)
  - [ ] Clérigo (Cleric)
  - [ ] Druida (Druid)
  - [ ] Guerreiro (Fighter)
  - [ ] Monge (Monk)
  - [ ] Paladino (Paladin)
  - [ ] Patrulheiro (Ranger)
  - [ ] Ladino (Rogue)
  - [ ] Feiticeiro (Sorcerer)
  - [ ] Bruxo (Warlock)
  - [ ] Mago (Wizard)
- [ ] Implementar progressão por nível (features, hit dice, proficiência)
- [ ] Implementar busca de classes no Vectorizer
- [ ] Implementar endpoint `/classes/list`
- [ ] Implementar endpoint `/classes/get/{class_name}`
- [ ] Implementar endpoint `/classes/level-progression/{class_name}/{level}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-ability-scores`, `memory-service`  
**Prioridade**: ALTA

---

### 1.4 Antecedentes (Backgrounds)
**Task ID**: `implement-backgrounds`

**Descrição**: Sistema completo de antecedentes D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Background` com:
  - Nome, descrição
  - Skill proficiencies
  - Tool proficiencies
  - Languages
  - Equipment
  - Feature (trait especial)
- [ ] Implementar antecedentes do PHB:
  - [ ] Acolito (Acolyte)
  - [ ] Artesão de Guilda (Guild Artisan)
  - [ ] Artista (Entertainer)
  - [ ] Charlatão (Charlatan)
  - [ ] Criminoso (Criminal)
  - [ ] Eremita (Hermit)
  - [ ] Forasteiro (Outlander)
  - [ ] Herói do Povo (Folk Hero)
  - [ ] Nobre (Noble)
  - [ ] Sábio (Sage)
  - [ ] Marinheiro (Sailor)
  - [ ] Soldado (Soldier)
- [ ] Implementar busca de antecedentes no Vectorizer
- [ ] Implementar endpoint `/backgrounds/list`
- [ ] Implementar endpoint `/backgrounds/get/{background_name}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: MÉDIA

---

### 1.5 Talentos (Feats)
**Task ID**: `implement-feats`

**Descrição**: Sistema completo de talentos D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Feat` com:
  - Nome, descrição
  - Prerequisites (atributos, nível, etc.)
  - Efeitos (aumentos de atributo, habilidades especiais)
- [ ] Implementar talentos do PHB e suplementos
- [ ] Implementar busca de talentos no Vectorizer
- [ ] Implementar validação de prerequisites
- [ ] Implementar aplicação de efeitos ao personagem
- [ ] Implementar endpoint `/feats/list`
- [ ] Implementar endpoint `/feats/get/{feat_name}`
- [ ] Implementar endpoint `/feats/validate/{feat_name}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-ability-scores`, `memory-service`  
**Prioridade**: MÉDIA

---

### 1.6 Perícias (Skills)
**Task ID**: `implement-skills`

**Descrição**: Sistema completo de perícias D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Skill` com:
  - Nome, ability associada
  - Descrição
- [ ] Implementar todas as 18 perícias:
  - [ ] Acrobacia (Dexterity)
  - [ ] Adestrar Animais (Wisdom)
  - [ ] Arcanismo (Intelligence)
  - [ ] Atletismo (Strength)
  - [ ] Atuação (Charisma)
  - [ ] Enganação (Charisma)
  - [ ] Furtividade (Dexterity)
  - [ ] História (Intelligence)
  - [ ] Intimidação (Charisma)
  - [ ] Intuição (Wisdom)
  - [ ] Investigação (Intelligence)
  - [ ] Medicina (Wisdom)
  - [ ] Natureza (Intelligence)
  - [ ] Percepção (Wisdom)
  - [ ] Persuasão (Charisma)
  - [ ] Prestidigitação (Dexterity)
  - [ ] Religião (Intelligence)
  - [ ] Sobrevivência (Wisdom)
- [ ] Implementar cálculo de modificador de perícia:
  - Modificador de habilidade + proficiência (se aplicável) + expertise (se aplicável)
- [ ] Implementar endpoint `/skills/list`
- [ ] Implementar endpoint `/skills/calculate-modifier`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-ability-scores`  
**Prioridade**: ALTA

---

## 🗡️ Fase 2: Sistema de Equipamentos (Equipment System)

### 2.1 Armas (Weapons)
**Task ID**: `implement-weapons`

**Descrição**: Sistema completo de armas D&D 5e com tabelas e propriedades.

**Tarefas**:
- [ ] Implementar estrutura `Weapon` com:
  - Nome, tipo (simples/marcial)
  - Categoria (corpo a corpo/à distância)
  - Dano (dice expression)
  - Tipo de dano (slashing, piercing, bludgeoning)
  - Propriedades (versatile, finesse, two-handed, etc.)
  - Alcance (melee/range)
  - Custo, peso
- [ ] Implementar tabela completa de armas do PHB:
  - [ ] Armas simples corpo a corpo
  - [ ] Armas simples à distância
  - [ ] Armas marciais corpo a corpo
  - [ ] Armas marciais à distância
- [ ] Implementar propriedades especiais:
  - [ ] Versatile (dano alternativo com duas mãos)
  - [ ] Finesse (usa Dex ou Str)
  - [ ] Two-handed (requer duas mãos)
  - [ ] Light (pode usar duas armas)
  - [ ] Heavy (pequenos têm desvantagem)
  - [ ] Reach (alcance aumentado)
  - [ ] Thrown (pode ser arremessada)
  - [ ] Ammunition (requer munição)
  - [ ] Loading (requer ação para recarregar)
- [ ] Implementar busca de armas no Vectorizer
- [ ] Implementar cálculo de dano de arma
- [ ] Implementar endpoint `/weapons/list`
- [ ] Implementar endpoint `/weapons/get/{weapon_name}`
- [ ] Implementar endpoint `/weapons/calculate-damage`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `implement-dice-rolling`  
**Prioridade**: ALTA

---

### 2.2 Armaduras (Armor)
**Task ID**: `implement-armor`

**Descrição**: Sistema completo de armaduras D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Armor` com:
  - Nome, tipo (light/medium/heavy/shield)
  - Armor Class (AC)
  - Strength requirement
  - Stealth disadvantage
  - Custo, peso
- [ ] Implementar tabela completa de armaduras:
  - [ ] Armaduras leves
  - [ ] Armaduras médias
  - [ ] Armaduras pesadas
  - [ ] Escudos
- [ ] Implementar cálculo de AC:
  - [ ] AC base da armadura
  - [ ] Modificador Dex (limitado por tipo de armadura)
  - [ ] Escudo (+2)
  - [ ] Outros modificadores
- [ ] Implementar busca de armaduras no Vectorizer
- [ ] Implementar endpoint `/armor/list`
- [ ] Implementar endpoint `/armor/get/{armor_name}`
- [ ] Implementar endpoint `/armor/calculate-ac`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `implement-ability-scores`  
**Prioridade**: ALTA

---

### 2.3 Equipamentos Gerais (Adventuring Gear)
**Task ID**: `implement-equipment`

**Descrição**: Sistema completo de equipamentos de aventura D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Equipment` com:
  - Nome, tipo (gear, tool, consumable, etc.)
  - Descrição
  - Custo, peso
  - Propriedades especiais
- [ ] Implementar categorias:
  - [ ] Equipamentos de aventura
  - [ ] Ferramentas
  - [ ] Itens de montaria e veículos
  - [ ] Equipamentos de acampamento
  - [ ] Equipamentos de exploração
- [ ] Implementar busca de equipamentos no Vectorizer
- [ ] Implementar endpoint `/equipment/list`
- [ ] Implementar endpoint `/equipment/get/{item_name}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: MÉDIA

---

### 2.4 Sistema de Inventário
**Task ID**: `implement-inventory`

**Descrição**: Sistema completo de gerenciamento de inventário.

**Tarefas**:
- [ ] Implementar estrutura `Inventory` com:
  - Itens equipados (armas, armaduras, etc.)
  - Itens carregados (bolsa, mochila)
  - Capacidade (peso máximo)
  - Moedas (pp, gp, ep, sp, cp)
- [ ] Implementar cálculos:
  - [ ] Peso total do inventário
  - [ ] Capacidade restante
  - [ ] Encumbrance (sobrecarga)
- [ ] Implementar operações:
  - [ ] Adicionar item
  - [ ] Remover item
  - [ ] Equipar item
  - [ ] Desequipar item
  - [ ] Trocar moedas
- [ ] Implementar endpoint `/inventory/get/{character_id}`
- [ ] Implementar endpoint `/inventory/add-item`
- [ ] Implementar endpoint `/inventory/remove-item`
- [ ] Implementar endpoint `/inventory/equip-item`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-weapons`, `implement-armor`, `implement-equipment`  
**Prioridade**: ALTA

---

## ⚔️ Fase 3: Sistema de Combate (Combat System)

### 3.1 Iniciativa e Ordem de Turnos
**Task ID**: `implement-initiative`

**Descrição**: Sistema completo de iniciativa e ordem de turnos.

**Tarefas**:
- [ ] Implementar cálculo de iniciativa:
  - [ ] 1d20 + modificador de Dexterity
  - [ ] Modificadores especiais (feats, magias)
- [ ] Implementar ordenação de turnos:
  - [ ] Ordenar por iniciativa (maior para menor)
  - [ ] Resolver empates (Dexterity, aleatório)
- [ ] Implementar estrutura `InitiativeTracker`:
  - [ ] Lista ordenada de participantes
  - [ ] Turno atual
  - [ ] Round atual
- [ ] Implementar progressão de turnos:
  - [ ] `nextTurn()` - avança para próximo turno
  - [ ] `nextRound()` - avança para próximo round
  - [ ] `getCurrentActor()` - retorna ator atual
- [ ] Integrar com `game-engine` (via Orquestrador)
- [ ] Implementar endpoint `/combat/initiative/roll`
- [ ] Implementar endpoint `/combat/initiative/order`
- [ ] Implementar endpoint `/combat/turn/next`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-ability-scores`, `game-engine` (via Orquestrador)  
**Prioridade**: ALTA

---

### 3.2 Ações em Combate
**Task ID**: `implement-combat-actions`

**Descrição**: Sistema completo de ações disponíveis em combate.

**Tarefas**:
- [ ] Implementar tipos de ação:
  - [ ] Action (ação padrão)
  - [ ] Bonus Action (ação bônus)
  - [ ] Reaction (reação)
  - [ ] Movement (movimento)
  - [ ] Free Action (ação livre)
- [ ] Implementar ações padrão:
  - [ ] Attack (ataque)
  - [ ] Cast a Spell (lançar magia)
  - [ ] Dash (correr)
  - [ ] Disengage (desengajar)
  - [ ] Dodge (esquivar)
  - [ ] Help (ajudar)
  - [ ] Hide (esconder)
  - [ ] Ready (preparar)
  - [ ] Search (procurar)
  - [ ] Use an Object (usar objeto)
- [ ] Implementar ações bônus:
  - [ ] Offhand Attack (ataque com arma secundária)
  - [ ] Certain Spells (magias específicas)
  - [ ] Class Features (features de classe)
- [ ] Implementar reações:
  - [ ] Opportunity Attack (ataque de oportunidade)
  - [ ] Certain Spells (magias específicas)
  - [ ] Class Features (features de classe)
- [ ] Implementar validação de ações:
  - [ ] Verificar se ação está disponível
  - [ ] Verificar recursos necessários
  - [ ] Verificar condições
- [ ] Implementar endpoint `/combat/actions/list`
- [ ] Implementar endpoint `/combat/actions/validate`
- [ ] Implementar endpoint `/combat/actions/execute`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-initiative`, `implement-attack-resolution`  
**Prioridade**: ALTA

---

### 3.3 Movimento em Combate
**Task ID**: `implement-combat-movement`

**Descrição**: Sistema completo de movimento em combate.

**Tarefas**:
- [ ] Implementar velocidade base:
  - [ ] Velocidade de caminhada (raça)
  - [ ] Velocidade de corrida (Dash)
  - [ ] Velocidade de escalada
  - [ ] Velocidade de natação
  - [ ] Velocidade de voo (se aplicável)
- [ ] Implementar restrições de movimento:
  - [ ] Terreno difícil (custa o dobro)
  - [ ] Obstáculos
  - [ ] Condições (grappled, restrained, etc.)
- [ ] Implementar tipos de movimento:
  - [ ] Normal movement
  - [ ] Dash (dobra velocidade)
  - [ ] Disengage (não provoca oportunidade)
- [ ] Implementar cálculo de distância:
  - [ ] Distância euclidiana
  - [ ] Distância de grade (se aplicável)
- [ ] Integrar com `game-engine` (via Orquestrador, posicionamento)
- [ ] Implementar endpoint `/combat/movement/calculate-speed`
- [ ] Implementar endpoint `/combat/movement/validate`
- [ ] Implementar endpoint `/combat/movement/execute`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-races`, `game-engine` (via Orquestrador)  
**Prioridade**: ALTA

---

### 3.4 Ataques Múltiplos e Two-Weapon Fighting
**Task ID**: `implement-multiple-attacks`

**Descrição**: Sistema de múltiplos ataques e combate com duas armas.

**Tarefas**:
- [ ] Implementar Extra Attack (classes):
  - [ ] Número de ataques por nível
  - [ ] Aplicação de modificadores
- [ ] Implementar Two-Weapon Fighting:
  - [ ] Requisitos (armas light)
  - [ ] Ataque bônus (sem modificador de atributo)
  - [ ] Feat: Two-Weapon Fighting Style (com modificador)
- [ ] Implementar validação:
  - [ ] Verificar se pode fazer múltiplos ataques
  - [ ] Verificar se pode usar duas armas
- [ ] Integrar com sistema de ataques existente
- [ ] Implementar endpoint `/combat/attacks/multiple`
- [ ] Implementar endpoint `/combat/attacks/two-weapon`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-combat-actions`, `implement-weapons`  
**Prioridade**: MÉDIA

---

## 🎲 Fase 4: Sistema de Magias (Spell System)

### 4.1 Estrutura de Magias
**Task ID**: `implement-spells-structure`

**Descrição**: Estrutura base para sistema de magias.

**Tarefas**:
- [ ] Implementar estrutura `Spell` com:
  - Nome, nível, escola
  - Tempo de conjuração
  - Alcance
  - Componentes (verbal, somático, material)
  - Materiais necessários
  - Duração
  - Descrição
  - Classes que podem usar
- [ ] Implementar níveis de magia (0-9)
- [ ] Implementar escolas de magia:
  - [ ] Abjuração (Abjuration)
  - [ ] Conjuração (Conjuration)
  - [ ] Adivinhação (Divination)
  - [ ] Encantamento (Enchantment)
  - [ ] Evocação (Evocation)
  - [ ] Ilusão (Illusion)
  - [ ] Necromancia (Necromancy)
  - [ ] Transmutação (Transmutation)
- [ ] Implementar busca de magias no Vectorizer
- [ ] Implementar endpoint `/spells/list`
- [ ] Implementar endpoint `/spells/get/{spell_name}`
- [ ] Implementar endpoint `/spells/search`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: ALTA

---

### 4.2 Spell Slots e Conjuração
**Task ID**: `implement-spellcasting`

**Descrição**: Sistema completo de spell slots e conjuração de magias.

**Tarefas**:
- [ ] Implementar estrutura `SpellSlots`:
  - [ ] Spell slots por nível de magia
  - [ ] Spell slots usados
  - [ ] Spell slots disponíveis
- [ ] Implementar tabelas de spell slots por classe:
  - [ ] Full casters (Wizard, Cleric, etc.)
  - [ ] Half casters (Paladin, Ranger)
  - [ ] Third casters (Eldritch Knight, Arcane Trickster)
  - [ ] Warlock (Pact Magic)
- [ ] Implementar Cantrips (magias nível 0):
  - [ ] Sempre disponíveis
  - [ ] Não consomem spell slots
- [ ] Implementar preparação de magias (classes preparadas):
  - [ ] Número de magias preparadas
  - [ ] Mudança de magias preparadas
- [ ] Implementar conjuração:
  - [ ] Validação de spell slot disponível
  - [ ] Consumo de spell slot
  - [ ] Validação de componentes
  - [ ] Validação de tempo de conjuração
- [ ] Implementar endpoint `/spellcasting/slots/get`
- [ ] Implementar endpoint `/spellcasting/slots/use`
- [ ] Implementar endpoint `/spellcasting/prepare`
- [ ] Implementar endpoint `/spellcasting/cast`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-spells-structure`, `implement-classes`  
**Prioridade**: ALTA

---

### 4.3 Efeitos de Magias
**Task ID**: `implement-spell-effects`

**Descrição**: Sistema de resolução de efeitos de magias.

**Tarefas**:
- [ ] Implementar tipos de efeito:
  - [ ] Dano (dice expression + tipo)
  - [ ] Cura
  - [ ] Condições
  - [ ] Buffs/Debuffs
  - [ ] Criação de objetos/criaturas
  - [ ] Transformação
- [ ] Implementar saving throws para magias:
  - [ ] DC de salvaguarda (8 + proficiência + modificador de atributo)
  - [ ] Tipo de salvaguarda
  - [ ] Efeito em sucesso/falha
- [ ] Implementar attack rolls para magias:
  - [ ] Spell attack modifier
  - [ ] Resolução de ataque
- [ ] Implementar upcasting (magias em nível superior):
  - [ ] Efeitos melhorados
  - [ ] Dano adicional
- [ ] Integrar com sistema de dano existente
- [ ] Integrar com sistema de condições existente
- [ ] Implementar endpoint `/spells/effects/resolve`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-spellcasting`, `implement-damage-calculation`, `implement-conditions`  
**Prioridade**: ALTA

---

## 👹 Fase 5: Sistema de Monstros (Monster System)

### 5.1 Estrutura Completa de Monstros (Monster Manual)
**Task ID**: `implement-monsters-structure`

**Descrição**: Estrutura completa de fichas de monstros baseada no Manual dos Monstros.

**Tarefas**:
- [ ] Implementar estrutura `Monster` completa com:
  - **Informações Básicas**:
    - Nome, tipo (beast, humanoid, dragon, undead, fiend, aberration, etc.)
    - Tamanho (tiny, small, medium, large, huge, gargantuan)
    - Alinhamento
  - **Estatísticas de Combate**:
    - Armor Class (AC) - com fonte (armor, natural, etc.)
    - Hit Points (HP) - com fórmula de hit dice
    - Speed (walking, flying, swimming, climbing, burrowing, etc.)
  - **Atributos**:
    - STR, DEX, CON, INT, WIS, CHA (com modificadores)
  - **Proficiências**:
    - Saving Throws proficiencies (com bônus)
    - Skill proficiencies (com bônus)
    - Damage resistances/immunities/vulnerabilities
    - Condition immunities
  - **Sensos e Idiomas**:
    - Senses (darkvision, blindsight, tremorsense, etc.)
    - Languages (idiomas falados)
  - **Desafio**:
    - Challenge Rating (CR)
    - XP value (baseado em CR)
    - Proficiency Bonus (baseado em CR)
- [ ] Implementar cálculo de Proficiency Bonus por CR:
  - CR 0-1/8 = +2
  - CR 1/4-1 = +2
  - CR 2-4 = +2
  - CR 5-8 = +3
  - CR 9-12 = +4
  - CR 13-16 = +5
  - CR 17-20 = +6
  - CR 21-24 = +7
  - CR 25-30 = +8
- [ ] Implementar cálculo de XP por CR (tabela completa)
- [ ] Implementar busca de monstros no Vectorizer (Manual dos Monstros)
- [ ] Implementar endpoint `/monsters/list`
- [ ] Implementar endpoint `/monsters/get/{monster_name}`
- [ ] Implementar endpoint `/monsters/search`
- [ ] Implementar endpoint `/monsters/get-by-cr/{cr}`
- [ ] Implementar endpoint `/monsters/get-by-type/{type}`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: ALTA

---

### 5.2 Ações Completas de Monstros (Monster Manual)
**Task ID**: `implement-monster-actions`

**Descrição**: Sistema completo de ações e habilidades de monstros conforme Manual dos Monstros.

**Tarefas**:
- [ ] Implementar estrutura `MonsterAction` completa:
  - **Ações Padrão**:
    - Nome, tipo (Action, Bonus Action, Reaction)
    - Descrição completa
    - To Hit (se aplicável) - com modificador
    - Reach/Range (alcance)
    - Targets (alvos)
    - Damage (dice expression + tipo de dano)
    - Effects (condições, efeitos especiais)
  - **Multiattack**:
    - Número de ataques
    - Tipos de ataques disponíveis
    - Combinações possíveis
- [ ] Implementar tipos de ação detalhados:
  - [ ] **Melee Weapon Attack**:
    - To Hit: STR ou DEX + proficiência
    - Reach: normalmente 5ft, pode ser maior
    - Damage: dice expression + modificador
  - [ ] **Ranged Weapon Attack**:
    - To Hit: DEX + proficiência
    - Range: normal/long
    - Damage: dice expression + modificador
  - [ ] **Melee Spell Attack**:
    - To Hit: modificador de spellcasting + proficiência
    - Reach: normalmente 5ft
    - Damage: conforme magia
  - [ ] **Ranged Spell Attack**:
    - To Hit: modificador de spellcasting + proficiência
    - Range: conforme magia
    - Damage: conforme magia
  - [ ] **Special Abilities**:
    - Breath Weapons
    - Innate Spellcasting
    - Spellcasting
    - Traits especiais
- [ ] Implementar **Legendary Actions**:
  - [ ] Número de ações por turno (geralmente 3)
  - [ ] Custo de cada ação (1, 2, ou 3)
  - [ ] Ações disponíveis (lista completa)
  - [ ] Resolução no final do turno de outro criatura
- [ ] Implementar **Lair Actions**:
  - [ ] Condições (monstro deve estar em seu covil)
  - [ ] Timing (iniciativa 20, perde empate)
  - [ ] Ações disponíveis (lista completa)
  - [ ] Efeitos regionais (se aplicável)
- [ ] Implementar **Regional Effects**:
  - [ ] Efeitos quando monstro está na região
  - [ ] Efeitos após morte do monstro
- [ ] Implementar busca de ações no Vectorizer
- [ ] Integrar com sistema de combate
- [ ] Implementar endpoint `/monsters/{monster_name}/actions`
- [ ] Implementar endpoint `/monsters/{monster_name}/legendary-actions`
- [ ] Implementar endpoint `/monsters/{monster_name}/lair-actions`
- [ ] Implementar endpoint `/monsters/actions/execute`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monsters-structure`, `implement-combat-actions`  
**Prioridade**: ALTA

---

### 5.3 Tipos e Categorias de Monstros
**Task ID**: `implement-monster-types`

**Descrição**: Sistema completo de tipos e categorias de monstros.

**Tarefas**:
- [ ] Implementar tipos de monstros:
  - [ ] Aberration (aberração)
  - [ ] Beast (fera)
  - [ ] Celestial (celestial)
  - [ ] Construct (constructo)
  - [ ] Dragon (dragão)
  - [ ] Elemental (elemental)
  - [ ] Fey (fada)
  - [ ] Fiend (diabólico)
  - [ ] Giant (gigante)
  - [ ] Humanoid (humanoide)
  - [ ] Monstrosity (monstruosidade)
  - [ ] Ooze (lodo)
  - [ ] Plant (planta)
  - [ ] Undead (morto-vivo)
- [ ] Implementar tamanhos:
  - [ ] Tiny (minúsculo) - 2.5x2.5ft
  - [ ] Small (pequeno) - 5x5ft
  - [ ] Medium (médio) - 5x5ft
  - [ ] Large (grande) - 10x10ft
  - [ ] Huge (enorme) - 15x15ft
  - [ ] Gargantuan (gigantesco) - 20x20ft ou maior
- [ ] Implementar alinhamentos:
  - [ ] Lawful Good, Neutral Good, Chaotic Good
  - [ ] Lawful Neutral, True Neutral, Chaotic Neutral
  - [ ] Lawful Evil, Neutral Evil, Chaotic Evil
  - [ ] Unaligned (não alinhado)
- [ ] Implementar busca por tipo/tamanho/alinhamento
- [ ] Implementar endpoint `/monsters/types/list`
- [ ] Implementar endpoint `/monsters/filter`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monsters-structure`  
**Prioridade**: MÉDIA

---

### 5.4 Challenge Rating e XP (Monster Manual Completo) (Monster Manual Completo)
**Task ID**: `implement-cr-xp`

**Descrição**: Sistema completo de Challenge Rating e XP baseado no Manual dos Monstros.

**Tarefas**:
- [ ] Implementar tabela completa de CR para XP (Manual dos Monstros):
  - [ ] CR 0 = 0 XP
  - [ ] CR 1/8 = 25 XP
  - [ ] CR 1/4 = 50 XP
  - [ ] CR 1/2 = 100 XP
  - [ ] CR 1 = 200 XP
  - [ ] CR 2 = 450 XP
  - [ ] CR 3 = 700 XP
  - [ ] CR 4 = 1,100 XP
  - [ ] CR 5 = 1,800 XP
  - [ ] CR 6 = 2,300 XP
  - [ ] CR 7 = 2,900 XP
  - [ ] CR 8 = 3,900 XP
  - [ ] CR 9 = 5,000 XP
  - [ ] CR 10 = 5,900 XP
  - [ ] CR 11 = 7,200 XP
  - [ ] CR 12 = 8,400 XP
  - [ ] CR 13 = 10,000 XP
  - [ ] CR 14 = 11,500 XP
  - [ ] CR 15 = 13,000 XP
  - [ ] CR 16 = 15,000 XP
  - [ ] CR 17 = 18,000 XP
  - [ ] CR 18 = 20,000 XP
  - [ ] CR 19 = 22,000 XP
  - [ ] CR 20 = 25,000 XP
  - [ ] CR 21 = 33,000 XP
  - [ ] CR 22 = 41,000 XP
  - [ ] CR 23 = 50,000 XP
  - [ ] CR 24 = 62,000 XP
  - [ ] CR 25 = 75,000 XP
  - [ ] CR 26 = 90,000 XP
  - [ ] CR 27 = 105,000 XP
  - [ ] CR 28 = 120,000 XP
  - [ ] CR 29 = 135,000 XP
  - [ ] CR 30 = 155,000 XP
- [ ] Implementar cálculo de XP de encontro (DMG):
  - [ ] XP base de cada monstro
  - [ ] Soma total de XP
  - [ ] Multiplicador por número de monstros:
    - 1 monstro = x1
    - 2 monstros = x1.5
    - 3-6 monstros = x2
    - 7-10 monstros = x2.5
    - 11-14 monstros = x3
    - 15+ monstros = x4
  - [ ] Ajuste por dificuldade desejada
- [ ] Implementar distribuição de XP:
  - [ ] XP total do encontro (após multiplicador)
  - [ ] Divisão igual entre participantes
  - [ ] Ajustes por nível (opcional, para balanceamento)
- [ ] Implementar busca de XP no Vectorizer
- [ ] Implementar endpoint `/monsters/cr-to-xp/{cr}`
- [ ] Implementar endpoint `/encounters/xp/calculate`
- [ ] Implementar endpoint `/encounters/xp/distribute`
- [ ] Implementar endpoint `/encounters/xp/adjust-difficulty`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monsters-structure`  
**Prioridade**: ALTA

---

## 📈 Fase 6: Sistema de Níveis e Progressão

### 6.1 Tabela de Níveis e XP
**Task ID**: `implement-leveling-table`

**Descrição**: Sistema de progressão de níveis e XP necessário.

**Tarefas**:
- [ ] Implementar tabela de XP por nível:
  - [ ] Nível 1 = 0 XP
  - [ ] Nível 2 = 300 XP
  - [ ] Nível 3 = 900 XP
  - [ ] ... até nível 20
- [ ] Implementar cálculo de nível atual:
  - [ ] Baseado em XP total
  - [ ] Retornar nível e XP para próximo nível
- [ ] Implementar level up:
  - [ ] Validação de XP suficiente
  - [ ] Aplicação de melhorias de nível
  - [ ] Features de classe
  - [ ] Aumentos de atributo (níveis 4, 8, 12, 16, 19)
  - [ ] Aumento de hit dice
- [ ] Implementar endpoint `/leveling/xp-table`
- [ ] Implementar endpoint `/leveling/calculate-level`
- [ ] Implementar endpoint `/leveling/level-up`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-cr-xp`, `implement-classes`  
**Prioridade**: ALTA

---

### 6.2 Hit Points e Hit Dice
**Task ID**: `implement-hit-points`

**Descrição**: Sistema completo de hit points e hit dice.

**Tarefas**:
- [ ] Implementar cálculo de HP máximo:
  - [ ] HP nível 1 = hit dice máximo + modificador CON
  - [ ] HP níveis seguintes = hit dice médio (ou rolagem) + modificador CON
- [ ] Implementar Hit Dice:
  - [ ] Tipo por classe (d6, d8, d10, d12)
  - [ ] Número disponível (igual ao nível)
  - [ ] Uso em descanso curto
- [ ] Implementar dano e cura:
  - [ ] Aplicar dano (reduzir HP)
  - [ ] Aplicar cura (aumentar HP, não ultrapassar máximo)
  - [ ] Death saves (quando HP = 0)
- [ ] Implementar Temporary Hit Points:
  - [ ] Acumulação (não soma, usa o maior)
  - [ ] Consumo antes de HP normal
- [ ] Implementar endpoint `/hp/calculate-max`
- [ ] Implementar endpoint `/hp/apply-damage`
- [ ] Implementar endpoint `/hp/apply-healing`
- [ ] Implementar endpoint `/hp/death-save`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-classes`, `implement-ability-scores`  
**Prioridade**: ALTA

---

## 🏕️ Fase 7: Descanso e Recuperação

### 7.1 Descanso Curto (Short Rest)
**Task ID**: `implement-short-rest`

**Descrição**: Sistema de descanso curto e recuperação.

**Tarefas**:
- [ ] Implementar duração (1 hora)
- [ ] Implementar recuperações:
  - [ ] Uso de Hit Dice (até metade do total)
  - [ ] Recuperação de certas features de classe
  - [ ] Recuperação de spell slots (Warlock)
- [ ] Implementar validação:
  - [ ] Verificar se pode fazer descanso curto
  - [ ] Verificar recursos disponíveis
- [ ] Implementar endpoint `/rest/short`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-hit-points`, `implement-spellcasting`  
**Prioridade**: MÉDIA

---

### 7.2 Descanso Longo (Long Rest)
**Task ID**: `implement-long-rest`

**Descrição**: Sistema de descanso longo e recuperação completa.

**Tarefas**:
- [ ] Implementar duração (8 horas, sendo 6 de sono)
- [ ] Implementar recuperações:
  - [ ] HP completo
  - [ ] Hit Dice (até metade do total)
  - [ ] Spell slots completos
  - [ ] Features de classe
  - [ ] Remoção de certas condições
- [ ] Implementar restrições:
  - [ ] Máximo 1 por 24 horas
  - [ ] Interrupções (combate, atividade extenuante)
- [ ] Implementar endpoint `/rest/long`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-hit-points`, `implement-spellcasting`, `implement-conditions`  
**Prioridade**: MÉDIA

---

## 🌍 Fase 8: Exploração e Viagem

### 8.1 Viagem e Ritmo de Viagem
**Task ID**: `implement-travel`

**Descrição**: Sistema de viagem e ritmos de viagem.

**Tarefas**:
- [ ] Implementar ritmos de viagem:
  - [ ] Normal (24 milhas/dia)
  - [ ] Rápido (30 milhas/dia, -5 em Passive Perception)
  - [ ] Lento (18 milhas/dia, mais stealth)
- [ ] Implementar tipos de terreno:
  - [ ] Estrada
  - [ ] Terreno difícil
  - [ ] Montanha
  - [ ] Floresta densa
- [ ] Implementar cálculo de distância:
  - [ ] Distância por dia
  - [ ] Tempo necessário
  - [ ] Recursos consumidos
- [ ] Implementar eventos de viagem:
  - [ ] Encontros aleatórios
  - [ ] Descobertas
  - [ ] Perigos
- [ ] Implementar endpoint `/travel/calculate`
- [ ] Implementar endpoint `/travel/events`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: BAIXA

---

### 8.2 Exploração e Percepção Passiva
**Task ID**: `implement-exploration`

**Descrição**: Sistema de exploração e percepção passiva.

**Tarefas**:
- [ ] Implementar Percepção Passiva:
  - [ ] 10 + modificador de Wisdom + proficiência (se aplicável)
  - [ ] Ajustes por condições
- [ ] Implementar detecção:
  - [ ] Armadilhas
  - [ ] Portas secretas
  - [ ] Criaturas escondidas
- [ ] Implementar investigação:
  - [ ] Testes de habilidade
  - [ ] Descobertas
- [ ] Implementar endpoint `/exploration/passive-perception`
- [ ] Implementar endpoint `/exploration/detect`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-skills`  
**Prioridade**: BAIXA

---

## 🎭 Fase 9: Encontros Sociais

### 9.1 Interação Social
**Task ID**: `implement-social-encounters`

**Descrição**: Sistema de encontros sociais e interação.

**Tarefas**:
- [ ] Implementar testes sociais:
  - [ ] Persuasão
  - [ ] Intimidação
  - [ ] Enganação
  - [ ] Atuação
- [ ] Implementar atitudes de NPCs:
  - [ ] Hostil
  - [ ] Indiferente
  - [ ] Amigável
- [ ] Implementar mudança de atitude:
  - [ ] Baseado em testes sociais
  - [ ] Baseado em ações
- [ ] Implementar endpoint `/social/attitude`
- [ ] Implementar endpoint `/social/interaction`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-skills`  
**Prioridade**: BAIXA

---

## 🌦️ Fase 10: Efeitos Ambientais

### 10.1 Clima e Ambiente
**Task ID**: `implement-environmental-effects`

**Descrição**: Sistema de efeitos ambientais e climáticos.

**Tarefas**:
- [ ] Implementar tipos de clima:
  - [ ] Normal
  - [ ] Frio extremo
  - [ ] Calor extremo
  - [ ] Altitude
  - [ ] Subaquático
- [ ] Implementar efeitos:
  - [ ] Dano por exposição
  - [ ] Modificadores de habilidade
  - [ ] Restrições de movimento
- [ ] Implementar iluminação:
  - [ ] Luz brilhante
  - [ ] Luz baixa (penumbra)
  - [ ] Escuridão
  - [ ] Magia escura
- [ ] Implementar endpoint `/environment/effects`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-conditions`  
**Prioridade**: BAIXA

---

## ✨ Fase 12: Itens Mágicos (Magic Items)

### 12.1 Estrutura de Itens Mágicos
**Task ID**: `implement-magic-items-structure`

**Descrição**: Sistema completo de itens mágicos D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `MagicItem` com:
  - Nome, tipo (weapon, armor, wondrous, etc.)
  - Raridade (common, uncommon, rare, very rare, legendary, artifact)
  - Requer attunement (sim/não)
  - Descrição
  - Propriedades mágicas
  - Custo (se aplicável)
- [ ] Implementar categorias:
  - [ ] Armas mágicas (+1, +2, +3, especiais)
  - [ ] Armaduras mágicas (+1, +2, +3, especiais)
  - [ ] Itens maravilhosos (wondrous items)
  - [ ] Itens consumíveis (poções, pergaminhos)
  - [ ] Artefatos
- [ ] Implementar raridades e níveis de poder
- [ ] Implementar busca de itens mágicos no Vectorizer
- [ ] Implementar endpoint `/magic-items/list`
- [ ] Implementar endpoint `/magic-items/get/{item_name}`
- [ ] Implementar endpoint `/magic-items/search`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `implement-weapons`, `implement-armor`  
**Prioridade**: ALTA

---

### 12.2 Attunement e Uso de Itens Mágicos
**Task ID**: `implement-magic-item-attunement`

**Descrição**: Sistema de attunement e uso de itens mágicos.

**Tarefas**:
- [ ] Implementar attunement:
  - [ ] Limite de 3 itens attuned por personagem
  - [ ] Tempo de attunement (1 hora curta de descanso)
  - [ ] Quebra de attunement (morte, distância, etc.)
- [ ] Implementar uso de itens:
  - [ ] Ativação de propriedades
  - [ ] Carga/uses (se aplicável)
  - [ ] Recarga (se aplicável)
- [ ] Implementar validação:
  - [ ] Verificar se pode attune
  - [ ] Verificar se pode usar
- [ ] Integrar com sistema de inventário
- [ ] Implementar endpoint `/magic-items/attune`
- [ ] Implementar endpoint `/magic-items/use`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-magic-items-structure`, `implement-inventory`  
**Prioridade**: ALTA

---

### 12.3 Tabelas de Tesouro
**Task ID**: `implement-treasure-tables`

**Descrição**: Sistema de geração de tesouros baseado em tabelas do DMG.

**Tarefas**:
- [ ] Implementar tabelas de tesouro:
  - [ ] Individual Treasure (CR 0-4, 5-10, 11-16, 17+)
  - [ ] Treasure Hoards (por nível de desafio)
- [ ] Implementar geração aleatória:
  - [ ] Moedas
  - [ ] Gemas
  - [ ] Objetos de arte
  - [ ] Itens mágicos (tabelas A-G)
- [ ] Implementar tabelas de itens mágicos:
  - [ ] Tabela A (common)
  - [ ] Tabela B (uncommon)
  - [ ] Tabela C (rare)
  - [ ] Tabela D (very rare)
  - [ ] Tabela E (legendary)
  - [ ] Tabela F (consumables)
  - [ ] Tabela G (artifacts)
- [ ] Implementar endpoint `/treasure/generate`
- [ ] Implementar endpoint `/treasure/hoard`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-magic-items-structure`, `implement-cr-xp`  
**Prioridade**: MÉDIA

---

## 🎲 Fase 13: Criação de Encontros (Encounter Building)

### 13.1 Sistema de Criação de Encontros
**Task ID**: `implement-encounter-builder`

**Descrição**: Sistema completo para criação balanceada de encontros.

**Tarefas**:
- [ ] Implementar cálculo de dificuldade:
  - [ ] Easy (25% dos recursos do dia)
  - [ ] Medium (50% dos recursos do dia)
  - [ ] Hard (75% dos recursos do dia)
  - [ ] Deadly (100%+ dos recursos do dia)
- [ ] Implementar XP Budget:
  - [ ] XP por nível de personagem
  - [ ] Multiplicador por número de monstros
  - [ ] Ajuste por dificuldade
- [ ] Implementar validação de encontro:
  - [ ] Verificar se encontro é balanceado
  - [ ] Avisar sobre encontros muito fáceis/difíceis
  - [ ] Sugerir ajustes
- [ ] Implementar busca de monstros por CR
- [ ] Implementar endpoint `/encounters/build`
- [ ] Implementar endpoint `/encounters/validate`
- [ ] Implementar endpoint `/encounters/suggest-monsters`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-monsters-structure`, `implement-cr-xp`  
**Prioridade**: ALTA

---

### 13.2 Variedade e Composição de Encontros
**Task ID**: `implement-encounter-variety`

**Descrição**: Sistema para criar encontros variados e interessantes.

**Tarefas**:
- [ ] Implementar tipos de encontro:
  - [ ] Combate puro
  - [ ] Combate com objetivos (proteger, destruir, etc.)
  - [ ] Combate com terreno especial
  - [ ] Combate com armadilhas
  - [ ] Combate social (pode ser evitado)
- [ ] Implementar composição:
  - [ ] Boss + minions
  - [ ] Múltiplos tipos de inimigos
  - [ ] Inimigos com sinergia
- [ ] Implementar terreno e ambiente:
  - [ ] Terreno difícil
  - [ ] Cobertura
  - [ ] Objetos interativos
  - [ ] Perigos ambientais
- [ ] Implementar objetivos secundários:
  - [ ] Resgatar NPCs
  - [ ] Destruir objetos
  - [ ] Coletar itens
  - [ ] Escapar
- [ ] Implementar endpoint `/encounters/variety`
- [ ] Implementar endpoint `/encounters/composition`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-encounter-builder`, `implement-traps`  
**Prioridade**: MÉDIA

---

## 🛡️ Fase 14: Armadilhas e Perigos (Traps & Hazards)

### 14.1 Sistema de Armadilhas
**Task ID**: `implement-traps`

**Descrição**: Sistema completo de armadilhas D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Trap` com:
  - Nome, tipo (mechanical, magical)
  - Severity (setback, dangerous, deadly)
  - Trigger (pressure plate, tripwire, magic, etc.)
  - Detection DC
  - Disable DC
  - Effects (damage, conditions, etc.)
- [ ] Implementar tipos de armadilhas:
  - [ ] Mecânicas (flechas, dardos, poços, etc.)
  - [ ] Mágicas (glyphs, sigils, etc.)
  - [ ] Combinadas
- [ ] Implementar resolução:
  - [ ] Detecção (Perception/Investigation)
  - [ ] Desarmamento (Thieves' Tools)
  - [ ] Ativação e efeitos
- [ ] Implementar busca de armadilhas no Vectorizer
- [ ] Implementar endpoint `/traps/list`
- [ ] Implementar endpoint `/traps/get/{trap_name}`
- [ ] Implementar endpoint `/traps/detect`
- [ ] Implementar endpoint `/traps/disable`
- [ ] Implementar endpoint `/traps/activate`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `implement-skills`  
**Prioridade**: MÉDIA

---

### 14.2 Perigos Ambientais
**Task ID**: `implement-hazards`

**Descrição**: Sistema de perigos ambientais e naturais.

**Tarefas**:
- [ ] Implementar estrutura `Hazard` com:
  - Nome, tipo
  - Descrição
  - Effects (damage, conditions)
  - Avoidance (como evitar)
- [ ] Implementar tipos de perigos:
  - [ ] Lava
  - [ ] Ácido
  - [ ] Gases venenosos
  - [ ] Queda
  - [ ] Afogamento
  - [ ] Fome/sede
- [ ] Implementar resolução:
  - [ ] Detecção
  - [ ] Evitação
  - [ ] Efeitos se ativado
- [ ] Integrar com sistema de dano
- [ ] Implementar endpoint `/hazards/list`
- [ ] Implementar endpoint `/hazards/get/{hazard_name}`
- [ ] Implementar endpoint `/hazards/resolve`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-damage-calculation`, `implement-conditions`  
**Prioridade**: BAIXA

---

## 🏛️ Fase 15: Divindades e Religião (Deities & Religion)

### 15.1 Sistema de Divindades
**Task ID**: `implement-deities`

**Descrição**: Sistema completo de divindades e panteões D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Deity` com:
  - Nome, título
  - Panteão (Forgotten Realms, Greyhawk, etc.)
  - Alinhamento
  - Domínios (para Clérigos)
  - Símbolo sagrado
  - Descrição
  - Dogmas e crenças
- [ ] Implementar panteões principais:
  - [ ] Forgotten Realms
  - [ ] Greyhawk
  - [ ] Dragonlance
  - [ ] Eberron
  - [ ] Outros
- [ ] Implementar domínios divinos:
  - [ ] Knowledge
  - [ ] Life
  - [ ] Light
  - [ ] Nature
  - [ ] Tempest
  - [ ] Trickery
  - [ ] War
  - [ ] Death
  - [ ] Grave
  - [ ] Forge
  - [ ] Order
  - [ ] Peace
  - [ ] Twilight
- [ ] Implementar busca de divindades no Vectorizer
- [ ] Implementar endpoint `/deities/list`
- [ ] Implementar endpoint `/deities/get/{deity_name}`
- [ ] Implementar endpoint `/deities/search-by-domain`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `implement-classes`  
**Prioridade**: MÉDIA

---

### 15.2 Integração com Clérigos e Paladinos
**Task ID**: `implement-divine-classes`

**Descrição**: Integração de divindades com classes divinas.

**Tarefas**:
- [ ] Implementar seleção de divindade para Clérigos:
  - [ ] Validação de domínios disponíveis
  - [ ] Aplicação de features divinas
- [ ] Implementar Oaths para Paladinos:
  - [ ] Oath of Devotion
  - [ ] Oath of the Ancients
  - [ ] Oath of Vengeance
  - [ ] Oath of Conquest
  - [ ] Oath of Redemption
  - [ ] Oath of the Crown
  - [ ] Oath of Glory
  - [ ] Oath of the Watchers
- [ ] Implementar features relacionadas:
  - [ ] Channel Divinity
  - [ ] Divine Smite
  - [ ] Aura effects
- [ ] Integrar com sistema de classes
- [ ] Implementar endpoint `/divine/select-deity`
- [ ] Implementar endpoint `/divine/select-oath`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-deities`, `implement-classes`  
**Prioridade**: MÉDIA

---

## 🌌 Fase 16: Lore e Cosmologia (Lore & Cosmology)

### 16.1 Cosmologia D&D
**Task ID**: `implement-cosmology`

**Descrição**: Sistema de planos e cosmologia D&D 5e.

**Tarefas**:
- [ ] Implementar estrutura `Plane` com:
  - Nome, tipo (Material, Inner, Outer, etc.)
  - Descrição
  - Características especiais
  - Criaturas nativas
- [ ] Implementar planos principais:
  - [ ] Material Plane
  - [ ] Inner Planes (Elemental)
  - [ ] Outer Planes (Celestial, Infernal, etc.)
  - [ ] Transitive Planes (Astral, Ethereal, Shadow)
- [ ] Implementar busca de lore no Vectorizer
- [ ] Implementar endpoint `/cosmology/planes/list`
- [ ] Implementar endpoint `/cosmology/planes/get/{plane_name}`
- [ ] Implementar endpoint `/cosmology/search`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: BAIXA

---

### 16.2 World Building e Lore
**Task ID**: `implement-world-building`

**Descrição**: Sistema para gerenciar lore e world building.

**Tarefas**:
- [ ] Implementar estrutura `World` com:
  - Nome, descrição
  - História
  - Geografia
  - Culturas
  - Organizações
  - Eventos importantes
- [ ] Implementar cenários oficiais:
  - [ ] Forgotten Realms
  - [ ] Greyhawk
  - [ ] Dragonlance
  - [ ] Eberron
  - [ ] Ravenloft
  - [ ] Outros
- [ ] Implementar busca de lore no Vectorizer
- [ ] Implementar geração de lore:
  - [ ] Cidades
  - [ ] NPCs importantes
  - [ ] Organizações
  - [ ] Eventos históricos
- [ ] Integrar com LLM para geração de conteúdo
- [ ] Implementar endpoint `/world/get/{world_name}`
- [ ] Implementar endpoint `/world/generate-content`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `llm-core` (gera INTENT DSL, coordenado via Orquestrador)  
**Prioridade**: BAIXA

---

## 👥 Fase 17: NPCs e Vilões (NPCs & Villains)

### 17.1 Sistema de NPCs
**Task ID**: `implement-npcs`

**Descrição**: Sistema completo de criação e gerenciamento de NPCs.

**Tarefas**:
- [ ] Implementar estrutura `NPC` com:
  - Nome, raça, classe (se aplicável)
  - Nível/CR
  - Atributos
  - Personalidade
  - Motivações
  - Relacionamentos
  - Informações conhecidas
  - Quests associadas
- [ ] Implementar tipos de NPCs:
  - [ ] Aliados
  - [ ] Neutros
  - [ ] Inimigos
  - [ ] Quest givers
  - [ ] Merchants
  - [ ] Informantes
- [ ] Implementar geração de NPCs:
  - [ ] Baseado em templates
  - [ ] Aleatório
  - [ ] Personalizado
- [ ] Implementar busca de NPCs no Vectorizer
- [ ] Integrar com LLM para personalidades
- [ ] Implementar endpoint `/npcs/create`
- [ ] Implementar endpoint `/npcs/get/{npc_id}`
- [ ] Implementar endpoint `/npcs/search`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `llm-core` (gera INTENT DSL, coordenado via Orquestrador)  
**Prioridade**: ALTA

---

### 17.2 Sistema de Vilões e Antagonistas
**Task ID**: `implement-villains`

**Descrição**: Sistema para criar e gerenciar vilões e antagonistas.

**Tarefas**:
- [ ] Implementar estrutura `Villain` com:
  - Nome, título
  - Tipo (BBEG, lieutenant, minion)
  - Motivações
  - Plano mestre
  - Recursos e aliados
  - Fraquezas conhecidas
  - História
- [ ] Implementar tipos de vilões:
  - [ ] Lich
  - [ ] Dragon
  - [ ] Cult Leader
  - [ ] Corrupt Noble
  - [ ] Demon/Devil
  - [ ] Outros
- [ ] Implementar organizações malignas:
  - [ ] Cults
  - [ ] Thieves' Guilds
  - [ ] Evil Empires
  - [ ] Outros
- [ ] Implementar progressão de vilão:
  - [ ] Fases do plano
  - [ ] Reações às ações dos jogadores
  - [ ] Escalação de ameaça
- [ ] Integrar com LLM para narrativa
- [ ] Implementar endpoint `/villains/create`
- [ ] Implementar endpoint `/villains/get/{villain_id}`
- [ ] Implementar endpoint `/villains/update-plan`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `llm-core` (gera INTENT DSL, coordenado via Orquestrador), `implement-npcs`  
**Prioridade**: ALTA

---

## 🎲 Fase 18: Tabelas Aleatórias (Random Tables)

### 18.1 Sistema de Tabelas Aleatórias
**Task ID**: `implement-random-tables`

**Descrição**: Sistema completo de tabelas aleatórias do DMG.

**Tarefas**:
- [ ] Implementar estrutura `RandomTable` com:
  - Nome, categoria
  - Entradas (com pesos, se aplicável)
  - Descrição
- [ ] Implementar categorias:
  - [ ] Encontros aleatórios
  - [ ] Clima
  - [ ] Eventos de viagem
  - [ ] Descobertas
  - [ ] NPCs aleatórios
  - [ ] Tesouros
  - [ ] Outros
- [ ] Implementar geração:
  - [ ] Roll aleatório
  - [ ] Roll com pesos
  - [ ] Roll múltiplo
- [ ] Implementar busca de tabelas no Vectorizer
- [ ] Implementar endpoint `/random-tables/list`
- [ ] Implementar endpoint `/random-tables/get/{table_name}`
- [ ] Implementar endpoint `/random-tables/roll`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`  
**Prioridade**: BAIXA

---

### 18.2 Encontros Aleatórios
**Task ID**: `implement-random-encounters`

**Descrição**: Sistema de geração de encontros aleatórios.

**Tarefas**:
- [ ] Implementar tabelas por ambiente:
  - [ ] Floresta
  - [ ] Deserto
  - [ ] Montanha
  - [ ] Urbano
  - [ ] Subterrâneo
  - [ ] Costeiro
  - [ ] Outros
- [ ] Implementar frequência:
  - [ ] Chance por dia/hora
  - [ ] Ajuste por atividade
- [ ] Implementar geração:
  - [ ] Seleção de monstros
  - [ ] Número de criaturas
  - [ ] Objetivos (se aplicável)
- [ ] Integrar com sistema de encontros
- [ ] Implementar endpoint `/random-encounters/generate`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `implement-random-tables`, `implement-encounter-builder`  
**Prioridade**: MÉDIA

---

## 🏰 Fase 19: Design de Dungeons e Aventuras

### 19.1 Sistema de Dungeons
**Task ID**: `implement-dungeon-design`

**Descrição**: Sistema para criar e gerenciar dungeons.

**Tarefas**:
- [ ] Implementar estrutura `Dungeon` com:
  - Nome, descrição
  - Salas (rooms)
  - Corredores
  - Níveis
  - Mapas
- [ ] Implementar tipos de salas:
  - [ ] Combate
  - [ ] Puzzle
  - [ ] Social
  - [ ] Descanso
  - [ ] Tesouro
  - [ ] Boss
- [ ] Implementar conexões:
  - [ ] Portas
  - [ ] Escadas
  - [ ] Passagens secretas
- [ ] Implementar geração:
  - [ ] Aleatória
  - [ ] Template-based
  - [ ] Manual
- [ ] Integrar com battlemap
- [ ] Implementar endpoint `/dungeons/create`
- [ ] Implementar endpoint `/dungeons/get/{dungeon_id}`
- [ ] Implementar endpoint `/dungeons/generate`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `game-engine` (via Orquestrador), `implement-encounter-builder`  
**Prioridade**: MÉDIA

---

### 19.2 Design de Aventuras
**Task ID**: `implement-adventure-design`

**Descrição**: Sistema para criar e estruturar aventuras.

**Tarefas**:
- [ ] Implementar estrutura `Adventure` com:
  - Nome, descrição
  - Nível recomendado
  - Duração estimada
  - Encontros
  - NPCs
  - Quests
  - Recompensas
- [ ] Implementar estrutura de aventura:
  - [ ] Hook (gancho inicial)
  - [ ] Rising Action
  - [ ] Climax
  - [ ] Resolution
- [ ] Implementar tipos:
  - [ ] One-shot
  - [ ] Multi-session
  - [ ] Campaign arc
- [ ] Integrar com LLM para geração
- [ ] Implementar endpoint `/adventures/create`
- [ ] Implementar endpoint `/adventures/get/{adventure_id}`
- [ ] Implementar endpoint `/adventures/generate`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `llm-core` (gera INTENT DSL, coordenado via Orquestrador), `implement-encounter-builder`, `implement-npcs`  
**Prioridade**: MÉDIA

---

## 📚 Fase 20: Melhores Práticas de DM (DM Best Practices)

### 20.1 Sistema de Ajuda para DM
**Task ID**: `implement-dm-assistant`

**Descrição**: Sistema de assistência e melhores práticas para DMs.

**Tarefas**:
- [ ] Implementar guias de melhores práticas:
  - [ ] Criação de encontros balanceados
  - [ ] Narração e descrição
  - [ ] Gerenciamento de regras
  - [ ] Improvisação
  - [ ] Gerenciamento de mesa
- [ ] Implementar sugestões contextuais:
  - [ ] Baseado em situação atual
  - [ ] Baseado em nível dos jogadores
  - [ ] Baseado em estilo de jogo
- [ ] Implementar busca de conselhos no Vectorizer
- [ ] Integrar com LLM para sugestões
- [ ] Implementar endpoint `/dm-assistant/advice`
- [ ] Implementar endpoint `/dm-assistant/suggestions`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: `memory-service`, `llm-core` (gera INTENT DSL, coordenado via Orquestrador)  
**Prioridade**: BAIXA

---

### 20.2 Ferramentas de DM
**Task ID**: `implement-dm-tools`

**Descrição**: Ferramentas úteis para DMs.

**Tarefas**:
- [ ] Implementar calculadoras:
  - [ ] XP calculator
  - [ ] Encounter builder
  - [ ] Treasure generator
  - [ ] NPC generator
- [ ] Implementar gerenciadores:
  - [ ] Initiative tracker
  - [ ] HP tracker
  - [ ] Condition tracker
  - [ ] Notes manager
- [ ] Implementar geradores:
  - [ ] Names
  - [ ] Descriptions
  - [ ] Quests
  - [ ] Locations
- [ ] Integrar com frontend
- [ ] Implementar endpoint `/dm-tools/calculate`
- [ ] Implementar endpoint `/dm-tools/generate`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: Várias fases anteriores  
**Prioridade**: MÉDIA

---

## 📊 Resumo Expandido de Prioridades

### Prioridade CRÍTICA (MVP)
1. ✅ Sistema de dados e ataques (já implementado)
2. ✅ Sistema de dano e condições (já implementado)
3. ⚠️ Sistema completo de personagem
4. ⚠️ Sistema completo de combate
5. ⚠️ Sistema básico de magias
6. ⚠️ Sistema básico de monstros
7. ⚠️ Criação de encontros balanceados

### Prioridade ALTA
1. Raças e Classes
2. Armas e Armaduras
3. Perícias
4. Iniciativa e turnos
5. Monstros básicos
6. Spell slots
7. Itens mágicos básicos
8. NPCs e Vilões
9. Sistema de XP e níveis

### Prioridade MÉDIA
1. Antecedentes
2. Talentos
3. Equipamentos gerais
4. Inventário
5. Descanso
6. Armadilhas
7. Divindades
8. Tabelas de tesouro
9. Variedade de encontros
10. Dungeons e Aventuras
11. Ferramentas de DM

### Prioridade BAIXA
1. Viagem e exploração
2. Encontros sociais
3. Efeitos ambientais
4. Cosmologia e Lore
5. World Building
6. Tabelas aleatórias
7. Ajuda para DM

---

## 📊 Estatísticas do Plano Expandido

- **Total de Tasks**: 80+
- **Fases**: 20
- **Endpoints Planejados**: 150+
- **Testes Necessários**: 800+
- **Tempo Estimado**: 12-18 meses (desenvolvimento completo)

---

## 🔄 Próximos Passos Imediatos

1. **Revisar TASKS_MASTER.md** e integrar todas estas tasks
2. **Priorizar implementação** baseado em MVP
3. **Criar issues no GitHub** para cada task
4. **Começar implementação** das tasks de prioridade CRÍTICA
5. **Consultar Vectorizer** continuamente durante implementação para validar regras
6. **Integrar com LLM** para geração de conteúdo (NPCs, aventuras, etc.)

---

**Última Atualização**: 2025-11-23  
**Próxima Revisão**: Após implementação de cada fase

---

## 🎮 Fase 11: Integração com Orquestrador e Game Engine

**Nota Importante**: O Game Engine agora trabalha em conjunto com o Orquestrador, que coordena o fluxo geral. Todas as integrações devem ser feitas via Orquestrador.

**Ver**: [MIGRATION_PLAN.md](MIGRATION_PLAN.md) para detalhes da nova arquitetura.

### 11.1 Sistema Completo de Personagem
**Task ID**: `implement-complete-character`

**Descrição**: Integrar todos os sistemas em uma estrutura completa de personagem (via Orquestrador).

**Tarefas**:
- [ ] Implementar estrutura `Character` completa:
  - [ ] Ability Scores
  - [ ] Race
  - [ ] Class (e nível)
  - [ ] Background
  - [ ] Skills
  - [ ] Feats
  - [ ] Equipment (armas, armaduras, itens)
  - [ ] Inventory
  - [ ] Spell Slots
  - [ ] Spells Known/Prepared
  - [ ] HP e Hit Dice
  - [ ] Conditions
- [ ] Implementar criação de personagem:
  - [ ] Wizard de criação passo a passo
  - [ ] Validação de escolhas
  - [ ] Cálculo automático de valores derivados
- [ ] Implementar serialização:
  - [ ] Save/Load de personagem
  - [ ] Export/Import
- [ ] Integrar com `game-engine`
- [ ] Implementar endpoint `/characters/create`
- [ ] Implementar endpoint `/characters/get/{character_id}`
- [ ] Implementar endpoint `/characters/update`
- [ ] Implementar endpoint `/characters/save`
- [ ] Implementar endpoint `/characters/load`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: TODAS as fases anteriores  
**Prioridade**: CRÍTICA

---

### 11.2 Integração Completa de Combate
**Task ID**: `implement-complete-combat`

**Descrição**: Integrar todos os sistemas de combate no game-engine (via Orquestrador e Turn Engine).

**Tarefas**:
- [ ] Integrar iniciativa com game-engine
- [ ] Integrar ações de combate
- [ ] Integrar movimento
- [ ] Integrar ataques (com armas)
- [ ] Integrar magias
- [ ] Integrar ações de monstros
- [ ] Implementar resolução completa de turno
- [ ] Implementar resolução completa de round
- [ ] Implementar eventos de combate:
  - [ ] ActorMoved
  - [ ] AttackMade
  - [ ] DamageDealt
  - [ ] SpellCast
  - [ ] ConditionApplied
  - [ ] ActorDied
- [ ] Integrar com LLM para narrativa
- [ ] Implementar endpoint `/combat/start`
- [ ] Implementar endpoint `/combat/execute-action`
- [ ] Implementar endpoint `/combat/end`
- [ ] Implementar testes unitários (95%+ cobertura)

**Dependências**: TODAS as fases de combate e magias  
**Prioridade**: CRÍTICA

---

## 📝 Resumo de Prioridades

### Prioridade CRÍTICA (MVP)
1. ✅ Sistema de dados e ataques (já implementado)
2. ✅ Sistema de dano e condições (já implementado)
3. ⚠️ Sistema completo de personagem
4. ⚠️ Sistema completo de combate
5. ⚠️ Sistema básico de magias

### Prioridade ALTA
1. Raças e Classes
2. Armas e Armaduras
3. Perícias
4. Iniciativa e turnos
5. Monstros básicos
6. Spell slots

### Prioridade MÉDIA
1. Antecedentes
2. Talentos
3. Equipamentos gerais
4. Inventário
5. XP e níveis
6. Descanso

### Prioridade BAIXA
1. Viagem e exploração
2. Encontros sociais
3. Efeitos ambientais

---

## 📊 Estatísticas do Plano

- **Total de Tasks**: 50+
- **Fases**: 11
- **Endpoints Planejados**: 100+
- **Testes Necessários**: 500+
- **Tempo Estimado**: 6-12 meses (desenvolvimento completo)

---

## 🔄 Próximos Passos Imediatos

1. **Revisar TASKS_MASTER.md** e integrar estas tasks
2. **Priorizar implementação** baseado em MVP
3. **Criar issues no GitHub** para cada task
4. **Começar implementação** das tasks de prioridade CRÍTICA
5. **Consultar Vectorizer** continuamente durante implementação para validar regras

---

**Última Atualização**: 2025-11-23  
**Próxima Revisão**: Após implementação de cada fase

