import React, { createContext, useContext, useState, ReactNode } from 'react';
import { DiceType } from '../dice/DiceConfig';
import { RollResult } from '../dice/RollManager';

export interface Participant {
    id: string;
    name: string;
    portrait: string; // URL
    color: string; // Dice color
    bonus: number;
    rollType: 'advantage' | 'disadvantage' | 'normal';
    diceType: DiceType;
}

export interface CinematicRollRequest {
    title: string; // e.g., "Attack Roll"
    subtitle: string; // e.g., "AC 15"
    participants: Participant[];
    onComplete?: (results: RollResult[]) => void;
    dc?: number;
}

interface CinematicRollContextType {
    rollRequest: CinematicRollRequest | null;
    triggerRoll: (request: CinematicRollRequest) => void;
    clearRoll: () => void;
}

const CinematicRollContext = createContext<CinematicRollContextType | undefined>(undefined);

export const CinematicRollProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [rollRequest, setRollRequest] = useState<CinematicRollRequest | null>(null);

    const triggerRoll = (request: CinematicRollRequest) => {
        setRollRequest(request);
    };

    const clearRoll = () => {
        setRollRequest(null);
    };

    return (
        <CinematicRollContext.Provider value={{ rollRequest, triggerRoll, clearRoll }}>
            {children}
        </CinematicRollContext.Provider>
    );
};

export const useCinematicRoll = () => {
    const context = useContext(CinematicRollContext);
    if (!context) {
        throw new Error('useCinematicRoll must be used within a CinematicRollProvider');
    }
    return context;
};
