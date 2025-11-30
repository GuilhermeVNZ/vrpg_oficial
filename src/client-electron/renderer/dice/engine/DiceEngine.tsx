import React, { forwardRef, useImperativeHandle, useState, useEffect, useRef } from 'react';
import { Canvas, useThree } from '@react-three/fiber';
import { Physics, usePlane } from '@react-three/cannon';
import { Environment } from '@react-three/drei';
import { Floor } from './Floor';
import { Dice, DiceHandle } from './Dice';

export interface DiceEngineHandle {
    roll: (notation: string, throwVector?: { x: number, y: number }) => Promise<number>;
    clear: () => void;
}

interface DiceEngineProps {
    onRollComplete?: (total: number) => void;
}

interface DiceState {
    id: string;
    type: string;
    position: [number, number, number];
    initialImpulse?: [number, number, number];
    initialTorque?: [number, number, number];
    restitution?: number;
}

// Component to create invisible walls around the screen edges
const Walls = () => {
    const { viewport, camera } = useThree();

    // Ensure camera looks at center
    useEffect(() => {
        camera.lookAt(0, 0, 0);
    }, [camera]);

    const width = viewport.width;
    const height = viewport.height;

    // Margin to account for dice radius (scale 0.75 * approx radius 1.5 ~ 1.125)
    // Adding a bit more for safety
    const WALL_MARGIN = 2.0;

    // Left Wall
    usePlane(() => ({
        position: [-(width / 2) + WALL_MARGIN, 0, 0],
        rotation: [0, Math.PI / 2, 0],
        material: { friction: 0.1, restitution: 0.5 }
    }));

    // Right Wall
    usePlane(() => ({
        position: [(width / 2) - WALL_MARGIN, 0, 0],
        rotation: [0, -Math.PI / 2, 0],
        material: { friction: 0.1, restitution: 0.5 }
    }));

    // Top Wall (far)
    usePlane(() => ({
        position: [0, 0, -(height / 2) + WALL_MARGIN],
        rotation: [0, 0, 0],
        material: { friction: 0.1, restitution: 0.5 }
    }));

    // Bottom Wall (near)
    usePlane(() => ({
        position: [0, 0, (height / 2) - WALL_MARGIN],
        rotation: [0, Math.PI, 0],
        material: { friction: 0.1, restitution: 0.5 }
    }));

    return null; // Walls are invisible
};

export const DiceEngine = forwardRef<DiceEngineHandle, DiceEngineProps>(({ onRollComplete }, ref) => {
    const [dice, setDice] = useState<DiceState[]>([]);
    const diceRefs = useRef<Map<string, DiceHandle>>(new Map());

    useImperativeHandle(ref, () => ({
        roll: async (notation: string, throwVector?: { x: number, y: number }) => {
            console.log('[DiceEngine] Rolling:', notation, 'throwVector:', throwVector);

            const match = notation.match(/(\d*)d(\d+)/);
            if (!match || !match[1] || !match[2]) {
                console.error('Invalid notation:', notation);
                return 0;
            }

            const count = parseInt(match[1]) || 1;
            const sides = parseInt(match[2]);
            const type = `d${sides}`;

            setDice([]);
            diceRefs.current.clear();

            // Calculate impulse and torque from throwVector if provided
            let initialImpulse: [number, number, number] | undefined;
            let initialTorque: [number, number, number] | undefined;
            let restitution = 0.5; // Default bounciness

            if (throwVector) {
                // Map 2D drag to 3D throw
                // X drag -> World X movement
                // Y drag -> World Z movement (depth)
                const magnitude = Math.sqrt(throwVector.x ** 2 + throwVector.y ** 2);
                const force = Math.min(magnitude * 0.15, 40); // Reduced force scaling

                // Normalize direction
                const dirX = throwVector.x / (magnitude || 1);
                const dirY = throwVector.y / (magnitude || 1);

                initialImpulse = [
                    dirX * force,      // Horizontal X
                    5,                 // Fixed low upward arc to keep dice visible
                    -dirY * force      // Depth (negative Y drag = forward)
                ];

                initialTorque = [
                    (Math.random() - 0.5) * force * 0.6,
                    (Math.random() - 0.5) * force * 0.6,
                    (Math.random() - 0.5) * force * 0.6
                ];

                // Dynamic restitution based on drag magnitude
                // Map drag strength to bounciness (0.3 to 0.9)
                restitution = Math.min(0.3 + (magnitude / 600), 0.9);
            }

            // Add new dice
            await new Promise<void>(resolve => {
                setTimeout(() => {
                    const newDice: DiceState[] = [];
                    for (let i = 0; i < count; i++) {
                        newDice.push({
                            id: Math.random().toString(36).substring(7),
                            type: type,
                            position: [
                                (Math.random() - 0.5) * 2,
                                10 + i * 2,
                                (Math.random() - 0.5) * 2
                            ],
                            initialImpulse,
                            initialTorque,
                            restitution
                        });
                    }
                    setDice(newDice);
                    resolve();
                }, 50);
            });

            // Wait for animation to settle
            // Ideally we check velocities, but fixed time is safer for now
            await new Promise(resolve => setTimeout(resolve, 3000));

            // Calculate total from physics results
            let total = 0;
            diceRefs.current.forEach((diceHandle) => {
                const value = diceHandle.getValue();
                console.log(`[DiceEngine] Die result: ${value}`);
                total += value;
            });

            // Fallback if something went wrong (e.g. no dice refs)
            if (total === 0 && count > 0) {
                console.warn('[DiceEngine] Failed to get physics results, falling back to random');
                for (let i = 0; i < count; i++) {
                    total += Math.floor(Math.random() * sides) + 1;
                }
            }

            if (onRollComplete) onRollComplete(total);
            return total;
        },
        clear: () => {
            setDice([]);
            diceRefs.current.clear();
        }
    }));

    return (
        <div style={{ width: '100%', height: '100%', position: 'absolute', top: 0, left: 0, pointerEvents: 'none', zIndex: 50 }}>
            <Canvas camera={{ position: [0, 15, 0], fov: 45 }} style={{ pointerEvents: 'none' }}>
                {/* Lighting & Environment */}
                <ambientLight intensity={0.125} />
                <directionalLight position={[5, 10, 5]} intensity={0.25} />
                <hemisphereLight args={[0xffffff, 0x444444, 0.125]} />
                <Environment preset="studio" />

                <Physics gravity={[0, -30, 0]} defaultContactMaterial={{ restitution: 0.5, friction: 0.1 }}>
                    <Floor />
                    <Walls />
                    {dice.map(d => (
                        <Dice
                            key={d.id}
                            position={d.position}
                            type={d.type}
                            initialImpulse={d.initialImpulse}
                            initialTorque={d.initialTorque}
                            restitution={d.restitution}
                            ref={(el) => {
                                if (el) diceRefs.current.set(d.id, el);
                                else diceRefs.current.delete(d.id);
                            }}
                        />
                    ))}
                </Physics>
            </Canvas>
        </div>
    );
});

DiceEngine.displayName = 'DiceEngine';
