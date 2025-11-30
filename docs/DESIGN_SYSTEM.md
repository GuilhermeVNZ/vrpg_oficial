# VRPG Client - Design System

## Visão Geral

**VRPG Design System — Glassmorphism Apple-Style + Foundry VTT + Baldur's Gate 3 + Solasta Hybrid**

O frontend do VRPG combina **glassmorphism funcional estilo Apple** (macOS Big Sur / iOS 17 / Vision OS) com a **visão macro de campanha do Foundry VTT**, a **experiência de personagem do BG3** e a **tática do Solasta**, criando uma mesa virtual viva comandada por IA com agência humana.

### 🎨 Estética Visual

**Glassmorphism Apple-Style**:
- Painéis translúcidos com perspectiva física (vidro real)
- Vidro interage com o ambiente, não é apenas blur genérico
- UI minimalista, confortável, sofisticada e com profundidade real
- **Regra fundamental**: Vidro é o container, nunca o conteúdo

### Princípios de Design

A UI opera em **camadas mentais** (não técnicas), equilibrando:

- **Visão macro persistente** (mapa como elemento dominante)
- **Interação contextual** (cards flutuantes, pop-ups)
- **Navegação modular** (painéis laterais retráteis)
- **Ação rápida** (hotbar estilo MMO/CRPG)

### Fluxo Cognitivo do Jogador

1. **Onde estou?** → Mapa central (canvas)
2. **Quem sou?** → Card flutuante de personagem
3. **O que posso fazer?** → Barra inferior (hotbar)
4. **O que aconteceu?** → Painel lateral direito (log/sistema)
5. **Ferramentas?** → Painel lateral esquerdo (toolbelt)

### Estrutura Principal (4 Zonas Funcionais)

A UI é dividida em **quatro zonas funcionais principais**:

1. **Área Central (80% do espaço horizontal)**: Cena / Battlemap / Story Scene
   - Elemento dominante da interface
   - Mostra mapas, cenários estáticos, imagens cinemáticas, dioramas, battlemaps
   - O foco da imersão: o jogador olha primeiro aqui
   - Layers: tokens posicionados, grid invisível/visível, marcações, highlights temporários

2. **Parte Superior**: Indicador de Estado / Rolagem
   - Ícone central com texto (ex: "Ready to Roll")
   - Status global do jogo: aguardando jogador, aguardando mestre, turno ativo, combate pausado, evento narrativo
   - Funciona como HUD do mestre, mas público

3. **Painel Lateral Direito**: Sheet / Log / Contexto
   - Área deslizável em cards empilhados
   - Mostra: habilidades ativas, buffs, stances, logs de ações, tooltips detalhados
   - Visual estilo statblock do Foundry
   - Dois modos: Modo Mestre (logs + controle) / Modo Jogador (ficha + ações + status)

4. **Rodapé**: Party UI + Action Bar
   - **Party UI**: Avatares circulares do party (estilo BG3)
     - Retratos grandes com borda
     - Anéis coloridos (HP, status) → leitura instantânea
     - 3 estados: Selecionado, Em fala, Em combate/turn
     - **AAF (Avatar Action First)**: Interação primária via avatares
   - **Action Bar**: Slots numerados 1-10
     - Itens, Skills, Macros rápidas
     - Abre ferramentas, rolagens, fichas
     - "Teclado da ação" estilo Divinity / Baldur

Tudo deve ser widescreen, limpo, moderno e responsivo — mas fiel ao "feeling" Foundry + BG3.

## Estilo Visual (Glassmorphism Apple-Style + BG3 / Solasta Hybrid)

> 📘 **CSS Base Completo**: Veja a seção "CSS Base e Componentes" abaixo para implementação prática com todos os componentes CSS prontos para uso.

### Filosofia de Design

A UI usa **glassmorphism funcional estilo Apple** (macOS Big Sur / iOS 17 / Vision OS) como base visual, combinado com elementos temáticos de BG3/Solasta.

**Princípios fundamentais**:
- **Vidro é o container, nunca o conteúdo**: Texto, ícones e retratos são sempre sólidos
- **Profundidade real**: Ambiente → Vidro com blur → Conteúdo sólido + contraste
- **Hierarquia visual**: Quanto mais importante → menos vidro
- **Performance**: Blur apenas em containers fixos, conteúdo interno nunca refaz blur

### Tokens de Design System (Design Tokens & CSS Base)

**Kit de Desenvolvimento Completo** - Design Tokens e CSS Base para implementação

#### 1. Design Tokens (Root Variables)

```css
/* -------------------------------------------------------------------------- */
/* VRPG Design Tokens (CSS Custom Properties) */
/* Theme: Glassmorphism Apple-Style + Foundry VTT + BG3/Solasta Hybrid */
/* -------------------------------------------------------------------------- */

:root {
  /* ===== CORES BASE & TEMÁTICAS ===== */

  /* Cores Primárias & Acentos (Estilo BG3/Solasta) */
  --vrpg-color-gold-primary: #D4AF37;      /* Ouro Frio - Bordas e acentos */
  --vrpg-color-gold-glow: rgba(212, 175, 55, 0.6); /* Brilho do Ouro */
  --vrpg-color-arcane-blue: #4A90E2;       /* Azul Arcano - Magia e seleção */
  --vrpg-color-arcane-glow: rgba(74, 144, 226, 0.7); /* Brilho Arcano */

  /* Cores de Status */
  --vrpg-color-status-health: #4CAF50;     /* Verde - HP */
  --vrpg-color-status-resource: #2196F3;   /* Azul - Mana/Recursos */

  /* Cores de Backgrounds & Sombras */
  --vrpg-color-bg-dark: #0F0F0F;           /* Preto Fundo */
  --vrpg-color-bg-overlay: rgba(0, 0, 0, 0.5); /* Overlay Escuro */
  --vrpg-color-shadow: rgba(0, 0, 0, 0.5); /* Sombra Genérica */

  /* ===== TIPOGRAFIA ===== */

  --vrpg-font-serif: 'Crimson Text', 'Georgia', serif;  /* Títulos, narrativa */
  --vrpg-font-sans: 'Inter', 'Roboto', sans-serif;      /* UI, botões, valores */

  /* ===== GLASSMORPHISM & EFEITOS VISUAIS ===== */

  /* Blur & Saturação do Vidro */
  --vrpg-glass-backdrop-blur: 16px;        /* Intensidade do desfoque */
  --vrpg-glass-backdrop-saturate: 180%;    /* Intensidade da saturação */
  --vrpg-glass-background: rgba(255, 255, 255, 0.05); /* Cor base do vidro translúcido */

  /* Bordas de Vidro */
  /* Borda Externa Brilhante */
  --vrpg-glass-border-gradient: linear-gradient(
    135deg,
    rgba(255, 255, 255, 0.2) 0%,
    rgba(212, 175, 55, 0.1) 50%,
    rgba(0, 0, 0, 0.1) 100%
  );
  /* Borda Interna (Reflexo) */
  --vrpg-glass-inner-border: inset 0 0 0 1px rgba(255, 255, 255, 0.1);

  /* Sombras de Vidro */
  --vrpg-glass-shadow-sm: 0 4px 16px 0 rgba(0, 0, 0, 0.2); /* Sombra suave */
  --vrpg-glass-shadow-md: 0 8px 32px 0 rgba(0, 0, 0, 0.3); /* Sombra média */

  /* Brilhos & Glows */
  --vrpg-glow-arcane-sm: 0 0 10px var(--vrpg-color-arcane-glow);
  --vrpg-glow-gold-sm: 0 0 10px var(--vrpg-color-gold-glow);

  /* ===== ESPAÇAMENTOS & ARREDONDAMENTOS ===== */

  --vrpg-spacing-sm: 8px;
  --vrpg-spacing-md: 16px;
  --vrpg-spacing-lg: 24px;

  --vrpg-radius-sm: 8px;
  --vrpg-radius-md: 12px;
  --vrpg-radius-lg: 16px; /* Padrão para painéis principais */
  
  /* ===== COMPATIBILIDADE COM TOKENS ANTIGOS ===== */
  
  /* Tokens de Vidro (mantidos para compatibilidade) */
  --glass-light: var(--vrpg-glass-background);
  --glass-dark: rgba(40, 40, 55, 0.18);
  --glass-border: rgba(255, 255, 255, 0.22);
  --glass-shadow: var(--vrpg-glass-shadow-md);
  
  /* Blur Levels */
  --blur-low: 12px;
  --blur-md: var(--vrpg-glass-backdrop-blur);
  --blur-high: 24px;
  --blur-ultra: 30px;
  
  /* Saturation (para backdrop-filter) */
  --saturate-normal: 135%;
  --saturate-high: var(--vrpg-glass-backdrop-saturate);
  
  /* Border Radius */
  --radius-sm: 12px;
  --radius-md: 18px;
  --radius-lg: 22px;
  
  /* Spacing */
  --spacing-xs: 6px;
  --spacing-sm: 10px;
  --spacing-md: 16px;
  --spacing-lg: 22px;
  --spacing-xl: 32px;
  
  /* ===== CORES TEMÁTICAS (BG3/Solasta) ===== */
  
  /* Cores Principais */
  --gold-frost: #D4AF37;              /* Dourado fosco - ornamentos */
  --arcane-blue: #4A90E2;             /* Azul arcano - elementos mágicos */
  --dark-brown: #3D2817;              /* Marrom escuro elegante - fundos */
  --soft-black: #0F0F0F;               /* Preto suave - backgrounds */
  --pure-black: #000000;               /* Preto puro - contraste */
  
  /* Cores de Status */
  --health-green: #4CAF50;            /* Verde - HP, vida */
  --damage-red: #F44336;              /* Vermelho - dano, perigo */
  --mana-blue: #2196F3;                /* Azul - mana, magia */
  --stamina-yellow: #FFC107;           /* Amarelo - stamina, energia */
  
  /* Cores de Persona */
  --player-blue: #00BCD4;              /* Azul - jogador falando */
  --dm-purple: #9C27B0;               /* Roxo - mestre IA falando */
  --npc-gold: #FFD700;                 /* Dourado - NPC falando */
  --npc-green: #4CAF50;                /* Verde - NPC alternativo */
  
  /* Efeitos */
  --glow-intensity: 0.6;               /* Intensidade do brilho */
  --shadow-depth: 0.3;                 /* Profundidade de sombras */
  --border-gold: rgba(212, 175, 55, 0.8); /* Borda dourada */
}
```

### Características Visuais

- **Ornamentos dourados discretos**: Bordas esculpidas com textura fantasy moderna
- **Bordas esculpidas**: Textura fantasy moderna com relevo sutil
- **Glow leve**: Efeito de brilho suave em elementos importantes
- **Hover com partículas finas**: Partículas sutis ao passar o mouse
- **UI de alto contraste**: Legibilidade garantida em todas as condições
- **Tipografia serif elegante**: Estilo BG3 com fontes serifadas para títulos

### Tipografia

```css
:root {
  /* Fontes Principais */
  --font-serif: 'Crimson Text', 'Georgia', serif;  /* Títulos, narrativa */
  --font-sans: 'Inter', 'Roboto', sans-serif;      /* UI, botões */
  --font-display: 'Cinzel', 'Playfair Display', serif; /* Títulos grandes */
  --font-mono: 'Fira Code', 'Courier New', monospace;   /* Dados, código */
}

/* Hierarquia Tipográfica */
.title-large {
  font-family: var(--font-display);
  font-size: 32px;
  font-weight: 700;
  color: var(--gold-frost);
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
  letter-spacing: 1px;
}

.title-medium {
  font-family: var(--font-serif);
  font-size: 24px;
  font-weight: 600;
  color: var(--arcane-blue);
}

.body-text {
  font-family: var(--font-sans);
  font-size: 16px;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.9);
}
```

## Layout Principal

