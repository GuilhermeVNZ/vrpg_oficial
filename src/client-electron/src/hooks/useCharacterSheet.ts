/**
 * Hook para gerenciar o estado da ficha de personagem
 */

import { useState, useCallback } from 'react';
import { CharacterData } from '../components/CharacterSheet';

export interface UseCharacterSheetReturn {
  isOpen: boolean;
  character: CharacterData | null;
  openSheet: (character: CharacterData) => void;
  closeSheet: () => void;
  toggleSheet: (character?: CharacterData) => void;
}

export const useCharacterSheet = (): UseCharacterSheetReturn => {
  const [isOpen, setIsOpen] = useState(false);
  const [character, setCharacter] = useState<CharacterData | null>(null);

  const openSheet = useCallback((char: CharacterData) => {
    setCharacter(char);
    setIsOpen(true);
  }, []);

  const closeSheet = useCallback(() => {
    setIsOpen(false);
    // Mantém o character em memória para animação de saída suave
    setTimeout(() => {
      setCharacter(null);
    }, 300);
  }, []);

  const toggleSheet = useCallback((char?: CharacterData) => {
    if (isOpen) {
      closeSheet();
    } else if (char) {
      openSheet(char);
    }
  }, [isOpen, openSheet, closeSheet]);

  return {
    isOpen,
    character,
    openSheet,
    closeSheet,
    toggleSheet,
  };
};









