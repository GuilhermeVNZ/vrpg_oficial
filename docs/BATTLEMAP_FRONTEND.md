# Plano de ImplementaÃ§Ã£o: Sistema de Battlemap IsomÃ©trico (VersÃ£o 2.0)

## VisÃ£o Geral

Este plano detalha a implementaÃ§Ã£o de um sistema de battlemap isomÃ©trico **AI-driven** para combate tÃ¡tico no VRPG Client. O sistema Ã© centrado em **narraÃ§Ã£o do jogador** interpretada pela IA, com suporte completo para **campanhas oficiais** (Curse of Strahd, Storm King's Thunder, etc.) e integraÃ§Ã£o profunda com sistemas de Ã¡udio/visual.

**Diferencial CrÃ­tico**: Este nÃ£o Ã© apenas um battlemap tÃ¡tico - Ã© um **teatro virtual** onde a IA Mestre orquestra tudo baseado em narraÃ§Ã£o natural, com feedback visual/auditivo rico e suporte tanto para conteÃºdo gerado proceduralmente quanto para aventuras oficiais prÃ©-existentes.

**Objetivo**: Criar um battlemap isomÃ©trico que:
- âœ… Reage a narrativa do jogador (sem action bar manual)
- âœ… Suporta campanhas oficiais com mapas/NPCs do database
- âœ… Integra mÃºsica, SFX e VFX dinamicamente
- âœ… Fornece tooltips informativos e seleÃ§Ã£o avanÃ§ada
- âœ… Numera instÃ¢ncias de monstros automaticamente (Goblin 1, 2, 3...)
- âœ… Exibe tokens de objetos (baÃºs, armadilhas, items)

## Contexto do Projeto

### Stack Atual
- **Frontend**: Electron + React 18 + TypeScript + PixiJS
- **Backend**: Rust (services) + TypeScript (orchestrator)  
- **LLM Pipeline**: Qwen 1.5B (reaÃ§Ã£o rÃ¡pida) + Qwen 14B (narrativa)
- **Assets**: 380 monstros com 9 frames de animaÃ§Ã£o (1024x1024px, top-down view)
- **Design System**: Glassmorphism BG3/Foundry-inspired
- **RenderizaÃ§Ã£o**: PixiJS para mapas e performance

### Estado Atual
- `CenterCanvas.tsx` renderiza apenas background estÃ¡tico
- Sistema de Turn Order especificado mas nÃ£o implementado
- Sprites top-down gerados com prompts detalhados (Dark Fantasy Anime style)
- Turn Engine existe em spec mas sem visualizaÃ§Ã£o tÃ¡tica
- Orquestrador coordena transiÃ§Ãµes de estado de cena

---

## 1. Arquitetura do Sistema

### 1.1 VisÃ£o Geral da Arquitetura

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              React Component Layer                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚        BattleMapContainer.tsx                  â”‚  â”‚
â”‚  â”‚  - Estado de combate (turn, initiative)        â”‚  â”‚
â”‚  â”‚  - ComunicaÃ§Ã£o com Orchestrator via IPC        â”‚  â”‚
â”‚  â”‚  - Gerenciamento de modo (combat/exploration)  â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                 â”‚                                       â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚      IsometricBattleMap.tsx (PixiJS)           â”‚  â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚  â”‚
â”‚  â”‚  â”‚  Layer 1: Terrain (AI-generated background) â”‚  â”‚
â”‚  â”‚  â”‚  Layer 2: Grid Overlay (isometric)        â”‚  â”‚
â”‚  â”‚  â”‚  Layer 3: Tokens (animated sprites)       â”‚  â”‚
â”‚  â”‚  â”‚  Layer 4: Effects (AoE, spells, markers)  â”‚  â”‚
â”‚  â”‚  â”‚  Layer 5: UI Overlay (highlights, paths)  â”‚  â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              PixiJS Rendering Engine                 â”‚
â”‚  - Sprite batching (optimal performance)             â”‚
â”‚  - Animated sprite sheets (9-frame idle)             â”‚
â”‚  - Camera system (zoom, pan, rotation)               â”‚
â”‚  - Particle effects for spells/abilities            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚          Grid & Pathfinding System                   â”‚
â”‚  - Isometric coordinate conversion                   â”‚
â”‚  - A* pathfinding on square grid                    â”‚
â”‚  - Line of Sight (raycasting)                        â”‚
â”‚  - Movement range calculation                        â”‚
â”‚  - AoE shape calculation                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚            AI Map Generation (On-the-fly)            â”‚
â”‚  - Orchestrator â†’ Qwen 14B: "Generate tavern map"   â”‚
â”‚  - Qwen 14B â†’ INTENT: GenerateBattlemap(...)        â”‚
â”‚  - Intent Executor â†’ Art Daemon (Flux + ControlNet)  â”‚
â”‚  - Art Daemon â†’ PNG com grid implÃ­cito (overlay)     â”‚
â”‚  - Cache de mapas gerados (sessÃ£o)                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 1.2 Componentes Principais

```typescript
BattleMapSystem/
â”œâ”€â”€ components/
â”‚   â”œâ”€â”€ BattleMapContainer.tsx         # Container React principal
â”‚   â”œâ”€â”€ IsometricRenderer.tsx          # PixiJS renderer wrapper
â”‚   â”œâ”€â”€ TurnOrderDisplay.tsx           # Barra de ordem de turno (topo)
â”‚   â””â”€â”€ TokenInfoPopup.tsx             # Card de personagem flutuante
â”‚
â”œâ”€â”€ pixi/
â”‚   â”œâ”€â”€ layers/
â”‚   â”‚   â”œâ”€â”€ TerrainLayer.ts            # Background do mapa (AI-generated)
â”‚   â”‚   â”œâ”€â”€ GridLayer.ts               # Grid isomÃ©trico overlay
â”‚   â”‚   â”œâ”€â”€ TokenLayer.ts              # Sprites animados
â”‚   â”‚   â””â”€â”€ EffectsLayer.ts            # Efeitos de spell/ability
â”‚   â”‚
â”‚   â”œâ”€â”€ sprites/
â”‚   â”‚   â”œâ”€â”€ AnimatedTokenSprite.ts     # Sprite com 9-frame animation
â”‚   â”‚   â”œâ”€â”€ SpriteLoader.ts            # Carregador assÃ­ncrono
â”‚   â”‚   â””â”€â”€ SpriteCache.ts             # Cache de texturas
â”‚   â”‚
â”‚   â””â”€â”€ camera/
â”‚       â”œâ”€â”€ IsometricCamera.ts         # Sistema de cÃ¢mera
â”‚       â””â”€â”€ CameraController.ts        # Pan, zoom, rotation
â”‚
â”œâ”€â”€ grid/
â”‚   â”œâ”€â”€ IsometricGrid.ts               # Sistema de grid
â”‚   â”œâ”€â”€ CoordinateConverter.ts         # Screen â†” Grid â†” Isometric
â”‚   â”œâ”€â”€ PathFinder.ts                  # A* pathfinding
â”‚   â”œâ”€â”€ LineOfSight.ts                 # Raycasting LoS
â”‚   â””â”€â”€ MovementRangeCalculator.ts     # CÃ¡lculo de alcance
â”‚
â”œâ”€â”€ ai/
â”‚   â”œâ”€â”€ MapGenerationService.ts        # Interface com Orchestrator
â”‚   â”œâ”€â”€ TerrainPromptBuilder.ts        # Prompts para ControlNet
â”‚   â””â”€â”€ MapCache.ts                    # Cache de mapas gerados
â”‚
â”œâ”€â”€ interactions/
â”‚   â”œâ”€â”€ TokenDragHandler.ts            # Drag & drop de tokens
â”‚   â”œâ”€â”€ GridClickHandler.ts            # Clique em cÃ©lula do grid
â”‚   â”œâ”€â”€ SelectionManager.ts            # SeleÃ§Ã£o de tokens
â”‚   â””â”€â”€ HighlightManager.ts            # Highlights de cÃ©lulas
â”‚
â””â”€â”€ state/
    â”œâ”€â”€ useBattleMapState.ts           # Hook de estado React
    â”œâ”€â”€ battleMapSlice.ts              # Zustand slice
    â””â”€â”€ types.ts                       # TypeScript types

```

### 1.3 PreservaÃ§Ã£o da UI Existente Durante Combate

**PrincÃ­pio Fundamental**: Durante o combate, **APENAS o background muda** (de imagem estÃ¡tica para battlemap isomÃ©trico). **TODOS** os elementos de UI existentes permanecem visÃ­veis e funcionais.

#### 1.3.1 UI Atual a Ser Preservada

**RightSidebar (Canto Superior Direito)**:
```typescript
// components/layout/RightSidebar.tsx (EXISTENTE - NÃƒO MEXER)
// Posicionamento absoluto: top: 24px, right: 24px
// Elementos:
// 1. Status Effects (buffs/debuffs) - TOPO
//    - Peaceful Crane Stance (buff)
//    - Poisoned (condition)
//    - Haste (buff)
//    â†’ StatusCard components com tipo: 'buff' | 'condition'
//
// 2. Spacer flexÃ­vel (empurra chat para baixo)
//
// 3. Chat Area - BAIXO
//    - Mensagens do GM (system: true, borda dourada)
//    - Mensagens dos jogadores
//    - Input de texto
//    - Voice Activity Indicator (orb pulsante roxo)
```

**TopBar (Canto Superior Esquerdo)**:
```typescript
// components/layout/TopBar.tsx (EXISTENTE - NÃƒO MEXER)
// Posicionamento absoluto: top: 24px, left: 24px
// Elementos:
// 1. Level Circle (64x64px)
//    - Borda dourada
//    - "LEVEL" + nÃºmero grande
//
// 2. XP Bar (300px width mÃ­nimo)
//    - "XP: 12,500 / 20,000"
//    - Barra de progresso azul arcana
//    - "Next Level" label
```

**BottomBar (Centro Inferior)**:
```typescript
// components/layout/BottomBar.tsx (EXISTENTE - NÃƒO MEXER)
// Posicionamento absoluto: bottom: 24px, left: 50%, transform: translateX(-50%)
// Elementos:
// 1. Party Portraits (acima da barra)
//    - 4 retratos circulares (110x110px cada)
//    - HP arc overlay (semi-cÃ­rculo verde)
//    - HP label na base
//
// 2. GM Narrative Panel (600px width max)
//    - "Gamemaster:" (dourado)
//    - Texto narrativo atual
//    - Background glassmorphism escuro
```

**LeftSidebar** (nÃ£o visualizado mas existe):
- Mantido como estÃ¡ (menu de inventÃ¡rio, ficha, etc.)

**PushToTalk** (canto inferior esquerdo):
- Mantido como estÃ¡ (botÃ£o de voz)

#### 1.3.2 EstratÃ©gia de IntegraÃ§Ã£o com Battlemap

**O que muda durante combate**:
```typescript
// CenterCanvas.tsx (ÃšNICO componente que muda)

// ANTES (Exploration Mode):
<CenterCanvas>
  <div style={{ backgroundImage: 'url(background.jpeg)' }} />
</CenterCanvas>

// DEPOIS (Combat Mode):
<CenterCanvas>
  <IsometricBattleMap 
    mapData={...} 
    participants={...}
  />
</CenterCanvas>
```

**O que NÃƒO muda**:
- âœ… RightSide

bar (buffs/debuffs visible no topo direito)
- âœ… TopBar (level + XP bar no topo esquerdo)
- âœ… BottomBar (party portraits + GM narrative)
- âœ… LeftSidebar (menus)
- âœ… PushToTalk (botÃ£o de voz)

#### 1.3.3 ModificaÃ§Ãµes NecessÃ¡rias na UI Existente

**StatusCard (buffs/debuffs) - NOVA INTEGRAÃ‡ÃƒO**:

Durante combate, buffs/debuffs precisam ser **atualizados dinamicamente** pelo backend:

```typescript
// Novo IPC Event para atualizar status effects
interface UpdateStatusEffectsEvent {
  characterId: string;
  effects: StatusEffect[];
}

interface StatusEffect {
  id: string;
  name: string;
  duration: string;           // "3 rounds", "1 min", "until rest"
  type: 'buff' | 'debuff' | 'condition';
  source?: string;            // "Spell", "Monk Feature", "Poison"
  effect: string;             // "Advantage on Athletics"
  description: string;        // DescriÃ§Ã£o completa
  icon?: string;              // Opcional: Ã­cone do efeito
}

// IPC Event: 'battlemap:update-status-effects'
window.electron.on('battlemap:update-status-effects', (event, data: UpdateStatusEffectsEvent) => {
  // RightSidebar atualiza lista de status effects
  setActiveStatuses(data.effects);
});
```

**BottomBar (Party) - ATUALIZAÃ‡ÃƒO DE HP**:

Durante combate, HP dos membros do party Ã© atualizado:

```typescript
// Novo IPC Event para atualizar HP de party member
interface UpdatePartyMemberHP {
  memberName: string;         // "Aramil", "Lyra", etc.
  newHP: number;
  maxHP: number;
  visualEffect?: 'damage' | 'heal';
}

// IPC Event: 'party:update-hp'
window.electron.on('party:update-hp', (event, data: UpdatePartyMemberHP) => {
  // BottomBar atualiza HP arc overlay com animaÃ§Ã£o
  updatePartyMemberHP(data.memberName, data.newHP, data.maxHP);
  
  if (data.visualEffect === 'damage') {
    // Flash vermelho no retrato
    flashPortrait(data.memberName, 'red');
  } else if (data.visualEffect === 'heal') {
    // PartÃ­culas verdes subindo
    showHealParticles(data.memberName);
  }
});
```

**GM Narrative Panel - NARRAÃ‡ÃƒO DE COMBATE**:

Durante combate, narraÃ§Ã£o continua sendo exibida no painel inferior:

```typescript
// JÃ¡ existe implicitamente via chat, mas adicionar evento especÃ­fico
interface UpdateGMNarrative {
  text: string;
  emphasis?: 'normal' | 'critical' | 'success' | 'failure';
}

// IPC Event: 'gm:narrate'
window.electron.on('gm:narrate', (event, data: UpdateGMNarrative) => {
  // BottomBar atualiza texto da narrativa
  // Pode adicionar styling baseado em emphasis
  updateGMNarrative(data.text, data.emphasis);
});
```

#### 1.3.4 Z-Index Layering Durante Combate

Garantir que UI overlay permaneÃ§a acima do battlemap:

```
Z-Index Hierarchy:
- Background (z-index: -1): CenterCanvas com IsometricBattleMap
- Battlemap Grid/Tokens (z-index: 1-5): Layers internas do PixiJS
- UI Sidebars (z-index: 10): LeftSidebar, RightSidebar, TopBar, BottomBar
- Push To Talk (z-index: 20): BotÃ£o de voz
- Modals/Overlays (z-index: 30+): Character sheet, inventory, etc.
- Tooltips (z-index: 100): Tooltips do battlemap
```

#### 1.3.5 Exemplo de Componente Atualizado: BattleMapContainer

```typescript
// components/battlemap/BattleMapContainer.tsx

import React, { useEffect, useState } from 'react';
import IsometricBattleMap from './IsometricBattleMap';
import { useBattleMapState } from '../state/useBattleMapState';

function BattleMapContainer() {
  const { combatState, updateCombatState } = useBattleMapState();

  useEffect(() => {
    // Escutar evento de inÃ­cio de combate
    const unsubscribe = window.electron.on('combat:start', (event, data) => {
      console.log('[BattleMap] Combat started:', data);
      updateCombatState(data);
    });

    return unsubscribe;
  }, []);

  // Se nÃ£o estÃ¡ em combate, nÃ£o renderizar nada (CenterCanvas mostra background estÃ¡tico)
  if (!combatState || combatState.mode !== 'combat') {
    return null;
  }

  // Se estÃ¡ em combate, renderizar battlemap
  // NOTA: Toda UI existente permanece visÃ­vel (RightSidebar, TopBar, BottomBar)
  return (
    <IsometricBattleMap 
      mapData={combatState.mapData}
      participants={combatState.participants}
      initiativeOrder={combatState.initiativeOrder}
      currentTurn={combatState.currentTurn}
    />
  );
}

export default BattleMapContainer;
```

#### 1.3.6 AtualizaÃ§Ã£o do CenterCanvas.tsx

```typescript
// components/layout/CenterCanvas.tsx (MODIFICAR)

import React from 'react';
import background from '../../assets/background.jpeg';
import BattleMapContainer from '../battlemap/BattleMapContainer';
import { useBattleMapState } from '../../state/useBattleMapState';

const CenterCanvas: React.FC = () => {
    const { combatState } = useBattleMapState();
    const isInCombat = combatState?.mode === 'combat';

    return (
        <div style={{ width: '100%', height: '100%', position: 'relative', overflow: 'hidden' }}>
            {/* Background estÃ¡tico (exploration mode) */}
            {!isInCombat && (
                <div style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    width: '100%',
                    height: '100%',
                    backgroundImage: `url(${background})`,
                    backgroundSize: 'cover',
                    backgroundPosition: 'center',
                    zIndex: 0
                }} />
            )}

            {/* Battlemap isomÃ©trico (combat mode) */}
            {isInCombat && (
                <BattleMapContainer />
            )}
        </div>
    );
};

export default CenterCanvas;
```

#### 1.3.7 Checklist de IntegraÃ§Ã£o

- [ ] **Preservar RightSidebar**:
  - [ ] StatusCard components permanecem visÃ­veis
  - [ ] Chat continua funcionando
  - [ ] Voice indicator mantÃ©m animaÃ§Ã£o
  - [ ] Adicionar listener IPC: `battlemap:update-status-effects`

- [ ] **Preservar TopBar**:
  - [ ] Level circle permanece visÃ­vel
  - [ ] XP bar permanece visÃ­vel
  - [ ] Manter styling glassmorphism

- [ ] **Preservar BottomBar**:
  - [ ] Party portraits permanecem visÃ­veis
  - [ ] HP arcs atualizados dinamicamente
  - [ ] GM narrative panel mostra narraÃ§Ã£o de combate
  - [ ] Adicionar listeners IPC: `party:update-hp`, `gm:narrate`

- [ ] **Atualizar CenterCanvas**:
  - [ ] Adicionar lÃ³gica de troca: background estÃ¡tico â†” battlemap
  - [ ] Importar e integrar BattleMapContainer
  - [ ] Garantir transiÃ§Ã£o suave

- [ ] **Z-Index correto**:
  - [ ] Battlemap fica atrÃ¡s de toda UI (z-index: -1 a 5)
  - [ ] UI sidebars ficam acima (z-index: 10+)
  - [ ] Tooltips do battlemap ficam no topo (z-index: 100)

---

## 2. Sistema de NarraÃ§Ã£o e Contratos IPC

### 2.1 VisÃ£o Geral: Combate Dirigido por NarraÃ§Ã£o

**PrincÃ­pio Fundamental**: O jogador **nÃ£o executa aÃ§Ãµes diretamente**. Ele **narra o que quer fazer**, e a IA Mestre interpreta, valida e executa.

```
Fluxo de NarraÃ§Ã£o â†’ AÃ§Ã£o:

1. JOGADOR (voz/texto): "Eu quero me mover para perto do esqueleto guerreiro 3 e atacÃ¡-lo com minha espada"
         â†“
2. ASR Service â†’ TranscriÃ§Ã£o de texto
         â†“
3. Qwen 1.5B (reactive): Processa intenÃ§Ã£o
         â†“  
4. Qwen 14B (narrative): Gera INTENTs estruturados
   - MoveToken(characterId: "player1", targetPosition: near(skeleton_warrior_3))
   - MeleeAttack(source: "player1", target: "skeleton_warrior_3", weapon: "longsword")
         â†“
5. Orchestrator â†’ Intent Executor
         â†“
6. ValidaÃ§Ã£o de Regras (rules5e-service):
   - Movimento estÃ¡ dentro do alcance? (30ft com Dash, padrÃ£o 30ft)
   - PosiÃ§Ã£o "perto de" Ã© vÃ¡lida? (cÃ©lula adjacente livre)
   - Ataque Ã© viÃ¡vel? (alcance de 5ft para melee, arma equipada)
         â†“
7a. SE INVÃLIDO:
   - Orquestrador â†’ Qwen 14B gera narraÃ§Ã£o de bloqueio
   - "VocÃª tenta se aproximar, mas hÃ¡ um muro de pedras bloqueando o caminho"
   - Frontend recebe: CombatNarration(type: "blocked", reason: "obstacle")
         â†“
7b. SE VÃLIDO:
   - Frontend recebe: MoveTokenCommand(tokenId, path: [{x, y}, ...])
   - Token anima movimento atÃ© posiÃ§Ã£o final
   - Rules5e-service rola ataque: 1d20 + modificadores
   - Frontend recebe: AttackRoll(total: 18, success: true, critical: false)
   - Frontend recebe: DamageRoll(damage: 8, type: "slashing")
   - Token do skeleto faz shake animation
   - HP bar atualiza
   - SF X: sword_slash.ogg
         â†“
8. Qwen 14B gera narraÃ§Ã£o Ã©pica:
   - "VocÃª corre em direÃ§Ã£o ao esqueleto, esquivando de um pilar... [narraÃ§Ã£o rica]"
   - TTS â†’ Voz do mestre
   - Combat log atualizado
```

**DiferenÃ§a CrÃ­tica**: O jogador **nÃ£o clica em botÃµes**. Ele **fala ou digita**, e o sistema reage.

### 2.2 Contratos IPC (Frontend â†” Backend)

#### 2.2.1 Endpoints que o Frontend ENVIA (Frontend â†’ Backend)

```typescript
// ipc/contracts.ts

/**
 * Jogador indica intenÃ§Ã£o de mover seu token (via click ou voice)
 * IMPORTANTE: Movimento nÃ£o Ã© EXECUTADO, apenas INDICADO como desejo
 */
interface PlayerMovementIntent {
  characterId: string;           // ID do personagem (sempre "player1" para single-player)
  targetGridPosition?: { x: number; y: number };  // PosiÃ§Ã£o final desejada (se clicou no grid)
  targetTokenId?: string;         // OU: ID do token alvo (ex: "skeleton_warrior_3")
  proximityType?: 'adjacent' | 'near' | 'behind';  // "perto de", "atrÃ¡s de", etc.
  narrative?: string;             // NarraÃ§Ã£o bruta (se foi por voz)
}

// IPC: 'battlemap:player-movement-intent'
window.electron.invoke('battlemap:player-movement-intent', data: PlayerMovementIntent)

/**
 * Jogador seleciona um token (para inspecionar ou contextualizar aÃ§Ã£o)
 */
interface TokenSelectionEvent {
  tokenId: string;                // ID do token selecionado
  tokenType: 'character' | 'npc' | 'monster' | 'object';
  selectionType: 'click' | 'hover';
}

// IPC: 'battlemap:token-selected'
window.electron.invoke('battlemap:token-selected', data: TokenSelectionEvent)

/**
 * Jogador encerra seu turno (via narraÃ§Ã£o: "eu termino meu turno")
 */
interface EndTurnDeclaration {
  characterId: string;
}

// IPC: 'battlemap:end-turn'
window.electron.invoke('battlemap:end-turn', data: EndTurnDeclaration)

/**
 * RequisiÃ§Ã£o de tooltip (hover em token ou cÃ©lula)
 */
interface TooltipRequest {
  targetType: 'token' | 'grid_cell' | 'effect';
  targetId: string;               // tokenId ou "grid_x_y" ou effectId
}

// IPC: 'battlemap:request-tooltip'
window.electron.invoke('battlemap:request-tooltip', data: TooltipRequest)
  â†’ Promise<TooltipData>
```

#### 2.2.2 Eventos que o Frontend RECEBE (Backend â†’ Frontend)

```typescript
/**
 * Combate iniciado - Backend diz ao Frontend para montar o battlemap
 */
interface CombatStartEvent {
  combatId: string;
  sceneId: string;
  campaignType: 'official' | 'procedural';  // CRÃTICO para escolha de mapa
  officialAdventure?: {
    name: 'curse_of_strahd' | 'storm_kings_thunder' | ...;
    locationId: string;           // ex: "barovia_tavern", "death_house_basement"
  };
  mapData: {
    source: 'database' | 'generated';
    imagePath?: string;           // Se database
    generationRequest?: MapGenerationRequest;  // Se gerado
    gridWidth: number;
    gridHeight: number;
  };
  participants: CombatParticipant[];  // Lista de todos os combatentes
  initiativeOrder: string[];      // IDs ordenados por iniciativa
}

interface CombatParticipant {
  id: string;                     // Unique ID (ex: "goblin_1", "skeleton_warrior_2")
  baseType: string;               // Tipo base (ex: "goblin", "skeleton_warrior")
  instanceNumber: number;         // NÃºmero da instÃ¢ncia (1, 2, 3...)
  displayName: string;            // "Goblin 1", "Esqueleto Guerreiro 2"
  tokenSource: 'database' | 'sprite_library';
  tokenPath: string;              // Caminho para sprite ou token oficial
  stats: {
    hp: number;
    maxHp: number;
    ac: number;
    initiative: number;
  };
  position: { x: number; y: number };
  team: 'player' | 'ally' | 'enemy' | 'neutral';
}

// IPC Event: 'combat:start'
window.electron.on('combat:start', (event, data: CombatStartEvent) => { })

/**
 * Comando para mover token (APÃ“S validaÃ§Ã£o do backend)
 */
interface MoveTokenCommand {
  tokenId: string;
  path: GridCoords[];             // Lista de cÃ©lulas do path A*
  duration: number;               // DuraÃ§Ã£o total da animaÃ§Ã£o (ms)
  sfx?: string;                   // SFX a tocar (ex: "footsteps.ogg")
}

// IPC Event: 'battlemap:move-token'
window.electron.on('battlemap:move-token', (event, data: MoveTokenCommand) => { })

/**
 * AtualizaÃ§Ã£o de HP de um token
 */
interface TokenHPUpdate {
  tokenId: string;
  newHP: number;
  maxHP: number;
  damageType?: 'slashing' | 'piercing' | 'fire' | 'necrotic' | ...;
  visualEffect?: 'shake' | 'flash_red' | 'death';
  sfx?: string;                   // ex: "hit_flesh.ogg"
}

// IPC Event: 'battlemap:update-hp'
window.electron.on('battlemap:update-hp', (event, data: TokenHPUpdate) => { })

/**
 * Efeito visual (AoE, spell effect, etc.)
 */
interface VisualEffectCommand {
  effectType: 'aoe_circle' | 'aoe_cone' | 'aoe_line' | 'spell_projectile' | 'buff' | 'debuff';
  position?: GridCoords;          // Centro do efeito (para AoE)
  radius?: number;                // Raio em cÃ©lulas
  affectedCells?: GridCoords[];   // Lista de cÃ©lulas afetadas
  color?: string;                 // Cor do efeito
  duration: number;               // DuraÃ§Ã£o da animaÃ§Ã£o (ms)
  vfx?: string;                   // Particle effect (ex: "fireball_explosion.json")
  sfx?: string;                   // Som associado
}

// IPC Event: 'battlemap:show-effect'
window.electron.on('battlemap:show-effect', (event, data: VisualEffectCommand) => { })

/**
 * Rolagem de dados visualizada
 */
interface DiceRollDisplay {
  rollType: 'attack' | 'damage' | 'save' | 'skill';
  diceNotation: string;           // ex: "1d20+5", "3d6"
  result: number;
  breakdown: string;              // ex: "[18] + 5 = 23"
  success?: boolean;              // Para attack rolls (acertou?)
  critical?: boolean;             // CrÃ­tico?
  sfx: string;                    // "dice_roll.ogg"
}

// IPC Event: 'battlemap:dice-roll'
window.electron.on('battlemap:dice-roll', (event, data: DiceRollDisplay) => { })

/**
 * MudanÃ§a de turno
 */
interface TurnChangeEvent {
  previousTokenId: string;
  currentTokenId: string;
  turnNumber: number;
  roundNumber: number;
  isPlayerTurn: boolean;
  narration?: string;             // NarraÃ§Ã£o do mestre sobre mudanÃ§a de turno
}

// IPC Event: 'combat:turn-change'
window.electron.on('combat:turn-change', (event, data: TurnChangeEvent) => { })

/**
 * MÃºsica de fundo mudou
 */
interface MusicChangeEvent {
  trackId: string;
  trackPath: string;              // ex: "combat_intense.ogg"
  fade: boolean;                  // Fade in/out?
  loop: boolean;
}

// IPC Event: 'audio:music-change'
window.electron.on('audio:music-change', (event, data: MusicChangeEvent) => { })

/**
 * Adicionar token de objeto ao mapa (baÃº, armadilha, item)
 */
interface AddObjectToken {
  objectId: string;
  objectType: 'chest' | 'trap' | 'item' | 'door' | 'lever' | 'corpse';
  position: GridCoords;
  sprite: string;                 // Caminho para sprite do objeto
  interactable: boolean;
  revealed: boolean;              // Se armadilha, estÃ¡ revelada?
}

// IPC Event: 'battlemap:add-object'
window.electron.on('battlemap:add-object', (event, data: AddObjectToken) => { })

/**
 * Tooltip data (resposta a request)
 */
interface TooltipData {
  title: string;                  // ex: "Goblin 2"
  stats?: {
    hp: string;                   // ex: "8/15 HP"
    ac: string;                   // ex: "AC 12"
    conditions?: string[];        // ["Poisoned", "Prone"]
  };
  description?: string;           // DescriÃ§Ã£o breve
  actions?: string[];             // AÃ§Ãµes disponÃ­veis
}


## 3. Visual Assets & UI Kit (Battle Tracker)

Baseado nos arquivos de referência fornecidos, os seguintes assets foram gerados e estão prontos para uso na UI do Battle Tracker:

### 3.1 Assets Gerados (Artifacts)

1.  **Background Texture**: `battle_tracker_wood_bg.png`
    -   *Descrição*: Textura de madeira escura polida (Dark Oak/Mahogany) sem emendas.
    -   *Uso*: Background do container principal do `TurnOrderDisplay`.

2.  **Card Frame**: `initiative_card_frame.png`
    -   *Descrição*: Moldura vertical ornamentada em ouro com centro de vidro escuro semi-transparente.
    -   *Uso*: Container para cada participante na ordem de iniciativa.

3.  **Active Glow**: `active_turn_glow.png`
    -   *Descrição*: Aura mágica dourada/brilhante.
    -   *Uso*: Overlay atrás do card do personagem ativo no turno atual.

4.  **Initiative Badge**: `initiative_badge.png`
    -   *Descrição*: Escudo metálico dourado.
    -   *Uso*: Badge no canto do card exibindo o valor da iniciativa.

### 3.2 Estrutura Visual do Componente

```tsx
// components/battlemap/TurnOrderDisplay.tsx

<div className="turn-order-container" style={{ backgroundImage: `url(assets/ui/battle_tracker_wood_bg.png)` }}>
  {participants.map(p => (
    <div className={`initiative-card ${isActive ? 'active' : ''}`}>
      {/* Active Glow Layer */}
      {isActive && <img src="assets/ui/active_turn_glow.png" className="glow-effect" />}
      
      {/* Frame Layer */}
      <div className="card-frame" style={{ backgroundImage: `url(assets/ui/initiative_card_frame.png)` }}>
        {/* Portrait */}
        <img src={p.tokenPath} className="portrait" />
        
        {/* Initiative Badge */}
        <div className="badge-container" style={{ backgroundImage: `url(assets/ui/initiative_badge.png)` }}>
          <span>{p.initiative}</span>
        </div>
      </div>
    </div>
  ))}
</div>
```


### 2.3 Database vs GeraÃ§Ã£o Procedural

#### 2.3.1 Fluxo de DecisÃ£o: Mapa

```typescript
// ai/MapGenerationService.ts

class MapGenerationService {
  async getMapForCombat(combatContext: CombatStartEvent): Promise<MapData> {
    // 1. Verificar se Ã© campanha oficial
    if (combatContext.campaignType === 'official' && combatContext.officialAdventure) {
      const { name, locationId } = combatContext.officialAdventure;
      
      // 2. Buscar no database
      const dbMap = await this.fetchOfficialMap(name, locationId);
      
      if (dbMap) {
        console.log(`[MapService] Using official map: ${name}/${locationId}`);
        return {
          source: 'database',
          imagePath: dbMap.imagePath,
          gridWidth: dbMap.gridWidth,
          gridHeight: dbMap.gridHeight,
          obstacles: dbMap.obstacles  // CÃ©lulas bloqueadas
        };
      }
    }

    // 3. Fallback: Gerar via IA
    console.log(`[MapService] Generating procedural map`);
    return await this.generateProceduralMap(combatContext.sceneId);
  }

  private async fetchOfficialMap(adventure: string, locationId: string): Promise<OfficialMap | null> {
    // Query database local (SQLite ou JSON)
    // Estrutura esperada:
    // campaigns/
    //   curse_of_strahd/
    //     maps/
    //       barovia_tavern.png
    //       barovia_tavern.json  (metadata: grid size, obstacles)
    
    const mapPath = `campaigns/${adventure}/maps/${locationId}`;
    
    try {
      const metadata = await window.electron.invoke('db:load-map-metadata', { adventure, locationId });
      return {
        imagePath: `${mapPath}.png`,
        gridWidth: metadata.gridWidth,
        gridHeight: metadata.gridHeight,
        obstacles: metadata.obstacles
      };
    } catch (err) {
      console.warn(`[MapService] Official map not found: ${mapPath}`);
      return null;
    }
  }

  private async generateProceduralMap(sceneId: string): Promise<MapData> {
    // LÃ³gica anterior de geraÃ§Ã£o via IA (Stable Diffusion + ControlNet)
    const mapCache = new MapCache();
    return await mapCache.getOrGenerate(sceneId, ...);
  }
}
```

#### 2.3.2 Fluxo de DecisÃ£o: NPCs e Monstros

```typescript
// pixi/sprites/SpriteLoader.ts (atualizado)

class SpriteLoader {
  async loadTokenSprite(participant: CombatParticipant): Promise<PIXI.Texture[]> {
    // 1. Verificar se Ã© token oficial
    if (participant.tokenSource === 'database') {
      return await this.loadOfficialToken(participant.tokenPath);
    }

    // 2. Usar sprite library procedural (nossos 380 monstros)
    return await this.loadMonsterSprite(participant.baseType);
  }

  private async loadOfficialToken(tokenPath: string): Promise<PIXI.Texture[]> {
    // Carregar token oficial da campanha
    // Ex: campaigns/curse_of_strahd/tokens/strahd.png (estÃ¡tico ou animado)
    
    const texture = await PIXI.Assets.load(tokenPath);
    
    // Se for estÃ¡tico, retornar array com 1 textura
    // Se for animado, carregar frames
    return [texture]; // Simplificado
  }
}
```

### 2.4 Sistema de Tooltips e SeleÃ§Ã£o AvanÃ§ada

#### 2.4.1 Tooltip Manager

```typescript
// interactions/TooltipManager.ts

interface TooltipConfig {
  target: PIXI.DisplayObject;
  data: TooltipData;
  position: {x: number, y: number};
}

class TooltipManager {
  private currentTooltip: PIXI.Container | null = null;
  private tooltipCache: Map<string, TooltipData> = new Map();

  /**
   * Mostrar tooltip ao passar mouse sobre token
   */
  async showTooltip(tokenId: string, mousePos: {x: number, y: number}) {
    // 1. Verificar cache
    let data = this.tooltipCache.get(tokenId);
    
    // 2. Se nÃ£o existe, requisitar ao backend
    if (!data) {
      data = await window.electron.invoke('battlemap:request-tooltip', {
        targetType: 'token',
        targetId: tokenId
      });
      this.tooltipCache.set(tokenId, data);
    }

    // 3. Renderizar tooltip
    this.renderTooltip(data, mousePos);
  }

  private renderTooltip(data: TooltipData, pos: {x: number, y: number}) {
    // Limpar tooltip anterior
    if (this.currentTooltip) {
      this.currentTooltip.destroy();
    }

    // Criar container do tooltip
    const tooltip = new PIXI.Container();
    tooltip.x = pos.x + 10;
    tooltip.y = pos.y + 10;

    // Background (glassmorphism)
    const bg = new PIXI.Graphics();
    bg.beginFill(0x000000, 0.8);
    bg.lineStyle(2, 0xD4AF37, 0.6); // Borda dourada
    bg.drawRoundedRect(0, 0, 200, 100, 8);
    bg.endFill();
    tooltip.addChild(bg);

    // TÃ­tulo
    const title = new PIXI.Text(data.title, {
      fontFamily: 'Crimson Text',
      fontSize: 18,
      fill: 0xD4AF37,
      fontWeight: 'bold'
    });
    title.x = 10;
    title.y = 10;
    tooltip.addChild(title);

    // Stats (se existir)
    if (data.stats) {
      const statsText = `${data.stats.hp}\nAC: ${data.stats.ac}`;
      const stats = new PIXI.Text(statsText, {
        fontFamily: 'Inter',
        fontSize: 14,
        fill: 0xFFFFFF
      });
      stats.x = 10;
      stats.y = 35;
      tooltip.addChild(stats);
    }

    this.currentTooltip = tooltip;
    // Adicionar ao stage
    // app.stage.addChild(tooltip);
  }

  hideTooltip() {
    if (this.currentTooltip) {
      this.currentTooltip.destroy();
      this.currentTooltip = null;
    }
  }
}
```

#### 2.4.2 Sistema de SeleÃ§Ã£o com Feedback Visual

```typescript
// interactions/SelectionManager.ts (atualizado)

class SelectionManager {
  private selectedToken: AnimatedTokenSprite | null = null;
  private selectionRing: PIXI.Graphics | null = null;

  selectToken(token: AnimatedTokenSprite) {
    // Limpar seleÃ§Ã£o anterior
    this.clearSelection();

    // Marcar como selecionado
    this.selectedToken = token;

    // Criar anel de seleÃ§Ã£o (glow dourado pulsante)
    this.selectionRing = new PIXI.Graphics();
    this.selectionRing.lineStyle(3, 0xFFD700, 1);
    this.selectionRing.drawCircle(0, 0, 70);  // Anel ao redor do token
    
    // Adicionar ao token
    token.getSprite().addChild(this.selectionRing);

    // AnimaÃ§Ã£o de pulso
    this.animateSelectionRing();

    // Notificar backend
    window.electron.invoke('battlemap:token-selected', {
      tokenId: token.tokenId,
      tokenType: token.type,
      selectionType: 'click'
    });
  }

  private animateSelectionRing() {
    if (!this.selectionRing) return;

    let scale = 1.0;
    let growing = true;

    const pulse = () => {
      if (!this.selectionRing) return;

      if (growing) {
        scale += 0.01;
        if (scale >= 1.1) growing = false;
      } else {
        scale -= 0.01;
        if (scale <= 0.9) growing = true;
      }

      this.selectionRing.scale.set(scale);
      requestAnimationFrame(pulse);
    };

    pulse();
  }

  clearSelection() {
    if (this.selectionRing) {
      this.selectionRing.destroy();
      this.selectionRing = null;
    }
    this.selectedToken = null;
  }
}
```

### 2.5 NumeraÃ§Ã£o AutomÃ¡tica de InstÃ¢ncias

#### 2.5.1 Sistema de Naming

**Backend** Ã© responsÃ¡vel por numerar instÃ¢ncias automaticamente:

```typescript
// Backend: game-engine ou Orchestrator

class CombatInstanceManager {
  private instanceCounters: Map<string, number> = new Map();

  /**
   * Adicionar participante ao combate com numeraÃ§Ã£o automÃ¡tica
   */
  addParticipantToCombat(baseType: string, stats: any, position: GridCoords): CombatParticipant {
    // Incrementar contador para este tipo
    const currentCount = this.instanceCounters.get(baseType) || 0;
    const instanceNumber = currentCount + 1;
    this.instanceCounters.set(baseType, instanceNumber);

    // Gerar ID e display name
    const id = `${baseType}_${instanceNumber}`;
    const displayName = this.formatDisplayName(baseType, instanceNumber);

    return {
      id,
      baseType,
      instanceNumber,
      displayName,
      // ... resto dos dados
    };
  }

  private formatDisplayName(baseType: string, instanceNumber: number): string {
    // Converter snake_case para Title Case
    const readable = baseType
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');

    return `${readable} ${instanceNumber}`;
  }
}

// Exemplo de uso:
// addParticipantToCombat('goblin', ...) â†’ { id: "goblin_1", displayName: "Goblin 1" }
// addParticipantToCombat('goblin', ...) â†’ { id: "goblin_2", displayName: "Goblin 2" }
// addParticipantToCombat('skeleton_warrior', ...) â†’ { id: "skeleton_warrior_1", displayName: "Skeleton Warrior 1" }
```

**Frontend** apenas exibe o `displayName` recebido:

```typescript
// components/TurnOrderDisplay.tsx

function TurnOrderCard({ participant }: { participant: CombatParticipant }) {
  return (
    <div className="turn-order-card">
      <img src={participant.tokenPath} alt={participant.displayName} />
      <span className="turn-order-name">{participant.displayName}</span>
      <span className="turn-order-hp">{participant.stats.hp}/{participant.stats.maxHp}</span>
    </div>
  );
}
```

### 2.6 IntegraÃ§Ã£o com Ãudio e Visual

#### 2.6.1 Sistema de SFX

```typescript
// audio/SFXManager.ts

class SFXManager {
  private audioContext: AudioContext;
  private sfxCache: Map<string, AudioBuffer> = new Map();

  async playSFX(sfxId: string, volume: number = 1.0) {
    // Carregar do cache ou fetch
    let buffer = this.sfxCache.get(sfxId);
    
    if (!buffer) {
      const response = await fetch(`assets/sfx/${sfxId}`);
      const arrayBuffer = await response.arrayBuffer();
      buffer = await this.audioContext.decodeAudioData(arrayBuffer);
      this.sfxCache.set(sfxId, buffer);
    }

    // Tocar
    const source = this.audioContext.createBufferSource();
    const gainNode = this.audioContext.createGain();
    
    source.buffer = buffer;
    gainNode.gain.value = volume;
    
    source.connect(gainNode);
    gainNode.connect(this.audioContext.destination);
    source.start(0);
  }
}

// IntegraÃ§Ã£o com eventos do backend:
window.electron.on('battlemap:move-token', (event, data: MoveTokenCommand) => {
  // ... mover token

  if (data.sfx) {
    sfxManager.playSFX(data.sfx, 0.7);
  }
});

window.electron.on('battlemap:update-hp', (event, data: TokenHPUpdate) => {
  // ... atualizar HP

  if (data.sfx) {
    sfxManager.playSFX(data.sfx, 1.0);
  }
});
```

#### 2.6.2 Sistema de MÃºsica DinÃ¢mica

```typescript
// audio/MusicManager.ts

class MusicManager {
  private currentTrack: HTMLAudioElement | null = null;
  private fadeDuration: number = 2000; // 2 segundos

  async changeMusic(trackPath: string, fade: boolean = true, loop: boolean = true) {
    const newTrack = new Audio(trackPath);
    newTrack.loop = loop;

    if (fade && this.currentTrack) {
      // Cross-fade
      await this.crossFade(this.currentTrack, newTrack);
    } else {
      // Parar atual e tocar novo
      if (this.currentTrack) {
        this.currentTrack.pause();
      }
      newTrack.play();
    }

    this.currentTrack = newTrack;
  }

  private async crossFade(oldTrack: HTMLAudioElement, newTrack: HTMLAudioElement): Promise<void> {
    newTrack.volume = 0;
    newTrack.play();

    const steps = 50;
    const interval = this.fadeDuration / steps;

    for (let i = 0; i <= steps; i++) {
      const progress = i / steps;
      oldTrack.volume = 1 - progress;
      newTrack.volume = progress;
      await new Promise(resolve => setTimeout(resolve, interval));
    }

    oldTrack.pause();
  }
}

// IntegraÃ§Ã£o com backend:
window.electron.on('audio:music-change', (event, data: MusicChangeEvent) => {
  musicManager.changeMusic(data.trackPath, data.fade, data.loop);
});
```

#### 2.6.3 Sistema de VFX (Particle Effects)

```typescript
// pixi/effects/VFXManager.ts

import * as particles from '@pixi/particle-emitter';

class VFXManager {
  private emitters: Map<string, particles.Emitter> = new Map();

  showEffect(effect: VisualEffectCommand) {
    switch (effect.effectType) {
      case 'aoe_circle':
        this.showAoECircle(effect);
        break;
      case 'spell_projectile':
        this.showProjectile(effect);
        break;
      case 'buff':
        this.showBuffParticles(effect);
        break;
    }

    // SFX associado
    if (effect.sfx) {
      sfxManager.playSFX(effect.sfx);
    }
  }

  private showAoECircle(effect: VisualEffectCommand) {
    if (!effect.position || !effect.radius) return;

    const converter = new CoordinateConverter();
    const center = converter.getCellCenter(effect.position.x, effect.position.y);

    // Criar cÃ­rculo visual
    const circle = new PIXI.Graphics();
    circle.beginFill(effect.color ? parseInt(effect.color, 16) : 0xFF0000, 0.3);
    circle.drawCircle(center.x, center.y, effect.radius * ISO_TILE_WIDTH);
    circle.endFill();

    // Adicionar ao stage
    effectsLayer.addChild(circle);

    // Remover apÃ³s duraÃ§Ã£o
    setTimeout(() => {
      circle.destroy();
    }, effect.duration);
  }

  private showProjectile(effect: VisualEffectCommand) {
    // Implementar projÃ©til animado (ex: bola de fogo)
    // Usando PIXI Particle Emitter ou animaÃ§Ã£o manual
  }

  private showBuffParticles(effect: VisualEffectCommand) {
    // PartÃ­culas subindo do token (ex: cura, buff)
  }
}
```

### 2.7 Tokens de Objetos (BaÃºs, Armadilhas, Items)

#### 2.7.1 Object Token Sprite

```typescript
// pixi/sprites/ObjectTokenSprite.ts

class ObjectTokenSprite {
  private sprite: PIXI.Sprite;
  private gridX: number;
  private gridY: number;
  private objectType: string;
  private interactable: boolean;

  constructor(
    objectData: AddObjectToken,
    converter: CoordinateConverter
  ) {
    this.gridX = objectData.position.x;
    this.gridY = objectData.position.y;
    this.objectType = objectData.objectType;
    this.interactable = objectData.interactable;

    // Carregar sprite
    const texture = PIXI.Texture.from(objectData.sprite);
    this.sprite = new PIXI.Sprite(texture);
    this.sprite.anchor.set(0.5);
    this.sprite.scale.set(0.1); // Ajustar tamanho

    // Posicionar no grid
    const pos = converter.getCellCenter(this.gridX, this.gridY);
    this.sprite.x = pos.x;
    this.sprite.y = pos.y;

    // Se interativo, adicionar hover effect
    if (this.interactable) {
      this.sprite.interactive = true;
      this.sprite.buttonMode = true;
      this.sprite.on('pointerover', () => this.onHover());
      this.sprite.on('pointerout', () => this.onHoverEnd());
    }

    // Se armadilha nÃ£o revelada, ficar invisÃ­vel ou semi-transparente
    if (objectData.objectType === 'trap' && !objectData.revealed) {
      this.sprite.alpha = 0.0;
    }
  }

  private onHover() {
    // Highlight dourado
    this.sprite.tint = 0xFFD700;
  }

  private onHoverEnd() {
    this.sprite.tint = 0xFFFFFF;
  }

  reveal() {
    // Revelar armadilha
    this.sprite.alpha = 1.0;
  }

  getSprite(): PIXI.Sprite {
    return this.sprite;
  }
}
```

IntegraÃ§Ã£o:

```typescript
window.electron.on('battlemap:add-object', (event, data: AddObjectToken) => {
  const objectToken = new ObjectTokenSprite(data, converter);
  objectLayer.addChild(objectToken.getSprite());
});
```

---

## 3. Sistema de Grid IsomÃ©trico

### 2.1 Geometria IsomÃ©trica

**Grid Quadrangular com ProjeÃ§Ã£o IsomÃ©trica**

Usaremos uma grade quadrangular (5ft Ã— 5ft no D&D) com projeÃ§Ã£o isomÃ©trica 2:1 para criar profundidade visual:

```
Grid LÃ³gico (quadrado):      ProjeÃ§Ã£o IsomÃ©trica (2:1):
â”Œâ”€â”¬â”€â”¬â”€â”¬â”€â”                    â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†
â”œâ”€â”¼â”€â”¼â”€â”¼â”€â”¤                   â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²
â”œâ”€â”¼â”€â”¼â”€â”¼â”€â”¤                  â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†
â”œâ”€â”¼â”€â”¼â”€â”¼â”€â”¤                 â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²
â””â”€â”´â”€â”´â”€â”´â”€â”˜                â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†
                        â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²â•±â”‚â•²
                       â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†â”€â”€â—†
```

**Constantes de ProjeÃ§Ã£o**:
```typescript
const ISO_TILE_WIDTH = 128;   // Largura visual da cÃ©lula isomÃ©trica
const ISO_TILE_HEIGHT = 64;   // Altura visual (2:1 ratio)
const ISO_ANGLE = Math.atan(0.5);  // ~26.57 graus
```

### 2.2 ConversÃ£o de Coordenadas

```typescript
// grid/CoordinateConverter.ts

interface GridCoords {
  x: number;  // Coluna
  y: number;  // Linha
}

interface ScreenCoords {
  x: number;  // Pixel X
  y: number;  // Pixel Y
}

class CoordinateConverter {
  /**
   * Converte coordenadas de grid (quadrado) para tela (isomÃ©trico)
   */
  gridToScreen(gridX: number, gridY: number): ScreenCoords {
    const screenX = (gridX - gridY) * (ISO_TILE_WIDTH / 2);
    const screenY = (gridX + gridY) * (ISO_TILE_HEIGHT / 2);
    return { x: screenX, y: screenY };
  }

  /**
   * Converte coordenadas de tela (isomÃ©trico) para grid (quadrado)
   */
  screenToGrid(screenX: number, screenY: number): GridCoords {
    const gridX = (screenX / (ISO_TILE_WIDTH / 2) + screenY / (ISO_TILE_HEIGHT / 2)) / 2;
    const gridY = (screenY / (ISO_TILE_HEIGHT / 2) - screenX / (ISO_TILE_WIDTH / 2)) / 2;
    return { 
      x: Math.floor(gridX), 
      y: Math.floor(gridY) 
    };
  }

  /**
   * ObtÃ©m centro de uma cÃ©lula do grid em coordenadas de tela
   */
  getCellCenter(gridX: number, gridY: number): ScreenCoords {
    const base = this.gridToScreen(gridX, gridY);
    return {
      x: base.x + ISO_TILE_WIDTH / 2,
      y: base.y + ISO_TILE_HEIGHT / 2
    };
  }
}
```

### 2.3 RenderizaÃ§Ã£o do Grid

```typescript
// pixi/layers/GridLayer.ts

import * as PIXI from 'pixi.js';

class GridLayer {
  private container: PIXI.Container;
  private gridGraphics: PIXI.Graphics;
  private gridWidth: number;
  private gridHeight: number;
  private converter: CoordinateConverter;

  constructor(gridWidth: number, gridHeight: number) {
    this.gridWidth = gridWidth;
    this.gridHeight = gridHeight;
    this.container = new PIXI.Container();
    this.gridGraphics = new PIXI.Graphics();
    this.container.addChild(this.gridGraphics);
    this.converter = new CoordinateConverter();
  }

  /**
   * Desenha grid isomÃ©trico completo
   */
  drawGrid(visible: boolean = true) {
    this.gridGraphics.clear();
    
    if (!visible) return;

    this.gridGraphics.lineStyle(1, 0xFFFFFF, 0.2); // Linhas brancas semi-transparentes

    // Desenhar linhas verticais (Z-axis)
    for (let x = 0; x <= this.gridWidth; x++) {
      const start = this.converter.gridToScreen(x, 0);
      const end = this.converter.gridToScreen(x, this.gridHeight);
      this.gridGraphics.moveTo(start.x, start.y);
      this.gridGraphics.lineTo(end.x, end.y);
    }

    // Desenhar linhas horizontais (X-axis)
    for (let y = 0; y <= this.gridHeight; y++) {
      const start = this.converter.gridToScreen(0, y);
      const end = this.converter.gridToScreen(this.gridWidth, y);
      this.gridGraphics.moveTo(start.x, start.y);
      this.gridGraphics.lineTo(end.x, end.y);
    }
  }

  /**
   * Highlight de uma cÃ©lula especÃ­fica
   */
  highlightCell(gridX: number, gridY: number, color: number = 0xFFFF00, alpha: number = 0.3) {
    const corners = [
      this.converter.gridToScreen(gridX, gridY),
      this.converter.gridToScreen(gridX + 1, gridY),
      this.converter.gridToScreen(gridX + 1, gridY + 1),
      this.converter.gridToScreen(gridX, gridY + 1)
    ];

    const highlight = new PIXI.Graphics();
    highlight.beginFill(color, alpha);
    highlight.moveTo(corners[0].x, corners[0].y);
    corners.forEach(corner => highlight.lineTo(corner.x, corner.y));
    highlight.closePath();
    highlight.endFill();
    
    this.container.addChild(highlight);
    return highlight; // Para remover depois
  }

  /**
   * Highlight de mÃºltiplas cÃ©lulas (para movimento, AoE, etc.)
   */
  highlightCells(cells: GridCoords[], color: number, alpha: number = 0.3) {
    const highlights: PIXI.Graphics[] = [];
    cells.forEach(cell => {
      const hl = this.highlightCell(cell.x, cell.y, color, alpha);
      highlights.push(hl);
    });
    return highlights;
  }

  getContainer(): PIXI.Container {
    return this.container;
  }
}
```

---

## 3. Sistema de Sprites Animados

### 3.1 Estrutura dos Sprites Existentes

**Assets DisponÃ­veis**:
- **380 monstros** em `assets-and-models/sprites/monsters/{monster_name}/`
- **9 frames de animaÃ§Ã£o** por monstro (`idle_01.png` a `idle_09.png`)
- **ResoluÃ§Ã£o**: 1024x1024px por frame
- **Estilo**: Top-down (80-90Â° camera angle), Dark Fantasy Anime
- **Metadata**: `idle.json` com `fps: 12`, `loop: true`

### 3.2 Carregamento e Cache de Sprites

```typescript
// pixi/sprites/SpriteLoader.ts

interface SpriteMetadata {
  character_id: string;
  animation_type: string;
  frame_count: number;
  frame_width: number;
  frame_height: number;
  fps: number;
  loop: boolean;
}

class SpriteLoader {
  private cache: Map<string, PIXI.Texture[]> = new Map();
  private basePath: string = 'assets-and-models/sprites/monsters/';

  /**
   * Carrega todos os frames de um monstro
   */
  async loadMonsterSprite(monsterName: string): Promise<PIXI.Texture[]> {
    // Verificar cache primeiro
    if (this.cache.has(monsterName)) {
      return this.cache.get(monsterName)!;
    }

    // Carregar metadata
    const metadata = await this.loadMetadata(monsterName);
    
    // Carregar todos os frames
    const frames: PIXI.Texture[] = [];
    for (let i = 1; i <= metadata.frame_count; i++) {
      const framePath = `${this.basePath}${monsterName}/idle_${String(i).padStart(2, '0')}.png`;
      const texture = await PIXI.Assets.load(framePath);
      frames.push(texture);
    }

    // Armazenar em cache
    this.cache.set(monsterName, frames);
    return frames;
  }

  private async loadMetadata(monsterName: string): Promise<SpriteMetadata> {
    const metadataPath = `${this.basePath}${monsterName}/idle.json`;
    const response = await fetch(metadataPath);
    return await response.json();
  }

  /**
   * Pre-load de sprites mais comuns (goblin, skeleton, etc.)
   */
  async preloadCommonMonsters() {
    const common = ['goblin', 'skeleton', 'zombie', 'orc', 'goblin01'];
    await Promise.all(common.map(m => this.loadMonsterSprite(m)));
  }
}
```

### 3.3 Sprite Animado com PixiJS

```typescript
// pixi/sprites/AnimatedTokenSprite.ts

class AnimatedTokenSprite {
  private sprite: PIXI.AnimatedSprite;
  private scale: number = 0.125; // 1024px â†’ 128px (ISO_TILE_WIDTH)
  private gridX: number;
  private gridY: number;
  private converter: CoordinateConverter;

  constructor(
    frames: PIXI.Texture[],
    gridX: number,
    gridY: number,
    fps: number = 12
  ) {
    this.gridX = gridX;
    this.gridY = gridY;
    this.converter = new CoordinateConverter();

    // Criar AnimatedSprite
    this.sprite = new PIXI.AnimatedSprite(frames);
    this.sprite.animationSpeed = fps / 60; // PixiJS usa 60 FPS base
    this.sprite.loop = true;
    this.sprite.play();

    // Ajustar escala e posiÃ§Ã£o
    this.sprite.anchor.set(0.5, 0.8); // Ã‚ncora na base do sprite (pÃ©s)
    this.sprite.scale.set(this.scale);
    this.updatePosition();

    // Shadow/glow effect
    this.addShadow();
  }

  /**
   * Atualiza posiÃ§Ã£o do sprite baseado no grid
   */
  private updatePosition() {
    const screenPos = this.converter.getCellCenter(this.gridX, this.gridY);
    this.sprite.x = screenPos.x;
    this.sprite.y = screenPos.y;
  }

  /**
   * Adiciona sombra circular no chÃ£o
   */
  private addShadow() {
    const shadow = new PIXI.Graphics();
    shadow.beginFill(0x000000, 0.3);
    shadow.drawEllipse(0, 40, 30, 15); // Sombra alongada
    shadow.endFill();
    shadow.filters = [new PIXI.BlurFilter(4)];
    this.sprite.addChild(shadow);
  }

  /**
   * Move token para nova posiÃ§Ã£o (com animaÃ§Ã£o)
   */
  moveTo(newGridX: number, newGridY: number, duration: number = 500) {
    this.gridX = newGridX;
    this.gridY = newGridY;
    
    const newPos = this.converter.getCellCenter(newGridX, newGridY);
    
    // AnimaÃ§Ã£o suave (pode usar gsap ou criar manualmente)
    this.animateMovement(newPos.x, newPos.y, duration);
  }

  private animateMovement(targetX: number, targetY: number, duration: number) {
    const startX = this.sprite.x;
    const startY = this.sprite.y;
    const startTime = Date.now();

    const animate = () => {
      const elapsed = Date.now() - startTime;
      const progress = Math.min(elapsed / duration, 1);
      
      // Easing (ease-out quad)
      const eased = 1 - Math.pow(1 - progress, 3);
      
      this.sprite.x = startX + (targetX - startX) * eased;
      this.sprite.y = startY + (targetY - startY) * eased;

      if (progress < 1) {
        requestAnimationFrame(animate);
      }
    };

    animate();
  }

  getSprite(): PIXI.AnimatedSprite {
    return this.sprite;
  }

  destroy() {
    this.sprite.destroy();
  }
}
```

---

## 4. GeraÃ§Ã£o de Mapas via IA

### 4.1 Fluxo de GeraÃ§Ã£o On-the-Fly

```
Mestre IA detecta combate iminente
          â†“
Qwen 14B gera INTENT: CombatStart(scene_description: "tavern fight")
          â†“
Orchestrator â†’ Intent Executor
          â†“
Executor solicita mapa: GenerateBattlemap(type: "tavern_interior", size: "20x15")
          â†“
Art Daemon (Stable Diffusion + ControlNet)
   - Prompt: "isometric tavern interior battlemap, D&D fantasy..."
   - ControlNet: Grid overlay guidance
   - Output: PNG 2048x1536 com grid implÃ­cito
          â†“
Cache local (sessÃ£o) + Retorna PNG path
          â†“
Frontend carrega PNG como TerrainLayer
```

### 4.2 Prompts para GeraÃ§Ã£o de Mapas

```typescript
// ai/TerrainPromptBuilder.ts

interface MapGenerationRequest {
  sceneType: 'tavern' | 'dungeon' | 'forest' | 'cave' | 'city_street';
  gridWidth: number;
  gridHeight: number;
  lighting: 'bright' | 'dim' | 'dark';
  features?: string[]; // ['bar', 'tables', 'fireplace']
}

class TerrainPromptBuilder {
  buildPrompt(request: MapGenerationRequest): string {
    const basePrompt = `
Isometric battlemap for D&D 5e combat, ${request.sceneType} theme,
Dark Fantasy Anime illustration style,
Top-down camera angle (80-90 degrees),
Grid: ${request.gridWidth}x${request.gridHeight} squares (5ft each),
Lighting: ${request.lighting},
${request.features?.join(', ') || 'standard layout'},
High detail, tactical clarity, clean grid visibility,
2D painted illustration with cel-shading,
NO characters or creatures,
Solid dark grey background for unoccupied spaces,
Professional battlemap quality
    `.trim();

    return basePrompt;
  }

  buildControlNetHint(gridWidth: number, gridHeight: number): string {
    // ControlNet pode usar um grid template como guidance
    return `grid_${gridWidth}x${gridHeight}.png`; // Pre-generated grid templates
  }
}
```

### 4.3 Cache de Mapas

```typescript
// ai/MapCache.ts

interface CachedMap {
  sceneId: string;
  imagePath: string;
  gridWidth: number;
  gridHeight: number;
  timestamp: number;
}

class MapCache {
  private cache: Map<string, CachedMap> = new Map();
  private maxCacheSize: number = 10;

  async getOrGenerate(
    sceneId: string,
    request: MapGenerationRequest
  ): Promise<CachedMap> {
    // Verificar cache primeiro
    if (this.cache.has(sceneId)) {
      return this.cache.get(sceneId)!;
    }

    // Gerar novo mapa
    const mapPath = await this.generateMap(request);
    
    const cached: CachedMap = {
      sceneId,
      imagePath: mapPath,
      gridWidth: request.gridWidth,
      gridHeight: request.gridHeight,
      timestamp: Date.now()
    };

    // Adicionar ao cache (com LRU eviction se necessÃ¡rio)
    this.addToCache(sceneId, cached);
    return cached;
  }

  private async generateMap(request: MapGenerationRequest): Promise<string> {
    // ComunicaÃ§Ã£o com Orchestrator via IPC
    const result = await window.electron.invoke('orchestrator:generate-map', request);
    return result.imagePath;
  }

  private addToCache(sceneId: string, cached: CachedMap) {
    if (this.cache.size >= this.maxCacheSize) {
      // Remover mapa mais antigo (LRU)
      const oldest = Array.from(this.cache.entries())
        .sort((a, b) => a[1].timestamp - b[1].timestamp)[0];
      this.cache.delete(oldest[0]);
    }
    this.cache.set(sceneId, cached);
  }
}
```

---

## 5. Sistema de CÃ¢mera e InteraÃ§Ãµes

### 5.1 CÃ¢mera IsomÃ©trica

```typescript
// pixi/camera/IsometricCamera.ts

class IsometricCamera {
  private viewport: PIXI.Container;
  private zoom: number = 1.0;
  private minZoom: number = 0.5;
  private maxZoom: number = 2.0;
  private pan: { x: number; y: number } = { x: 0, y: 0 };

  constructor(viewport: PIXI.Container) {
    this.viewport = viewport;
  }

  /**
   * Zoom in/out
   */
  setZoom(newZoom: number, centerX?: number, centerY?: number) {
    const clampedZoom = Math.max(this.minZoom, Math.min(this.maxZoom, newZoom));
    
    if (centerX !== undefined && centerY !== undefined) {
      // Zoom mantendo ponto central
      const dx = centerX - this.viewport.x;
      const dy = centerY - this.viewport.y;
      
      this.viewport.scale.set(clampedZoom);
      this.viewport.x = centerX - dx * (clampedZoom / this.zoom);
      this.viewport.y = centerY - dy * (clampedZoom / this.zoom);
    } else {
      this.viewport.scale.set(clampedZoom);
    }

    this.zoom = clampedZoom;
  }

  /**
   * Pan (arrastar cÃ¢mera)
   */
  pan(deltaX: number, deltaY: number) {
    this.viewport.x += deltaX;
    this.viewport.y += deltaY;
    this.pan.x = this.viewport.x;
    this.pan.y = this.viewport.y;
  }

  /**
   * Centralizar em uma posiÃ§Ã£o do grid
   */
  centerOnCell(gridX: number, gridY: number, screenWidth: number, screenHeight: number) {
    const converter = new CoordinateConverter();
    const cellCenter = converter.getCellCenter(gridX, gridY);
    
    this.viewport.x = screenWidth / 2 - cellCenter.x * this.zoom;
    this.viewport.y = screenHeight / 2 - cellCenter.y * this.zoom;
    this.pan = { x: this.viewport.x, y: this.viewport.y };
  }
}
```

### 5.2 Drag & Drop de Tokens

```typescript
// interactions/TokenDragHandler.ts

class TokenDragHandler {
  private isDragging: boolean = false;
  private dragTarget: AnimatedTokenSprite | null = null;
  private originalGridPos: GridCoords | null = null;

  handlePointerDown(token: AnimatedTokenSprite, event: PIXI.FederatedPointerEvent) {
    this.isDragging = true;
    this.dragTarget = token;
    this.originalGridPos = { x: token.gridX, y: token.gridY };
    
    token.getSprite().alpha = 0.7; // Visual feedback
  }

  handlePointerMove(event: PIXI.FederatedPointerEvent, converter: CoordinateConverter) {
    if (!this.isDragging || !this.dragTarget) return;

    // Converter posiÃ§Ã£o do mouse para grid
    const gridPos = converter.screenToGrid(event.global.x, event.global.y);
    
    // Snap para cÃ©lula mais prÃ³xima
    const snappedX = Math.floor(gridPos.x);
    const snappedY = Math.floor(gridPos.y);

    // Validar movimento (range, LoS, etc.)
    if (this.isValidMove(snappedX, snappedY)) {
      // Preview do movimento
      this.showMovementPreview(snappedX, snappedY);
    }
  }

  handlePointerUp(pathFinder: PathFinder) {
    if (!this.isDragging || !this.dragTarget || !this.originalGridPos) return;

    // Calcular path A*
    const path = pathFinder.findPath(
      this.originalGridPos.x,
      this.originalGridPos.y,
      this.dragTarget.gridX,
      this.dragTarget.gridY
    );

    // Executar movimento se vÃ¡lido
    if (path) {
      this.executeMovement(path);
    } else {
      // Cancelar movimento
      this.dragTarget.moveTo(this.originalGridPos.x, this.originalGridPos.y);
    }

    // Limpar estado
    this.dragTarget.getSprite().alpha = 1.0;
    this.isDragging = false;
    this.dragTarget = null;
  }

  private isValidMove(gridX: number, gridY: number): boolean {
    // Verificar se estÃ¡ dentro do alcance de movimento
    // Verificar se cÃ©lula nÃ£o estÃ¡ ocupada
    // Verificar se terreno Ã© passÃ¡vel
    return true; // Simplificado
  }

  private executeMovement(path: GridCoords[]) {
    // Animar token ao longo do path
    let index = 0;
    const moveNext = () => {
      if (index >= path.length || !this.dragTarget) return;
      
      const cell = path[index];
      this.dragTarget.moveTo(cell.x, cell.y, 300);
      
      index++;
      setTimeout(moveNext, 300);
    };

    moveNext();

    // Enviar comando para backend
    this.sendMoveCommand(path);
  }

  private sendMoveCommand(path: GridCoords[]) {
    window.electron.invoke('orchestrator:move-token', {
      tokenId: this.dragTarget?.tokenId,
      path: path
    });
  }
}
```

---

## 6. Pathfinding e Line of Sight

### 6.1 A* Pathfinding

```typescript
// grid/PathFinder.ts

interface PathNode {
  x: number;
  y: number;
  g: number; // Custo do inÃ­cio atÃ© aqui
  h: number; // HeurÃ­stica atÃ© o destino
  f: number; // g + h
  parent: PathNode | null;
}

class PathFinder {
  private gridWidth: number;
  private gridHeight: number;
  private obstacles: Set<string>; // Set de "x,y" strings

  constructor(gridWidth: number, gridHeight: number) {
    this.gridWidth = gridWidth;
    this.gridHeight = gridHeight;
    this.obstacles = new Set();
  }

  /**
   * A* pathfinding
   */
  findPath(startX: number, startY: number, endX: number, endY: number): GridCoords[] | null {
    const openList: PathNode[] = [];
    const closedList: Set<string> = new Set();

    const startNode: PathNode = {
      x: startX,
      y: startY,
      g: 0,
      h: this.heuristic(startX, startY, endX, endY),
      f: 0,
      parent: null
    };
    startNode.f = startNode.g + startNode.h;
    openList.push(startNode);

    while (openList.length > 0) {
      // Pegar nÃ³ com menor F
      let currentIndex = 0;
      for (let i = 1; i < openList.length; i++) {
        if (openList[i].f < openList[currentIndex].f) {
          currentIndex = i;
        }
      }

      const current = openList[currentIndex];

      // Chegou no destino
      if (current.x === endX && current.y === endY) {
        return this.reconstructPath(current);
      }

      // Mover para closed list
      openList.splice(currentIndex, 1);
      closedList.add(`${current.x},${current.y}`);

      // Explorar vizinhos (4-direÃ§Ãµes ou 8-direÃ§Ãµes)
      const neighbors = this.getNeighbors(current.x, current.y);
      
      for (const neighbor of neighbors) {
        const key = `${neighbor.x},${neighbor.y}`;
        
        if (closedList.has(key) || this.obstacles.has(key)) {
          continue;
        }

        const gScore = current.g + 1; // Custo uniforme (pode ser variÃ¡vel por terreno)
        const hScore = this.heuristic(neighbor.x, neighbor.y, endX, endY);

        const existingNode = openList.find(n => n.x === neighbor.x && n.y === neighbor.y);

        if (!existingNode) {
          openList.push({
            x: neighbor.x,
            y: neighbor.y,
            g: gScore,
            h: hScore,
            f: gScore + hScore,
            parent: current
          });
        } else if (gScore < existingNode.g) {
          existingNode.g = gScore;
          existingNode.f = gScore + hScore;
          existingNode.parent = current;
        }
      }
    }

    return null; // Sem path encontrado
  }

  private heuristic(x1: number, y1: number, x2: number, y2: number): number {
    // Manhattan distance (adequado para grid quadrangular)
    return Math.abs(x1 - x2) + Math.abs(y1 - y2);
  }

  private getNeighbors(x: number, y: number): GridCoords[] {
    const neighbors: GridCoords[] = [];
    
    // 4-direÃ§Ãµes (N, S, E, W)
    const directions = [
      { x: 0, y: -1 }, // Norte
      { x: 0, y: 1 },  // Sul
      { x: 1, y: 0 },  // Leste
      { x: -1, y: 0 }  // Oeste
    ];

    for (const dir of directions) {
      const nx = x + dir.x;
      const ny = y + dir.y;
      
      if (nx >= 0 && nx < this.gridWidth && ny >= 0 && ny < this.gridHeight) {
        neighbors.push({ x: nx, y: ny });
      }
    }

    return neighbors;
  }

  private reconstructPath(node: PathNode): GridCoords[] {
    const path: GridCoords[] = [];
    let current: PathNode | null = node;

    while (current !== null) {
      path.unshift({ x: current.x, y: current.y });
      current = current.parent;
    }

    return path;
  }

  addObstacle(x: number, y: number) {
    this.obstacles.add(`${x},${y}`);
  }

  removeObstacle(x: number, y: number) {
    this.obstacles.delete(`${x},${y}`);
  }
}
```

### 6.2 Line of Sight (Raycast)

```typescript
// grid/LineOfSight.ts

class LineOfSight {
  private obstacles: Set<string>;

  constructor() {
    this.obstacles = new Set();
  }

  /**
   * Verifica se hÃ¡ linha de visÃ£o entre dois pontos (Bresenham's)
   */
  hasLineOfSight(x0: number, y0: number, x1: number, y1: number): boolean {
    const points = this.bresenhamLine(x0, y0, x1, y1);
    
    // Verificar se algum ponto intermediÃ¡rio Ã© um obstÃ¡culo
    for (let i = 1; i < points.length - 1; i++) {
      const key = `${points[i].x},${points[i].y}`;
      if (this.obstacles.has(key)) {
        return false;
      }
    }

    return true;
  }

  /**
   * Algoritmo de Bresenham para traÃ§ar linha no grid
   */
  private bresenhamLine(x0: number, y0: number, x1: number, y1: number): GridCoords[] {
    const points: GridCoords[] = [];
    let dx = Math.abs(x1 - x0);
    let dy = Math.abs(y1 - y0);
    let sx = x0 < x1 ? 1 : -1;
    let sy = y0 < y1 ? 1 : -1;
    let err = dx - dy;

    let x = x0;
    let y = y0;

    while (true) {
      points.push({ x, y });

      if (x === x1 && y === y1) break;

      const e2 = 2 * err;
      if (e2 > -dy) {
        err -= dy;
        x += sx;
      }
      if (e2 < dx) {
        err += dx;
        y += sy;
      }
    }

    return points;
  }

  addObstacle(x: number, y: number) {
    this.obstacles.add(`${x},${y}`);
  }
}
```

---

## 7. IntegraÃ§Ã£o com Sistema de Combate

### 7.1 Fluxo de Iniciativa â†’ Battlemap

```
1. Jogador/IA: "I attack the goblin!"
       â†“
2. Orchestrator detecta aÃ§Ã£o de combate sem combate ativo
       â†“
3. Orchestrator â†’ FSM: Transition to CombatTurnBased
       â†“
4. Orchestrator gera INTENT: CombatStart(participants: [...])
       â†“
5. Intent Executor:
   - Rola iniciativa para todos
   - Ordena participantes
   - Solicita geraÃ§Ã£o de mapa (se nÃ£o existir)
   - Posiciona tokens no grid inicial
       â†“
6. Frontend (React):
   - CenterCanvas muda de modo (exploration â†’ combat)
   - BattleMapContainer monta componente
   - Carrega mapa gerado
   - Renderiza tokens em posiÃ§Ãµes iniciais
   - Mostra Turn Order no topo
       â†“
7. Turn Engine ativo:
   - Turno avanÃ§a automaticamente
   - UI destaca token ativo
   - Jogador pode mover/agir
   - IA controla NPCs/monstros
```

### 7.2 IntegraÃ§Ã£o com Orquestrador

```typescript
// components/BattleMapContainer.tsx

interface BattleMapState {
  mode: 'exploration' | 'combat';
  participants: Combatant[];
  currentTurn: number;
  mapData: CachedMap | null;
}

function BattleMapContainer() {
  const [state, setState] = useState<BattleMapState>({
    mode: 'exploration',
    participants: [],
    currentTurn: 0,
    mapData: null
  });

  useEffect(() => {
    // Escutar eventos do Orchestrator
    const unsubscribe = window.electron.on('orchestrator:combat-start', async (data) => {
      console.log('Combat initiated:', data);

      // Gerar ou recuperar mapa
      const mapCache = new MapCache();
      const map = await mapCache.getOrGenerate(data.sceneId, data.mapRequest);

      // Atualizar estado
      setState({
        mode: 'combat',
        participants: data.participants,
        currentTurn: 0,
        mapData: map
      });
    });

    window.electron.on('orchestrator:combat-end', () => {
      setState(prev => ({ ...prev, mode: 'exploration' }));
    });

    window.electron.on('orchestrator:turn-advance', (data) => {
      setState(prev => ({ ...prev, currentTurn: data.turn }));
    });

    return unsubscribe;
  }, []);

  if (state.mode === 'exploration') {
    return <ExplorationMapView />;
  }

  return (
    <div className="battle-map-container">
      {/* Turn Order no topo */}
      <TurnOrderDisplay 
        participants={state.participants}
        currentTurn={state.currentTurn}
      />

      {/* Mapa isomÃ©trico */}
      <IsometricBattleMap 
        mapData={state.mapData}
        participants={state.participants}
      />

      {/* Action Bar no rodapÃ© */}
      <ActionBar />
    </div>
  );
}
```

---

## 8. Plano de ImplementaÃ§Ã£o (Step-by-Step)

### Fase 1: Foundation (1-2 semanas)

#### Step 1.1: Setup Base do PixiJS
- [ ] Criar `components/battlemap/IsometricRenderer.tsx`
- [ ] Configurar PixiJS Application com viewport
- [ ] Implementar sistema de layers (Container hierarchy)
- [ ] Criar `CoordinateConverter.ts` com testes unitÃ¡rios
- [ ] **VerificaÃ§Ã£o**: Grid de teste renderiza corretamente em isomÃ©trico

#### Step 1.2: Grid System
- [ ] Implementar `GridLayer.ts`
- [ ] Desenhar grid isomÃ©trico completo
- [ ] Implementar highlight de cÃ©lulas
- [ ] Adicionar toggle para mostrar/ocultar grid (debug)
- [ ] **VerificaÃ§Ã£o**: Grid overlay funciona com zoom/pan

#### Step 1.3: Camera System
- [ ] Implementar `IsometricCamera.ts`
- [ ] Zoom com scroll wheel (manter ponto central)
- [ ] Pan com drag do mouse
- [ ] Centralizar em cÃ©lula especÃ­fica
- [ ] **VerificaÃ§Ã£o**: Controles de cÃ¢mera suaves e responsivos

### Fase 2: Sprites e AnimaÃ§Ã£o (2 semanas)

#### Step 2.1: Sprite Loader
- [ ] Implementar `SpriteLoader.ts`
- [ ] Sistema de cache de texturas
- [ ] Pre-load de monstros comuns
- [ ] **VerificaÃ§Ã£o**: Sprites carregam sem lag

#### Step 2.2: Animated Token
- [ ] Criar `AnimatedTokenSprite.ts`
- [ ] Renderizar sprite em posiÃ§Ã£o do grid
- [ ] AnimaÃ§Ã£o de 9 frames (loop)
- [ ] Sombra no chÃ£o
- [ ] **VerificaÃ§Ã£o**: Token anima suavemente

#### Step 2.3: Token Movement
- [ ] Implementar movimento animado entre cÃ©lulas
- [ ] Easing para movimento suave
- [ ] **VerificaÃ§Ã£o**: Token se move fluidamente

### Fase 3: InteraÃ§Ãµes (1-2 semanas)

#### Step 3.1: Click Handling
- [ ] Implementar `GridClickHandler.ts`
- [ ] Converter clique de mouse para grid coords
- [ ] Highlight de cÃ©lula clicada
- [ ] **VerificaÃ§Ã£o**: Clique detecta cÃ©lula correta

#### Step 3.2: Drag & Drop
- [ ] Implementar `TokenDragHandler.ts`
- [ ] Arrastar token com mouse
- [ ] Snap para grid durante drag
- [ ] Preview de movimento vÃ¡lido
- [ ] **VerificaÃ§Ã£o**: Drag & drop funciona suavemente

#### Step 3.3: SeleÃ§Ã£o de Tokens
- [ ] Implementar `SelectionManager.ts`
- [ ] Highlight de token selecionado
- [ ] Mostrar info card ao clicar
- [ ] **VerificaÃ§Ã£o**: SeleÃ§Ã£o visual clara

### Fase 4: Pathfinding e LoS (1 semana)

#### Step 4.1: A* Pathfinding
- [ ] Implementar `PathFinder.ts`
- [ ] A* com obstÃ¡culos
- [ ] Calcular range de movimento
- [ ] **VerificaÃ§Ã£o**: Path calculado evita obstÃ¡culos

#### Step 4.2: Line of Sight
- [ ] Implementar `LineOfSight.ts`
- [ ] Bresenham raycast
- [ ] Visualizar LoS (opcional debug)
- [ ] **VerificaÃ§Ã£o**: LoS detecta obstÃ¡culos corretamente

### Fase 5: AI Map Generation (2 semanas)

#### Step 5.1: Terrain Layer
- [ ] Implementar `TerrainLayer.ts`
- [ ] Carregar PNG como background
- [ ] Escalar para fit no grid
- [ ] **VerificaÃ§Ã£o**: Background renderiza corretamente

#### Step 5.2: Prompt Builder
- [ ] Implementar `TerrainPromptBuilder.ts`
- [ ] Gerar prompts por tipo de cena
- [ ] ControlNet hints para grid guidance
- [ ] **VerificaÃ§Ã£o**: Prompts sÃ£o coerentes

#### Step 5.3: Map Generation Service
- [ ] Implementar `MapGenerationService.ts`
- [ ] ComunicaÃ§Ã£o com Orchestrator via IPC
- [ ] Cache de mapas gerados
- [ ] **VerificaÃ§Ã£o**: Mapa gerado em <10s

#### Step 5.4: Backend Integration
- [ ] Implementar endpoint `orchestrator:generate-map`
- [ ] IntegraÃ§Ã£o com Art Daemon (Stable Diffusion)
- [ ] Salvar PNG gerado em cache local
- [ ] **VerificaÃ§Ã£o**: Mapa gerado e carregado com sucesso

### Fase 6: Combat Integration (2 semanas)

#### Step 6.1: Turn Order Display
- [ ] Implementar `TurnOrderDisplay.tsx`
- [ ] Cards horizontais com retratos
- [ ] Highlight do turno ativo
- [ ] **VerificaÃ§Ã£o**: Turn order atualiza corretamente

#### Step 6.2: Combat State Management
- [ ] Criar `useBattleMapState.ts` hook
- [ ] Zustand slice para estado de combate
- [ ] Sincronizar com Orchestrator
- [ ] **VerificaÃ§Ã£o**: Estado sincronizado corretamente

#### Step 6.3: Combat Actions
- [ ] Integrar movimento com Turn Engine
- [ ] Enviar aÃ§Ãµes para backend
- [ ] Receber atualizaÃ§Ãµes de HP/status
- [ ] **VerificaÃ§Ã£o**: AÃ§Ãµes refletidas no backend

#### Step 6.4: Initiative Trigger
- [ ] Detectar transiÃ§Ã£o `Exploration â†’ CombatTurnBased`
- [ ] Montar BattleMap automaticamente
- [ ] Posicionar tokens iniciais
- [ ] **VerificaÃ§Ã£o**: Combate inicia corretamente

### Fase 7: Polish e OtimizaÃ§Ã£o (1 semana)

#### Step 7.1: Performance
- [ ] Implementar sprite batching (PixiJS)
- [ ] Culling de sprites fora da tela
- [ ] Otimizar re-renders React
- [ ] **VerificaÃ§Ã£o**: 60 FPS constante com 20+ tokens

#### Step 7.2: Visual Effects
- [ ] Implementar `EffectsLayer.ts`
- [ ] Efeitos de AoE (cÃ­rculo, cone, linha)
- [ ] PartÃ­culas de spell (opcional)
- [ ] **VerificaÃ§Ã£o**: Efeitos renderizam sem lag

#### Step 7.3: UI/UX Polish
- [ ] Tooltips informativos
- [ ] Feedback visual de aÃ§Ãµes
- [ ] AnimaÃ§Ãµes de transiÃ§Ã£o suaves
- [ ] **VerificaÃ§Ã£o**: UX fluida e intuitiva

---

## 9. VerificaÃ§Ã£o e Qualidade

### 9.1 Testes CrÃ­ticos

```typescript
// __tests__/CoordinateConverter.test.ts
describe('CoordinateConverter', () => {
  it('converts grid to screen correctly', () => {
    const converter = new CoordinateConverter();
    const screen = converter.gridToScreen(5, 3);
    // Assert screen.x e screen.y corretos
  });

  it('converts screen to grid correctly', () => {
    const converter = new CoordinateConverter();
    const grid = converter.screenToGrid(128, 64);
    // Assert grid.x e grid.y corretos
  });

  it('round-trips correctly', () => {
    const converter = new CoordinateConverter();
    const original = { x: 10, y: 7 };
    const screen = converter.gridToScreen(original.x, original.y);
    const grid = converter.screenToGrid(screen.x, screen.y);
    expect(grid.x).toBe(original.x);
    expect(grid.y).toBe(original.y);
  });
});

// __tests__/PathFinder.test.ts
describe('PathFinder', () => {
  it('finds shortest path without obstacles', () => {
    const finder = new PathFinder(10, 10);
    const path = finder.findPath(0, 0, 5, 5);
    expect(path).not.toBeNull();
    expect(path!.length).toBe(11); // Manhattan distance
  });

  it('avoids obstacles', () => {
    const finder = new PathFinder(10, 10);
    finder.addObstacle(2, 0);
    finder.addObstacle(2, 1);
    finder.addObstacle(2, 2);
    const path = finder.findPath(0, 1, 4, 1);
    expect(path).not.toBeNull();
    // Path deve contornar obstÃ¡culos
  });

  it('returns null when no path exists', () => {
    const finder = new PathFinder(5, 5);
    // Criar parede completa
    for (let y = 0; y < 5; y++) {
      finder.addObstacle(2, y);
    }
    const path = finder.findPath(0, 2, 4, 2);
    expect(path).toBeNull();
  });
});
```

### 9.2 Performance Benchmarks

| MÃ©trica | Target | CrÃ­tico |
|---------|--------|---------|
| FPS (20 tokens) | 60 | 30 |
| Sprite load time | <500ms | <2s |
| Map generation | <10s | <30s |
| Path calculation | <50ms | <200ms |
| Click response | <16ms | <50ms |

### 9.3 Checklist de Qualidade

- [ ] **PrecisÃ£o Visual**: Grid alinha perfeitamente com terreno
- [ ] **Responsividade**: UI responde em <16ms (60 FPS)
- [ ] **ConsistÃªncia**: Estado sincronizado entre frontend/backend
- [ ] **Acessibilidade**: Tooltips informativos, atalhos de teclado
- [ ] **Performance**: 60 FPS com 30+ tokens simultÃ¢neos
- [ ] **AI Quality**: Mapas gerados sÃ£o taticamente claros
- [ ] **IntegraÃ§Ã£o**: Combate flui naturalmente (iniciativa â†’ mapa â†’ aÃ§Ãµes)

---

## 10. ConsideraÃ§Ãµes TÃ©cnicas Importantes

### 10.1 Compatibilidade de Assets

**Sprites Top-Down vs IsomÃ©trico**:
- Os sprites existentes sÃ£o top-down (80-90Â° camera)
- Em vista isomÃ©trica, eles funcionam bem pois a Ã¢ncora Ã© na base (pÃ©s)
- Como nÃ£o hÃ¡ rotaÃ§Ã£o de sprites, todos ficam orientados da mesma forma
- Isso Ã© aceitÃ¡vel e comum em jogos tÃ¡ticos 2D (ver Divinity: Original Sin)

**Grid Overlay vs Mapa Gerado**:
- Mapa gerado tem grid "pintado" no terreno (guidance para IA)
- Grid overlay Ã© semi-transparente e pode ser toggle on/off
- Alinhamento Ã© crÃ­tico: usar ControlNet com template de grid

### 10.2 LimitaÃ§Ãµes e Trade-offs

**GeraÃ§Ã£o de Mapa**:
- âš ï¸ LatÃªncia inicial de 10-30s para gerar mapa
- âœ… SoluÃ§Ã£o: Cache + pre-generation de mapas comuns
- âœ… Fallback: Usar templates prÃ©-gerados enquanto gera

**Sprite Resolution**:
- âš ï¸ Sprites 1024x1024 sÃ£o grandes para renderizaÃ§Ã£o em tempo real
- âœ… SoluÃ§Ã£o: Escalar para 128px e usar sprite batching do PixiJS
- âœ… OtimizaÃ§Ã£o: Compartilhar texturas entre instÃ¢ncias iguais

**Complexidade de Pathfinding**:
- âš ï¸ Mapas grandes (30x30) podem ter path calculation custoso
- âœ… SoluÃ§Ã£o: Limitar alcance de movimento (6 squares tÃ­pico em D&D)
- âœ… OtimizaÃ§Ã£o: Calcular apenas quando necessÃ¡rio (nÃ£o em hover)

### 10.3 Extensibilidade Futura

**Recursos para VersÃµes Futuras**:
- [ ] RotaÃ§Ã£o de sprites (4 ou 8 direÃ§Ãµes)
- [ ] ElevaÃ§Ã£o (terreno multi-nÃ­vel)
- [ ] Fog of War (visÃ£o limitada por LoS)
- [ ] AnimaÃ§Ãµes de ataque (alÃ©m de idle)
- [ ] Efeitos de clima (chuva, neve)
- [ ] IluminaÃ§Ã£o dinÃ¢mica (tochas, spells)
- [ ] DestruiÃ§Ã£o de terreno (portas quebrando)
- [ ] Multiplayer (sincronizaÃ§Ã£o de tokens)

---

## 11. PrÃ³ximos Passos Recomendados

1. **Validar com ProtÃ³tipo MÃ­nimo**:
   - Implementar Fase 1 completa (Grid + Camera)
   - Validar conversÃ£o de coordenadas com testes visuais
   - Confirmar que fundaÃ§Ã£o estÃ¡ sÃ³lida antes de prosseguir

2. **Gerar 1 Mapa de Teste com IA**:
   - Usar Stable Diffusion localmente com ControlNet
   - Validar qualidade e alinhamento de grid
   - Ajustar prompts se necessÃ¡rio

3. **Integrar com Sistema de Combate Existente**:
   - Conectar com Turn Engine
   - Validar fluxo completo (iniciativa â†’ mapa â†’ aÃ§Ãµes)
   - Garantir que sistema Ã© viÃ¡vel antes de polir

4. **Iterar em Qualidade Visual**:
   - Refinar rendering de sprites
   - Adicionar efeitos visuais
   - Polish de UI/UX

---

## ConclusÃ£o

Este plano fornece uma arquitetura completa e **AI-driven** para um sistema de battlemap isomÃ©trico no VRPG que vai muito alÃ©m de um mapa tÃ¡tico simples. Ã‰ um **teatro virtual interativo** que responde Ã  narraÃ§Ã£o natural do jogador.

### Diferenci

ais TÃ©cnicos Principais

1. **Combate por NarraÃ§Ã£o** (nÃ£o action-bar)
   - Jogador fala: "Eu me movo para perto do Goblin 2 e ataco"
   - IA interpreta, valida regras D&D 5e, e executa visual/auditivamente

2. **Suporte Dual: Oficial + Procedural**
   - Campanhas oficiais (Curse of Strahd, etc.) usam mapas/NPCs do database
   - Aventuras procedurais geram tudo via IA on-the-fly

3. **Contratos IPC Completos**
   - 10+ endpoints documentados (Frontend â†” Backend)
   - Backend sabe exatamente o que enviar, Frontend sabe exatamente como reagir

4. **Tooltips e SeleÃ§Ã£o AvanÃ§ada**
   - Hover em tokens mostra HP/AC/condiÃ§Ãµes
   - SeleÃ§Ã£o visual com anel dourado pulsante
   - IntegraÃ§Ã£o com sistema de narraÃ§Ã£o

5. **NumeraÃ§Ã£o AutomÃ¡tica**
   - Backend: `goblin_1`, `goblin_2`, `skeleton_warrior_1`
   - Frontend exibe: "Goblin 1", "Goblin 2", "Skeleton Warrior 1"
   - Jogador pode referenciar: "ataco o esqueleto guerreiro 3"

6. **IntegraÃ§Ã£o Audiovisual Completa**
   - SFX Manager: sword_slash.ogg, hit_flesh.ogg, footsteps.ogg
   - Music Manager: Cross-fade dinÃ¢mico para combate intenso
   - VFX Manager: Particle effects para feitiÃ§os, AoE, buffs

7. **Tokens de Objetos**
   - BaÃºs, armadilhas, items, portas, alavancas
   - Interativos (hover effect dourado)
   - Armadilhas ocultÃ¡veis/revelÃ¡veis

### Requisitos para Backend (NÃƒO implementar agora, apenas SABER o que enviar)

**Endpoints que o Backend precisa implementar**:
- `combat:start` â†’ Informar frontend para montar battlemap
- `battlemap:move-token` â†’ Comandar movimento validado
- `battlemap:update-hp` â†’ Atualizar HP com animaÃ§Ã£o
- `battlemap:show-effect` â†’ Mostrar VFX (fireball, cure wounds, etc.)
- `battlemap:dice-roll` â†’ Exibir rolagem animada
- `combat:turn-change` â†’ AvanÃ§ar turno com feedback visual
- `audio:music-change` â†’ Mudar mÃºsica de fundo
- `battlemap:add-object` â†’ Adicionar baÃº/armadilha ao mapa

**Dados que o Backend precisa fornecer**:
- `CombatParticipant` com `displayName`, `instanceNumber`, `tokenSource`
- `CombatStartEvent` com `campaignType`, `officialAdventure` (se aplicÃ¡vel)
- `MapData` com `source: 'database' | 'generated'`

### Viabilidade TÃ©cnica

âœ… **ALTA** 
- Stack tecnolÃ³gico adequado (PixiJS + React + TypeScript)  
- Assets existentes compatÃ­veis (sprites top-down 1024x1024)  
- GeraÃ§Ã£o de IA factÃ­vel (Stable Diffusion + ControlNet para mapas)  
- IPC contracts claros e bem definidos
- Suporte a database para aventuras oficiais (estrutura de pastas definida)

### Qualidade da ExperiÃªncia

â­â­â­â­â­ **EXCEPCIONAL**
- **NarraÃ§Ã£o Natural**: Jogador fala como faria numa mesa real
- **Feedback Rico**: Visual (animaÃ§Ãµes, VFX) + Auditivo (SFX, mÃºsica)
- **AdaptÃ¡vel**: Funciona tanto para Curse of Strahd quanto para dungeons procedurais
- **Taticamente Claro**: Grid isomÃ©trico mantÃ©m regras D&D
- **Imersivo**: Sprites animados, mÃºsica dinÃ¢mica, efeitos de spell

### Alcance dos Objetivos

âœ… **Estilo FoundryVTT com Perspectiva**: ALCANÃ‡ADO
- Grid quadrangular (5ft Ã— 5ft) mantÃ©m regras D&D
- Vista isomÃ©trica (2:1 ratio) adiciona profundidade visual
- Sprites 2D top-down funcionam perfeitamente em perspectiva
- Sistema modular e extensÃ­vel

âœ… **AI-Driven Combat**: ALCANÃ‡ADO
- NarraÃ§Ã£o â†’ InterpretaÃ§Ã£o â†’ ValidaÃ§Ã£o â†’ ExecuÃ§Ã£o
- Mestre IA orquestra tudo baseado em regras D&D 5e
- Jogador nem precisa saber mecÃ¢nicas, apenas narra

âœ… **Campanha Oficial Support**: ALCANÃ‡ADO
- Database lookup para mapas/NPCs existentes
- Fallback para geraÃ§Ã£o procedural
- Estrutura de pastas clara (`campaigns/{nome}/maps/` e `/tokens/`)

âœ… **Tooltips e Interatividade**: ALCANÃ‡ADO  
- Hover mostra informaÃ§Ãµes (HP, AC, conditions)
- SeleÃ§Ã£o visual clara (anel dourado pulsante)
- Object tokens interativos

âœ… **Ãudio/Visual Integration**: ALCANÃ‡ADO
- SFX, MÃºsica dinÃ¢mica, VFX particle effects
- Sincronizado com aÃ§Ãµes de combate
- Cross-fade, volume control, cache de assets

### PrÃ³ximos Passos (RecomendaÃ§Ãµes de ImplementaÃ§Ã£o)

**FASE 1: FundaÃ§Ã£o (2-3 semanas)**
- [ ] Coordenadas isomÃ©tricas (CoordinateConverter)
- [ ] Grid rendering (GridLayer com highlight)
- [ ] CÃ¢mera (IsometricCamera com zoom/pan)
- [ ] Sprites animados (AnimatedTokenSprite com 9 frames)

**FASE 2: IPC e Contratos (1-2 semanas)**
- [ ] Definir interfaces TypeScript compartilhadas
- [ ] Setup de listeners (window.electron.on)
- [ ] Mock de eventos de backend para testes
- [ ] ValidaÃ§Ã£o de contratos

**FASE 3: Database e Assets (1 semana)**
- [ ] Estrutura de pastas para campanhas oficiais
- [ ] Map loader (database vs generated)
- [ ] Token loader (official vs sprite library)

**FASE 4: Tooltips e SeleÃ§Ã£o (1 semana)**
- [ ] TooltipManager com cache
- [ ] SelectionManager com anel pulsante
- [ ] Hover effects em tokens e objects

**FASE 5: Ãudio/Visual (1-2 semanas)**
- [ ] SFXManager (Web Audio API)
- [ ] MusicManager (cross-fade)
- [ ] VFXManager (particle effects, AoE circles)

**FASE 6: Combat Integration (2 semanas)**
- [ ] Turn Order Display (BG3-style)
- [ ] Move token animation (path smoothing)
- [ ] HP update with shake effect
- [ ] Dice roll visualization

**FASE 7: Polish (1 semana)**
- [ ] Performance optimization (60 FPS com 30+ tokens)
- [ ] Tooltip styling (glassmorphism)
- [ ] Transition animations
- [ ] Object tokens (chests, traps)

**Estimativa Total**: 10-12 semanas para sistema completo funcional e polido

### ConsideraÃ§Ãµes Finais

Este sistema Ã© significativamente mais complexo que um battlemap tradicional, mas tambÃ©m **muito mais imersivo e alinhado com a proposta de IA-driven VRPG**. A complexidade Ã© justificada pelos seguintes motivos:

1. **ExperiÃªncia de Mesa Real**: Jogador fala naturalmente, nÃ£o clica botÃµes
2. **Flexibilidade**: Suporta tanto aventuras oficiais quanto procedurais
3. **Riqueza Sensorial**: Visual + Auditivo = ImersÃ£o 10/10
4. **EscalÃ¡vel**: Arquitetura modular permite iterar incrementalmente

**CrÃ­tico para Sucesso**:
- Backend deve implementar os contratos IPC conforme especificado
- Database de campanhas oficiais deve seguir estrutura de pastas documentada
- Performance optimization Ã© essencial (60 FPS constante)
- Testes visuais contÃ­nuos para validar conversÃ£o isomÃ©trica

**O plano estÃ¡ completo, detalhado e pronto para implementaÃ§Ã£o faseada.**