### Estrutura Base (Foundry + BG3 Hybrid)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  [Indicador de Estado: "Ready to Roll"] - Topo Central (80px)                │
│  Status global: aguardando jogador/mestre, turno ativo, combate pausado      │
├──────┬──────────────────────────────────────────────────────┬───────────────┤
│      │                                                        │               │
│ Tool │                                                        │   Sheet /     │
│ Belt │         CENA CENTRAL (80% horizontal)                 │   Log /        │
│ Left │         - Mapas, cenários, imagens cinemáticas        │   Contexto    │
│      │         - Battlemaps, dioramas                        │   (Direita)   │
│      │         - Layers: tokens, grid, marcações             │   - Cards     │
│      │         - Highlights temporários                      │     empilhados│
│      │         - Zoom livre, pan, arrastar                   │   - Habilidades│
│      │                                                        │   - Buffs     │
│      │         [Overlay de Estado] - Centro (quando ativo)   │   - Logs      │
│      │         [Rolagem de Dados] - Centro (quando ativo)    │   - Tooltips  │
│      │                                                        │               │
├──────┴──────────────────────────────────────────────────────┴───────────────┤
│  RODAPÉ: Party UI + Action Bar (120px)                                      │
│  [Avatar1] [Avatar2] [Avatar3] ... [TALK] [1] [2] [3] [4] [5] [6] [7] [8] │
│  (AAF - Avatar Action First)                                                │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Hierarquia Visual (Camadas Mentais)

| Camada | Função | Elemento |
|--------|--------|----------|
| **Raiz (Mapa)** | Mundo, espaço, sigilo, progressão | Canvas central dominante |
| **Ferramentas (Left)** | Operação do Mestre IA | Toolbelt vertical (ícones) |
| **Narrativa (Pop-up)** | Personagem, identidade | Card flutuante de personagem |
| **Sistema (Right)** | Regras, recursos, mecânica | Painel lateral direito |
| **Ação (Bottom)** | Execução, gesto, input | Hotbar com slots numerados |

### Resoluções Suportadas

- **21:9 (Ultrawide)**: 2560×1080, 3440×1440
- **16:9 (Standard)**: 1920×1080, 2560×1440, 3840×2160 (4K)
- **Adaptação**: Todos os layouts adaptam sem distorção

## Componentes Principais

### 1. Mapa Central (Canvas Dominante) — Foundry Style

**Dimensões**: Centro da tela, ocupando 70-80% da área disponível  
**Estilo**: Canvas 2D/3D com zoom livre, pan, arrastar (estilo Foundry)  
**Função**: Elemento dominante — toda UI gira ao redor do mapa

> 💡 **Filosofia**: O mapa é a "mesa". Ele não te joga para menus fechados — ele te ancora na worldbuilding.

```css
.map-container {
  position: relative;
  width: 100%;
  height: calc(100vh - 200px); /* Topo + Hotbar */
  background: var(--dark-brown);
  overflow: hidden;
  /* Sem bordas — mapa é o elemento raiz */
  box-shadow: 
    inset 0 0 100px rgba(0, 0, 0, 0.8),
    inset 0 0 200px rgba(61, 40, 23, 0.3);
}

/* ===== GLASSMORPHISM: Overlay de estado no centro do mapa ===== */
.map-overlay-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1000;
  padding: var(--spacing-lg) var(--spacing-xl);
  background: rgba(255, 255, 255, 0.12);
  backdrop-filter: blur(var(--blur-high)) saturate(var(--saturate-high));
  -webkit-backdrop-filter: blur(var(--blur-high)) saturate(var(--saturate-high));
  border: 2px solid var(--border-glass-strong);
  border-radius: var(--radius-lg);
  box-shadow: 
    0 8px 20px var(--shadow-medium),
    inset 0 0 30px rgba(212, 175, 55, 0.1);
  text-align: center;
  pointer-events: none;
}

/* ===== CONTEÚDO SÓLIDO (nunca translúcido) ===== */
.map-overlay-state-text {
  font-family: var(--font-display);
  font-size: 32px;
  font-weight: 700;
  color: var(--text-light); /* SEMPRE sólido */
  text-shadow: 
    0 0 20px rgba(212, 175, 55, 0.8),
    0 0 40px rgba(212, 175, 55, 0.4);
  margin-bottom: 12px;
}

.map-overlay-state-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto;
  filter: drop-shadow(0 0 15px rgba(212, 175, 55, 0.8));
  /* Ícone sempre sólido */
}

.map-canvas {
  width: 100%;
  height: 100%;
  position: relative;
  cursor: grab;
  /* Renderizado via PixiJS ou Three.js */
  /* Suporta zoom livre, pan, arrastar */
  /* Modos: Exploração (mapa mundo) / Combate (battlemap grid) */
}

.map-canvas:active {
  cursor: grabbing;
}

/* Overlay de estado no centro do mapa */
.map-overlay-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1000;
  padding: 24px 48px;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(8px);
  border: 3px solid var(--border-gold);
  border-radius: 16px;
  box-shadow: 
    0 0 50px rgba(212, 175, 55, 0.6),
    inset 0 0 30px rgba(212, 175, 55, 0.1);
  text-align: center;
  pointer-events: none;
}

.map-overlay-state-text {
  font-family: var(--font-display);
  font-size: 32px;
  font-weight: 700;
  color: var(--gold-frost);
  text-shadow: 
    0 0 20px rgba(212, 175, 55, 0.8),
    0 0 40px rgba(212, 175, 55, 0.4);
  margin-bottom: 12px;
}

.map-overlay-state-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto;
  filter: drop-shadow(0 0 15px rgba(212, 175, 55, 0.8));
}

.token {
  position: absolute;
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 3px solid var(--gold-frost);
  box-shadow: 
    0 0 15px rgba(212, 175, 55, 0.6),
    0 4px 8px rgba(0, 0, 0, 0.4);
  cursor: pointer;
  transition: all 0.3s ease;
}

.token::before {
  content: '';
  position: absolute;
  bottom: -8px;
  left: 50%;
  transform: translateX(-50%);
  width: 60px;
  height: 8px;
  background: radial-gradient(ellipse, 
    rgba(212, 175, 55, 0.4) 0%, 
    transparent 70%);
  border-radius: 50%;
  filter: blur(4px);
}

.token:hover {
  transform: scale(1.15) translateY(-4px);
  box-shadow: 
    0 0 25px rgba(212, 175, 55, 0.8),
    0 8px 16px rgba(0, 0, 0, 0.5);
  z-index: 100;
}

.token.selected {
  border-color: var(--arcane-blue);
  box-shadow: 
    0 0 30px rgba(74, 144, 226, 0.8),
    0 0 15px rgba(212, 175, 55, 0.6);
}

.token.player {
  border-color: var(--player-blue);
}

.token.npc {
  border-color: var(--npc-gold);
}

.token.enemy {
  border-color: var(--damage-red);
}
```

**Características**:
- **Zoom livre**: Scroll wheel, pinch, ou controles de zoom
- **Pan**: Arrastar com mouse/touch, ou setas do teclado
- **Modos de visualização**:
  - **Exploração**: Mapa do mundo (regiões, cidades, geografias)
  - **Combate**: Battlemap com grid (tática, alcance, LoS)
  - **Cena**: Close-up de localização específica
- **Tokens**: Com halo ou círculo no chão (sombra projetada)
- **Overlays**: Ícones de região, marcadores, pontos de interesse
- **Interações**: Clique, voz, ou atalhos de teclado
- **Assets dinâmicos**: Cenas geradas por IA (Flux + LoRA) como backgrounds
- **Estado overlay**: "Game Paused", "Mestre IA pensando...", "Carregando assets..."

### 2. Turn Order — Topo da Tela

**Dimensões**: 100% width × 80px height  
**Função**: Mostrar ordem de turnos em combate  
**Estilo**: Linha horizontal de cards, estilo BG3

```css
.turn-order-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 24px;
  background: linear-gradient(180deg, 
    rgba(15, 15, 15, 0.95) 0%, 
    rgba(0, 0, 0, 0.98) 100%);
  border-bottom: 2px solid var(--border-gold);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.6);
  overflow-x: auto;
  overflow-y: hidden;
}

.turn-order-card {
  min-width: 120px;
  height: 64px;
  background: linear-gradient(135deg, 
    rgba(61, 40, 23, 0.9) 0%, 
    rgba(15, 15, 15, 0.9) 100%);
  border: 2px solid rgba(212, 175, 55, 0.4);
  border-radius: 8px;
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
}

.turn-order-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(212, 175, 55, 0.1) 0%, 
    transparent 100%);
  border-radius: 6px;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.turn-order-card:hover::before {
  opacity: 1;
}

.turn-order-card.active {
  border-color: var(--gold-frost);
  box-shadow: 
    0 0 20px rgba(212, 175, 55, 0.6),
    inset 0 0 20px rgba(212, 175, 55, 0.1);
  transform: translateY(-4px);
}

.turn-order-card.active::before {
  opacity: 1;
  background: linear-gradient(135deg, 
    rgba(212, 175, 55, 0.3) 0%, 
    transparent 100%);
}

.turn-order-portrait {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 2px solid var(--gold-frost);
  object-fit: cover;
  box-shadow: 0 0 10px rgba(212, 175, 55, 0.4);
}

.turn-order-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.turn-order-name {
  font-family: var(--font-serif);
  font-size: 14px;
  font-weight: 600;
  color: var(--gold-frost);
  text-shadow: 0 0 5px rgba(212, 175, 55, 0.5);
}

.turn-order-hp {
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--health-green);
  display: flex;
  align-items: center;
  gap: 4px;
}

.hp-bar {
  width: 60px;
  height: 4px;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 2px;
  overflow: hidden;
}

.hp-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, 
    var(--health-green) 0%, 
    #66BB6A 100%);
  transition: width 0.3s ease;
  box-shadow: 0 0 5px var(--health-green);
}
```

**Características (Turn Order)**:
- Aparece somente em combate
- Cada card tem retrato, nome, HP e status
- O card ativo fica iluminado com brilho dourado
- O turno avança automaticamente conforme o mestre IA narra
- Scroll horizontal se houver muitos participantes

**Características (Talking Cards - Fora de Combate)**:
- Mostra participantes ativos da cena (players, NPCs relevantes)
- Cards horizontais com retrato e nome
- Card do falante atual fica destacado (borda colorida, glow)
- Pode incluir múltiplos cards simultaneamente (conversação em grupo)
- Substitui Turn Order quando não há combate ativo

### 3. Talking Cards (Quem Está Falando)

**Dimensões**: 100% width × 80px height  
**Função**: Mostrar quem está na cena conversando (fora de combate)  
**Formato**: Cards horizontais com retrato

```css
.talking-cards-container {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 24px;
  background: linear-gradient(180deg, 
    rgba(15, 15, 15, 0.95) 0%, 
    rgba(0, 0, 0, 0.98) 100%);
  border-bottom: 2px solid var(--border-gold);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.6);
  overflow-x: auto;
}

.talking-card {
  min-width: 140px;
  height: 64px;
  background: linear-gradient(135deg, 
    rgba(61, 40, 23, 0.8) 0%, 
    rgba(15, 15, 15, 0.8) 100%);
  border: 2px solid rgba(212, 175, 55, 0.3);
  border-radius: 8px;
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: all 0.3s ease;
  position: relative;
}

.talking-card.speaking {
  border-color: var(--player-blue);
  box-shadow: 
    0 0 25px rgba(0, 188, 212, 0.6),
    inset 0 0 20px rgba(0, 188, 212, 0.1);
  animation: pulse-speaking 1.5s ease-in-out infinite;
}

.talking-card.speaking.player {
  border-color: var(--player-blue);
  box-shadow: 0 0 25px rgba(0, 188, 212, 0.6);
}

.talking-card.speaking.dm {
  border-color: var(--dm-purple);
  box-shadow: 0 0 25px rgba(156, 39, 176, 0.6);
}

.talking-card.speaking.npc {
  border-color: var(--npc-gold);
  box-shadow: 0 0 25px rgba(255, 215, 0, 0.6);
}

@keyframes pulse-speaking {
  0%, 100% {
    box-shadow: 0 0 25px currentColor;
  }
  50% {
    box-shadow: 0 0 40px currentColor;
  }
}

.talking-card-portrait {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 2px solid var(--gold-frost);
  object-fit: cover;
  box-shadow: 0 0 10px rgba(212, 175, 55, 0.4);
}

.talking-card-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.talking-card-name {
  font-family: var(--font-serif);
  font-size: 14px;
  font-weight: 600;
  color: var(--gold-frost);
}

.talking-card-status {
  font-family: var(--font-sans);
  font-size: 11px;
  color: rgba(255, 255, 255, 0.6);
  font-style: italic;
}

.waveform-indicator {
  position: absolute;
  top: -20px;
  left: 50%;
  transform: translateX(-50%);
  width: 80px;
  height: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2px;
}

.waveform-bar {
  width: 3px;
  background: currentColor;
  border-radius: 1.5px;
  animation: waveform-pulse 0.6s ease-in-out infinite;
  animation-delay: calc(var(--i) * 0.1s);
}

@keyframes waveform-pulse {
  0%, 100% {
    height: 4px;
    opacity: 0.4;
  }
  50% {
    height: 12px;
    opacity: 1;
  }
}
```

