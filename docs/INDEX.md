# VRPG Client - Índice de Documentação

## Visão Geral

Este documento serve como índice centralizado de toda a documentação do VRPG Client, organizada conforme os padrões do rulebook.

**Última Atualização**: 2025-01-XX

## Documentos de Status e Implementação

- **[CHANGELOG.md](CHANGELOG.md)** - Registro de mudanças e melhorias
- **[STATUS.md](STATUS.md)** - Status atual do projeto (inclui status GPU e implementação detalhada)

---

## 📚 Documentação Principal

### Início Rápido
- **[README.md](../README.md)** - Visão geral do projeto e quick start
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Arquitetura técnica completa
- **[CONFIGURATION.md](CONFIGURATION.md)** - Configuração de todos os módulos
- **[ROADMAP.md](ROADMAP.md)** - Roadmap de implementação por fases

### Arquitetura e Design
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Arquitetura técnica, módulos, fluxos
- **[PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md)** - Arquitetura de pipeline com 3 agentes (Orquestrador + Qwen-1.5B + Qwen-14B)
- **[FRONTEND_ARCHITECTURE.md](FRONTEND_ARCHITECTURE.md)** - Arquitetura do frontend (Electron + React)
- **[DESIGN_SYSTEM.md](DESIGN_SYSTEM.md)** - Sistema de design UI (Glassmorphism, CSS base e componentes)
- **[ORCHESTRATOR.md](ORCHESTRATOR.md)** - Orquestrador e coordenação
- **[INTENT_DSL.md](INTENT_DSL.md)** - DSL de Intenções

### Especificações Técnicas
- **[specs/](specs/)** - Especificações técnicas detalhadas
  - **[specs/README.md](specs/README.md)** - Guia sobre especificações
  - **[specs/ORCHESTRATOR_SPEC.md](specs/ORCHESTRATOR_SPEC.md)** - Orquestrador
  - **[specs/INTENT_DSL_SPEC.md](specs/INTENT_DSL_SPEC.md)** - INTENT DSL
  - **[specs/TURN_ENGINE_SPEC.md](specs/TURN_ENGINE_SPEC.md)** - Turn Engine
  - **[specs/FSM_SPEC.md](specs/FSM_SPEC.md)** - Máquina de estados
  - **[specs/TTS_SERVICE_SPEC.md](specs/TTS_SERVICE_SPEC.md)** - Serviço TTS
  - **[specs/RULES5E_SERVICE_SPEC.md](specs/RULES5E_SERVICE_SPEC.md)** - Regras D&D 5e
  - **[specs/MEMORY_SERVICE_SPEC.md](specs/MEMORY_SERVICE_SPEC.md)** - Serviço de memória
  - **[specs/LLM_CORE_SPEC.md](specs/LLM_CORE_SPEC.md)** - LLM Core
  - **[specs/IPC_PROTOCOL_SPEC.md](specs/IPC_PROTOCOL_SPEC.md)** - Protocolo IPC

### Componentes Implementados
- **[CHARACTER_SHEET_COMPONENT.md](CHARACTER_SHEET_COMPONENT.md)** - Character Sheet (React)
- **[JOURNAL_COMPONENT.md](JOURNAL_COMPONENT.md)** - Journal (React)
- **[GameplayInterface](../src/client-electron/src/components/GameplayInterface.tsx)** - Interface principal
- **Voice HUD**: Documentado em [DESIGN_SYSTEM.md](DESIGN_SYSTEM.md)

### Guias de Desenvolvimento
- **[guides/](guides/)** - Guias práticos passo a passo
  - **[guides/README.md](guides/README.md)** - Guia sobre guias

### Implementação e Tarefas
- **[TASKS.md](TASKS.md)** - Tasks consolidadas de implementação (documento principal)
- **[TASKS_PIPELINE_MIGRATION.md](TASKS_PIPELINE_MIGRATION.md)** - Tasks detalhadas de migração para pipeline de 3 agentes
- **[TASKS_COMPLETE_DND5E.md](TASKS_COMPLETE_DND5E.md)** - Tasks completas do sistema D&D 5e
- **[TESTS_MASTER.md](TESTS_MASTER.md)** - Master test plan completo
- **[TESTS_TASKS.md](TESTS_TASKS.md)** - Testes detalhados por task (foco em pipeline de 3 agentes)

### Integração e Serviços
- **[MCP_INTEGRATION.md](MCP_INTEGRATION.md)** - Integração MCP (inclui Synap e comunicação unificada)
- **[vectorizer-setup.md](vectorizer-setup.md)** - Configuração e instalação do Vectorizer
- **[vectorizer-persistence.md](vectorizer-persistence.md)** - Persistência de dados do Vectorizer no Docker
- **[vectorizer-collections.md](vectorizer-collections.md)** - Collections do Vectorizer para livros D&D 5e

