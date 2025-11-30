import React, { useMemo, useEffect, useState, forwardRef, useImperativeHandle, useRef } from 'react';
import { useConvexPolyhedron } from '@react-three/cannon';
import * as THREE from 'three';
import { DICE_GEOMETRY } from './DiceGeometry';
import { createNumberTexture } from '../NumberTexture';
import { diceAppearanceService } from '../DiceAppearanceService';
import { Colorset } from '../DiceConfig';

interface DiceProps {
    type: string; // 'd4', 'd6', 'd8', 'd10', 'd12', 'd20'
    position: [number, number, number];
    material?: string;
    initialImpulse?: [number, number, number];
    initialTorque?: [number, number, number];
    restitution?: number;
}

export interface DiceHandle {
    getValue: () => number;
}

// Helper to calculate planar UVs for a face
const getPlanarUVs = (vertices: number[][], faceIndices: number[]): number[] => {
    // 1. Calculate Centroid
    const centroid = new THREE.Vector3();
    let validVertices = 0;
    faceIndices.forEach(idx => {
        const v = vertices[idx];
        if (v) {
            centroid.add(new THREE.Vector3(v[0], v[1], v[2]));
            validVertices++;
        }
    });

    if (validVertices === 0) return [];
    centroid.divideScalar(validVertices);

    // 2. Calculate Normal (using first 3 available vertices)
    if (faceIndices.length < 3) return [];

    const v0Arr = vertices[faceIndices[0]];
    const v1Arr = vertices[faceIndices[1]];
    const v2Arr = vertices[faceIndices[2]];

    if (!v0Arr || !v1Arr || !v2Arr) return [];

    const v0 = new THREE.Vector3(...v0Arr);
    const v1 = new THREE.Vector3(...v1Arr);
    const v2 = new THREE.Vector3(...v2Arr);

    const normal = new THREE.Vector3().crossVectors(
        new THREE.Vector3().subVectors(v1, v0),
        new THREE.Vector3().subVectors(v2, v0)
    ).normalize();

    // 3. Define Basis Vectors (Tangent/Bitangent)
    // Choose an arbitrary up vector, if normal is up, choose forward
    let up = new THREE.Vector3(0, 1, 0);
    if (Math.abs(normal.dot(up)) > 0.9) {
        up = new THREE.Vector3(0, 0, 1);
    }
    const tangent = new THREE.Vector3().crossVectors(normal, up).normalize();
    const bitangent = new THREE.Vector3().crossVectors(normal, tangent).normalize();

    // 4. Project Vertices
    const uvs: number[] = [];
    faceIndices.forEach(idx => {
        const vArr = vertices[idx];
        if (vArr) {
            const v = new THREE.Vector3(...vArr);
            const rel = new THREE.Vector3().subVectors(v, centroid);

            // Project onto tangent/bitangent
            // Scale factor adjusted to 0.55 to ensure UVs stay within [0, 1] and avoid clamping distortion
            const u = rel.dot(tangent) * 0.55 + 0.5;
            const vProj = rel.dot(bitangent) * 0.55 + 0.5;

            uvs.push(u, vProj);
        } else {
            uvs.push(0.5, 0.5);
        }
    });

    return uvs;
};