**Características**:
- Cards horizontais com retrato
- Um card pulsa brilho quando a entidade está falando
- Para a IA mestre, o card pode exibir "DM speaking…"
- Waveform pequeno acima do card ativo
- Cores diferentes por tipo:
  - **Player**: Azul (#00BCD4)
  - **DM**: Roxo (#9C27B0)
  - **NPC**: Dourado (#FFD700) ou Verde (#4CAF50)

### 4. Card de Personagem Flutuante (HUD Pop-up) — Foundry Style

**Dimensões**: Variável (flutuante, posicionado dinamicamente)  
**Função**: HUD pop-up com avatar + stats principais  
**Estilo**: Card vertical flutuante estilo Foundry

> 💡 **Filosofia**: UX 10/10. Evita abrir ficha completa, exibe status só do personagem relevante, permite roleplay instantâneo. O card "fala": ele é personagem → jogador. Essa UI reduz atrito — você não precisa vasculhar sistemas, você "conversa" com o card.

```css
.character-card-popup {
  position: absolute;
  min-width: 200px;
  max-width: 280px;
  /* GLASSMORPHISM: Card flutuante → vidro médio */
  background: rgba(255, 255, 255, 0.10);
  backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  border: 1.5px solid var(--border-glass);
  border-radius: var(--radius-md);
  box-shadow: 
    0 8px 20px var(--shadow-medium),
    inset 0 0 30px rgba(212, 175, 55, 0.1);
  padding: var(--spacing-md);
  z-index: 2000;
  pointer-events: auto;
  animation: card-appear 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes card-appear {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.character-card-portrait {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  border: 2px solid var(--border-glass-strong);
  object-fit: cover;
  margin: 0 auto var(--spacing-md);
  box-shadow: 
    0 0 30px rgba(212, 175, 55, 0.6),
    inset 0 0 20px rgba(212, 175, 55, 0.2);
  /* Retrato sempre sólido */
}

.character-card-name {
  font-family: var(--font-display);
  font-size: 20px;
  font-weight: 700;
  color: var(--text-light); /* SEMPRE sólido */
  text-align: center;
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
  margin-bottom: var(--spacing-md);
}

.character-card-stats {
  display: flex;
  justify-content: center;
  gap: 16px;
  margin-bottom: 12px;
}

.character-card-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.character-card-stat-icon {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  border: 2px solid var(--gold-frost);
  box-shadow: 0 0 10px rgba(212, 175, 55, 0.4);
}

.character-card-stat-value {
  font-family: var(--font-display);
  font-size: 24px;
  font-weight: 700;
  color: var(--gold-frost);
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
}

.character-card-actions {
  display: flex;
  gap: 8px;
  margin-top: 12px;
}

.character-card-action-button {
  flex: 1;
  padding: 8px;
  background: rgba(61, 40, 23, 0.6);
  border: 2px solid rgba(212, 175, 55, 0.3);
  border-radius: 6px;
  color: var(--gold-frost);
  font-family: var(--font-sans);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.character-card-action-button:hover {
  border-color: var(--gold-frost);
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
  background: rgba(212, 175, 55, 0.1);
}

.character-card-play-button {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: radial-gradient(circle, 
    rgba(212, 175, 55, 0.3) 0%, 
    rgba(15, 15, 15, 0.8) 100%);
  border: 3px solid var(--gold-frost);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  position: absolute;
  top: 16px;
  right: 16px;
}

.character-card-play-button:hover {
  transform: scale(1.1);
  box-shadow: 0 0 30px rgba(212, 175, 55, 0.8);
}
```

**Características**:
- **Aparece ao clicar em token**: Card flutuante surge próximo ao token
- **Portrait grande**: Avatar do personagem em destaque
- **Stats principais**: HP, recursos, condições (ícones coloridos)
- **Botão play**: Reproduz voz do personagem (se disponível)
- **Ações rápidas**: Botões contextuais (interagir, atacar, etc.)
- **Fechamento**: Clique fora ou ESC fecha o card
- **Posicionamento inteligente**: Evita bordas da tela

### 5. Rodapé — Party UI + Action Bar (BG3 Style)

**Dimensões**: 100% width × 120px height  
**Função**: Avatares do party + barra de ações rápidas  
**Estilo**: Híbrido BG3 + Divinity — AAF (Avatar Action First)

> 🎮 **Filosofia**: **AAF (Avatar Action First)** — A interação primária é via avatares. A Action Bar é o "teclado da ação" estilo Divinity / Baldur. É velocidade, não lore.

#### 5.1 Party UI (Avatares Circulares)

```css
.party-ui {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md) var(--spacing-lg);
  /* GLASSMORPHISM: HUD inferior → vidro médio */
  background: rgba(255, 255, 255, 0.10);
  backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  border-top: 1.5px solid var(--border-glass);
  box-shadow: 
    0 -4px 20px var(--shadow-medium),
    inset 0 0 20px rgba(0, 0, 0, 0.1);
}

.party-avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  border: 2px solid var(--border-glass-strong);
  object-fit: cover;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  box-shadow: 
    0 0 20px rgba(212, 175, 55, 0.4),
    inset 0 0 20px rgba(212, 175, 55, 0.1);
  /* Retrato sempre sólido */
}

/* Anéis coloridos (HP, status) → leitura instantânea */
.party-avatar::before {
  content: '';
  position: absolute;
  top: -4px;
  left: -4px;
  right: -4px;
  bottom: -4px;
  border-radius: 50%;
  border: 4px solid var(--health-green);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.party-avatar.hp-high::before {
  border-color: var(--health-green);
  opacity: 0.6;
}

.party-avatar.hp-medium::before {
  border-color: var(--stamina-yellow);
  opacity: 0.6;
}

.party-avatar.hp-low::before {
  border-color: var(--damage-red);
  opacity: 0.8;
}

/* 3 Estados: Selecionado, Em fala, Em combate/turn */
.party-avatar.selected {
  border-color: var(--arcane-blue);
  box-shadow: 
    0 0 30px rgba(74, 144, 226, 0.8),
    inset 0 0 30px rgba(74, 144, 226, 0.2);
  transform: scale(1.1);
}

.party-avatar.speaking {
  border-color: var(--player-blue);
  box-shadow: 
    0 0 30px rgba(0, 188, 212, 0.8),
    inset 0 0 30px rgba(0, 188, 212, 0.2);
  animation: pulse-speaking 1.5s ease-in-out infinite;
}

.party-avatar.in-combat {
  border-color: var(--damage-red);
  box-shadow: 
    0 0 30px rgba(244, 67, 54, 0.8),
    inset 0 0 30px rgba(244, 67, 54, 0.2);
}

.party-avatar.active-turn {
  border-color: var(--gold-frost);
  box-shadow: 
    0 0 40px rgba(212, 175, 55, 1),
    inset 0 0 40px rgba(212, 175, 55, 0.3);
  animation: pulse-turn 1s ease-in-out infinite;
}

@keyframes pulse-speaking {
  0%, 100% {
    box-shadow: 
      0 0 30px rgba(0, 188, 212, 0.8),
      inset 0 0 30px rgba(0, 188, 212, 0.2);
  }
  50% {
    box-shadow: 
      0 0 50px rgba(0, 188, 212, 1),
      inset 0 0 50px rgba(0, 188, 212, 0.3);
  }
}

@keyframes pulse-turn {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

.party-avatar:hover {
  transform: scale(1.15) translateY(-4px);
  box-shadow: 
    0 0 40px rgba(212, 175, 55, 0.8),
    inset 0 0 40px rgba(212, 175, 55, 0.2);
}
```

**Características**:
- **Retratos grandes**: 80px × 80px, borda dourada
- **Anéis coloridos**: HP (verde/amarelo/vermelho), status (azul/roxo/dourado)
- **Leitura instantânea**: Cores comunicam estado sem texto
- **3 Estados principais**:
  - **Selecionado**: Borda azul, scale 1.1
  - **Em fala**: Borda azul, pulse animado
  - **Em combate/turn**: Borda vermelha/dourada, pulse mais intenso
- **Clicável**: Abre ficha completa ou card flutuante
- **AAF (Avatar Action First)**: Interação primária via avatares

#### 5.2 Action Bar (Slots Numerados)

**Dimensões**: Continuação do rodapé, à direita dos avatares  
**Função**: Slots numerados 1-10 (ações rápidas)  
**Estilo**: "Teclado da ação" estilo Divinity / Baldur

```css
.hotbar {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  /* GLASSMORPHISM: Action bar → vidro médio */
  background: rgba(255, 255, 255, 0.10);
  backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  border-top: 1.5px solid var(--border-glass);
  box-shadow: 
    0 -4px 20px var(--shadow-medium),
    inset 0 0 20px rgba(0, 0, 0, 0.1);
  position: relative;
}

.hotbar-section {
  display: flex;
  align-items: center;
  gap: 8px;
}

.hotbar-section-spacer {
  flex: 1;
}

.action-bar::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(212, 175, 55, 0.5) 50%, 
    transparent 100%);
}

.hotbar-slot {
  width: 64px;
  height: 64px;
  /* GLASSMORPHISM: Slots → vidro leve */
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(var(--blur-low)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-low)) saturate(var(--saturate-normal));
  border: 1.2px solid var(--border-glass-weak);
  border-radius: var(--radius-sm);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

/* ===== MICROFEEDBACK APPLE ===== */
.hotbar-slot:hover {
  /* Hover gera specular highlight suave */
  background: rgba(255, 255, 255, 0.12);
  border-color: var(--border-glass);
  box-shadow: 
    0 0 15px rgba(212, 175, 55, 0.4),
    inset 0 0 15px rgba(255, 255, 255, 0.05);
  transform: translateY(-2px);
}

.hotbar-slot:active {
  /* Botão pressionado → "glass pressure" */
  transform: translateY(0) scale(0.98);
  box-shadow: 
    0 0 10px rgba(212, 175, 55, 0.3),
    inset 0 0 20px rgba(0, 0, 0, 0.1);
}

.hotbar-slot.active {
  /* Ativo → borda quente + aumento de brilho interno */
  background: rgba(255, 255, 255, 0.14);
  border-color: var(--border-glass-strong);
  box-shadow: 
    0 0 25px rgba(74, 144, 226, 0.8),
    inset 0 0 20px rgba(74, 144, 226, 0.2);
}

.hotbar-slot-number {
  position: absolute;
  top: 2px;
  left: 4px;
  font-family: var(--font-mono);
  font-size: 10px;
  color: rgba(212, 175, 55, 0.6);
  font-weight: 600;
}

.action-slot::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(212, 175, 55, 0.1) 0%, 
    transparent 100%);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.action-slot:hover {
  transform: translateY(-4px);
  box-shadow: 
    0 0 20px rgba(212, 175, 55, 0.6),
    inset 0 0 20px rgba(212, 175, 55, 0.1);
}

.action-slot:hover::before {
  opacity: 1;
}

.action-slot.active {
  border-color: var(--arcane-blue);
  box-shadow: 
    0 0 25px rgba(74, 144, 226, 0.8),
    inset 0 0 20px rgba(74, 144, 226, 0.2);
}

.action-slot.cooldown {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-icon {
  width: 40px;
  height: 40px;
  object-fit: contain;
  filter: drop-shadow(0 0 5px rgba(212, 175, 55, 0.5));
}

.action-tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  margin-bottom: 8px;
  padding: 8px 12px;
  background: rgba(0, 0, 0, 0.95);
  border: 1px solid var(--border-gold);
  border-radius: 6px;
  color: var(--gold-frost);
  font-family: var(--font-sans);
  font-size: 12px;
  white-space: nowrap;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.3s ease;
  z-index: 1000;
}

.action-slot:hover .action-tooltip {
  opacity: 1;
}

/* Botão TALK integrado */
.talk-button {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: radial-gradient(circle, 
    rgba(212, 175, 55, 0.3) 0%, 
    rgba(15, 15, 15, 0.8) 100%);
  border: 3px solid var(--gold-frost);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  margin-right: 16px;
}

.talk-button::before {
  content: '●';
  font-size: 24px;
  color: var(--gold-frost);
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.8);
}

.talk-button:hover {
  transform: scale(1.1);
  box-shadow: 0 0 30px rgba(212, 175, 55, 0.8);
}

.talk-button.active {
  background: radial-gradient(circle, 
    rgba(0, 188, 212, 0.4) 0%, 
    rgba(15, 15, 15, 0.8) 100%);
  border-color: var(--player-blue);
  box-shadow: 0 0 40px rgba(0, 188, 212, 0.8);
  animation: pulse-talking 1s ease-in-out infinite;
}

@keyframes pulse-talking {
  0%, 100% {
    box-shadow: 0 0 40px rgba(0, 188, 212, 0.8);
  }
  50% {
    box-shadow: 0 0 60px rgba(0, 188, 212, 1);
  }
}

/* Indicadores de status */
.voice-status {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto;
  padding: 8px 16px;
  background: rgba(15, 15, 15, 0.8);
  border: 1px solid rgba(212, 175, 55, 0.3);
  border-radius: 6px;
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--gold-frost);
}

.latency-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
}

.latency-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--health-green);
  box-shadow: 0 0 5px var(--health-green);
}

.latency-dot.warning {
  background: var(--stamina-yellow);
  box-shadow: 0 0 5px var(--stamina-yellow);
}

.latency-dot.error {
  background: var(--damage-red);
  box-shadow: 0 0 5px var(--damage-red);
}
```

**Conteúdo da Action Bar**:
- **Botão TALK** (entre avatares e slots): Botão redondo que acende quando o jogador fala
- **Slots 1-10** (centro-direita): Ações rápidas
  - Itens, Skills, Macros rápidas
  - Abre ferramentas, rolagens, fichas
  - Cada slot mostra número pequeno no canto superior esquerdo
  - Ícone da ação/ítem no centro
  - Highlight do slot ativo (número do jogador abaixo, ex: "Victor" abaixo do slot 8)
- **Seção de Players** (canto direito): Lista de jogadores online com indicadores coloridos
- **Indicadores de Status** (canto direito): Latência (ms), FPS, status de sessão

**Características**:
- **"Teclado da ação"**: Acesso rápido via teclas 1-0
- **Arrastar e soltar**: Itens/habilidades podem ser arrastados para slots
- **Feedback visual**: Hover, ativo, cooldown, indisponível
- **Estilo BG3**: Ícones grandes, limpos, bordas douradas/metálicas

**Características**:
- **Slots numerados**: Teclas 1-9, 0 ativam slots diretamente
- **Número do slot**: Pequeno número no canto superior esquerdo de cada slot
- **Nome do jogador**: Aparece abaixo do slot ativo (ex: "Victor" abaixo do slot 8)
- **Arrastar e soltar**: Itens/habilidades podem ser arrastados para slots
- **Tooltips**: Nome e descrição ao hover
- **Feedback visual**: Hover, ativo, cooldown, indisponível
- **Estilo BG3**: Ícones grandes, limpos, bordas douradas/metálicas
- **Waveform animada**: Ao lado do botão TALK quando ativo
- **Posicionamento**: Centralizado na parte inferior, acima da barra de status

### 6. Emblemas / Status de Jogador (Telemetria Social)

**Dimensões**: Variável (canto inferior esquerdo)  
**Função**: Telemetria social — players online, latência, FPS

> 📊 **Filosofia**: Sabe quem está presente, flow multiplayer, debug rápido. No VRPG híbrido (IA + players): Servidor do Mestre IA, Clients leves. Esse painel ajuda em co-op.

```css
.player-status-panel {
  position: fixed;
  bottom: 140px;
  left: 16px;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(8px);
  border: 2px solid rgba(212, 175, 55, 0.3);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  z-index: 1000;
  min-width: 200px;
}

.player-status-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background 0.3s ease;
}

.player-status-item:hover {
  background: rgba(212, 175, 55, 0.1);
}

.player-status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  box-shadow: 0 0 5px currentColor;
}

.player-status-dot.online {
  background: var(--health-green);
  color: var(--health-green);
}

.player-status-dot.away {
  background: var(--stamina-yellow);
  color: var(--stamina-yellow);
}

.player-status-dot.offline {
  background: rgba(255, 255, 255, 0.3);
  color: rgba(255, 255, 255, 0.3);
}

.player-status-name {
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--gold-frost);
  flex: 1;
}

.player-status-role {
  font-family: var(--font-sans);
  font-size: 10px;
  color: rgba(255, 255, 255, 0.6);
  font-style: italic;
}

.telemetry-info {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid rgba(212, 175, 55, 0.3);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.telemetry-item {
  display: flex;
  justify-content: space-between;
  font-family: var(--font-mono);
  font-size: 10px;
  color: rgba(255, 255, 255, 0.7);
}

.telemetry-label {
  color: rgba(255, 255, 255, 0.5);
}

.telemetry-value {
  color: var(--gold-frost);
  font-weight: 600;
}

.telemetry-value.good {
  color: var(--health-green);
}

.telemetry-value.warning {
  color: var(--stamina-yellow);
}

.telemetry-value.error {
  color: var(--damage-red);
}
```

**Conteúdo**:
- **Lista de Players**: 
  - Nome do jogador
  - Role entre parênteses (ex: "[GM]", "[Haruko "Mansur"]")
  - Dot colorido indicando status:
    - 🟢 Verde: Online/Ativo
    - 🟡 Amarelo: Away/Ausente
    - 🔵 Azul: Conectado mas inativo
    - 🟣 Roxo: Outro status
    - ⚫ Cinza: Offline
- **Latência**: Tempo de resposta do servidor (ex: "Latency 12ms")
- **FPS**: Taxa de quadros atual (ex: "FPS 60" ou "FPS 61")
- **Formato**: "Latency XXms FPS XX" em uma linha

**Posicionamento**: Canto inferior esquerdo, acima da hotbar

**Características**:
- **Sempre visível**: Painel fixo no canto inferior esquerdo
- **Cores de status**: Verde (bom), Amarelo (atenção), Vermelho (problema)
- **Atualização em tempo real**: Telemetria atualiza constantemente
- **Formato compacto**: Lista vertical de players com dots coloridos
- **Informação essencial**: Apenas nome, role, e status visual

### 5. Painel Lateral Esquerdo — Toolbelt do Mestre (Foundry Style)

**Dimensões**: 60px width (expandido: 200px) × calc(100vh - 200px) height  
**Função**: Ferramentas do Mestre IA (toolbelt, não narrativa)  
**Estilo**: Ícones verticais estilo Foundry

> ⚙️ **Filosofia**: É utilitário, não narrativo. É o painel onde o GM "opera".  
> No VRPG: Mestre IA manipula invisivelmente, Jogador humano usa UI simplificada.

```css
.toolbelt-sidebar {
  width: 60px; /* Sempre compacto por padrão */
  height: calc(100vh - 200px);
  background: linear-gradient(180deg, 
    rgba(15, 15, 15, 0.98) 0%, 
    rgba(0, 0, 0, 1) 100%);
  border-right: 2px solid var(--border-gold);
  box-shadow: 4px 0 20px rgba(0, 0, 0, 0.8);
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px 8px;
  position: relative;
  /* Sempre visível, nunca completamente escondido */
}

.toolbelt-sidebar.expanded {
  width: 200px;
}

/* Tooltip que aparece ao hover */
.toolbelt-tooltip {
  position: absolute;
  left: 70px;
  padding: 8px 12px;
  background: rgba(0, 0, 0, 0.95);
  border: 1px solid var(--border-gold);
  border-radius: 6px;
  color: var(--gold-frost);
  font-family: var(--font-sans);
  font-size: 12px;
  white-space: nowrap;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.3s ease;
  z-index: 1000;
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
}

.toolbelt-button:hover + .toolbelt-tooltip {
  opacity: 1;
}

.toolbelt-button {
  width: 44px;
  height: 44px;
  background: linear-gradient(135deg, 
    rgba(61, 40, 23, 0.6) 0%, 
    rgba(15, 15, 15, 0.8) 100%);
  border: 2px solid rgba(212, 175, 55, 0.3);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  margin: 0 auto;
}

.toolbelt-sidebar.expanded .toolbelt-button {
  width: 100%;
  height: 56px;
  padding: 0 12px;
  justify-content: flex-start;
  gap: 12px;
}

.menu-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(212, 175, 55, 0.2) 50%, 
    transparent 100%);
  transition: left 0.5s ease;
}

.menu-button:hover::before {
  left: 100%;
}

.menu-button:hover {
  border-color: var(--gold-frost);
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
  transform: translateX(4px);
}

.menu-button.active {
  background: linear-gradient(135deg, 
    rgba(212, 175, 55, 0.2) 0%, 
    rgba(74, 144, 226, 0.2) 100%);
  border-color: var(--arcane-blue);
  box-shadow: 
    0 0 20px rgba(74, 144, 226, 0.4),
    inset 0 0 20px rgba(212, 175, 55, 0.1);
}

.menu-icon {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
  filter: drop-shadow(0 0 5px rgba(212, 175, 55, 0.5));
}

.menu-label {
  font-family: var(--font-serif);
  font-size: 14px;
  font-weight: 600;
  color: var(--gold-frost);
  text-shadow: 0 0 5px rgba(212, 175, 55, 0.5);
  white-space: nowrap;
}

.menu-sidebar.collapsed .menu-label {
  display: none;
}

/* Painéis laterais que abrem */
.side-panel {
  position: fixed;
  left: 200px;
  top: 80px;
  width: 400px;
  height: calc(100vh - 200px);
  background: linear-gradient(180deg, 
    rgba(15, 15, 15, 0.98) 0%, 
    rgba(0, 0, 0, 1) 100%);
  border: 2px solid var(--border-gold);
  border-left: none;
  border-radius: 0 8px 8px 0;
  box-shadow: 4px 0 30px rgba(0, 0, 0, 0.8);
  padding: 24px;
  transform: translateX(-100%);
  transition: transform 0.3s ease;
  z-index: 100;
  overflow-y: auto;
}

.side-panel.open {
  transform: translateX(0);
}

.side-panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid var(--border-gold);
}

.side-panel-title {
  font-family: var(--font-display);
  font-size: 24px;
  font-weight: 700;
  color: var(--gold-frost);
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
}

.side-panel-close {
  width: 32px;
  height: 32px;
  border: 2px solid var(--border-gold);
  border-radius: 50%;
  background: transparent;
  color: var(--gold-frost);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.side-panel-close:hover {
  background: rgba(212, 175, 55, 0.2);
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
}
```

**Ícones do Toolbelt** (estilo Foundry, vertical, de cima para baixo):
- **👤 User/Players**: Gerenciamento de jogadores e permissões
- **⛶ Full Screen**: Alternar tela cheia
- **🗺️ Scenes**: Alternar entre cenas/mapas (com preview do mapa atual)
- **⚙️ Settings**: Configurações gerais
- **🎯 Target**: Ferramenta de targeting
- **💬 Chat**: Toggle do painel de chat
- **⚔️ Combat**: Rastreador de combate (iniciar/pausar)
- **📖 Journal**: Entradas de diário e notas
- **📚 Compendium**: Packs de conteúdo (spells, itens, bestiário)
- **🎲 Roll Tables**: Tabelas de rolagem
- **🎵 Playlists**: Música e sons ambiente
- **📁 File Browser**: Navegador de arquivos
- **⚙️ Configuration**: Configurações avançadas
- **🧩 Modules**: Módulos e extensões
- **▶️ Play**: Controles de reprodução (música, animações)

**Características**:
- **Sempre visível**: Toolbelt nunca desaparece completamente
- **Compacto por padrão**: Apenas ícones, expande ao hover ou clique
- **Tooltips**: Nome da ferramenta aparece ao hover
- **Ativo destacado**: Ícone ativo fica iluminado
- **Mestre IA usa invisivelmente**: Jogador vê apenas resultados visuais

### 6. Painel Lateral Direito — Sistema / Ficha / Chat (Foundry Style)

**Dimensões**: 300px width (recolhido: 0px) × calc(100vh - 200px) height  
**Função**: Painel cognitivo — densidade de informação  
**Estilo**: Abas verticais estilo Foundry

> 📖 **Filosofia**: É o painel cognitivo. Explica o jogo, expõe regras, mostra recursos persistentes, permite referências rápidas.  
> No VRPG: O mestre IA atualiza esse painel como se fosse um assistente. O jogador consulta passivamente. Todo o "blablabla técnico" fica aqui — não na voz.

```css
.system-panel {
  width: 300px;
  height: calc(100vh - 200px);
  /* GLASSMORPHISM: Painel lateral → vidro médio */
  background: rgba(255, 255, 255, 0.10);
  backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  border-left: 1.5px solid var(--border-glass);
  box-shadow: 
    -4px 0 30px var(--shadow-medium),
    inset 0 0 20px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}

/* Abas verticais estilo Foundry */
.system-panel-tabs {
  display: flex;
  flex-direction: column;
  width: 60px;
  background: rgba(0, 0, 0, 0.8);
  border-right: 1px solid rgba(212, 175, 55, 0.3);
  padding: 8px 4px;
  gap: 4px;
}

.system-panel-tab {
  width: 52px;
  height: 52px;
  /* GLASSMORPHISM: Tabs → vidro leve */
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(var(--blur-low)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-low)) saturate(var(--saturate-normal));
  border: 1.2px solid var(--border-glass-weak);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
}

.system-panel-tab:hover {
  border-color: var(--gold-frost);
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
}

.system-panel-tab.active {
  /* Tab ativa → mais sólida */
  background: rgba(255, 255, 255, 0.14);
  border-color: var(--border-glass-strong);
  box-shadow: 
    0 0 20px rgba(212, 175, 55, 0.6),
    inset 0 0 20px rgba(212, 175, 55, 0.1);
}

/* ===== MICROFEEDBACK APPLE ===== */
.system-panel-tab:hover {
  /* Hover gera specular highlight suave */
  background: rgba(255, 255, 255, 0.12);
  border-color: var(--border-glass);
  box-shadow: 
    0 0 15px rgba(212, 175, 55, 0.4),
    inset 0 0 15px rgba(255, 255, 255, 0.05);
  transform: translateY(-1px);
}

.system-panel-tab:active {
  /* Botão pressionado → "glass pressure" */
  transform: translateY(0) scale(0.98);
  box-shadow: 
    0 0 10px rgba(212, 175, 55, 0.3),
    inset 0 0 20px rgba(0, 0, 0, 0.1);
}

.system-panel-content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  display: none;
}

.system-panel-content.active {
  display: block;
}

.history-panel.collapsed {
  width: 0;
  padding: 0;
  border: none;
  overflow: hidden;
}

.history-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-section-title {
  font-family: var(--font-serif);
  font-size: 18px;
  font-weight: 600;
  color: var(--gold-frost);
  text-shadow: 0 0 5px rgba(212, 175, 55, 0.5);
  padding-bottom: 8px;
  border-bottom: 1px solid rgba(212, 175, 55, 0.3);
}

/* Histórico de Áudio (WhatsApp in-game style) */
.audio-history-item {
  background: linear-gradient(135deg, 
    rgba(61, 40, 23, 0.6) 0%, 
    rgba(15, 15, 15, 0.8) 100%);
  border: 1px solid rgba(212, 175, 55, 0.3);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.audio-history-item:hover {
  border-color: var(--gold-frost);
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
  transform: translateX(-4px);
}

.audio-history-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.audio-history-portrait {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 1.2px solid var(--border-glass);
  object-fit: cover;
  /* Retrato sempre sólido */
}

.audio-history-name {
  font-family: var(--font-serif);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-light); /* SEMPRE sólido */
}

.audio-history-player {
  width: 100%;
  height: 40px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(212, 175, 55, 0.3);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.audio-history-player:hover {
  border-color: var(--gold-frost);
  box-shadow: 0 0 10px rgba(212, 175, 55, 0.3);
}

.audio-history-play-button {
  width: 24px;
  height: 24px;
  border: 2px solid var(--gold-frost);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--gold-frost);
  font-size: 10px;
}

/* ===== GLASSMORPHISM: Histórico de Rolagens ===== */
.roll-history-item {
  /* GLASSMORPHISM: Roll items → vidro leve */
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  -webkit-backdrop-filter: blur(var(--blur-md)) saturate(var(--saturate-normal));
  border: 1.2px solid var(--border-glass-weak);
  border-radius: var(--radius-sm);
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.roll-history-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.roll-history-name {
  font-family: var(--font-serif);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-light); /* SEMPRE sólido */
}

.roll-history-result {
  font-family: var(--font-display);
  font-size: 20px;
  font-weight: 700;
  color: var(--arcane-blue); /* SEMPRE sólido */
  text-shadow: 0 0 10px rgba(74, 144, 226, 0.5);
}

.roll-history-details {
  font-family: var(--font-sans);
  font-size: 11px;
  color: rgba(255, 255, 255, 0.6);
}

.roll-history-item.critical {
  border-color: var(--health-green);
  box-shadow: 0 0 15px rgba(76, 175, 80, 0.4);
}

.roll-history-item.critical .roll-history-result {
  color: var(--health-green);
  text-shadow: 0 0 10px rgba(76, 175, 80, 0.5);
}

.roll-history-item.failure {
  border-color: var(--damage-red);
  box-shadow: 0 0 15px rgba(244, 67, 54, 0.4);
}

.roll-history-item.failure .roll-history-result {
  color: var(--damage-red);
  text-shadow: 0 0 10px rgba(244, 67, 54, 0.5);
}

/* Histórico Visual (Imagens) */
.image-history-item {
  width: 100%;
  aspect-ratio: 16 / 9;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(212, 175, 55, 0.3);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
}

.image-history-item:hover {
  border-color: var(--gold-frost);
  box-shadow: 0 0 15px rgba(212, 175, 55, 0.4);
  transform: scale(1.05);
}

.image-history-thumbnail {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.image-history-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(180deg, 
    transparent 0%, 
    rgba(0, 0, 0, 0.8) 100%);
  padding: 8px;
  font-family: var(--font-sans);
  font-size: 11px;
  color: var(--gold-frost);
}
```

**Abas do Painel Sistema** (estilo Foundry, vertical):
- **💬 Chat** (ativa): Log de conversas, narrativas, eventos, cards de habilidades
- **👤 Actors**: Fichas de personagens e NPCs
- **📖 Journal**: Entradas de diário, notas, lore
- **🎲 Roll Tables**: Tabelas de rolagem customizadas
- **📁 File Browser**: Navegador de arquivos e assets
- **⚙️ Settings**: Configurações do jogo
- **</> API**: Console de API e scripts

**Modos do Painel**:
- **Modo Mestre**: Logs + controle de contexto
- **Modo Jogador**: Ficha + ações + status

**Conteúdo por Aba** (Cards Empilhados Estilo Statblock):
- **Chat**: 
  - Cards de habilidades/spells (estilo statblock do Foundry)
    - Nome da habilidade em destaque
    - Tipo e custo (ex: "PERMANENT", "SUPPLEMENTAL | 3M")
    - Duração, keywords, excelência
    - Descrição completa formatada
    - Timestamp relativo (ex: "25m 3s ago")
  - Histórico de áudio (WhatsApp in-game style)
  - Rolagens com resultados destacados
  - Narrativas do mestre IA
  - Input de mensagem na parte inferior
  - Botão de pin para fixar mensagens importantes
- **Actors**: Ficha completa, stats, habilidades, recursos
- **Journal**: Entradas de lore, eventos, notas do mestre IA
- **Roll Tables**: Tabelas de rolagem customizadas (tesouros, encontros, etc.)
- **File Browser**: Navegador de assets (imagens, áudio, documentos)
- **Settings**: Configurações do jogo, módulos, preferências
- **API**: Console para desenvolvedores (opcional, avançado)

**Cards Empilhados**:
- Área deslizável vertical
- Cada card é um statblock completo
- Scroll suave, cards grandes e legíveis
- Visual idêntico ao Foundry: estilo statblock profissional

**Características**:
- **Abas verticais**: Ícones na esquerda do painel, conteúdo na direita
- **Aba ativa**: Background branco/claro no ícone da aba ativa
- **Recolhível**: Pode ser minimizado completamente (width: 0)
- **Scroll suave**: Conteúdo longo com scroll elegante
- **Atualização automática**: Mestre IA atualiza painéis em tempo real
- **Densidade de informação**: Todo o "técnico" fica aqui, não na voz
- **Chat Cards**: Habilidades/spells aparecem como cards no chat (estilo Foundry)
  - Título da habilidade em destaque
  - Tipo e custo (ex: "PERMANENT", "SUPPLEMENTAL | 3M")
  - Duração, keywords, excelência
  - Descrição completa formatada
  - Timestamp relativo (ex: "25m 3s ago")

### 7. Rolagem de Dados no Centro da Tela (BG3 Style)

**Dimensões**: Variável (centro da tela)  
**Função**: Animação de rolagem de dados quando o mestre IA solicitar

```css
.dice-roll-overlay {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 10000;
  pointer-events: none;
}

.dice-container {
  position: relative;
  width: 200px;
  height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.dice-3d {
  width: 120px;
  height: 120px;
  position: relative;
  transform-style: preserve-3d;
  animation: dice-roll 2s ease-out;
}

@keyframes dice-roll {
  0% {
    transform: rotateX(0deg) rotateY(0deg) rotateZ(0deg);
  }
  25% {
    transform: rotateX(360deg) rotateY(180deg) rotateZ(90deg);
  }
  50% {
    transform: rotateX(720deg) rotateY(360deg) rotateZ(180deg);
  }
  75% {
    transform: rotateX(1080deg) rotateY(540deg) rotateZ(270deg);
  }
  100% {
    transform: rotateX(1440deg) rotateY(720deg) rotateZ(360deg);
  }
}

.dice-face {
  position: absolute;
  width: 120px;
  height: 120px;
  background: linear-gradient(135deg, 
    rgba(212, 175, 55, 0.9) 0%, 
    rgba(61, 40, 23, 0.9) 100%);
  border: 3px solid var(--gold-frost);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-display);
  font-size: 48px;
  font-weight: 700;
  color: var(--gold-frost);
  text-shadow: 0 0 20px rgba(212, 175, 55, 0.8);
  box-shadow: 
    0 0 30px rgba(212, 175, 55, 0.6),
    inset 0 0 30px rgba(212, 175, 55, 0.2);
}

.dice-result {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-family: var(--font-display);
  font-size: 64px;
  font-weight: 700;
  color: var(--gold-frost);
  text-shadow: 
    0 0 30px rgba(212, 175, 55, 1),
    0 0 60px rgba(212, 175, 55, 0.8);
  animation: result-appear 0.5s ease-out 2s;
  opacity: 0;
  z-index: 10;
}

@keyframes result-appear {
  from {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.5);
  }
  to {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
}

.dice-result.critical {
  color: var(--health-green);
  text-shadow: 
    0 0 30px rgba(76, 175, 80, 1),
    0 0 60px rgba(76, 175, 80, 0.8);
  animation: critical-pulse 1s ease-in-out infinite 2.5s;
}

.dice-result.failure {
  color: var(--damage-red);
  text-shadow: 
    0 0 30px rgba(244, 67, 54, 1),
    0 0 60px rgba(244, 67, 54, 0.8);
}

@keyframes critical-pulse {
  0%, 100% {
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    transform: translate(-50%, -50%) scale(1.2);
  }
}

.dice-roll-info {
  position: absolute;
  bottom: -60px;
  left: 50%;
  transform: translateX(-50%);
  font-family: var(--font-serif);
  font-size: 18px;
  color: var(--gold-frost);
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
  text-align: center;
  white-space: nowrap;
}

.dice-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  z-index: 9999;
  animation: backdrop-fade-in 0.3s ease-out;
}

@keyframes backdrop-fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
```

**Características**:
- Surge um dado 3D/2D estilizado no centro da tela
- Ele rola de maneira animada (rotação 3D)
- Resultado aparece com brilho e ícone (crítico, falha, normal)
- Som de rolagem suave (via Web Audio API)
- Backdrop escurecido durante a animação
- Essa animação é crucial para imersão

### 8. Indicador de Quem Está Falando (Essencial)

**Função**: Mostrar visualmente quem está falando em qualquer momento  
**Modelos Possíveis**: 3 estilos diferentes podem ser implementados

```css
.speaking-indicator {
  position: fixed;
  top: 100px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 5000;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 24px;
  background: linear-gradient(135deg, 
    rgba(15, 15, 15, 0.95) 0%, 
    rgba(0, 0, 0, 0.98) 100%);
  border: 2px solid var(--border-gold);
  border-radius: 12px;
  box-shadow: 
    0 0 30px rgba(212, 175, 55, 0.6),
    inset 0 0 20px rgba(212, 175, 55, 0.1);
  animation: indicator-appear 0.3s ease-out;
}

@keyframes indicator-appear {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}

.speaking-indicator.player {
  border-color: var(--player-blue);
  box-shadow: 
    0 0 30px rgba(0, 188, 212, 0.6),
    inset 0 0 20px rgba(0, 188, 212, 0.1);
}

.speaking-indicator.dm {
  border-color: var(--dm-purple);
  box-shadow: 
    0 0 30px rgba(156, 39, 176, 0.6),
    inset 0 0 20px rgba(156, 39, 176, 0.1);
}

.speaking-indicator.npc {
  border-color: var(--npc-gold);
  box-shadow: 
    0 0 30px rgba(255, 215, 0, 0.6),
    inset 0 0 20px rgba(255, 215, 0, 0.1);
}

.speaking-portrait {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 2px solid currentColor;
  object-fit: cover;
  box-shadow: 0 0 15px currentColor;
  animation: portrait-pulse 1.5s ease-in-out infinite;
}

@keyframes portrait-pulse {
  0%, 100% {
    box-shadow: 0 0 15px currentColor;
  }
  50% {
    box-shadow: 0 0 25px currentColor;
  }
}

.speaking-name {
  font-family: var(--font-serif);
  font-size: 18px;
  font-weight: 600;
  color: currentColor;
  text-shadow: 0 0 10px currentColor;
}

.speaking-waveform {
  width: 120px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 3px;
}

.speaking-waveform-bar {
  width: 4px;
  background: currentColor;
  border-radius: 2px;
  animation: waveform-animate 0.8s ease-in-out infinite;
  animation-delay: calc(var(--i) * 0.1s);
  box-shadow: 0 0 5px currentColor;
}

@keyframes waveform-animate {
  0%, 100% {
    height: 4px;
    opacity: 0.4;
  }
  50% {
    height: 20px;
    opacity: 1;
  }
}
```

**Características**:
- Quando o jogador fala, seu retrato acende azul
- Quando o Mestre IA fala, card "DM" acende roxo
- Quando um NPC fala, card do NPC pisca em dourado/verde
- Waveform pequeno acima do indicador ativo
- Aparece no topo central da tela durante a fala
- Desaparece suavemente quando a fala termina

**3 Modelos de Implementação**:

**A. Baldur's Gate 3 Style (Cards)**:
- O retrato do personagem se anima
- Mostra boca/movimento
- Glow circular colorido

**B. Chat Overlay Style**:
- Pequeno bubble acima do avatar
- Fade-out automático

**C. Cinematic Ribbon**:
- Uma faixa no topo com "Shinta está falando"
- Retrato + nome + glow

## UX Importante

### Múltiplas Formas de Interação

Todas as ações podem ser feitas por:

1. **Voz**: Comando de voz via ASR
2. **Clique**: Interação direta com mouse/touch
3. **Atalhos**: Teclas de atalho (C, I, J, M, etc.)

### Atalhos de Teclado

```typescript
const KEYBOARD_SHORTCUTS = {
  'KeyI': 'inventory',      // Inventário
  'KeyC': 'character',      // Ficha
  'KeyJ': 'journal',        // Diário
  'KeyM': 'map',            // Mapa
  'KeyP': 'party',          // Grupo
  'KeyS': 'settings',       // Configurações
  'KeyR': 'rules',          // Regras
  'Space': 'talk',          // Falar (push-to-talk)
  'Enter': 'confirm',       // Confirmar ação
  'Escape': 'close',        // Fechar painéis
  'Tab': 'next-target',    // Próximo alvo
};
```

### Responsividade Widescreen

```css
/* 21:9 Ultrawide */
@media (min-width: 2560px) and (aspect-ratio: 21/9) {
  .battlemap-container {
    max-width: 1800px;
    margin: 0 auto;
  }
  
  .action-bar {
    max-width: 1800px;
    margin: 0 auto;
  }
}

/* 16:9 Standard */
@media (min-width: 1920px) and (aspect-ratio: 16/9) {
  .battlemap-container {
    max-width: 1600px;
    margin: 0 auto;
  }
}

/* 4K */
@media (min-width: 3840px) {
  .battlemap-container {
    max-width: 2400px;
    margin: 0 auto;
  }
  
  /* Escalar fontes proporcionalmente */
  :root {
    font-size: 1.25rem;
  }
}
```

## Animações e Transições

### Transições Suaves

```css
.smooth-transition {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.fade-in {
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.slide-in-left {
  animation: slideInLeft 0.4s ease-out;
}

@keyframes slideInLeft {
  from {
    opacity: 0;
    transform: translateX(-30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
```

### Partículas e Efeitos

```css
.particle-effect {
  position: absolute;
  width: 100%;
  height: 100%;
  pointer-events: none;
  overflow: hidden;
}

.particle {
  position: absolute;
  width: 2px;
  height: 2px;
  background: var(--gold-frost);
  border-radius: 50%;
  box-shadow: 0 0 5px var(--gold-frost);
  animation: particle-float 3s ease-in-out infinite;
}

@keyframes particle-float {
  0% {
    transform: translateY(0) translateX(0);
    opacity: 0;
  }
  50% {
    opacity: 1;
  }
  100% {
    transform: translateY(-100px) translateX(50px);
    opacity: 0;
  }
}
```

## Acessibilidade

### Contraste e Legibilidade

- Contraste mínimo 4.5:1 para texto normal
- Contraste mínimo 3:1 para texto grande
- Suporte a modo de alto contraste
- Escalabilidade de fonte até 200%

### Navegação por Teclado

```css
.keyboard-nav:focus-visible {
  outline: 2px solid var(--gold-frost);
  outline-offset: 2px;
  box-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
}

.skip-link {
  position: absolute;
  top: -40px;
  left: 6px;
  background: var(--gold-frost);
  color: #000;
  padding: 8px;
  text-decoration: none;
  border-radius: 4px;
  transition: top 0.3s;
  font-weight: 600;
}

.skip-link:focus {
  top: 6px;
}
```

### Screen Reader Support

- Labels apropriados em todos os elementos interativos
- Landmarks semânticos (nav, main, aside)
- Estados dinâmicos anunciados via aria-live
- Descrições de elementos visuais via aria-describedby

## Adaptação ao VRPG com IA

### 🤖 O Mestre IA Toma o Papel do GM

O Mestre IA não usa as ferramentas visualmente — ele opera via INTENTS:

- **Canvas se atualiza sozinho**: NPCs aparecem → tokens surgem
- **Ações automáticas**: Você anuncia ação → grid revela alcance
- **Narrativa visual**: Mestre descreve → art pipeline cria imagem
- **Toolbelt invisível**: Mestre IA manipula tokens, mede distâncias, cria áreas de efeito sem intervenção do jogador

### 🎮 Fluxo de Interação com IA

**Fluxo Narrativo Visual**:

1. **O mestre descreve** → CENA VIZUAL (imagem full)
2. **A IA pergunta** → "O que você faz?"
3. **O jogador clica**:
   - Skill
   - Item
   - Move
   - Fala com NPC
4. **UI mostra feedback no painel direito**:
   - Explicação
   - Duração
   - Resultado
5. **Cena atualiza** → (nova imagem ou battlemap)

**Sistema de UI com IA**:

```
Cena carregada
  ↓
Party UI render
  ↓
Painel mostra contexto
  ↓
Jogador escolhe ação
  ↓
IA interpreta intenção
  ↓
IA gera narrativa + efeitos
  ↓
UI atualiza fichas/estado
```

### 🧠 O que torna essa UI superior ao Foundry tradicional?

- ✅ **Não depende de token 2D / grid**: Prioriza narrativa visual
- ✅ **Efeito cinematográfico**: Interface tipo ARPG/CRPG moderno
- ✅ **Híbrido perfeito**: Foundry + Baldur + Visual Novel narrada
- ✅ **Atende múltiplos públicos**: RPGistas old-school + público moderno casual

> 💡 **Resumo para vender**: "A UI é centrada em narrativa visual com retrato emocional + painel contextual de ficha e habilidades + barra de ações no rodapé. O mapa ou cena ocupa o centro e funciona como palco, enquanto o painel direito mostra contexto e o topo exibe estados globais do turno. O jogador interage através da barra inferior e avatares clicáveis, em um estilo Baldur's Gate 3."

### 🎮 Jogador Humano Mantém o Feeling

O jogador mantém agência total:

- **Clica em slots**: Hotbar com ações rápidas
- **Rola dado**: Interface de rolagem física
- **Move personagem**: Arrasta token no mapa
- **Navega em cenas**: Explora mapa do mundo
- **Interage com cards**: Clica em personagens para ver stats

> 💡 **Nada disso tira agency**. A UI de Foundry resolve isso brilhantemente — mantém gameplay legível, dá sensação de "mundo aberto", nunca força jogador a fechar menu.

### 🌅 VRPG Interface Goals

A UI deve:

- ✅ **Manter gameplay legível**: Informação clara, não sobrecarregada
- ✅ **Dar sensação de "mundo aberto"**: Mapa como elemento central
- ✅ **Nunca forçar jogador a fechar menu**: Painéis retráteis, não modais bloqueantes
- ✅ **Equilibrar visão macro e interação contextual**: Mapa grande + cards flutuantes
- ✅ **Suportar múltiplos modos**: Exploração, combate, roleplay, preparação

### 🚀 Evolução: Foundry + BG3 + Solasta

O VRPG mistura:

- **🔥 Foundry** = Visão macro de campanha, gestão de worldbuilding
- **🎮 BG3** = Experiência de personagem, foco em combate e roleplay
- **⚔️ Solasta** = Tática, grid, precisão de regras

**Resultado**:
- **Mapa grande** (exploração & lore) — Foundry
- **Battlemaps dinâmicos** (tática) — Solasta
- **HUD emocional** (IA companion) — BG3
- **Cards flutuantes** (identidade de personagem) — Foundry + BG3
- **Hotbar rápida** (ação) — BG3 + MMO

> O VRPG se torna: **"uma mesa viva comandada por IA, com UI de CRPG e agência humana."**

## Modos de Visualização do Mapa

### Modo Exploração (Mapa do Mundo)

**Características**:
- Mapa completo da campanha (regiões, cidades, geografias)
- Zoom livre para ver detalhes ou visão macro
- Marcadores de localização (cidades, pontos de interesse)
- Tokens de players/NPCs mostrando posição atual
- Overlays de região (nomes, fronteiras, características)

**Uso**: Fora de combate, exploração, roleplay, navegação

### Modo Combate (Battlemap com Grid)

**Características**:
- Grid visível (5ft squares)
- Zoom fixo para tática
- Medição de alcance automática
- Áreas de efeito visíveis
- LoS (Line of Sight) calculado
- Highlight de alcance de movimento

**Uso**: Durante combate, tática, posicionamento

### Modo Cena (Close-up de Localização)

**Características**:
- Close-up de localização específica
- Foco em detalhes visuais
- Tokens maiores para interação
- Background gerado por IA (Flux + LoRA)

**Uso**: Diálogos importantes, cenas narrativas, interações específicas

## Overlay de Estado no Centro do Mapa

**Função**: Comunicar estado do sistema sem intrusão

```css
.map-overlay-state {
  /* Já definido acima */
}

/* Estados possíveis */
.map-overlay-state.game-paused {
  /* "GAME PAUSED" com ícone de pausa */
}

.map-overlay-state.ai-thinking {
  /* "MESTRE IA PENSANDO..." com spinner */
}

.map-overlay-state.loading-assets {
  /* "CARREGANDO ASSETS..." com barra de progresso */
}

.map-overlay-state.setup-scene {
  /* "PREPARANDO CENA..." com ícone de configuração */
}
```

**Estados**:
- **"Ready to Roll"**: Pronto para rolagem (com ícone de dado acima)
- **"Game Paused"**: Jogo pausado manualmente
- **"Mestre IA pensando..."**: IA processando, gerando resposta
- **"Carregando assets..."**: Gerando imagens (LoRA, battlemaps)
- **"Preparando cena..."**: Setup de nova cena/localização
- **"Tempo de descanso narrativo"**: Pausa natural na narrativa

**Posicionamento**: Centro superior do mapa, abaixo de ícones de controle (se houver)

> 💡 **Filosofia**: Evita confusão. No VRPG você usaria para: Mestre IA pensando, Setup de cena, Carregamento de assets (LoRA / battlemaps), Tempo de descanso narrativo. É a "mesa dizendo: segura".

## Sistema de Camadas (UX Top-Tier)

A UI opera em **camadas mentais**, não técnicas:

| Camada | Função | Elemento | Visibilidade |
|--------|--------|----------|--------------|
| **Raiz (Mapa)** | Mundo, espaço, sigilo, progressão | Canvas central dominante | Sempre |
| **Ferramentas (Left)** | Operação do Mestre IA | Toolbelt vertical (ícones) | Sempre (compacto) |
| **Narrativa (Pop-up)** | Personagem, identidade | Card flutuante de personagem | Contextual |
| **Sistema (Right)** | Regras, recursos, mecânica | Painel lateral direito | Retrátil |
| **Ação (Bottom)** | Execução, gesto, input | Hotbar com slots numerados | Sempre |
| **Estado (Overlay)** | Comunicação de sistema | Overlay central no mapa | Quando necessário |

> Isso é **gold standard**. Foundry faz UI de RPG como se fosse: planilha de Notion + Google Maps + Baldur's Gate, mas com modularidade infinita.

## Resumo: Por Que Essa UI Funciona

Ela respeita o **fluxo cognitivo do jogador**:

1. **Onde estou?** → Mapa central (canvas dominante)
2. **Quem sou?** → Portrait + card flutuante
3. **O que posso fazer?** → Barra inferior (hotbar)
4. **O que aconteceu?** → Painel lateral direito (sistema/log)
5. **Ferramentas de interação?** → Painel lateral esquerdo (toolbelt)

> Esse fluxo é muito mais humano que um inventário gigante.

## Componentes Implementados

### 1. Voice HUD (Interface de Voz)

**Localização**: `src/client-electron/src/components/VoiceHUD.tsx`

**Descrição**: Componente flutuante na parte inferior central da tela que indica o estado da interação por voz.

**Características**:
- **3 Estados Visuais**:
  - **Listening** (Ouvindo): Barras azuis arcanas pulsam suavemente
  - **Processing** (Processando): Barras douradas com animação sincronizada
  - **Speaking** (IA Falando): Barras roxas com animação vigorosa
- **Glassmorphism**: Efeito de vidro fosco com blur e transparência
- **Animação de Entrada**: Desliza de baixo para cima com efeito elástico
- **Auto-hide**: Esconde automaticamente após período de inatividade
- **Typewriter Effect**: Texto aparece letra por letra quando a IA fala

**Design Tokens Utilizados**:
- `--vrpg-color-arcane-blue`: Azul arcano para estado "listening"
- `--vrpg-color-gold-primary`: Dourado para estado "processing"
- `--vrpg-glass-backdrop-blur`: Blur do vidro
- `--vrpg-spacing-md`: Espaçamentos

**Estrutura de Arquivos**:
```
src/client-electron/src/
├── components/
│   ├── VoiceHUD.tsx      # Componente React principal
│   └── VoiceHUD.css      # Estilos com glassmorphism
└── hooks/
    └── useVoiceHUD.ts    # Hook para gerenciar estado
```

**Uso Básico**:

```tsx
import React from 'react';
import { VoiceHUD } from './components/VoiceHUD';
import { useVoiceHUD } from './hooks/useVoiceHUD';

function App() {
  const voiceHUD = useVoiceHUD();

  const handleVoiceStart = () => {
    voiceHUD.startListening();
  };

  const handleVoiceProcess = () => {
    voiceHUD.startProcessing();
  };

  const handleVoiceResponse = (text: string) => {
    voiceHUD.startSpeaking(text);
  };

  return (
    <div>
      <VoiceHUD
        state={voiceHUD.state}
        statusText={voiceHUD.statusText}
        onClose={voiceHUD.hide}
      />
    </div>
  );
}
```

**Estados do HUD**:
- `listening` (Ouvindo): Cor azul arcano, animação de onda suave
- `processing` (Processando): Cor dourada, pulsação rápida sincronizada
- `speaking` (Falando): Cor roxa, vibração vigorosa
- `hidden` (Oculto): HUD não visível

**Props do Componente**:
- `state` (obrigatório): Estado atual do HUD (`VoiceHUDState`)
- `statusText` (opcional): Texto customizado
- `onClose` (opcional): Callback quando fecha
- `autoHideDelay` (padrão: 5000ms): Delay para auto-hide

**Hook `useVoiceHUD`**:
```typescript
const {
  state,           // Estado atual
  statusText,      // Texto de status
  startListening,  // Inicia estado "ouvindo"
  startProcessing, // Muda para "processando"
  startSpeaking,   // Muda para "falando" (aceita texto opcional)
  showError,       // Mostra mensagem de erro
  hide,            // Esconde o HUD
} = useVoiceHUD(autoHideDelay);
```

**Integração com Orchestrator**:
```tsx
import { useEffect } from 'react';
import { VoiceHUD } from './components/VoiceHUD';
import { useVoiceHUD } from './hooks/useVoiceHUD';

function GameInterface() {
  const voiceHUD = useVoiceHUD();

  useEffect(() => {
    // Escutar eventos do Orchestrator via IPC/WebSocket
    window.electron?.ipcRenderer.on('voice:listening', () => {
      voiceHUD.startListening();
    });
    window.electron?.ipcRenderer.on('voice:processing', () => {
      voiceHUD.startProcessing();
    });
    window.electron?.ipcRenderer.on('voice:response', (_, text) => {
      voiceHUD.startSpeaking(`IA Falando: "${text}"`);
    });
  }, [voiceHUD]);

  return <VoiceHUD {...voiceHUD} onClose={voiceHUD.hide} />;
}
```

**Acessibilidade**:
- ARIA Labels: `role="status"` e `aria-live="polite"`
- Reduced Motion: Respeita `prefers-reduced-motion`
- Keyboard Navigation: Botão de fechar acessível via teclado
- Focus States: Indicadores visuais de foco

**Performance**:
- Animações usam `transform` e `opacity` (GPU-accelerated)
- `backdrop-filter` otimizado para containers fixos
- Auto-hide previne vazamento de memória

### 2. Character Sheet (Ficha de Personagem D&D 5e)

**Localização**: `src/client-electron/src/components/CharacterSheet.tsx`

**Descrição**: Ficha completa de personagem D&D 5e com layout atualizado baseado em referência visual moderna.

**Layout**:
- **Cabeçalho**:
  - Retrato circular à esquerda (opcional)
  - Identidade do personagem (nome, nível, classe, raça, background)
  - Painel "Combat Status" à direita (CA, Iniciativa, Velocidade, HP, Hit Dice)
- **Barra de Atributos Horizontal**:
  - 6 atributos em colunas verticais (formato de pílula)
  - Cada atributo mostra: Label (STR/DEX/CON/INT/WIS/CHA), Score, Modifier, Ícone
  - Modificadores positivos destacados em dourado
- **Sistema de Abas**:
  - **Principal**: Grid de 3 colunas
    - **Esquerda**: Skills & Saves + Proficiencies
    - **Centro**: Attacks & Spellcasting + Features & Traits
    - **Direita**: Equipment + Personality
  - **Magias**: Grimório completo com slots e magias conhecidas
  - **Inventário**: Lista detalhada de itens e moedas
  - **Talentos & Traits**: Características de classe e raça

**Campos Suportados**:
- Informações básicas (nome, nível, classe, raça, background, alinhamento, XP)
- Retrato circular
- Atributos e modificadores
- HP, CA, Iniciativa, Velocidade, Hit Dice
- Saving Throws com proficiência
- Skills com proficiência
- Proficiencies (ícones)
- Ações e ataques (com versatilidade, alcance, dano)
- Maneuver DC (para Battle Master)
- Features e traits (com usos)
- Spells (CD, bônus, slots, magias conhecidas)
- Inventory (itens e moedas)
- Personality (traits, ideals, bonds, flaws)

**Design Tokens Utilizados**:
- `--vrpg-color-gold-primary`: Bordas douradas e acentos
- `--vrpg-color-arcane-blue`: Azul arcano para elementos mágicos
- `--vrpg-glass-backdrop-blur`: Blur do vidro
- `--vrpg-font-serif`: Fonte serif para títulos
- `--vrpg-font-sans`: Fonte sans para conteúdo

**Documentação**: Ver [CHARACTER_SHEET_COMPONENT.md](CHARACTER_SHEET_COMPONENT.md)

### 3. Journal (Diário de Campanha)

**Localização**: `src/client-electron/src/components/Journal.tsx`

**Descrição**: Diário de campanha com busca e filtros para visualizar missões, lore e notas.

**Layout**:
- **Modal Overlay**: Painel de vidro centralizado
- **Sidebar Esquerda**:
  - Barra de busca
  - Filtros por tipo (Tudo, Missões, Lore, Notas)
  - Lista rolável de entradas
- **Área de Leitura Direita**:
  - Estado vazio quando nada selecionado
  - Conteúdo da entrada selecionada
  - Badge de tipo e data
  - Título e corpo formatado (suporta HTML)

**Tipos de Entrada**:
- **Quest** (Missão): Badge dourado, ícones 📜, ⚔️, 🗺️
- **Lore** (Lore): Badge azul arcano, ícones 🏛️, 📚, 🔮
- **Note** (Nota): Badge cinza translúcido, ícones 📝, 🌿, 💡

**Funcionalidades**:
- Busca em tempo real (título e conteúdo)
- Filtros por tipo
- Seleção visual (entrada selecionada destacada em azul arcano)
- Scrollbars customizadas
- Responsivo (mobile/tablet/desktop)

**Design Tokens Utilizados**:
- `--vrpg-color-gold-primary`: Badges de missões
- `--vrpg-color-arcane-blue`: Badges de lore e seleção
- `--vrpg-glass-backdrop-blur`: Blur do vidro

**Documentação**: Ver [JOURNAL_COMPONENT.md](JOURNAL_COMPONENT.md)

### 4. Gameplay Interface (Interface Principal do Jogo)

**Localização**: `src/client-electron/src/components/GameplayInterface.tsx`

**Descrição**: Interface principal durante a sessão de jogo, com background da cena e overlay da UI.

**Estrutura**:
- **Camada 1: Background da Cena** (z-index: 0)
  - Imagem gerada pelo difusor (Flux + LoRA)
  - Overlay escuro opcional para contraste
- **Camada 2: Overlay da UI** (z-index: 10)
  - Layout em Grid CSS:
    - **Top-Left**: Nível e barra de XP
    - **Sidebar**: Menu de botões (Ficha, Inventário, Habilidades, Diário, Mapa, Configurações)
    - **Top-Right**: Botão toggle UI + Chat panel
    - **Footer**: Push-to-talk, Party frame, Action bar

**Componentes do Footer**:
- **Push-to-Talk** (Esquerda):
  - Botão de microfone
  - Indicadores de latência e FPS
  - Estado ativo visual
- **Party Frame** (Centro):
  - 4 retratos circulares com barras de HP
  - Estados: HP alto (verde), médio (amarelo), crítico (vermelho)
  - Clicável para abrir ficha
- **Action Bar** (Centro):
  - 10 slots numerados (1-0)
  - Slots podem conter ícones de ações/itens
  - Slot 10 marcado como "TALK"
  - Hotkeys visíveis

**Chat Panel** (Top-Right):
- Histórico de mensagens
- Cards de habilidades (estilo statblock)
- Input de mensagem com visualizador de áudio
- Scroll customizado

**Modo Screenshot**:
- Botão toggle UI (📷/👁️) no topo direito
- Atalho de teclado: `H`
- Esconde toda a UI mantendo apenas o botão toggle visível
- Permite captura de screenshots limpos da cena

**Design Tokens Utilizados**:
- `--vrpg-color-gold-primary`: Acentos dourados
- `--vrpg-color-arcane-blue`: Azul arcano
- `--vrpg-glass-backdrop-blur`: Blur do vidro
- `--vrpg-spacing-md`: Espaçamentos

**Responsividade**:
- Desktop: Layout completo em grid
- Tablet: Adapta mantendo estrutura
- Mobile: Layout em coluna única, sidebar limitada

### 5. Integração de Componentes

Todos os componentes seguem o mesmo padrão de design:

**Glassmorphism Consistente**:
- Painéis translúcidos com `backdrop-filter: blur()`
- Bordas sutis com acentos dourados/azuis
- Sombras profundas para profundidade

**Design Tokens Unificados**:
- Cores: `--vrpg-color-gold-primary`, `--vrpg-color-arcane-blue`
- Tipografia: `--vrpg-font-serif`, `--vrpg-font-sans`
- Espaçamentos: `--vrpg-spacing-sm/md/lg`
- Bordas: `--vrpg-radius-sm/md/lg`
- Blur: `--vrpg-glass-backdrop-blur`

**Acessibilidade**:
- Navegação por teclado (ESC fecha modais)
- ARIA labels apropriados
- Focus states visíveis
- Suporte a `prefers-reduced-motion`

**Performance**:
- Animações GPU-accelerated (transform, opacity)
- Blur apenas em containers fixos
- Scrollbars customizadas leves
- Componentes modulares e reutilizáveis

---

Este design system garante uma experiência visual imersiva, combinando a **visão macro de campanha do Foundry VTT** com a **experiência de personagem do BG3** e a **tática do Solasta**, mantendo a identidade fantasy moderna do VRPG Client com widescreen otimizado e múltiplas formas de interação.

## CSS Base e Componentes

### CSS Base - Estilos Globais e Utilitários

```css
/* -------------------------------------------------------------------------- */
/* CSS Base & Utilitários */
/* -------------------------------------------------------------------------- */

/* Estilo Global para a Aplicação */
body, html {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  font-family: var(--vrpg-font-sans);
  color: white; /* Texto branco por padrão sobre vidro escuro */
  background-color: var(--vrpg-color-bg-dark); /* Cor de fundo fallback */
  overflow: hidden; /* Evita barras de rolagem indesejadas na UI principal */
}

/* Títulos com Fonte Serif */
h1, h2, h3, .vrpg-title {
  font-family: var(--vrpg-font-serif);
  color: var(--vrpg-color-gold-primary);
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  margin: 0;
}

/* ===== UTILITÁRIOS DE GLASSMORPHISM ===== */

/* Painel de Vidro Padrão */
.vrpg-glass-panel {
  /* A mágica do Glassmorphism */
  background: var(--vrpg-glass-background);
  backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  -webkit-backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate)); /* Safari */

  /* Bordas e Reflexos */
  border-radius: var(--vrpg-radius-lg);
  border: 1px solid rgba(212, 175, 55, 0.3);
  box-shadow: var(--vrpg-glass-inner-border), var(--vrpg-glass-shadow-md);

  padding: var(--vrpg-spacing-md);
  overflow: hidden; /* Para conter elementos filhos e manter o formato */
  position: relative; /* Para posicionamento de ornamentos */
}

/* Borda Dourada Ornamentada (Opcional - para painéis principais como a Ficha) */
.vrpg-ornate-border {
  /* Pode ser implementado com pseudo-elementos ou SVG borders */
  border: 2px solid var(--vrpg-color-gold-primary);
  box-shadow: var(--vrpg-glow-gold-sm), var(--vrpg-glass-shadow-md), inset 0 0 20px var(--vrpg-color-gold-glow);
}

/* Glows e Acentos */
.vrpg-glow-arcane {
  box-shadow: var(--vrpg-glow-arcane-sm);
}

.vrpg-text-gold {
  color: var(--vrpg-color-gold-primary);
}
```

### Componentes CSS - Exemplos Práticos

#### HUD de Rolagem de Dados

```css
/* Container Principal da Rolagem */
.vrpg-dice-rolling-hud {
  display: flex;
  gap: var(--vrpg-spacing-md);
  /* Posicionamento centralizado na tela */
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1000; /* Garante que fique sobre tudo */
}

/* Painel Lateral de Modificadores */
.vrpg-dice-modifiers-panel {
  width: 180px; /* Largura fixa conforme o design */
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  /* Aplica o estilo de vidro */
  background: var(--vrpg-glass-background);
  backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  -webkit-backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  border-radius: 30px; /* Arredondamento maior nas laterais */
  border: 2px solid var(--vrpg-color-arcane-blue); /* Borda Arcano */
  box-shadow: var(--vrpg-glow-arcane-sm), var(--vrpg-glass-shadow-md);
  padding: var(--vrpg-spacing-md);
}

.vrpg-modifier-total {
  font-family: var(--vrpg-font-sans);
  font-size: 64px;
  font-weight: 700;
  color: white;
  text-shadow: 0 0 10px var(--vrpg-color-arcane-blue);
  margin-bottom: var(--vrpg-spacing-md);
}

/* Painel Central de Rolagem (Área dos Dados) */
.vrpg-dice-tray-panel {
  width: 600px; /* Largura maior para a área de rolagem */
  height: 350px;
  /* Aplica o estilo de vidro e a borda ornamentada dourada */
  background: var(--vrpg-glass-background);
  backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  -webkit-backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  border: 2px solid var(--vrpg-color-gold-primary);
  border-radius: var(--vrpg-radius-lg);
  box-shadow: var(--vrpg-glow-gold-sm), var(--vrpg-glass-shadow-md), inset 0 0 20px var(--vrpg-color-gold-glow);
  padding: var(--vrpg-spacing-lg);
  position: relative;
}

/* Exemplo de acento azul no canto superior esquerdo */
.vrpg-dice-tray-panel::before {
  content: '';
  position: absolute;
  top: -5px;
  left: -5px;
  width: 20px;
  height: 20px;
  background-color: var(--vrpg-color-arcane-blue);
  border-radius: 50%;
  box-shadow: 0 0 15px var(--vrpg-color-arcane-blue);
  z-index: 1;
}

/* Container dos Botões */
.vrpg-dice-buttons-container {
  display: flex;
  gap: var(--vrpg-spacing-lg);
  justify-content: center;
  margin-top: var(--vrpg-spacing-md);
}

/* Botões de Ação (Rolar/Cancelar) */
.vrpg-action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--vrpg-spacing-md) var(--vrpg-spacing-lg);
  min-width: 180px;
  
  /* Estilo do Botão */
  background: var(--vrpg-glass-background);
  backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  -webkit-backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  border-radius: 30px; /* Formato de pílula/losango */
  border: 2px solid var(--vrpg-color-arcane-blue);
  box-shadow: var(--vrpg-glow-arcane-sm), var(--vrpg-glass-shadow-sm);
  
  /* Tipografia */
  font-family: var(--vrpg-font-serif);
  font-size: 20px;
  font-weight: 600;
  color: var(--vrpg-color-arcane-blue);
  text-transform: uppercase;
  letter-spacing: 1px;
  
  cursor: pointer;
  transition: all 0.3s ease;
}

.vrpg-action-button:hover {
  background: rgba(74, 144, 226, 0.15); /* Aumenta a opacidade do fundo */
  box-shadow: 0 0 20px var(--vrpg-color-arcane-blue), var(--vrpg-glass-shadow-md);
  transform: translateY(-2px); /* Leve elevação */
}

.vrpg-button-icon {
  margin-right: var(--vrpg-spacing-sm);
  width: 24px;
  height: 24px;
  fill: currentColor; /* Ícone assume a cor do texto */
}
```

#### Estrutura da Ficha de Personagem

```css
/* Container Principal da Ficha */
.vrpg-character-sheet-frame {
  width: 1200px; /* Largura exemplo */
  height: 700px; /* Altura exemplo */
  display: grid;
  /* Define o grid com base nas áreas da imagem */
  grid-template-columns: 1fr 1fr 1fr; /* 3 colunas principais */
  grid-template-rows: auto auto auto; /* Linhas se adaptam ao conteúdo */
  gap: var(--vrpg-spacing-md);
  padding: var(--vrpg-spacing-lg);
  
  /* Aplica o estilo de painel de vidro com borda dourada */
  background: var(--vrpg-glass-background);
  backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  -webkit-backdrop-filter: blur(var(--vrpg-glass-backdrop-blur)) saturate(var(--vrpg-glass-backdrop-saturate));
  border: 2px solid var(--vrpg-color-gold-primary);
  border-radius: var(--vrpg-radius-lg);
  box-shadow: var(--vrpg-glow-gold-sm), var(--vrpg-glass-shadow-md), inset 0 0 20px var(--vrpg-color-gold-glow);
  
  /* Adiciona o acento azul no topo central */
  position: relative;
}

/* Painéis Internos (Slots de Atributos, Caixas de Texto, etc.) */
.vrpg-inner-glass-slot {
  /* Painéis menores dentro da ficha principal */
  background: rgba(255, 255, 255, 0.03); /* Vidro mais sutil internamente */
  border: 1px solid rgba(212, 175, 55, 0.2); /* Borda dourada fina e sutil */
  border-radius: var(--vrpg-radius-sm);
  
  /* Efeito de luz nas bordas superior/inferior (brilho horizontal) */
  position: relative;
  overflow: hidden;
  padding: var(--vrpg-spacing-sm);
}

/* Exemplo do brilho horizontal nas bordas internas */
.vrpg-inner-glass-slot::before {
  content: '';
  position: absolute;
  top: 0;
  left: 10%;
  right: 10%;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--vrpg-color-gold-glow), transparent);
  opacity: 0.5;
}
```

### Como Usar o CSS Base

1. **Copie os tokens** (`:root { ... }`) e cole no início do seu arquivo CSS principal (ex: `styles.css` ou `theme.css`).

2. **Copie o CSS base** e as classes utilitárias para definir os estilos padrões.

3. **Utilize as classes** `.vrpg-glass-panel`, `.vrpg-ornate-border`, etc., nos seus elementos HTML para aplicar o visual instantaneamente.

4. **Adapte e expanda** os exemplos de componentes para construir o restante da interface, sempre referenciando as variáveis (`var(--nome-da-variavel)`) para manter a consistência.

### Notas de Implementação

- **Performance**: O `backdrop-filter` pode ser custoso. Use apenas em containers fixos, não em elementos que se movem frequentemente.
- **Compatibilidade**: O prefixo `-webkit-backdrop-filter` é necessário para Safari.
- **Fallback**: Em navegadores sem suporte a `backdrop-filter`, o painel aparecerá com background sólido semi-transparente.
- **Responsividade**: Todos os valores de espaçamento e tamanhos devem ser ajustados para diferentes resoluções usando media queries.

---

## Referências de Componentes

Para implementação prática e exemplos de uso, consulte:

- [CHARACTER_SHEET_COMPONENT.md](CHARACTER_SHEET_COMPONENT.md) - Character Sheet
- [JOURNAL_COMPONENT.md](JOURNAL_COMPONENT.md) - Journal
