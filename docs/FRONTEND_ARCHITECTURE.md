# VRPG Client - Frontend Architecture

## Visão Geral

O frontend do VRPG Client é uma aplicação **Electron + React 18 + TypeScript** que implementa uma interface de mesa virtual de RPG com design **Glassmorphism** inspirado em Baldur's Gate 3 e Solasta.

**Desenvolvido com**: Antigravity IDE (Google) - Gemini 3 e Sonnet 4.5  
**Status**: ✅ Estrutura base implementada | ⚠️ Integração com serviços pendente | ⚠️ Dados mock (hardcoded)  
**Dev Server**: `http://localhost:5173/` (Vite)

**Nota**: Por enquanto, o foco está no backend. A integração frontend-backend será feita posteriormente.

---

## Stack Tecnológica

### Core
- **Electron 28**: Framework desktop multiplataforma
- **React 18.3.1**: Biblioteca UI com Composition API
- **TypeScript 5.3.3**: Tipagem estática
- **Vite 5.0.10**: Build tool e dev server

### Estilização
- **CSS Custom Properties (Design Tokens)**: Sistema de design tokens centralizado
- **Glassmorphism**: Efeitos de vidro fosco com blur e transparência
- **Inline Styles**: Estilos inline para componentes React (sem CSS-in-JS)
  - **Padrão**: Todos os componentes usam `style={{}}` diretamente no JSX
  - **Vantagem**: Flexibilidade dinâmica, sem overhead de runtime
  - **Design Tokens**: Referenciados via CSS variables (`var(--gold-frost)`, etc.)

### Gerenciamento de Estado
- **React Context API**: Para gerenciamento de modais
- **React Hooks**: useState, useEffect, useContext

### Build & Dev Tools
- **@vitejs/plugin-react**: Plugin Vite para React
- **Vitest**: Framework de testes
- **Playwright**: Testes E2E
- **ESLint**: Linting de código

---

## Estrutura de Diretórios

```
src/client-electron/
├── renderer/                    # Aplicação React (renderer process)
│   ├── App.tsx                  # Componente raiz
│   ├── main.tsx                # Entry point React
│   ├── assets/                 # Assets estáticos
│   │   └── background.jpg      # Imagem de fundo
│   ├── components/
│   │   ├── layout/             # Componentes de layout
│   │   │   ├── MainLayout.tsx   # Layout principal (grid system)
│   │   │   ├── TopBar.tsx       # Barra superior (Level, XP)
│   │   │   ├── LeftSidebar.tsx  # Sidebar esquerda (menu de modais)
│   │   │   ├── RightSidebar.tsx # Sidebar direita (cards, chat)
│   │   │   ├── BottomBar.tsx    # Barra inferior (party, narrativa)
│   │   │   ├── CenterCanvas.tsx # Canvas central (background)
│   │   │   └── PushToTalk.tsx   # Componente push-to-talk
│   │   └── modals/              # Sistema de modais
│   │       ├── Modal.tsx        # Componente base de modal
│   │       ├── ModalContext.tsx # Context para gerenciar modais
│   │       ├── ModalManager.tsx # Gerenciador de modais
│   │       ├── CharacterSheetModal.tsx
│   │       ├── AbilitiesModal.tsx
│   │       ├── InventoryModal.tsx
│   │       ├── SpellsModal.tsx
│   │       ├── MapModal.tsx
│   │       ├── JournalModal.tsx
│   │       ├── CompendiumModal.tsx
│   │       └── SettingsModal.tsx
│   └── styles/
│       ├── global.css          # Estilos globais
│       └── design-tokens.css    # Design tokens (CSS variables)
└── src/                         # Componentes legados (a migrar)
    ├── components/
    │   ├── CharacterSheet.tsx
    │   ├── GameplayInterface.tsx
    │   ├── Journal.tsx
    │   └── VoiceHUD.tsx
    └── hooks/
        ├── useCharacterSheet.ts
        ├── useJournal.ts
        └── useVoiceHUD.ts
```

---

## Arquitetura de Layout

### Grid System

O layout principal usa **CSS Grid** com 3 colunas e 3 linhas:

```
┌─────────────────────────────────────────┐
│ TopBar (spans full width)               │
├──────────┬──────────────────┬───────────┤
│          │                  │           │
│ Left     │ Center Canvas   │ Right     │
│ Sidebar  │ (Background)    │ Sidebar   │
│          │                  │           │
├──────────┴──────────────────┴───────────┤
│ BottomBar (spans full width)            │
└─────────────────────────────────────────┘
```

**Grid Template**:
- **Colunas**: `auto 1fr auto` (Left Sidebar | Center | Right Sidebar)
- **Linhas**: `auto 1fr auto` (TopBar | Content | BottomBar)

