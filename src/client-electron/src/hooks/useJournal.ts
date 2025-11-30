/**
 * Hook para gerenciar o estado do Journal
 */

import { useState, useCallback } from 'react';
import { JournalEntry } from '../components/Journal';

export interface UseJournalReturn {
  isOpen: boolean;
  openJournal: () => void;
  closeJournal: () => void;
  toggleJournal: () => void;
}

export const useJournal = (): UseJournalReturn => {
  const [isOpen, setIsOpen] = useState(false);

  const openJournal = useCallback(() => {
    setIsOpen(true);
  }, []);

  const closeJournal = useCallback(() => {
    setIsOpen(false);
  }, []);

  const toggleJournal = useCallback(() => {
    setIsOpen((prev) => !prev);
  }, []);

  return {
    isOpen,
    openJournal,
    closeJournal,
    toggleJournal,
  };
};