export const Dice = forwardRef<DiceHandle, DiceProps>(({ type, position, initialImpulse, initialTorque, restitution = 0.5 }, ref) => {
    const geometryData = DICE_GEOMETRY[type as keyof typeof DICE_GEOMETRY];
    const quaternion = useRef<[number, number, number, number]>([0, 0, 0, 1]);

    // Get appearance configuration
    const appearance = useMemo(() => diceAppearanceService.getAppearanceForDice(type), [type]);

    // Create Colorset from appearance
    const colorset: Colorset = useMemo(() => ({
        foreground: appearance.labelColor,
        background: appearance.diceColor,
        outline: appearance.outlineColor,
        edge: appearance.edgeColor,
        material: appearance.material
    }), [appearance]);

    // Physics Body
    const { vertices = [], faces = [], faceValues = [] } = geometryData ?? {};

    const processedFaces = useMemo(() => {
        if (!faces) return [];
        return faces.map((face: number[]) => {
            if (geometryData.skipLastFaceIndex) {
                return face.slice(0, -1);
            }
            return face;
        });
    }, [faces, geometryData]);

    const [physicsRef, api] = useConvexPolyhedron(() => ({
        mass: 1,
        position,
        args: [vertices, processedFaces],
        material: { friction: 0.1, restitution },
        linearDamping: 0.1,
        angularDamping: 0.1,
    }));

    // Subscribe to quaternion updates
    useEffect(() => {
        const unsubscribe = api.quaternion.subscribe((v) => {
            quaternion.current = v;
        });
        return unsubscribe;
    }, [api.quaternion]);

    // Expose getValue
    useImperativeHandle(ref, () => ({
        getValue: () => {
            if (!geometryData || !faceValues) return 0;

            const q = new THREE.Quaternion(...quaternion.current);
            let maxDot = -Infinity;
            let resultValue = 0;

            faces.forEach((face: number[], index: number) => {
                // Calculate local normal
                const faceIndices = geometryData.skipLastFaceIndex ? face.slice(0, -1) : face;

                if (faceIndices.length < 3) return;

                const v0Arr = vertices[faceIndices[0]];
                const v1Arr = vertices[faceIndices[1]];
                const v2Arr = vertices[faceIndices[2]];

                if (!v0Arr || !v1Arr || !v2Arr) return;

                const v0 = new THREE.Vector3(...v0Arr);
                const v1 = new THREE.Vector3(...v1Arr);
                const v2 = new THREE.Vector3(...v2Arr);

                const localNormal = new THREE.Vector3().crossVectors(
                    new THREE.Vector3().subVectors(v1, v0),
                    new THREE.Vector3().subVectors(v2, v0)
                ).normalize();

                // Transform to world space
                const worldNormal = localNormal.applyQuaternion(q);

                // Check alignment with Up (0, 1, 0)
                const dot = worldNormal.y;

                if (dot > maxDot) {
                    maxDot = dot;
                    resultValue = faceValues[index];
                }
            });

            return resultValue;
        }
    }));

    if (!geometryData) return null;
    // Special handling for D2 (Cylinder)
    if (geometryData.type === 'Cylinder') {
        // Load PBR textures if applicable (same logic as main effect)
        const [pbrTextures, setPbrTextures] = useState<{ map?: THREE.Texture, normalMap?: THREE.Texture, roughnessMap?: THREE.Texture, aoMap?: THREE.Texture } | null>(null);

        useEffect(() => {
            if (appearance.textureType === 'pbr' && appearance.texturePath) {
                const loader = new THREE.TextureLoader();
                const loadTex = (type: 'map' | 'normalMap' | 'roughnessMap' | 'aoMap', filename: string) => {
                    loader.load(`${appearance.texturePath}/${filename}`, (tex) => {
                        tex.wrapS = THREE.RepeatWrapping;
                        tex.wrapT = THREE.RepeatWrapping;
                        tex.flipY = false;
                        if (appearance.textureScale) tex.repeat.set(appearance.textureScale, appearance.textureScale);
                        setPbrTextures(prev => ({ ...prev, [type]: tex }));
                    }, undefined, (err) => console.warn(`[Dice] Failed to load texture ${filename}`, err));
                };

                const maps = appearance.textureMaps || {};
                if (maps.map) loadTex('map', maps.map); else loadTex('map', 'base.png');
                if (maps.normalMap) loadTex('normalMap', maps.normalMap); else loadTex('normalMap', 'normal.png');
                if (maps.roughnessMap) loadTex('roughnessMap', maps.roughnessMap); else loadTex('roughnessMap', 'roughness.png');
                if (maps.aoMap) loadTex('aoMap', maps.aoMap);
            } else {
                setPbrTextures(null);
            }
        }, [appearance.textureType, appearance.texturePath, appearance.textureScale, appearance.textureMaps]);

        const materials = useMemo(() => {
            const sideMaterialParams: THREE.MeshStandardMaterialParameters = {
                color: colorset.background,
                roughness: 0.2, // Reduced for more reflection
                metalness: 0.7, // Increased for metallic look
                envMapIntensity: 1.0,
            };

            if (pbrTextures?.map) sideMaterialParams.map = pbrTextures.map;
            if (pbrTextures?.normalMap) sideMaterialParams.normalMap = pbrTextures.normalMap;
            if (pbrTextures?.roughnessMap) {
                sideMaterialParams.roughnessMap = pbrTextures.roughnessMap;
                sideMaterialParams.roughness = 1.0;
            }
            if (pbrTextures?.aoMap) {
                sideMaterialParams.aoMap = pbrTextures.aoMap;
                sideMaterialParams.aoMapIntensity = 1.0;
            }

            const sideMaterial = new THREE.MeshStandardMaterial(sideMaterialParams);

            const createCapMaterial = (value: number) => {
                const bgImage = pbrTextures?.map?.image;
                const texture = createNumberTexture(value, colorset, bgImage, 0.55);

                const params: THREE.MeshStandardMaterialParameters = {
                    map: texture,
                    color: 0xffffff,
                    roughness: 0.2, // Reduced for more reflection
                    metalness: 0.7, // Increased for metallic look
                    envMapIntensity: 1.0,
                };

                if (pbrTextures?.normalMap) params.normalMap = pbrTextures.normalMap;
                if (pbrTextures?.roughnessMap) {
                    params.roughnessMap = pbrTextures.roughnessMap;
                    params.roughness = 1.0;
                }
                if (pbrTextures?.aoMap) {
                    params.aoMap = pbrTextures.aoMap;
                    params.aoMapIntensity = 1.0;
                }

                return new THREE.MeshStandardMaterial(params);
            };

            return [sideMaterial, createCapMaterial(2), createCapMaterial(1)];
        }, [colorset, pbrTextures]);

        const geometry = useMemo(() => new THREE.CylinderGeometry(1.5, 1.5, 0.3, 32), []);

        // Physics for Cylinder (using Cylinder shape instead of ConvexPolyhedron for simplicity/accuracy)
        const [physicsRef, api] = useConvexPolyhedron(() => ({
            mass: 1,
            position,
            args: [vertices, faces], // Note: D2 physics mesh is usually a simplified hull, but here we use the same vertices/faces if available, or fallback
            // Actually D2 usually needs a specific cylinder shape in Cannon, but ConvexPolyhedron works if vertices are correct.
            // For now, assuming vertices/faces are correct for the D2 "coin" shape.
            material: { friction: 0.1, restitution },
            linearDamping: 0.1,
            angularDamping: 0.1,
        }));

        // Re-using the same impulse/torque logic
        useEffect(() => {
            const impulse = initialImpulse || [(Math.random() - 0.5) * 30, (Math.random() - 0.5) * 30, (Math.random() - 0.5) * 30] as [number, number, number];
            const torque = initialTorque || [(Math.random() - 0.5) * 40, (Math.random() - 0.5) * 40, (Math.random() - 0.5) * 40] as [number, number, number];
            api.applyImpulse(impulse, [0, 0, 0]);
            api.applyTorque(torque);
        }, [api, initialImpulse, initialTorque]);

        return (
            <mesh ref={physicsRef as any} geometry={geometry} material={materials} scale={0.75} />
        );
    }

    // Load PBR textures if applicable
    const [pbrTextures, setPbrTextures] = useState<{ map?: THREE.Texture, normalMap?: THREE.Texture, roughnessMap?: THREE.Texture, aoMap?: THREE.Texture } | null>(null);

    useEffect(() => {
        if (appearance.textureType === 'pbr' && appearance.texturePath) {
            const loader = new THREE.TextureLoader();

            // Helper to load texture
            const loadTex = (type: 'map' | 'normalMap' | 'roughnessMap' | 'aoMap', filename: string) => {
                loader.load(`${appearance.texturePath}/${filename}`, (tex) => {
                    tex.wrapS = THREE.RepeatWrapping;
                    tex.wrapT = THREE.RepeatWrapping;
                    tex.flipY = false;
                    if (appearance.textureScale) {
                        tex.repeat.set(appearance.textureScale, appearance.textureScale);
                    }
                    setPbrTextures(prev => ({ ...prev, [type]: tex }));
                }, undefined, (err) => {
                    console.warn(`[Dice] Failed to load texture ${filename}`, err);
                });
            };

            // Use configured maps or defaults
            const maps = appearance.textureMaps || {};

            if (maps.map) loadTex('map', maps.map);
            else loadTex('map', 'base.png');

            if (maps.normalMap) loadTex('normalMap', maps.normalMap);
            else loadTex('normalMap', 'normal.png');

            if (maps.roughnessMap) loadTex('roughnessMap', maps.roughnessMap);
            else loadTex('roughnessMap', 'roughness.png');

            if (maps.aoMap) loadTex('aoMap', maps.aoMap);
        } else {
            setPbrTextures(null);
        }
    }, [appearance.textureType, appearance.texturePath, appearance.textureScale, appearance.textureMaps]);

    // 1. Generate Materials
    const materials = useMemo(() => {
        if (!faceValues) return [new THREE.MeshStandardMaterial({ color: colorset.background })];

        return faceValues.map((value: number) => {
            // If we have a PBR map, pass its image to createNumberTexture to blend it
            const bgImage = pbrTextures?.map?.image;
            // Use fontScale 0.55 to compensate for the UV zoom (0.55)
            const texture = createNumberTexture(value, colorset, bgImage, 0.55);

            const materialParams: THREE.MeshStandardMaterialParameters = {
                map: texture, // The blended texture (Marble + Number)
                color: 0xffffff, // White so it doesn't tint the texture
                roughness: 0.5,
                metalness: 0.1,
                polygonOffset: true,
                polygonOffsetFactor: 1,
            };

            // Apply other PBR maps if available
            if (pbrTextures?.normalMap) {
                materialParams.normalMap = pbrTextures.normalMap;
            }
            if (pbrTextures?.roughnessMap) {
                materialParams.roughnessMap = pbrTextures.roughnessMap;
                materialParams.roughness = 1.0; // Let map control roughness
            }
            if (pbrTextures?.aoMap) {
                materialParams.aoMap = pbrTextures.aoMap;
                materialParams.aoMapIntensity = 1.0;
            }

            return new THREE.MeshStandardMaterial(materialParams);
        });
    }, [faceValues, colorset, pbrTextures]);

    // 2. Build Custom BufferGeometry with Groups
    const geometry = useMemo(() => {
        const geo = new THREE.BufferGeometry();
        const positionArray: number[] = [];
        const uvArray: number[] = [];
        const indicesArray: number[] = [];

        let vertexOffset = 0;

        faces.forEach((face: number[]) => {
            const rawMaterialIndex = face[face.length - 1];
            const materialIndex = (rawMaterialIndex !== undefined ? rawMaterialIndex : 1) - 1;

            const faceVertexIndices = face.slice(0, -1);
            const numVertices = faceVertexIndices.length;

            const faceUVs = getPlanarUVs(vertices, faceVertexIndices);

            faceVertexIndices.forEach((vIdx: number, i: number) => {
                const v = vertices[vIdx];
                if (v) {
                    positionArray.push(v[0], v[1], v[2]);
                } else {
                    positionArray.push(0, 0, 0);
                }

                if (faceUVs.length > i * 2) {
                    uvArray.push(faceUVs[i * 2], faceUVs[i * 2 + 1]);
                } else {
                    uvArray.push(0.5, 0.5);
                }
            });

            for (let i = 1; i < numVertices - 1; i++) {
                indicesArray.push(vertexOffset, vertexOffset + i, vertexOffset + i + 1);
            }

            const numTriangles = numVertices - 2;
            const numIndices = numTriangles * 3;
            const startIndex = indicesArray.length - numIndices;

            geo.addGroup(startIndex, numIndices, materialIndex);

            vertexOffset += numVertices;
        });

        geo.setAttribute('position', new THREE.Float32BufferAttribute(positionArray, 3));
        geo.setAttribute('uv', new THREE.Float32BufferAttribute(uvArray, 2));
        geo.setIndex(indicesArray);
        geo.computeVertexNormals();

        return geo;
    }, [vertices, faces]);

    // Apply initial impulse and torque
    useEffect(() => {
        const impulse = initialImpulse || [
            (Math.random() - 0.5) * 30,
            (Math.random() - 0.5) * 30,
            (Math.random() - 0.5) * 30
        ] as [number, number, number];
        const torque = initialTorque || [
            (Math.random() - 0.5) * 40,
            (Math.random() - 0.5) * 40,
            (Math.random() - 0.5) * 40
        ] as [number, number, number];

        api.applyImpulse(impulse, [0, 0, 0]);
        api.applyTorque(torque);
    }, [api, initialImpulse, initialTorque]);

    return (
        <mesh
            ref={physicsRef as any}
            geometry={geometry}
            material={materials}
            scale={0.75}
        />
    );
});

Dice.displayName = 'Dice';