### Pipelines e Assets
- **[AUDIO_PIPELINE.md](AUDIO_PIPELINE.md)** - Pipeline de áudio (inclui Voice INTENTS)
- **[ASSETS_GENERATION.md](ASSETS_GENERATION.md)** - Geração de assets (inclui pipeline visual, LoRA guidelines e biblioteca de prompts)
- **[ART_DIRECTION_SPRITES.md](ART_DIRECTION_SPRITES.md)** - Direção artística para sprites de animação do battlemap
- **[ART_DIRECTION_EQUIPMENT_ICONS.md](ART_DIRECTION_EQUIPMENT_ICONS.md)** - Direção artística para ícones de equipamentos D&D 5e

### Qualidade e Performance
- **[PERFORMANCE.md](PERFORMANCE.md)** - Estratégias de otimização (inclui análise de latência e otimização GPU)
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Deploy e distribuição

### Filosofia e Design
- **[DM_MINDSET.md](DM_MINDSET.md)** - Mindset do Mestre IA
- **[QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md)** - Especificação do Qwen-1.5B ("Mestre Reflexo")
- **[QWEN_14B_SPEC.md](QWEN_14B_SPEC.md)** - Especificação do Qwen-14B ("Mestre Real")
- **[CHARACTER_AGENTS.md](CHARACTER_AGENTS.md)** - Agentes Jogadores IA
- **[COMBAT_FLOW.md](COMBAT_FLOW.md)** - Fluxo de combate
- **[RULES_ENGINE.md](RULES_ENGINE.md)** - Engine de regras
- **[TRAINING_PIPELINE.md](TRAINING_PIPELINE.md)** - Pipeline de treinamento

---

## 🔍 Busca Rápida por Tópico

### Componentes Frontend
- **Voice HUD**: [DESIGN_SYSTEM.md](DESIGN_SYSTEM.md#1-voice-hud-interface-de-voz)
- **Character Sheet**: [CHARACTER_SHEET_COMPONENT.md](CHARACTER_SHEET_COMPONENT.md)
- **Journal**: [JOURNAL_COMPONENT.md](JOURNAL_COMPONENT.md)
- **Gameplay Interface**: `src/client-electron/src/components/GameplayInterface.tsx`
- **Design System**: [DESIGN_SYSTEM.md](DESIGN_SYSTEM.md) (inclui CSS base e componentes)

### Backend Services
- **Orchestrator**: [ORCHESTRATOR.md](ORCHESTRATOR.md), [specs/ORCHESTRATOR_SPEC.md](specs/ORCHESTRATOR_SPEC.md)
- **Pipeline Architecture**: [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura de 3 agentes
- **LLM Core**: [specs/LLM_CORE_SPEC.md](specs/LLM_CORE_SPEC.md) - Dual model inference (1.5B + 14B)
- **Qwen-1.5B**: [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md) - "Mestre Reflexo" (reação rápida)
- **Qwen-14B**: [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md) - "Mestre Real" (narrativa completa)
- **Rules5e**: [specs/RULES5E_SERVICE_SPEC.md](specs/RULES5E_SERVICE_SPEC.md)
- **Memory**: [specs/MEMORY_SERVICE_SPEC.md](specs/MEMORY_SERVICE_SPEC.md)
- **TTS**: [specs/TTS_SERVICE_SPEC.md](specs/TTS_SERVICE_SPEC.md)

### Integração
- **MCP**: [MCP_INTEGRATION.md](MCP_INTEGRATION.md)
- **IPC**: [specs/IPC_PROTOCOL_SPEC.md](specs/IPC_PROTOCOL_SPEC.md)

---

## 📖 Guias de Início Rápido

### Para Desenvolvedores
1. Leia [ARCHITECTURE.md](ARCHITECTURE.md) para entender a arquitetura
2. Leia [TASKS.md](TASKS.md) para ver todas as tarefas consolidadas
3. Leia [ROADMAP.md](ROADMAP.md) para ver o progresso
4. Siga [CONFIGURATION.md](CONFIGURATION.md) para configurar o ambiente

### Para Implementadores
1. Leia [TASKS.md](TASKS.md) para escolher uma tarefa
2. Consulte [specs/README.md](specs/README.md) para criar especificações
3. Implemente seguindo [TESTS_MASTER.md](TESTS_MASTER.md) para testes
4. Consulte [guides/](guides/) para guias práticos quando necessário

### Para Testadores
1. Leia [TESTS_MASTER.md](TESTS_MASTER.md) para ver todos os testes e estratégia
2. Execute testes conforme documentação

### Para Designers
1. Leia [DESIGN_SYSTEM.md](DESIGN_SYSTEM.md) para especificações completas, CSS base e componentes

---

## 🔗 Links Úteis

### Documentação Externa
- [AGENTS.md](../AGENTS.md) - Regras e diretrizes para AI assistants
- [Rulebook Documentation](../../rulebook-main/rulebook/RULEBOOK.md) - Diretrizes do rulebook

### Recursos do Projeto
- [README.md](../README.md) - Visão geral do projeto
- [SERVERS_README.md](../SERVERS_README.md) - Documentação dos servidores

---

## 📝 Notas

- **Convenções**: Todos os documentos seguem as diretrizes do rulebook
- **Formato**: Documentação em Markdown, seguindo padrões do projeto
- **Estrutura**: Organizada conforme padrões do rulebook (AGENTS.md)

---

**Índice de Documentação VRPG Client** - Navegação centralizada de toda a documentação do projeto.
