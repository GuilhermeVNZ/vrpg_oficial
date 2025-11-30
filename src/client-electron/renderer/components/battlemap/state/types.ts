export interface GridCoords {
    x: number;
    y: number;
}

export interface CombatParticipant {
    id: string;
    baseType: string;
    instanceNumber: number;
    displayName: string;
    tokenPath: string;
    stats: {
        hp: number;
        maxHp: number;
        ac: number;
        initiative: number;
    };
    position: GridCoords;
    team: 'player' | 'ally' | 'enemy' | 'neutral';
}

export interface MapData {
    source: 'database' | 'generated';
    imagePath?: string;
    gridWidth: number;
    gridHeight: number;
    obstacles?: GridCoords[];
}

export interface CombatState {
    mode: 'exploration' | 'combat';
    combatId?: string;
    mapData?: MapData;
    participants?: CombatParticipant[];
    initiativeOrder?: string[];
    currentTurn?: string;
}