### Z-Index Layers

- **z-index: -1**: Background layer (CenterCanvas)
- **z-index: 10**: UI panels (TopBar, Sidebars, BottomBar)
- **z-index: 20**: Overlays (PushToTalk, interactive elements)
- **z-index: 1000**: Modals

---

## Sistema de Design

### Design Tokens

O sistema de design usa **CSS Custom Properties** definidas em `design-tokens.css`:

#### Cores Principais
- **Gold Primary** (`#D4AF37`): Ouro frio - bordas e acentos
- **Arcane Blue** (`#4A90E2`): Azul arcano - magia e seleção
- **Background Dark** (`#0F0F0F`): Preto fundo

#### Glassmorphism
- **Glass Background**: `rgba(255, 255, 255, 0.05)`
- **Backdrop Blur**: `16px`
- **Backdrop Saturate**: `180%`
- **Glass Border**: `rgba(255, 255, 255, 0.22)`

#### Tipografia
- **Serif**: `'Crimson Text'` - Títulos, narrativa
- **Sans**: `'Inter'` - UI, botões, valores
- **Display**: `'Cinzel'` - Títulos grandes
- **Mono**: `'Fira Code'` - Dados, código

### Classes Utilitárias

#### `.glass-panel`
Painel com efeito glassmorphism:
- Background translúcido
- Backdrop blur
- Borda sutil
- Sombra suave

#### `.glass-interactive`
Elemento interativo com glassmorphism:
- Hover effects
- Transições suaves
- Glow effects

---

## Componentes Principais

### 1. MainLayout

**Localização**: `renderer/components/layout/MainLayout.tsx`

**Responsabilidades**:
- Define estrutura de grid principal
- Gerencia camadas de z-index
- Renderiza componentes de layout

**Estrutura**:
```tsx
<MainLayout>
  <TopBar />
  <LeftSidebar />
  <CenterCanvas />
  <RightSidebar />
  <PushToTalk />
  <BottomBar />
</MainLayout>
```

### 2. TopBar

**Localização**: `renderer/components/layout/TopBar.tsx`

**Funcionalidades**:
- Exibe nível do personagem (círculo)
- Barra de XP com progresso
- Posicionamento absoluto (top-left)

**Estados**:
- Level: Hardcoded (5)
- XP: Hardcoded (12,500 / 20,000)

### 3. LeftSidebar

**Localização**: `renderer/components/layout/LeftSidebar.tsx`

**Funcionalidades**:
- Menu vertical com ícones SVG customizados
- 8 botões de acesso rápido a modais:
  - Character Sheet (C)
  - Abilities (A)
  - Inventory (I)
  - Spells (S)
  - Map (M)
  - Journal (J)
  - Compendium (B)
  - Settings (Esc)

**Características**:
- Ícones SVG com gradientes e glow effects
- Hover effects com glow dourado
- Integração com ModalContext

### 4. RightSidebar

**Localização**: `renderer/components/layout/RightSidebar.tsx`

**Funcionalidades**:
- Cards de status/efeitos (ex: "Peaceful Crane Stance")
- Área de chat com mensagens
- Input de texto
- Indicador de voz (Ouvindo/Processando/Falando)

**Estrutura**:
- Cards de efeitos (scrollable)
- Chat bubbles
- Voice status bar
- Text input

### 5. BottomBar

**Localização**: `renderer/components/layout/BottomBar.tsx`

**Funcionalidades**:
- Retratos de party members (círculos com HP arcs)
- Painel de narrativa do Gamemaster
- Posicionamento centralizado

**Características**:
- Retratos com bordas glassmorphism
- HP arcs SVG (círculos com progresso)
- Painel de narrativa com glow dourado

### 6. CenterCanvas

**Localização**: `renderer/components/layout/CenterCanvas.tsx`

**Responsabilidades**:
- Renderiza background image
- Canvas para futura renderização de mapas (PixiJS)

**Estado Atual**:
- Background image estática
- Preparado para integração com PixiJS

### 7. PushToTalk

**Localização**: `renderer/components/layout/PushToTalk.tsx`

**Status**: ⚠️ Implementação pendente

**Funcionalidades Planejadas**:
- Botão push-to-talk
- Indicador visual de estado (listening/processing)
- Integração com ASR service

---

## Sistema de Modais

### Arquitetura

O sistema de modais usa **React Context API** para gerenciamento de estado global:

```
ModalContext (Provider)
  ├── activeModal: ModalType | null
  ├── openModal(modal: ModalType): void
  └── closeModal(): void
```

### ModalContext

**Localização**: `renderer/components/modals/ModalContext.tsx`

