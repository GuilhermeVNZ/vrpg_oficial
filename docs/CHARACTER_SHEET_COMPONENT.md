# Character Sheet Component - Documentação

## Visão Geral

O **Character Sheet** é um componente completo de ficha de personagem D&D 5e com glassmorphism, seguindo o Design System do VRPG Client. Ele fornece uma interface elegante e organizada para visualizar e gerenciar todas as informações de um personagem.

## Características

- ✅ **Glassmorphism Completo**: Efeito de vidro fosco com blur e transparência
- ✅ **Layout Responsivo**: CSS Grid que se adapta a diferentes tamanhos de tela
- ✅ **Sistema de Abas**: Navegação suave entre diferentes seções
- ✅ **Dados Completos**: Suporta todos os campos de uma ficha D&D 5e
- ✅ **Integrado**: Usa Design Tokens do VRPG Client
- ✅ **Acessível**: Suporta navegação por teclado e ARIA labels

## Estrutura de Arquivos

```
src/client-electron/src/
├── components/
│   ├── CharacterSheet.tsx    # Componente React principal
│   └── CharacterSheet.css    # Estilos com glassmorphism
└── hooks/
    └── useCharacterSheet.ts   # Hook para gerenciar estado
```

## Uso Básico

### Exemplo 1: Uso Simples com Hook

```tsx
import React from 'react';
import { CharacterSheet } from './components/CharacterSheet';
import { useCharacterSheet } from './hooks/useCharacterSheet';

function App() {
  const characterSheet = useCharacterSheet();

  const exampleCharacter = {
    name: 'Valeros Arcane',
    level: 5,
    class: 'Mago',
    subclass: 'Evocação',
    race: 'Alto Elfo',
    background: 'Sábio',
    abilityScores: {
      strength: 10,
      dexterity: 16,
      constitution: 14,
      intelligence: 18,
      wisdom: 12,
      charisma: 8,
    },
    hp: { current: 32, max: 32 },
    ac: 15,
    initiative: 3,
    speed: 9,
    skills: [
      { name: 'Arcanismo', ability: 'intelligence', proficient: true, modifier: 7 },
      { name: 'Atletismo', ability: 'strength', proficient: false, modifier: 0 },
      { name: 'Intuição', ability: 'wisdom', proficient: true, modifier: 4 },
    ],
    actions: [
      {
        name: 'Adaga',
        bonus: 6,
        damage: '1d4+3',
        range: 'Corpo a corpo ou Arremesso (6/18m)',
        type: 'Perfurante',
      },
      {
        name: 'Raio de Fogo (Truque)',
        bonus: 7,
        damage: '2d10',
        range: 'Alcance 36m',
        type: 'Ígneo',
      },
    ],
    spells: {
      spellSaveDC: 15,
      spellAttackBonus: 7,
      slots: {
        1: { used: 0, total: 4 },
      },
      known: {
        0: ['Mãos Mágicas', 'Raio de Fogo', 'Prestidigitação'],
        1: ['Escudo Arcano (Shield)', 'Mísseis Mágicos', 'Detectar Magia (Ritual)'],
      },
    },
    inventory: {
      currency: { pp: 0, gp: 120, ep: 0, sp: 45, cp: 10 },
      items: [
        { name: 'Mochila de Aventureiro', quantity: 1 },
        { name: 'Corda de Cânhamo (15m)', quantity: 1 },
        { name: 'Rações de Viagem', quantity: 10 },
        { name: 'Grimório', quantity: 1 },
      ],
    },
    features: [
      {
        name: 'Visão no Escuro (Elfo)',
        source: 'Raça',
        description: 'Você enxerga na penumbra a até 18 metros como se fosse luz plena, e no escuro como se fosse penumbra.',
      },
      {
        name: 'Recuperação Arcana (Mago)',
        source: 'Classe',
        description: 'Uma vez por dia, durante um descanso curto, você pode recuperar slots de magia gastos.',
      },
    ],
  };

  return (
    <div>
      <button onClick={() => characterSheet.openSheet(exampleCharacter)}>
        Abrir Ficha
      </button>
      
      {characterSheet.character && (
        <CharacterSheet
          character={characterSheet.character}
          isOpen={characterSheet.isOpen}
          onClose={characterSheet.closeSheet}
        />
      )}
    </div>
  );
}
```

### Exemplo 2: Integração com Game Engine

