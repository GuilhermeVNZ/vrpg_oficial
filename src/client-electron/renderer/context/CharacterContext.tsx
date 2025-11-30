import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { Character } from '../types/Character';
import { createDemoCharacter } from '../data/demoCharacter';

interface CharacterContextType {
    character: Character | null;
    setCharacter: (character: Character | null) => void;
    updateCharacter: (updates: Partial<Character>) => void;
    saveCharacter: () => void;
    loadCharacter: (id: string) => void;
    deleteCharacter: (id: string) => void;
}

const CharacterContext = createContext<CharacterContextType | undefined>(undefined);

const STORAGE_KEY = 'vrpg-characters';

export const CharacterProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [character, setCharacter] = useState<Character | null>(null);

    // Load character from localStorage on mount
    useEffect(() => {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
            try {
                const characters = JSON.parse(stored);
                // Load the first character by default (or implement character selection)
                if (characters.length > 0) {
                    setCharacter(characters[0]);
                } else {
                    // No characters saved, load demo character
                    setCharacter(createDemoCharacter());
                }
            } catch (error) {
                console.error('Failed to load characters from localStorage:', error);
                // On error, load demo character
                setCharacter(createDemoCharacter());
            }
        } else {
            // No stored characters, load demo character
            setCharacter(createDemoCharacter());
        }
    }, []);

    // Save character to localStorage whenever it changes
    useEffect(() => {
        if (!character) return;

        const stored = localStorage.getItem(STORAGE_KEY);
        let characters: Character[] = [];

        if (stored) {
            try {
                characters = JSON.parse(stored);
            } catch (error) {
                console.error('Failed to parse stored characters:', error);
            }
        }

        // Update or add character
        const index = characters.findIndex(c => c.id === character.id);
        if (index >= 0) {
            characters[index] = character;
        } else {
            characters.push(character);
        }

        localStorage.setItem(STORAGE_KEY, JSON.stringify(characters));
    }, [character]);

    const updateCharacter = (updates: Partial<Character>) => {
        if (!character) return;
        setCharacter({ ...character, ...updates });
    };

    const saveCharacter = () => {
        if (!character) return;

        const stored = localStorage.getItem(STORAGE_KEY);
        let characters: Character[] = [];

        if (stored) {
            try {
                characters = JSON.parse(stored);
            } catch (error) {
                console.error('Failed to parse stored characters:', error);
            }
        }

        const index = characters.findIndex(c => c.id === character.id);
        if (index >= 0) {
            characters[index] = character;
        } else {
            characters.push(character);
        }

        localStorage.setItem(STORAGE_KEY, JSON.stringify(characters));
    };

    const loadCharacter = (id: string) => {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (!stored) return;

        try {
            const characters: Character[] = JSON.parse(stored);
            const found = characters.find(c => c.id === id);
            if (found) {
                setCharacter(found);
            }
        } catch (error) {
            console.error('Failed to load character:', error);
        }
    };

    const deleteCharacter = (id: string) => {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (!stored) return;

        try {
            let characters: Character[] = JSON.parse(stored);
            characters = characters.filter(c => c.id !== id);
            localStorage.setItem(STORAGE_KEY, JSON.stringify(characters));

            if (character?.id === id) {
                setCharacter(null);
            }
        } catch (error) {
            console.error('Failed to delete character:', error);
        }
    };

    return (
        <CharacterContext.Provider
            value={{
                character,
                setCharacter,
                updateCharacter,
                saveCharacter,
                loadCharacter,
                deleteCharacter,
            }}
        >
            {children}
        </CharacterContext.Provider>
    );
};

export const useCharacter = () => {
    const context = useContext(CharacterContext);
    if (!context) {
        throw new Error('useCharacter must be used within CharacterProvider');
    }
    return context;
};