**Funcionalidades**:
- Gerencia estado do modal ativo
- Fornece hooks `useModal()` para componentes
- Gerencia keyboard shortcuts:
  - `C`: Character Sheet
  - `A`: Abilities
  - `I`: Inventory
  - `S`: Spells
  - `M`: Map
  - `J`: Journal
  - `B`: Compendium
  - `Esc`: Settings (ou fecha modal)

### Modal (Componente Base)

**Localização**: `renderer/components/modals/Modal.tsx`

**Props**:
- `isOpen: boolean`
- `onClose: () => void`
- `title: string`
- `children: ReactNode`
- `maxWidth?: string` (default: '900px')

**Características**:
- Backdrop com blur
- Animação de entrada (fadeIn + slideIn)
- Scrollbar customizada (dourada)
- Close on click outside
- Previne scroll do body quando aberto

### ModalManager

**Localização**: `renderer/components/modals/ModalManager.tsx`

**Responsabilidades**:
- Renderiza modais baseado em `activeModal`
- Gerencia lifecycle de modais

**Modais Disponíveis**:
1. CharacterSheetModal
2. AbilitiesModal
3. InventoryModal
4. SpellsModal
5. MapModal
6. JournalModal
7. CompendiumModal
8. SettingsModal

---

## Estilos e CSS

### Global Styles

**Localização**: `renderer/styles/global.css`

**Funcionalidades**:
- Reset CSS básico
- Scrollbar customizada (dourada)
- Classes utilitárias de glassmorphism
- Tipografia global

### Design Tokens

**Localização**: `renderer/styles/design-tokens.css`

**Estrutura**:
- Cores base e temáticas
- Tokens de glassmorphism
- Tipografia
- Espaçamentos e arredondamentos
- Compatibilidade com tokens antigos

**Uso**:
```css
.element {
  background: var(--vrpg-glass-background);
  border: 1px solid var(--glass-border);
  color: var(--gold-frost);
}
```

---

## Integração com Backend

### Status Atual

⚠️ **Integração pendente** - O frontend está atualmente com dados hardcoded (mock data).

**Dados Mock Identificados**:
- `CharacterSheetModal`: `mockCharacter` (personagem completo D&D 5e)
- `JournalModal`: `mockJournal` (quests e notes)
- `BottomBar`: `party` array (4 membros com HP)
- `RightSidebar`: Cards de efeitos e mensagens de chat hardcoded
- `TopBar`: Level e XP hardcoded (5, 12,500/20,000)

### Serviços a Integrar

1. **Game Engine**:
   - Dados de personagem (level, XP, HP)
   - Party members
   - Estado da sessão

2. **ASR Service**:
   - Push-to-talk
   - Status de voz (listening/processing)

3. **TTS Service**:
   - Status de fala da IA
   - Texto sendo falado

4. **Memory Service**:
   - Journal entries
   - Compendium data
   - Busca semântica

5. **Orchestrator**:
   - Narrativa do Gamemaster
   - Eventos de jogo
   - Transições de estado

### Comunicação

**Planejado**: IPC (Inter-Process Communication) via Electron

```
Renderer Process (React)
  ↕ IPC
Main Process (Electron)
  ↕ HTTP/WebSocket
Backend Services (Rust)
```

---

## Build e Desenvolvimento

### Scripts NPM

```json
{
  "dev": "concurrently \"npm run dev:client\" \"npm run dev:services\"",
  "dev:client": "vite",
  "build": "npm run build:client && npm run build:services && npm run build:electron",
  "build:client": "vite build",
  "build:electron": "electron-builder build"
}
```

### Vite Configuration

**Localização**: `vite.config.ts`

**Configurações**:
- Plugin React
- Alias `@` para `src/client-electron/renderer`
- Base path relativo para Electron

### Electron Builder

**Configuração**: `package.json > build`

**Targets**:
- Windows: NSIS installer
- macOS: DMG
- Linux: AppImage + DEB

---

## Testes

### Estrutura Planejada

```
tests/
├── unit/              # Testes unitários de componentes
├── integration/       # Testes de integração
└── e2e/              # Testes end-to-end (Playwright)
```

### Cobertura Mínima

- **95%** de cobertura (conforme AGENTS.md)
- **Vitest** para testes unitários
- **Playwright** para testes E2E

---

## Próximos Passos

### Alta Prioridade

1. **Integração com Backend**:
   - [ ] Conectar com Game Engine para dados de personagem
   - [ ] Integrar ASR/TTS services
   - [ ] Conectar com Memory Service para Journal/Compendium

2. **Componentes Pendentes**:
   - [ ] Implementar PushToTalk completamente
   - [ ] Implementar modais (CharacterSheet, Journal, etc.)
   - [ ] Integrar VoiceHUD com sistema de voz

3. **PixiJS Integration**:
   - [ ] Renderização de mapas no CenterCanvas
   - [ ] Sistema de grid
   - [ ] Tokens e miniaturas

