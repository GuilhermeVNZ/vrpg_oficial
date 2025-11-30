# VRPG Client - Virtual RPG Engine

## Visão Geral

O **VRPG Client** é um engine de mesa virtual de RPG com IA local, oferecendo uma experiência imersiva de D&D 5e com Mestre IA, reconhecimento de voz e interface futurística.

## Características Principais

- 🎭 **Pipeline de 3 Agentes**: Orquestrador + Qwen-1.5B (reação rápida) + Qwen-14B (narrativa completa)
- ⚡ **Latência Ultra-Baixa**: Respostas em < 6s com reação inicial em < 1.2s
- 🎤 **Interação por Voz**: ASR/TTS local para comunicação natural
- 🎮 **Interface Futurística**: Design electro-static stippling em Electron
- 🔒 **100% Offline**: Execução local sem dependência de APIs externas
- 🎲 **Regras D&D 5e**: Motor determinístico em Rust
- 🧠 **Memória Semântica**: Sistema de memória de longo prazo com MCP
- 💾 **Persistência de Sessão**: Save/Load completo de estados de jogo

## Arquitetura Modular

```
vrpg-client/
├── docs/                    # Documentação técnica
├── src/
│   ├── client-electron/     # Interface Electron + React
│   ├── game-engine/         # Lógica de sessão e combate
│   ├── llm-core/           # Serviço LLM + Synap + LessTokens
│   ├── asr-service/        # Reconhecimento de voz (Whisper)
│   ├── tts-service/        # Síntese de voz multi-persona
│   ├── rules5e-service/    # Motor de regras D&D 5e
│   ├── memory-service/     # Memória + Classify + Vectorizer
│   └── infra-runtime/      # Inicialização e observabilidade
├── assets/                 # Modelos, arte, áudio
├── config/                 # Configurações centralizadas
└── tests/                  # Testes e CI/CD
```

## Tecnologias

### Frontend
- **Electron** + **React 18** + **TypeScript**
- **PixiJS** para renderização de mapas
- **Tailwind CSS** + **Framer Motion**
- **Web Audio API** para captura/reprodução

### Backend Services
- **Rust** para serviços críticos (Rules5e, ASR, TTS)
- **LLM Local** (GGUF/Candle) + **Synap** para conversação
- **Whisper** local para ASR
- **XTTS/Piper** para TTS multi-voz

### Integração MCP
- **Transmutation**: Conversão de documentos (PDF, DOCX, imagens, áudio) para Markdown
- **Synap**: Conversação entre modelos (Mestre ↔ NPCs ↔ Players IA)
- **Classify**: Classificação de memórias antes da indexação
- **Nexus + Lexum + Vectorizer**: Busca semântica avançada
- **LessTokens**: Compressão de prompts para APIs externas

### Geração de Assets
- **Geração de Imagens**: NPCs, cenas, objetos e eventos via Stable Diffusion
- **LoRA Training**: Modelos adaptados para personagens específicos
- **Estrutura de Aventuras**: Organização de assets por campanha
- **Indexação Automática**: Assets gerados indexados no sistema de memória

## Fluxos Críticos

### Voz → Voz (Interação Principal)
1. **Captura** → ASR (Whisper) → **Texto**
2. **Texto** + **Estado** → LLM Core (+ Synap) → **Resposta IA**
3. **Resposta** → TTS → **Áudio** → **Reprodução**

**Target**: < 300ms de latência total

### Documento → Memória (Processamento de Campanha)
1. **Documento** → Transmutation → **Markdown**
2. **Markdown** → Classify → **Categorias**
3. **Conteúdo + Categorias** → Vectorizer → **Indexação**

**Formatos**: PDF, DOCX, XLSX, PPTX, imagens (OCR), áudio/vídeo (transcrição)

## Instalação Rápida

```bash
# Clone o repositório
git clone <repo-url> vrpg-client
cd vrpg-client

# Instalar dependências
npm install
cargo build --release

# Configurar modelos (primeira execução)
npm run setup-models

# Iniciar aplicação
npm run dev
```

## Configuração

Edite `config/vrpg.json`:

```json
{
  "services": {
    "llm": { "port": 7002, "model": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf" },
    "asr": { "port": 7001, "model": "whisper-large-v3.bin" },
    "tts": { "port": 7003 },
    "rules": { "port": 7004 },
    "memory": { "port": 7005 }
  },
  "mcp": {
    "synap_endpoint": "http://localhost:8001",
    "vectorizer_endpoint": "http://localhost:8002"
  },
  "ui": {
    "theme": "cyberpunk",
    "voice_activation": "push_to_talk"
  }
}
```

