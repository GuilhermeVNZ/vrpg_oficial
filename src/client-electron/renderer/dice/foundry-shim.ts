/**
 * Foundry VTT Shim
 * 
 * This file mocks the Foundry VTT global environment to allow Dice So Nice
 * to run in a standalone Electron app.
 */

// @ts-nocheck - DSN uses many Foundry-specific globals

import { settingsManager, userFlagsManager } from '../store/diceSettings';

// Mock i18n localization
const mockI18n = {
    localize: (key: string) => {
        const translations: Record<string, string> = {
            "DICESONICE.System.Standard": "Standard",
            "DICESONICE.System.SpectrumDice": "Spectrum Dice",
            "DICESONICE.System.FoundryVTT": "Foundry VTT",
            "DICESONICE.System.Dot": "Dot",
            "DICESONICE.System.DotBlack": "Dot Black",
        };
        return translations[key] || key;
    },
    format: (key: string, data?: any) => mockI18n.localize(key)
};

// Enhanced User with real persistence
class MockUser {
    id = 'user1';
    name = 'Player';
    isGM = false;

    getFlag(module: string, key: string) {
        return userFlagsManager.getFlag(module, key);
    }

    async setFlag(module: string, key: string, value: any) {
        await userFlagsManager.setFlag(module, key, value);
        return this;
    }
}

// Mock Users collection
class MockUsers extends Map {
    constructor() {
        super();
        const defaultUser = new MockUser();
        this.set(defaultUser.id, defaultUser);
    }
}

// Mock Canvas with proper PIXI-like ticker
const mockCanvas = {
    app: {
        ticker: {
            _head: {
                next: null,
                destroy() {
                    return null;
                }
            },
            add: function (fn: Function, context?: any) {
                const listener = {
                    fn: fn,
                    context: context,
                    next: this._head.next,
                    destroy() {
                        return this.next;
                    }
                };
                this._head.next = listener;

                // RAF-based ticker simulation
                const tick = () => {
                    fn.call(context, 16); // ~60fps delta
                    requestAnimationFrame(tick);
                };
                tick();
                return this;
            },
            remove: function (fn: Function) {
                let listener = this._head.next;
                let prev = this._head;

                while (listener) {
                    if (listener.fn === fn) {
                        prev.next = listener.next;
                        break;
                    }
                    prev = listener;
                    listener = listener.next;
                }
                return this;
            },
            _cancelIfNeeded() {
                // noop
            }
        },
        renderer: {
            context: {
                extensions: {
                    floatTextureLinear: true
                }
            }
        }
    },
    scene: null,
    mouseInteractionManager: {
        activate() { },
        object: { interactive: true }
    },
    darknessLevel: 0
};

// Mock dice3d configuration with uniforms for shaders
const mockUniforms = {
    globalBloom: { value: 0 },
    iridescenceLookUp: { value: null },
    iridescenceNoise: { value: null },
    boost: { value: 1.0 },
    time: { value: 0 },
    bloomStrength: { value: 1.5 },
    bloomRadius: { value: 0.4 },
    bloomThreshold: { value: 0.85 }
};

// Update time uniform
setInterval(() => {
    mockUniforms.time.value = performance.now() / 1000;
}, 16);

// Mock audio manager
const mockAudio = {
    pending: [],
    playing: new Map(),
    locked: false
};

//  Main game object mock with real settings
const game: any = {
    i18n: mockI18n,
    user: new MockUser(),
    users: new MockUsers(),
    canvas: mockCanvas,
    audio: mockAudio,
    dice3d: {
        DiceFactory: null,
        box: null,
        uniforms: mockUniforms,
        dice3dRenderers: {}
    },
    settings: {
        get: (module: string, key: string) => {
            return settingsManager.get(module, key);
        },
        set: (module: string, key: string, value: any) => {
            return settingsManager.set(module, key, value);
        },
        onChange: (key: string, callback: Function) => {
            settingsManager.onChange(key, callback);
        }
    }
};

// Mock CONFIG with Proxy to handle unknown dice types
const CONFIG: any = {
    Dice: {
        terms: new Proxy({
            Die: class Die {
                static DENOMINATION = "DIE";
                static name = "Die";
                faces = 6;
                getResultLabel(result: any) {
                    return String(result.result);
                }
            },
            Coin: class Coin {
                static DENOMINATION = "COIN";
                static name = "Coin";
                faces = 2;
                getResultLabel(result: any) {
                    return result.result === 1 ? "Heads" : "Tails";
                }
            },
            FateDie: class FateDie {
                static DENOMINATION = "FATE";
                static name = "FateDie";
                faces = 3;
                getResultLabel(result: any) {
                    const labels = ["-", "0", "+"];
                    return labels[result.result - 1] || "0";
                }
            }
        }, {
            get(target: any, prop: string) {
                if (target[prop]) {
                    return target[prop];
                }
                return class GenericDie {
                    static DENOMINATION = prop.toUpperCase();
                    static name = `${prop}`;
                    faces = 6;
                    getResultLabel(result: any) {
                        return String(result.result);
                    }
                };
            }
        })
    }
};

// Mock foundry utils
const foundry: any = {
    utils: {
        mergeObject: (target: any, source: any, options?: any) => {
            return Object.assign({}, target, source);
        },
        duplicate: (obj: any) => JSON.parse(JSON.stringify(obj)),
        isEmpty: (obj: any) => {
            if (obj == null) return true;
            if (Array.isArray(obj) || typeof obj === 'string') return obj.length === 0;
            if (typeof obj === 'object') return Object.keys(obj).length === 0;
            return false;
        }
    },
    dice: {
        terms: CONFIG.Dice.terms
    },
    applications: {
        settings: {
            menus: {
                FontConfig: {
                    getAvailableFonts: () => ["Arial", "Modesto", "Times New Roman"],
                    loadFont: (fontName: string, options: any) => Promise.resolve()
                }
            }
        }
    }
};

// Mock Hooks system
const Hooks: any = {
    _callbacks: new Map<string, Function[]>(),

    on(event: string, fn: Function) {
        if (!this._callbacks.has(event)) {
            this._callbacks.set(event, []);
        }
        this._callbacks.get(event)!.push(fn);
    },

    callAll(event: string, ...args: any[]) {
        const callbacks = this._callbacks.get(event) || [];
        for (const cb of callbacks) {
            try {
                cb(...args);
            } catch (e) {
                console.error(`Hook error in ${event}:`, e);
            }
        }
    }
};

// Expose globals
if (typeof window !== 'undefined') {
    (window as any).game = game;
    (window as any).CONFIG = CONFIG;
    (window as any).foundry = foundry;
    (window as any).Hooks = Hooks;
    (window as any).canvas = mockCanvas;
}

export { game, CONFIG, foundry, Hooks };