### Média Prioridade

1. **Melhorias de UX**:
   - [ ] Animações de transição
   - [ ] Feedback visual de ações
   - [ ] Loading states

2. **Acessibilidade**:
   - [ ] ARIA labels
   - [ ] Navegação por teclado
   - [ ] Screen reader support

3. **Performance**:
   - [ ] Lazy loading de modais
   - [ ] Memoização de componentes
   - [ ] Virtual scrolling para listas grandes

---

## Padrões de Implementação

### Componentes Funcionais

Todos os componentes são **React Functional Components** com TypeScript:
- Sem classes, apenas funções
- Hooks para estado (`useState`, `useEffect`, `useContext`)
- Props tipadas com interfaces TypeScript

**Exemplo**:
```tsx
interface ComponentProps {
    isOpen: boolean;
    onClose: () => void;
}

const Component: React.FC<ComponentProps> = ({ isOpen, onClose }) => {
    // ...
};
```

### Inline Styles Pattern

**Padrão Consistente**:
- Todos os estilos são inline via `style={{}}`
- Design tokens acessados via CSS variables
- Valores dinâmicos calculados inline quando necessário

**Exemplo**:
```tsx
<div style={{
    background: 'rgba(15, 15, 15, 0.7)',
    backdropFilter: 'blur(16px) saturate(180%)',
    border: '1px solid var(--glass-border)',
    color: 'var(--gold-frost)',
    padding: '20px',
    borderRadius: '16px'
}}>
```

### Mock Data Pattern

**Status Atual**: Componentes usam dados mock hardcoded:
- `mockCharacter` em `CharacterSheetModal`
- `mockJournal` em `JournalModal`
- `party` array em `BottomBar`
- Dados estáticos em `RightSidebar` (cards, chat)

**Próximo Passo**: Substituir por chamadas ao backend via IPC/HTTP.

### Component Composition

**Padrão**: Componentes pequenos e focados:
- Componentes de layout separados
- Modais como componentes independentes
- Componentes auxiliares inline (ex: `Section`, `AttributeBox` em CharacterSheetModal)

**Exemplo de Composição**:
```tsx
// Componente auxiliar inline
const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{ /* styles */ }}>
        <div>{title}</div>
        {children}
    </div>
);
```

### SVG Icons Pattern

**Padrão**: Ícones SVG customizados inline:
- Cada ícone é um componente React funcional
- Gradientes e filtros definidos inline
- Reutilizáveis e tipados

**Exemplo** (LeftSidebar):
```tsx
const CharacterSheetIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
        <defs>
            <linearGradient id="arcaneGold">...</linearGradient>
            <filter id="glow">...</filter>
        </defs>
        <g stroke="url(#arcaneGold)" filter="url(#glow)">...</g>
    </svg>
);
```

## Notas Técnicas

### Por que Inline Styles?

O frontend usa **inline styles** ao invés de CSS-in-JS ou CSS modules para:
- **Simplicidade**: Menos configuração
- **Performance**: Menos overhead de runtime
- **Flexibilidade**: Fácil de ajustar dinamicamente
- **Consistência**: Padrão único em todo o código

**Trade-off**: Menos reutilização de estilos, mas compensado pelo sistema de design tokens e componentes auxiliares.

### Glassmorphism Implementation

O efeito glassmorphism é alcançado através de:
- `backdrop-filter: blur(16px) saturate(180%)`
- Background translúcido (`rgba(255, 255, 255, 0.05)`)
- Bordas sutis com transparência
- Sombras suaves

**Compatibilidade**: Requer navegadores modernos (Electron usa Chromium).

### TypeScript Strict Mode

**Padrão**: TypeScript com tipagem estrita:
- Interfaces para todas as props
- Tipos explícitos para estados
- Sem `any` (exceto quando necessário)

**Exemplo**:
```tsx
interface ModalContextType {
    activeModal: ModalType;
    openModal: (modal: ModalType) => void;
    closeModal: () => void;
}
```

### Glassmorphism Implementation

O efeito glassmorphism é alcançado através de:
- `backdrop-filter: blur(16px) saturate(180%)`
- Background translúcido (`rgba(255, 255, 255, 0.05)`)
- Bordas sutis com transparência
- Sombras suaves

**Compatibilidade**: Requer navegadores modernos (Electron usa Chromium).

---

## Referências

- **[DESIGN_SYSTEM.md](DESIGN_SYSTEM.md)** - Sistema de design completo
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Arquitetura geral do projeto
- **[TASKS_MASTER.md](TASKS_MASTER.md)** - Tasks de implementação

---

**Última Atualização**: 2025-01-XX  
**Status**: ✅ Estrutura base implementada | ⚠️ Integração pendente