## Desenvolvimento

### Estrutura de Comandos

```bash
# Frontend (Electron)
npm run dev:client          # Desenvolvimento
npm run build:client        # Build produção
npm run test:client         # Testes frontend

# Backend Services
cargo run --bin llm-core     # Serviço LLM
cargo run --bin asr-service  # Reconhecimento de voz
cargo run --bin tts-service  # Síntese de voz
cargo test                   # Testes Rust

# Integração
npm run dev:full            # Todos os serviços
npm run test:e2e           # Testes end-to-end
```

### Arquivos de Configuração

- `config/vrpg.json` - Configuração principal
- `config/voices.json` - Configuração de vozes TTS
- `config/mcp.json` - Endpoints MCP services
- `.env` - Variáveis de ambiente (APIs externas)

## Documentação

**📚 [Índice Completo de Documentação](docs/INDEX.md)** - Navegação centralizada de toda a documentação

### Documentação Principal
- 📖 **[Arquitetura](docs/ARCHITECTURE.md)** - Arquitetura técnica completa
- 🎨 **[Design System](docs/DESIGN_SYSTEM.md)** - Sistema de design UI (Glassmorphism)
- 🔧 **[Configuração](docs/CONFIGURATION.md)** - Configuração de todos os módulos
- 🗺️ **[Roadmap](docs/ROADMAP.md)** - Roadmap de implementação por fases
- 📊 **[Status](docs/STATUS.md)** - Status atual do projeto

### Componentes Implementados
- 🎤 **[Voice HUD](docs/VOICE_HUD_COMPONENT.md)** - Interface de voz
- 📋 **[Character Sheet](docs/CHARACTER_SHEET_COMPONENT.md)** - Ficha de personagem
- 📖 **[Journal](docs/JOURNAL_COMPONENT.md)** - Diário de campanha
- 🎮 **[Gameplay Interface](../src/client-electron/src/components/GameplayInterface.tsx)** - Interface principal

### Implementação e Testes
- 📋 **[Tasks Master](docs/TASKS_MASTER.md)** - Lista completa de tarefas
- 📋 **[Tasks Componentes](docs/TASKS_COMPONENTS.md)** - Tasks para componentes implementados
- 🧪 **[Tests Master](docs/TESTS_MASTER.md)** - Plano completo de testes
- 🧪 **[Tests Componentes](docs/TESTS_COMPONENTS.md)** - Testes para componentes implementados
- 🧪 **[Testing](docs/TESTING.md)** - Estratégia de testes

### Especificações Técnicas
- 📐 **[Especificações](docs/specs/)** - Especificações técnicas detalhadas
  - Ver [docs/specs/README.md](docs/specs/README.md) para lista completa

### Integração e Serviços
- 📡 **[Integração MCP](docs/MCP_INTEGRATION.md)** - Integração com serviços MCP
- 🎨 **[Geração de Assets](docs/ASSETS_GENERATION.md)** - Pipeline de geração
- ⚡ **[Performance](docs/PERFORMANCE.md)** - Estratégias de otimização
- 🚀 **[Deploy](docs/DEPLOYMENT.md)** - Estratégias de deploy

## Roadmap

Para ver o roadmap completo e detalhado de implementação, consulte:
- **[ROADMAP.md](docs/ROADMAP.md)** - Roadmap completo por fases
- **[TASKS_MASTER.md](docs/TASKS_MASTER.md)** - Lista completa de tarefas
- **[TESTS_MASTER.md](docs/TESTS_MASTER.md)** - Plano completo de testes

### Status Atual
- ✅ Arquitetura modular definida
- ✅ Documentação completa criada
- ✅ Tasks master list criadas
- ✅ Test plan completo criado
- 🔄 Implementação em planejamento

### Próximos Passos
1. Completar Fase 0 (Infraestrutura Base)
2. Iniciar Fase 1 (Serviços Core)
3. Seguir ordem de prioridade conforme TASKS_MASTER.md

## Contribuição

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

## Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

## Suporte

- 📧 **Email**: suporte@vrpg-client.com
- 💬 **Discord**: [VRPG Community](https://discord.gg/vrpg)
- 📚 **Wiki**: [Documentação Completa](docs/)
- 🐛 **Issues**: [GitHub Issues](https://github.com/vrpg-client/issues)

---

**VRPG Client** - Transformando a experiência de RPG com IA local e tecnologia de ponta! 🎲✨
