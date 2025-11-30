# Journal Component - Documentação

## Visão Geral

O **Journal** (Diário de Campanha) é um componente completo para visualizar e gerenciar entradas de diário (missões, lore, notas) com glassmorphism, seguindo o Design System do VRPG Client. Ele fornece uma interface elegante de duas colunas com busca e filtros.

## Características

- ✅ **Glassmorphism Completo**: Efeito de vidro fosco com blur e transparência
- ✅ **Layout de Duas Colunas**: Lista de entradas + área de leitura
- ✅ **Busca e Filtros**: Busca em tempo real e filtros por tipo
- ✅ **Tipos de Entrada**: Missões, Lore, Notas
- ✅ **Integrado**: Usa Design Tokens do VRPG Client
- ✅ **Acessível**: Suporta navegação por teclado e ARIA labels

## Estrutura de Arquivos

```
src/client-electron/src/
├── components/
│   ├── Journal.tsx    # Componente React principal
│   └── Journal.css    # Estilos com glassmorphism
└── hooks/
    └── useJournal.ts  # Hook para gerenciar estado
```

## Uso Básico

### Exemplo 1: Uso Simples com Hook

```tsx
import React from 'react';
import { Journal, JournalEntry } from './components/Journal';
import { useJournal } from './hooks/useJournal';

function App() {
  const journal = useJournal();

  const exampleEntries: JournalEntry[] = [
    {
      id: 1,
      title: 'O Chamado do Lorde Neverember',
      type: 'quest',
      date: '12/10/1492 CV',
      icon: '📜',
      content: `
        <p>Recebemos uma convocação urgente para comparecer ao Salão da Justiça em Neverwinter.</p>
        <p><strong>Objetivo:</strong> Falar com Lorde Neverember e obter detalhes sobre a missão.</p>
      `,
    },
    {
      id: 2,
      title: 'A História de Netheril (Fragmento)',
      type: 'lore',
      date: '10/10/1492 CV',
      icon: '🏛️',
      content: `
        <p>Anotações encontradas em uma biblioteca abandonada:</p>
        <p>"...e assim caíram as cidades flutuantes, quando Karsus em sua húbris tentou roubar o manto da divindade de Mystryl..."</p>
      `,
    },
    {
      id: 3,
      title: 'Ingredientes para Poção de Cura',
      type: 'note',
      date: '05/10/1492 CV',
      icon: '🌿',
      content: `
        <p>Lembrar de coletar:</p>
        <ul>
          <li>3x Raízes de Musgo-Vermelho</li>
          <li>1x Frasco de Água Benta</li>
          <li>Pó de prata (cerca de 10 po)</li>
        </ul>
      `,
    },
  ];

  return (
    <div>
      <button onClick={journal.openJournal}>
        Abrir Diário
      </button>
      
      <Journal
        entries={exampleEntries}
        isOpen={journal.isOpen}
        onClose={journal.closeJournal}
      />
    </div>
  );
}
```

### Exemplo 2: Integração com Game Engine

```tsx
import { useEffect, useState } from 'react';
import { Journal, JournalEntry } from './components/Journal';
import { useJournal } from './hooks/useJournal';

function GameInterface() {
  const journal = useJournal();
  const [entries, setEntries] = useState<JournalEntry[]>([]);

  useEffect(() => {
    // Carregar entradas do Game Engine via IPC/WebSocket
    const loadEntries = async () => {
      // Exemplo com IPC do Electron
      const data = await window.electron?.ipcRenderer.invoke('journal:getEntries');
      if (data) {
        setEntries(data);
      }
    };

    loadEntries();

    // Escutar novas entradas
    const handleNewEntry = (entry: JournalEntry) => {
      setEntries((prev) => [...prev, entry]);
    };

    window.electron?.ipcRenderer.on('journal:newEntry', (_, entry) => {
      handleNewEntry(entry);
    });

    return () => {
      // Cleanup listeners
    };
  }, []);

  return (
    <>
      {/* Seu conteúdo do jogo aqui */}
      
      <Journal
        entries={entries}
        isOpen={journal.isOpen}
        onClose={journal.closeJournal}
      />
    </>
  );
}
```

## Interface JournalEntry

```typescript
interface JournalEntry {
  id: string | number;        // ID único da entrada
  title: string;               // Título da entrada
  type: 'quest' | 'lore' | 'note'; // Tipo de entrada
  date: string;                // Data da entrada (formato livre)
  icon: string;                // Emoji ou ícone
  content: string;             // Conteúdo HTML ou texto
}
```

## Props do Componente

### `Journal`

| Prop | Tipo | Padrão | Descrição |
|------|------|--------|-----------|
| `entries` | `JournalEntry[]` | **obrigatório** | Lista de entradas do diário |
| `isOpen` | `boolean` | **obrigatório** | Controla visibilidade |
| `onClose` | `() => void` | **obrigatório** | Callback quando fecha |

## Hook `useJournal`

O hook fornece métodos convenientes para controlar o diário:

