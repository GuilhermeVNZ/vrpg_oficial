import { useEffect, useRef, useImperativeHandle, forwardRef } from 'react';
import './foundry-shim'; // Initialize Foundry globals
// @ts-ignore - DSN modules are JavaScript
import { DiceFactory } from '../dice-so-nice/DiceFactory.js';
// @ts-ignore - DSN modules are JavaScript
import { DiceBox } from '../dice-so-nice/DiceBox.js';

export interface DiceSoNiceConfig {
    autoscale?: boolean;
    bumpMapping?: boolean;
    shadows?: boolean;
    shadowQuality?: 'low' | 'medium' | 'high' | 'none';
    antialiasing?: boolean;
    glow?: boolean;
    useHighDPI?: boolean;
}

export interface DiceRollResult {
    total: number;
    rolls: Array<{
        dice: string;
        value: number;
    }>;
}

export interface DiceSoNiceHandle {
    roll: (notation: string) => Promise<DiceRollResult>;
    clear: () => void;
}

interface DiceSoNiceProps {
    config?: DiceSoNiceConfig;
    onRollComplete?: (result: DiceRollResult) => void;
}

const defaultConfig: DiceSoNiceConfig = {
    autoscale: true,
    bumpMapping: false,
    shadows: false,
    shadowQuality: 'none',
    antialiasing: false,
    glow: false,
    useHighDPI: false
};

export const DiceSoNice = forwardRef<DiceSoNiceHandle, DiceSoNiceProps>(
    ({ config = {}, onRollComplete }, ref) => {
        const containerRef = useRef<HTMLDivElement>(null);
        const diceBoxRef = useRef<any>(null);
        const diceFactoryRef = useRef<any>(null);
        const initializedRef = useRef(false);

        useEffect(() => {
            if (!containerRef.current || initializedRef.current) return;

            console.log('[DiceSoNice] Initializing...');
            initializedRef.current = true;

            const mergedConfig = {
                ...defaultConfig,
                ...config,
                boxType: 'board',
                dimensions: {
                    width: window.innerWidth,
                    height: window.innerHeight
                },
                sounds: true,
                soundsVolume: 0.5,
                soundsSurface: 'felt',
                muteSoundSecretRolls: false,
                showExtraDice: false,
                throwingForce: 'medium',
                immersiveDarkness: false,
                scale: 100,
                speed: 1
            };

            const factory = new DiceFactory();
            factory.setQualitySettings(mergedConfig);
            diceFactoryRef.current = factory;

            if ((window as any).game) {
                (window as any).game.dice3d = {
                    ...(window as any).game.dice3d,
                    DiceFactory: factory
                };
            }

            const box = new DiceBox(containerRef.current, factory, mergedConfig);
            diceBoxRef.current = box;

            box.initialize().then(() => {
                console.log('[DiceSoNice] Initialized successfully');
                box.canvas_container.style.pointerEvents = 'none';
            }).catch((error: any) => {
                console.error('[DiceSoNice] Initialization failed:', error);
            });

            return () => {
                console.log('[DiceSoNice] Cleaning up...');
                if (diceBoxRef.current) {
                    diceBoxRef.current.clearAll();
                }
                initializedRef.current = false;
            };
        }, [config]);

        useImperativeHandle(ref, () => ({
            async roll(notation: string): Promise<DiceRollResult> {
                if (!diceBoxRef.current) {
                    throw new Error('DiceBox not initialized');
                }

                console.log('[DiceSoNice] Rolling:', notation);

                const match = notation.match(/(\d*)d(\d+)([+-]\d+)?/i);
                if (!match) {
                    throw new Error(`Invalid dice notation: ${notation}`);
                }

                const count = parseInt(match[1] || '1', 10);
                const faces = parseInt(match[2], 10);
                const modifier = match[3] ? parseInt(match[3], 10) : 0;

                const diceData: any[] = [];
                for (let i = 0; i < count; i++) {
                    diceData.push({
                        result: Math.floor(Math.random() * faces) + 1,
                        type: `d${faces}`
                    });
                }

                try {
                    const { diceAppearanceService } = await import('./DiceAppearanceService');

                    const dsnConfig = diceAppearanceService.buildDsnConfig(diceData[0]?.type || 'd20');

                    const throwData = {
                        dice: diceData.map((d) => ({
                            resultLabel: String(d.result),
                            d20Result: d.result,
                            type: d.type,
                            vectors: {},
                            options: {}
                        })),
                        dsnConfig
                    };

                    console.log('[DiceSoNiceRoll] Calling start_throw with:', JSON.stringify(throwData, null, 2));

                    await new Promise((resolve) => {
                        diceBoxRef.current.start_throw([throwData], () => {
                            resolve(undefined);
                        });
                    });

                    const total = diceData.reduce((sum, d) => sum + d.result, 0) + modifier;
                    const result: DiceRollResult = {
                        total,
                        rolls: diceData.map(d => ({
                            dice: d.type,
                            value: d.result
                        }))
                    };

                    console.log('[DiceSoNice] Roll complete:', result);
                    onRollComplete?.(result);

                    return result;
                } catch (error) {
                    console.error('[DiceSoNice] Roll failed:', error);
                    throw error;
                }
            },

            clear() {
                if (diceBoxRef.current) {
                    diceBoxRef.current.clearAll();
                }
            }
        }));

        return (
            <div
                ref={containerRef}
                style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    width: '100%',
                    height: '100%',
                    pointerEvents: 'none',
                    zIndex: 9999
                }}
            />
        );
    }
);

DiceSoNice.displayName = 'DiceSoNice';
