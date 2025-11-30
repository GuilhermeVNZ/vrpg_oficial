import { useEffect, useRef, useImperativeHandle, forwardRef } from 'react';
import DiceBox from '@3d-dice/dice-box';

export interface DiceRollResult {
    total: number;
    rolls: Array<{
        dice: string;
        value: number;
    }>;
}

export interface DiceBoxHandle {
    roll: (notation: string) => Promise<DiceRollResult>;
    clear: () => void;
}

interface DiceBoxWrapperProps {
    onRollComplete?: (result: DiceRollResult) => void;
}

export const DiceBoxWrapper = forwardRef<DiceBoxHandle, DiceBoxWrapperProps>(
    ({ onRollComplete }, ref) => {
        const containerRef = useRef<HTMLDivElement>(null);
        const diceBoxRef = useRef<any | null>(null);
        const initializedRef = useRef(false);

        useEffect(() => {
            if (!containerRef.current || initializedRef.current) return;

            console.log('[DiceBox] Initializing...');
            initializedRef.current = true;

            // Create canvas element with explicit dimensions
            const canvas = document.createElement('canvas');
            canvas.id = 'dice-canvas';
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            canvas.style.position = 'absolute';
            canvas.style.top = '0';
            canvas.style.left = '0';
            canvas.style.width = '100%';
            canvas.style.height = '100%';
            canvas.style.pointerEvents = 'none';
            canvas.style.zIndex = '9998'; // Below UI but visible
            containerRef.current.appendChild(canvas);


            // Initialize DiceBox
            // Use relative path - DiceBox will resolve it correctly
            const assetPath = '/assets/';

            // Minimal configuration to reduce potential conflicts
            const diceBox = new DiceBox({
                container: '#dice-canvas',
                assetPath,
                theme: 'default',
                themeColor: '#D4AF37',
                scale: 5, // Standard scale
                offscreen: false, // Ensure onscreen rendering
            });

            console.log('[DiceBox] Instance created with minimal config');

            // Add event listeners to track theme loading
            diceBox.onThemeConfigLoaded = (themeData: any) => {
                console.log('[DiceBox] Theme config loaded:', themeData);
            };

            diceBox.onThemeLoaded = (themeData: any) => {
                console.log('[DiceBox] Theme loaded successfully:', themeData);
            };

            diceBox.onMeshLoaded = (meshData: any) => {
                console.log('[DiceBox] Mesh loaded:', meshData);
            };

            diceBox.init().then(async () => {
                console.log('[DiceBox] Initialized successfully');
                // Give physics engine a moment to settle
                await new Promise(resolve => setTimeout(resolve, 500));
                diceBoxRef.current = diceBox;
                console.log('[DiceBox] Ready to roll');
            }).catch((error: any) => {
                console.error('[DiceBox] Initialization failed:', error);
                console.error('[DiceBox] Error stack:', error.stack);
            });

            return () => {
                console.log('[DiceBox] Cleaning up...');
                if (diceBoxRef.current) {
                    diceBoxRef.current.clear();
                }
                if (canvas.parentNode) {
                    canvas.parentNode.removeChild(canvas);
                }
                initializedRef.current = false;
            };
        }, []);

        useImperativeHandle(ref, () => ({
            async roll(notation: string): Promise<DiceRollResult> {
                // Wait for initialization if needed
                if (!diceBoxRef.current) {
                    console.log('[DiceBox] Waiting for initialization...');
                    let attempts = 0;
                    while (!diceBoxRef.current && attempts < 20) {
                        await new Promise(resolve => setTimeout(resolve, 100));
                        attempts++;
                    }
                    if (!diceBoxRef.current) {
                        throw new Error('DiceBox failed to initialize');
                    }
                }

                console.log('[DiceBox] Rolling:', notation);

                try {
                    const rollResult = await diceBoxRef.current.roll(notation);

                    console.log('[DiceBox] Raw result:', rollResult);

                    // Parse the result - dice-box returns array of individual dice
                    const total = rollResult.reduce((sum: number, die: any) => sum + die.value, 0);

                    const result: DiceRollResult = {
                        total,
                        rolls: rollResult.map((die: any) => ({
                            dice: `d${die.sides || die.qty}`,
                            value: die.value
                        }))
                    };

                    console.log('[DiceBox] Processed result:', result);
                    onRollComplete?.(result);

                    return result;
                } catch (error) {
                    console.error('[DiceBox] Roll failed:', error);
                    throw error;
                }
            },

            clear() {
                if (diceBoxRef.current) {
                    diceBoxRef.current.clear();
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

DiceBoxWrapper.displayName = 'DiceBoxWrapper';