```typescript
const {
  isOpen,        // Estado de abertura
  openJournal,   // Abre o diário
  closeJournal,  // Fecha o diário
  toggleJournal, // Alterna estado
} = useJournal();
```

## Funcionalidades

### Busca
- Busca em tempo real no título e conteúdo das entradas
- Case-insensitive
- Atualiza a lista enquanto digita

### Filtros
- **Tudo**: Mostra todas as entradas
- **Missões**: Apenas entradas do tipo `quest`
- **Lore**: Apenas entradas do tipo `lore`
- **Notas**: Apenas entradas do tipo `note`

### Seleção
- Clique em uma entrada para visualizar
- Entrada selecionada destacada em azul arcano
- Área de leitura mostra o conteúdo completo
- Estado vazio quando nada está selecionado

## Tipos de Entrada

### Quest (Missão)
- Badge dourado
- Ícone: 📜, ⚔️, 🗺️
- Usado para missões e objetivos

### Lore (Lore)
- Badge azul arcano
- Ícone: 🏛️, 📚, 🔮
- Usado para informações de mundo e história

### Note (Nota)
- Badge cinza translúcido
- Ícone: 📝, 🌿, 💡
- Usado para anotações pessoais e lembretes

## Customização

### Cores

As cores são definidas usando Design Tokens do VRPG:

```css
:root {
  --accent-gold: var(--vrpg-color-gold-primary);
  --accent-blue: var(--vrpg-color-arcane-blue);
  --glass-bg: rgba(15, 18, 25, 0.85);
}
```

### Layout

O layout usa CSS Grid e pode ser ajustado editando:

- `.journal-body`: Grid principal (sidebar + conteúdo)
- `.journal-sidebar`: Largura da sidebar (padrão: 350px)
- `.journal-content-area`: Área de leitura

## Acessibilidade

- ✅ **ARIA Labels**: `role="dialog"`, `aria-modal`, `aria-labelledby`
- ✅ **Keyboard Navigation**: ESC fecha, Tab navega, Enter/Space seleciona
- ✅ **Focus States**: Indicadores visuais de foco
- ✅ **Reduced Motion**: Respeita `prefers-reduced-motion`

## Performance

- Busca e filtros usam `useMemo` para otimização
- Scrollbars customizadas para melhor UX
- Animações suaves com `transform` e `opacity`
- Componente é leve e não impacta performance geral

## Responsividade

- **Desktop**: Layout de duas colunas completo
- **Tablet**: Layout adapta mantendo duas colunas
- **Mobile**: Layout em coluna única, sidebar limitada a 40vh

## Integração com Orchestrator

Para integrar com o sistema de memória do VRPG Client:

```tsx
// No componente que se comunica com o Orchestrator
import { useEffect, useState } from 'react';
import { Journal, JournalEntry } from './components/Journal';
import { useJournal } from './hooks/useJournal';

function GameInterface() {
  const journal = useJournal();
  const [entries, setEntries] = useState<JournalEntry[]>([]);

  useEffect(() => {
    // Quando o Orchestrator cria uma nova entrada de diário
    const handleJournalEntry = (entry: JournalEntry) => {
      setEntries((prev) => [...prev, entry]);
    };

    // Exemplo com IPC do Electron
    window.electron?.ipcRenderer.on('orchestrator:journalEntry', (_, entry) => {
      handleJournalEntry(entry);
    });

    return () => {
      // Cleanup listeners
    };
  }, []);

  return (
    <Journal
      entries={entries}
      isOpen={journal.isOpen}
      onClose={journal.closeJournal}
    />
  );
}
```

## Exemplo de Dados

```typescript
const exampleEntries: JournalEntry[] = [
  {
    id: 1,
    title: 'O Mistério das Ruínas de Netheril',
    type: 'quest',
    date: '12 de Outubro, 1492 CV',
    icon: '📜',
    content: `
      <p>Recebemos uma convocação urgente para comparecer ao Salão da Justiça em Neverwinter.</p>
      <p><strong>Objetivo:</strong> Investigar as ruínas de Netheril e descobrir a origem das perturbações mágicas.</p>
      <p><strong>Recompensa:</strong> 500 PO e acesso à biblioteca arcana.</p>
    `,
  },
  {
    id: 2,
    title: 'A História de Netheril (Fragmento)',
    type: 'lore',
    date: '10/10/1492 CV',
    icon: '🏛️',
    content: `
      <p>Anotações encontradas em uma biblioteca abandonada:</p>
      <p>"...e assim caíram as cidades flutuantes, quando Karsus em sua húbris tentou roubar o manto da divindade de Mystryl. A trama mágica se partiu, e o império que tocava os céus despencou para a terra em fogo e ruína..."</p>
      <p>Estes fragmentos sugerem que artefatos daquela era ainda possuem um poder instável e perigoso.</p>
    `,
  },
];
```

---

**Nota**: Este componente foi projetado para funcionar perfeitamente com o Design System do VRPG Client e está pronto para integração com o Orchestrator e sistema de memória do jogo.









