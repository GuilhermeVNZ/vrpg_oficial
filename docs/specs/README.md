# Technical Specifications

Esta pasta contém especificações técnicas detalhadas de features e componentes do VRPG Client.

## Quando Criar Especificações

Especificações devem ser criadas quando:

1. **Uma nova feature é proposta** e precisa de especificação técnica detalhada
2. **Um componente complexo** precisa de documentação técnica profunda
3. **Uma integração** requer especificação de API/contrato
4. **Uma refatoração significativa** precisa de especificação de design

## Estrutura Esperada

### Especificações Planejadas

#### Core System
- **ORCHESTRATOR_SPEC.md** - Especificação técnica completa do Orquestrador
- **INTENT_DSL_SPEC.md** - Especificação formal da INTENT DSL (gramática, semântica)
- **FSM_SPEC.md** - Especificação da máquina de estados de cena

#### Services
- **RULES5E_SERVICE_SPEC.md** - Especificação técnica do serviço de regras D&D 5e
- **LLM_CORE_SPEC.md** - Especificação técnica do LLM Core
- **MEMORY_SERVICE_SPEC.md** - Especificação técnica do serviço de memória
- **ASR_SERVICE_SPEC.md** - Especificação técnica do serviço ASR
- **TTS_SERVICE_SPEC.md** - Especificação técnica do serviço TTS

#### Game Systems
- **TURN_ENGINE_SPEC.md** - Especificação técnica do Turn Engine
- **COMBAT_FLOW_SPEC.md** - Especificação técnica do fluxo de combate
- **VOICE_INTENTS_SPEC.md** - Especificação técnica do sistema Voice INTENTS

#### Integration
- **IPC_PROTOCOL_SPEC.md** - Especificação do protocolo IPC (Electron ↔ Rust)
- **WEBSOCKET_PROTOCOL_SPEC.md** - Especificação do protocolo WebSocket
- **MCP_INTEGRATION_SPEC.md** - Especificação técnica da integração MCP

### Quando Serão Criadas

Essas especificações serão criadas seguindo o workflow do Rulebook:

1. **Task Creation**: `rulebook task create <task-id>`
2. **Proposal**: Escrever `proposal.md` explicando o porquê
3. **Spec Delta**: Escrever `specs/<module>/spec.md` com especificação técnica
4. **Validation**: `rulebook task validate <task-id>`
5. **Implementation**: Implementar seguindo a especificação
6. **Archive**: `rulebook task archive <task-id>` (aplica spec ao main)

### Formato das Especificações

As especificações seguem o formato OpenSpec-compatible:

```markdown
# Feature Specification Name

## ADDED Requirements

### Requirement: Feature Name
The system SHALL/MUST do something specific and testable.

#### Scenario: Scenario Name
Given some precondition
When an action occurs
Then an expected outcome happens

## MODIFIED Requirements

### Requirement: Existing Feature
The system SHALL/MUST do something modified.

#### Scenario: Modified scenario
Given updated precondition
When action occurs
Then new expected outcome
```

## Relação com Outros Documentos

- **docs/ARCHITECTURE.md** - Visão geral da arquitetura (alto nível)
- **docs/specs/ORCHESTRATOR_SPEC.md** - Especificação técnica detalhada do Orquestrador

- **docs/ORCHESTRATOR.md** - Documentação de uso e design do Orquestrador
- **docs/specs/ORCHESTRATOR_SPEC.md** - Especificação técnica formal (requisitos, cenários)

- **docs/INTENT_DSL.md** - Documentação da INTENT DSL (uso, exemplos)
- **docs/specs/INTENT_DSL_SPEC.md** - Especificação formal (gramática, semântica)

## Status Atual

**Status**: Pastas criadas, aguardando criação das especificações conforme tasks do Rulebook.

As especificações serão criadas quando:
- Uma task do Rulebook for criada para uma feature
- A especificação técnica detalhada for necessária
- Um componente complexo precisar de documentação formal

## Workflow de Criação

1. **Criar Task no Rulebook**:
   ```bash
   rulebook task create add-orchestrator-spec
   ```

2. **Escrever Proposal** (`rulebook/tasks/add-orchestrator-spec/proposal.md`):
   - Por que a especificação é necessária
   - O que será especificado
   - Impacto

3. **Escrever Spec Delta** (`rulebook/tasks/add-orchestrator-spec/specs/orchestrator/spec.md`):
   - Requisitos com SHALL/MUST
   - Cenários com Given/When/Then
   - Formato OpenSpec-compatible

4. **Validar**:
   ```bash
   rulebook task validate add-orchestrator-spec
   ```

5. **Copiar para docs/specs/**:
   - Após validação, copiar a spec para `docs/specs/ORCHESTRATOR_SPEC.md`
   - Manter sincronizado com a spec do Rulebook

6. **Archive** (após implementação):
   ```bash
   rulebook task archive add-orchestrator-spec
   ```

## Nota Importante

As especificações em `docs/specs/` são **documentação de referência** baseada nas specs do Rulebook. A fonte da verdade são as specs em `rulebook/tasks/<task-id>/specs/`, que são aplicadas ao main quando a task é arquivada.