```tsx
import { useEffect } from 'react';
import { CharacterSheet } from './components/CharacterSheet';
import { useCharacterSheet } from './hooks/useCharacterSheet';

function GameInterface() {
  const characterSheet = useCharacterSheet();

  useEffect(() => {
    // Escutar eventos do Game Engine via IPC/WebSocket
    const handleOpenCharacterSheet = (characterData: CharacterData) => {
      characterSheet.openSheet(characterData);
    };

    // Exemplo com IPC do Electron
    window.electron?.ipcRenderer.on('character:open', (_, data) => {
      handleOpenCharacterSheet(data);
    });

    return () => {
      // Cleanup listeners
    };
  }, [characterSheet]);

  return (
    <>
      {/* Seu conteúdo do jogo aqui */}
      
      {characterSheet.character && (
        <CharacterSheet
          character={characterSheet.character}
          isOpen={characterSheet.isOpen}
          onClose={characterSheet.closeSheet}
        />
      )}
    </>
  );
}
```

## Interface CharacterData

```typescript
interface CharacterData {
  name: string;
  level: number;
  class: string;
  subclass?: string;
  race: string;
  background: string;
  abilityScores: {
    strength: number;
    dexterity: number;
    constitution: number;
    intelligence: number;
    wisdom: number;
    charisma: number;
  };
  hp: {
    current: number;
    max: number;
  };
  ac: number;
  initiative: number;
  speed: number;
  skills: Array<{
    name: string;
    ability: string;
    proficient: boolean;
    modifier: number;
  }>;
  actions: Array<{
    name: string;
    bonus: number;
    damage?: string;
    range?: string;
    type: string;
  }>;
  spells: {
    spellSaveDC: number;
    spellAttackBonus: number;
    slots: {
      [level: number]: { used: number; total: number };
    };
    known: {
      [level: number]: string[];
    };
  };
  inventory: {
    currency: {
      pp: number;
      gp: number;
      ep: number;
      sp: number;
      cp: number;
    };
    items: Array<{
      name: string;
      quantity: number;
    }>;
  };
  features: Array<{
    name: string;
    source: string;
    description: string;
  }>;
}
```

## Props do Componente

### `CharacterSheet`

| Prop | Tipo | Padrão | Descrição |
|------|------|--------|-----------|
| `character` | `CharacterData` | **obrigatório** | Dados do personagem |
| `isOpen` | `boolean` | **obrigatório** | Controla visibilidade |
| `onClose` | `() => void` | **obrigatório** | Callback quando fecha |

## Hook `useCharacterSheet`

O hook fornece métodos convenientes para controlar a ficha:

```typescript
const {
  isOpen,        // Estado de abertura
  character,     // Dados do personagem atual
  openSheet,     // Abre a ficha com um personagem
  closeSheet,    // Fecha a ficha
  toggleSheet,   // Alterna estado (aceita character opcional)
} = useCharacterSheet();
```

## Seções da Ficha

### 1. Cabeçalho
- **Identidade**: Nome, nível, classe, raça, background
- **Vitals**: CA, Iniciativa, Deslocamento, HP (com barra visual)

### 2. Sidebar (Atributos)
- 6 Atributos principais (FOR, DES, CON, INT, SAB, CAR)
- Mostra modificador e valor base
- Destaque visual para modificadores positivos

### 3. Conteúdo Principal (Abas)

#### Perícias
- Lista de perícias com indicador de proficiência
- Modificadores calculados automaticamente

#### Ações
- Cards de ações e ataques
- Bônus de acerto, dano e alcance

#### Magias
- CD de resistência e bônus de ataque
- Slots por nível
- Lista de magias conhecidas

#### Inventário
- Moedas (PP, PO, PE, PP, PC)
- Lista de itens com quantidades

#### Talentos & Traits
- Características de classe e raça
- Descrições completas

## Customização

### Cores

As cores são definidas usando Design Tokens do VRPG:

```css
:root {
  --accent-gold: var(--vrpg-color-gold-primary);
  --accent-blue: var(--vrpg-color-arcane-blue);
  --sheet-bg-glass: rgba(15, 18, 25, 0.85);
}
```

### Layout

O layout usa CSS Grid e pode ser ajustado editando:

- `.sheet-body`: Grid principal (sidebar + conteúdo)
- `.sheet-header`: Grid do cabeçalho
- `.tabs-nav`: Navegação de abas

## Acessibilidade

- ✅ **ARIA Labels**: `role="dialog"`, `aria-modal`, `aria-labelledby`
- ✅ **Keyboard Navigation**: ESC fecha a ficha, Tab navega entre elementos
- ✅ **Focus States**: Indicadores visuais de foco
- ✅ **Reduced Motion**: Respeita `prefers-reduced-motion`

## Performance

- Animações usam `transform` e `opacity` (GPU-accelerated)
- `backdrop-filter` otimizado para containers fixos
- Scrollbars customizadas para melhor UX
- Componente é leve e não impacta performance geral

## Responsividade

- **Desktop**: Layout completo com sidebar e abas
- **Tablet**: Sidebar vira linha horizontal
- **Mobile**: Layout em coluna única, abas roláveis

---

**Nota**: Este componente foi projetado para funcionar perfeitamente com o Design System do VRPG Client e está pronto para integração com o Game Engine e sistema de dados do jogo.









