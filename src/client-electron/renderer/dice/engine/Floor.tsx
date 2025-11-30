import React from 'react';
import { usePlane } from '@react-three/cannon';

export const Floor: React.FC = () => {
    const [ref] = usePlane(() => ({
        rotation: [-Math.PI / 2, 0, 0],
        position: [0, 0, 0],
        material: { friction: 0.1, restitution: 0.5 }
    }));

    return (
        <mesh ref={ref as any}>
            <planeGeometry args={[100, 100]} />
            <meshBasicMaterial visible={false} />
        </mesh>
    );
};
