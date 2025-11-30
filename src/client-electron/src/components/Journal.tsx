/**
 * Journal Component - Diário de Campanha
 * 
 * Componente de diário completo com glassmorphism, seguindo o Design System
 * do VRPG Client. Layout de duas colunas com lista de entradas e área de leitura.
 */

import React, { useState, useEffect, useMemo } from 'react';
import './Journal.css';

export type JournalEntryType = 'quest' | 'lore' | 'note';

export interface JournalEntry {
  id: string | number;
  title: string;
  type: JournalEntryType;
  date: string;
  icon: string;
  content: string; // HTML ou texto
}

interface JournalProps {
  entries: JournalEntry[];
  isOpen: boolean;
  onClose: () => void;
}

type FilterType = 'all' | JournalEntryType;

export const Journal: React.FC<JournalProps> = ({
  entries,
  isOpen,
  onClose,
}) => {
  const [selectedEntryId, setSelectedEntryId] = useState<string | number | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [activeFilter, setActiveFilter] = useState<FilterType>('all');

  // Fechar com ESC
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = '';
    };
  }, [isOpen, onClose]);

  // Filtrar e buscar entradas
  const filteredEntries = useMemo(() => {
    return entries.filter((entry) => {
      const matchesFilter = activeFilter === 'all' || entry.type === activeFilter;
      const matchesSearch = entry.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           entry.content.toLowerCase().includes(searchTerm.toLowerCase());
      return matchesFilter && matchesSearch;
    });
  }, [entries, activeFilter, searchTerm]);

  // Entrada selecionada
  const selectedEntry = useMemo(() => {
    if (!selectedEntryId) return null;
    return entries.find((e) => e.id === selectedEntryId) || null;
  }, [entries, selectedEntryId]);

  const capitalize = (str: string): string => {
    return str.charAt(0).toUpperCase() + str.slice(1);
  };

  const getTypeLabel = (type: JournalEntryType): string => {
    switch (type) {
      case 'quest':
        return 'Missão';
      case 'lore':
        return 'Lore';
      case 'note':
        return 'Nota';
      default:
        return capitalize(type);
    }
  };

  if (!isOpen) {
    return null;
  }

  return (
    <div
      className="journal-modal-overlay active"
      onClick={(e) => {
        if (e.target === e.currentTarget) {
          onClose();
        }
      }}
      role="dialog"
      aria-modal="true"
      aria-labelledby="journal-title"
    >
      <div className="journal-container glass-panel-ornate">
        <header className="modal-header">
          <h2 id="journal-title" className="modal-title">
            Diário de Campanha
          </h2>
          <button
            className="close-btn"
            onClick={onClose}
            aria-label="Fechar Diário"
            type="button"
          >
            ✕
          </button>
        </header>

        <main className="journal-body">
          <aside className="journal-sidebar glass-sub-panel">
            <div className="search-bar-container">
              <span className="search-icon" aria-hidden="true">🔍</span>
              <input
                type="text"
                id="journal-search"
                className="glass-input"
                placeholder="Buscar entradas..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                aria-label="Buscar no diário"
              />
            </div>

            <div className="journal-filters" role="tablist">
              {(['all', 'quest', 'lore', 'note'] as FilterType[]).map((filter) => (
                <button
                  key={filter}
                  className={`filter-btn ${activeFilter === filter ? 'active' : ''}`}
                  onClick={() => setActiveFilter(filter)}
                  role="tab"
                  aria-selected={activeFilter === filter}
                  type="button"
                >
                  {filter === 'all' ? 'Tudo' : getTypeLabel(filter as JournalEntryType)}
                </button>
              ))}
            </div>

            <ul className="entry-list" id="entry-list-container" role="list">
              {filteredEntries.length === 0 ? (
                <li className="entry-item-empty">
                  <span>Nenhuma entrada encontrada</span>
                </li>
              ) : (
                filteredEntries.map((entry) => (
                  <li
                    key={entry.id}
                    className={`entry-item ${selectedEntryId === entry.id ? 'active' : ''}`}
                    onClick={() => setSelectedEntryId(entry.id)}
                    role="listitem"
                    tabIndex={0}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' || e.key === ' ') {
                        e.preventDefault();
                        setSelectedEntryId(entry.id);
                      }
                    }}
                  >
                    <span className="entry-icon" aria-hidden="true">
                      {entry.icon}
                    </span>
                    <div className="entry-info">
                      <h4 className="entry-title-sm">{entry.title}</h4>
                      <span className="entry-meta">
                        {getTypeLabel(entry.type)} | {entry.date}
                      </span>
                    </div>
                  </li>
                ))
              )}
            </ul>
          </aside>

          <article className="journal-content-area glass-sub-panel" id="reading-pane">
            {!selectedEntry ? (
              <div id="empty-state" className="empty-state-container">
                <div className="empty-icon" aria-hidden="true">📖</div>
                <h3>Selecione uma entrada</h3>
                <p>Escolha uma nota ou missão à esquerda para ler os detalhes.</p>
              </div>
            ) : (
              <div id="entry-content" className="entry-content-visible">
                <header className="entry-header">
                  <div className="entry-badges">
                    <span className={`badge ${selectedEntry.type}`}>
                      {getTypeLabel(selectedEntry.type)}
                    </span>
                    <span className="entry-date">{selectedEntry.date}</span>
                  </div>
                  <h1 className="entry-title-lg title-accent">{selectedEntry.title}</h1>
                </header>
                <div
                  className="entry-body-text scrollable-content"
                  id="reading-body"
                  dangerouslySetInnerHTML={{ __html: selectedEntry.content }}
                />
              </div>
            )}
          </article>
        </main>
      </div>
    </div>
  );
};

export default Journal;









