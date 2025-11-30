
import { MeshStandardMaterial, MeshPhysicalMaterial } from 'three';

export const DICE_MATERIALS = {
    plastic: new MeshStandardMaterial({
        color: 0xeeeeee,
        roughness: 0.6,
        metalness: 0,
    }),
    metal: new MeshStandardMaterial({
        color: 0xdddddd,
        roughness: 0.2,
        metalness: 1,
    }),
    wood: new MeshStandardMaterial({
        color: 0x8B4513,
        roughness: 1,
        metalness: 0,
    }),
    glass: new MeshPhysicalMaterial({
        color: 0x88ccff,
        roughness: 0.1,
        metalness: 0,
        transmission: 0.9,
        thickness: 1,
    }),
    gold: new MeshStandardMaterial({
        color: 0xFFD700,
        roughness: 0.3,
        metalness: 1,
    })
};
