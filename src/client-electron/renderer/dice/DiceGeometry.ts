// src/client-electron/renderer/dice/DiceGeometry.ts
import * as THREE from 'three';
import * as CANNON from 'cannon-es';
import { DiceType } from './DiceConfig';

/**
 * Returns a Three.js geometry for the given dice type.
 * For unsupported types a simple box geometry is returned as a fallback.
 */
export function createGeometry(type: DiceType): THREE.BufferGeometry {
    switch (type) {
        case 'd4':
            // Tetrahedron (regular)
            return new THREE.TetrahedronGeometry(1, 0);
        case 'd6':
            return new THREE.BoxGeometry(1, 1, 1);
        case 'd8':
            return new THREE.OctahedronGeometry(1, 0);
        case 'd10':
            // Pentagonal trapezohedron – approximated with custom vertices
            // For brevity we reuse an icosahedron and will map faces later.
            return new THREE.IcosahedronGeometry(1, 0);
        case 'd12':
            return new THREE.DodecahedronGeometry(1, 0);
        case 'd20':
            return new THREE.IcosahedronGeometry(1, 0);
        case 'd100':
            // d100 is two d10s; we will use a d10 geometry for each.
            return new THREE.IcosahedronGeometry(1, 0);
        default:
            return new THREE.BoxGeometry(1, 1, 1);
    }
}

/**
 * Converts a Three.js BufferGeometry into a Cannon‑es ConvexPolyhedron.
 * This mirrors the approach used by Dice So Nice: extract unique vertices
 * and faces, then build the ConvexPolyhedron.
 */
export function createConvexPolyhedron(geometry: THREE.BufferGeometry): CANNON.ConvexPolyhedron {
    const posAttr = geometry.getAttribute('position');
    const vertices: CANNON.Vec3[] = [];
    const vertexMap = new Map<string, number>();

    // Extract unique vertices
    for (let i = 0; i < posAttr.count; i++) {
        const x = posAttr.getX(i);
        const y = posAttr.getY(i);
        const z = posAttr.getZ(i);
        const key = `${x.toFixed(6)},${y.toFixed(6)},${z.toFixed(6)}`;
        if (!vertexMap.has(key)) {
            vertexMap.set(key, vertices.length);
            vertices.push(new CANNON.Vec3(x, y, z));
        }
    }

    // Build face index arrays
    const index = geometry.getIndex();
    const faces: number[][] = [];
    if (!index) {
        // Non‑indexed geometry – iterate in triples
        for (let i = 0; i < posAttr.count; i += 3) {
            const aKey = `${posAttr.getX(i).toFixed(6)},${posAttr.getY(i).toFixed(6)},${posAttr.getZ(i).toFixed(6)}`;
            const bKey = `${posAttr.getX(i + 1).toFixed(6)},${posAttr.getY(i + 1).toFixed(6)},${posAttr.getZ(i + 1).toFixed(6)}`;
            const cKey = `${posAttr.getX(i + 2).toFixed(6)},${posAttr.getY(i + 2).toFixed(6)},${posAttr.getZ(i + 2).toFixed(6)}`;
            faces.push([
                vertexMap.get(aKey)!,
                vertexMap.get(bKey)!,
                vertexMap.get(cKey)!
            ]);
        }
    } else {
        for (let i = 0; i < index.count; i += 3) {
            const a = index.getX(i);
            const b = index.getX(i + 1);
            const c = index.getX(i + 2);
            const aKey = `${posAttr.getX(a).toFixed(6)},${posAttr.getY(a).toFixed(6)},${posAttr.getZ(a).toFixed(6)}`;
            const bKey = `${posAttr.getX(b).toFixed(6)},${posAttr.getY(b).toFixed(6)},${posAttr.getZ(b).toFixed(6)}`;
            const cKey = `${posAttr.getX(c).toFixed(6)},${posAttr.getY(c).toFixed(6)},${posAttr.getZ(c).toFixed(6)}`;
            faces.push([
                vertexMap.get(aKey)!,
                vertexMap.get(bKey)!,
                vertexMap.get(cKey)!
            ]);
        }
    }

    return new CANNON.ConvexPolyhedron({ vertices, faces });
}
