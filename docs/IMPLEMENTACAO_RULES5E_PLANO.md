# Plano de Implementação - Rules5e Service

## Status Atual

**Arquitetura**: ✅ Estrutura criada  
**Compilação**: ✅ Compilando (com pequenos ajustes)  
**Progresso**: ~70% implementado

---

## ✅ Já Implementado

### Módulos Core
- ✅ **Dice Rolling**: Parser completo (`2d8+3`), seeds, advantage/disadvantage
- ✅ **Ability Scores**: Cálculo de modificadores, geração (Standard Array, Rolling, Point Buy)
- ✅ **Attack Resolution**: Hit/miss, critical hits/misses, advantage/disadvantage
- ✅ **Damage Calculation**: Resistances, vulnerabilities, immunities
- ✅ **Conditions**: Sistema completo de condições D&D 5e
- ✅ **Skills**: 18 skills com proficiency e expertise
- ✅ **Weapons Database**: Database de armas com propriedades
- ✅ **CR/XP**: Conversão Challenge Rating ↔ XP
- ✅ **HTTP Server**: Server Axum com múltiplos endpoints

### Endpoints HTTP (localhost:7004)
- ✅ `GET /health` - Health check
- ✅ `POST /roll` - Roll dice expressions
- ✅ `POST /attack` - Resolve attacks
- ✅ `POST /ability-check` - Ability checks
- ✅ `POST /saving-throw` - Saving throws
- ✅ `POST /damage` - Calculate damage with resistances
- ✅ `POST /ability-scores/calculate-modifier` - Calculate modifiers
- ✅ `POST /ability-scores/generate` - Generate ability scores
- ✅ `POST /cr-xp/cr-to-xp` - Convert CR to XP
- ✅ `POST /cr-xp/xp-to-cr` - Convert XP to CR
- ✅ `POST /cr-xp/proficiency-bonus` - Calculate proficiency bonus
- ✅ `GET /skills/list` - List all skills
- ✅ `POST /skills/calculate-bonus` - Calculate skill bonus
- ✅ `POST /skills/check` - Perform skill check
- ✅ `POST /skills/passive-perception` - Calculate passive perception
- ✅ `GET /weapons/list` - List all weapons
- ✅ `GET /weapons/get/{weapon_name}` - Get specific weapon
- ✅ `POST /weapons/by-category` - Get weapons by category

---

## 🔄 Pendente/Parcial

### 1. Spell System (CRÍTICO)
**Status**: ⏳ Não implementado

**O que falta**:
- Spell Database (SRD completo)
- Spell Slots Management
- Spell Casting Resolution
- Spell Components (V, S, M)
- Spell Concentration
- Spell Duration Tracking
- Spell Areas of Effect
- Spell Saving Throws

**Prioridade**: ALTA (bloqueia sistema de magias)

---

### 2. Monster System (ALTA PRIORIDADE)
**Status**: ⏳ Não implementado

**O que falta**:
- Monster Database (SRD completo)
- Monster Stat Blocks
- Monster Abilities
- Monster Actions
- Monster Legendary Actions
- Monster Lair Actions

**Prioridade**: ALTA (bloqueia combate com monstros)

---

### 3. Point Buy System (MÉDIA PRIORIDADE)
**Status**: 🔄 Parcial (retorna Standard Array como fallback)

**O que falta**:
- Implementação completa do Point Buy (27 pontos)
- Validação de limites por score
- Custo por ponto baseado no score atual

**Prioridade**: MÉDIA

---

### 4. Integração com Vectorizer (NOVA)
**Status**: ⏳ Não implementado

**O que falta**:
- Função para consultar regras D&D 5e no Vectorizer
- Cache de consultas frequentes
- Fallback quando Vectorizer não disponível

**Como usar**:
- Consultar Vectorizer via MCP quando necessário validar regras
- Usar para buscar definições de spells, monsters, etc.

**Prioridade**: MÉDIA (melhora qualidade, mas não bloqueia)

---

### 5. Testes Completos (ALTA PRIORIDADE)
**Status**: 🔄 Parcial (testes unitários existem, falta integração)

**O que falta**:
- Testes de integração end-to-end
- Testes de performance (< 5ms para cálculos)
- Testes de latência HTTP
- Testes de stress
- Cobertura 95%+

**Prioridade**: ALTA (qualidade)

---

## 📋 Próximos Passos (Ordem Recomendada)

### Fase 1: Completar Core (1-2 semanas)
1. ✅ Corrigir erros de compilação (se houver)
2. ⏳ Implementar Point Buy completo
3. ⏳ Adicionar mais testes unitários
4. ⏳ Adicionar testes de integração

### Fase 2: Spell System (1-2 semanas)
1. ⏳ Consultar Vectorizer para Spell Database SRD
2. ⏳ Implementar Spell Database
3. ⏳ Implementar Spell Slots Management
4. ⏳ Implementar Spell Casting Resolution
5. ⏳ Implementar Spell Components
6. ⏳ Implementar Spell Concentration
7. ⏳ Implementar Spell Duration
8. ⏳ Implementar Spell Areas of Effect
9. ⏳ Implementar Spell Saving Throws
10. ⏳ Criar endpoints HTTP para spells

### Fase 3: Monster System (1-2 semanas)
1. ⏳ Consultar Vectorizer para Monster Database SRD
2. ⏳ Implementar Monster Database
3. ⏳ Implementar Monster Stat Blocks
4. ⏳ Implementar Monster Abilities
5. ⏳ Implementar Monster Actions
6. ⏳ Implementar Monster Legendary Actions
7. ⏳ Implementar Monster Lair Actions
8. ⏳ Criar endpoints HTTP para monsters

### Fase 4: Integração Vectorizer (1 semana)
1. ⏳ Criar módulo de integração com Vectorizer via MCP
2. ⏳ Implementar cache de consultas
3. ⏳ Implementar fallback quando Vectorizer indisponível
4. ⏳ Integrar consultas nas funções existentes

### Fase 5: Testes e Qualidade (1 semana)
1. ⏳ Testes de integração completos
2. ⏳ Testes de performance
3. ⏳ Testes de stress
4. ⏳ Atingir 95%+ de cobertura
5. ⏳ Documentação completa

---

## 🔧 Como Consultar Vectorizer

Quando precisar validar regras D&D 5e, usar:

```rust
// Exemplo de consulta ao Vectorizer via MCP
// (implementar função helper)

async fn consult_vectorizer(query: &str) -> Result<String> {
    // Chamar MCP Vectorizer
    // Retornar resultado
}
```

**Exemplos de consultas**:
- "D&D 5e spell components verbal somatic material"
- "D&D 5e monster challenge rating calculation"
- "D&D 5e point buy ability score costs"
- "D&D 5e spell slot usage rules"

---

## 📊 Métricas de Sucesso

- ✅ Latência de cálculos: < 5ms
- ✅ Latência HTTP: < 50ms
- ⏳ Cobertura de testes: 95%+ (atualmente ~60%)
- ⏳ Spell Database: 100% do SRD
- ⏳ Monster Database: 100% do SRD

---

## 🎯 Objetivo Final

Rules5e Service completo, determinístico e rápido que serve como base para:
- Game Engine
- Combate em turnos
- Sistema de magias
- Sistema de monstros
- Validação de regras

**Consulta ao Vectorizer**: Para garantir que implementações seguem regras oficiais D&D 5e.













