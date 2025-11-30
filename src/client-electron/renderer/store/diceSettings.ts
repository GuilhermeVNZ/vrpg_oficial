/**
 * Dice Settings Store
 * 
 * Manages all Dice So Nice settings with localStorage persistence
 */

export interface DiceSettings {
    // Visual Settings
    'dice-so-nice.enabledSimultaneousRolls': boolean;
    'dice-so-nice.diceTextureList': string[];
    'dice-so-nice.sfxVolume': number;
    'dice-so-nice.sfxLine': string;
    'dice-so-nice.animateRollTable': boolean;
    'dice-so-nice.muteSoundSecretRolls': boolean;
    'dice-so-nice.enableFlavorColorset': boolean;
    'dice-so-nice.allowInteractivity': boolean;
    'dice-so-nice.globalAnimationSpeed': string;
    'dice-so-nice.maxDiceNumber': number;
    'dice-so-nice.diceCanBeFlipped': boolean;

    // Quality Settings
    'dice-so-nice.shadows': boolean;
    'dice-so-nice.shadowQuality': 'none' | 'low' | 'medium' | 'high';
    'dice-so-nice.bumpMapping': boolean;
    'dice-so-nice.antialiasing': 'none' | 'msaa' | 'smaa';
    'dice-so-nice.animationSpeed': number;
    'dice-so-nice.throwingForce': 'weak' | 'medium' | 'strong';

    // Advanced
    'dice-so-nice.useHighDPI': boolean;
    'dice-so-nice.glow': boolean;
    'dice-so-nice.immersiveDarkness': boolean;
}

export interface UserAppearance {
    global: {
        colorset: string;
        system: string;
        material: string;
        labelColor: string;
        diceColor: string;
        outlineColor: string;
        edgeColor: string;
    };
    // Per-dice-type overrides (e.g., d20, d6)
    [diceType: string]: any;
}

const DEFAULT_SETTINGS: Partial<DiceSettings> = {
    'dice-so-nice.enabledSimultaneousRolls': true,
    'dice-so-nice.diceTextureList': [],
    'dice-so-nice.sfxVolume': 0.5,
    'dice-so-nice.sfxLine': 'none',
    'dice-so-nice.animateRollTable': true,
    'dice-so-nice.muteSoundSecretRolls': false,
    'dice-so-nice.enableFlavorColorset': true,
    'dice-so-nice.allowInteractivity': false,
    'dice-so-nice.globalAnimationSpeed': '1',
    'dice-so-nice.maxDiceNumber': 20,
    'dice-so-nice.diceCanBeFlipped': true,
    'dice-so-nice.shadows': false,
    'dice-so-nice.shadowQuality': 'none',
    'dice-so-nice.bumpMapping': false,
    'dice-so-nice.antialiasing': 'none',
    'dice-so-nice.animationSpeed': 1,
    'dice-so-nice.throwingForce': 'medium',
    'dice-so-nice.useHighDPI': false,
    'dice-so-nice.glow': false,
    'dice-so-nice.immersiveDarkness': false
};

const DEFAULT_APPEARANCE: UserAppearance = {
    global: {
        colorset: 'custom',
        system: 'standard',
        material: 'plastic',
        labelColor: '#000000',
        diceColor: '#ffffff',
        outlineColor: '#000000',
        edgeColor: '#000000'
    }
};

export class SettingsManager {
    private settings: Map<string, any> = new Map();
    private listeners: Map<string, Function[]> = new Map();
    private storageKey = 'vrpg-dice-settings';

    constructor() {
        this.loadFromStorage();
    }

    get(module: string, key: string): any {
        const fullKey = `${module}.${key}`;
        return this.settings.get(fullKey);
    }

    async set(module: string, key: string, value: any): Promise<any> {
        const fullKey = `${module}.${key}`;
        this.settings.set(fullKey, value);
        this.saveToStorage();
        this.notifyListeners(fullKey, value);
        return value;
    }

    onChange(key: string, callback: Function): void {
        if (!this.listeners.has(key)) {
            this.listeners.set(key, []);
        }
        this.listeners.get(key)!.push(callback);
    }

    private notifyListeners(key: string, value: any): void {
        const callbacks = this.listeners.get(key) || [];
        callbacks.forEach(cb => {
            try {
                cb(value);
            } catch (error) {
                console.error(`[SettingsManager] Error in listener for ${key}:`, error);
            }
        });
    }

    private loadFromStorage(): void {
        try {
            const stored = localStorage.getItem(this.storageKey);
            if (stored) {
                const parsed = JSON.parse(stored);
                Object.entries(parsed).forEach(([key, value]) => {
                    this.settings.set(key, value);
                });
            }

            // Merge with defaults for missing keys
            Object.entries(DEFAULT_SETTINGS).forEach(([key, value]) => {
                if (!this.settings.has(key)) {
                    this.settings.set(key, value);
                }
            });
        } catch (error) {
            console.error('[SettingsManager] Failed to load settings:', error);
            this.loadDefaults();
        }
    }

    private saveToStorage(): void {
        try {
            const obj: Record<string, any> = {};
            this.settings.forEach((value, key) => {
                obj[key] = value;
            });
            localStorage.setItem(this.storageKey, JSON.stringify(obj));
        } catch (error) {
            console.error('[SettingsManager] Failed to save settings:', error);
        }
    }

    private loadDefaults(): void {
        Object.entries(DEFAULT_SETTINGS).forEach(([key, value]) => {
            this.settings.set(key, value);
        });
    }

    reset(): void {
        this.settings.clear();
        this.loadDefaults();
        this.saveToStorage();
    }
}

export class UserFlagsManager {
    private flags: Map<string, any> = new Map();
    private storageKey = 'vrpg-dice-user-flags';

    constructor() {
        this.loadFromStorage();
    }

    getFlag(module: string, key: string): any {
        const fullKey = `${module}.${key}`;

        // Special handling for appearance
        if (fullKey === 'dice-so-nice.appearance') {
            return this.getAppearance();
        }

        if (fullKey === 'dice-so-nice.preserveDrawingBuffer') {
            return this.flags.get(fullKey) ?? false;
        }

        return this.flags.get(fullKey);
    }

    async setFlag(module: string, key: string, value: any): Promise<void> {
        const fullKey = `${module}.${key}`;
        this.flags.set(fullKey, value);
        this.saveToStorage();
    }

    private getAppearance(): UserAppearance {
        const stored = this.flags.get('dice-so-nice.appearance');
        if (stored) {
            return stored;
        }
        return DEFAULT_APPEARANCE;
    }

    private loadFromStorage(): void {
        try {
            const stored = localStorage.getItem(this.storageKey);
            if (stored) {
                const parsed = JSON.parse(stored);
                Object.entries(parsed).forEach(([key, value]) => {
                    this.flags.set(key, value);
                });
            }
        } catch (error) {
            console.error('[UserFlagsManager] Failed to load flags:', error);
        }
    }

    private saveToStorage(): void {
        try {
            const obj: Record<string, any> = {};
            this.flags.forEach((value, key) => {
                obj[key] = value;
            });
            localStorage.setItem(this.storageKey, JSON.stringify(obj));
        } catch (error) {
            console.error('[UserFlagsManager] Failed to save flags:', error);
        }
    }
}

// Singleton instances
export const settingsManager = new SettingsManager();
export const userFlagsManager = new UserFlagsManager();
