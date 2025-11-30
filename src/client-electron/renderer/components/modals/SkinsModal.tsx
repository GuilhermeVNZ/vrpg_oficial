import React, { useRef, useEffect, useState, useMemo } from 'react';
import Modal from './Modal';
import * as THREE from 'three';
import { DICE_GEOMETRY } from '../../dice/engine/DiceGeometry';
import { createNumberTexture } from '../../dice/NumberTexture';
import { Colorset } from '../../dice/DiceConfig';

interface SkinsModalProps {
    isOpen: boolean;
    onClose: () => void;
}



import { SKIN_KITS, SkinKit, DICE_TYPES, DiceType } from '../../dice/DiceConfig';

// Helper to calculate planar UVs for a face (same as Dice.tsx)
const getPlanarUVs = (vertices: number[][], faceIndices: number[]): number[] => {
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

    let up = new THREE.Vector3(0, 1, 0);
    if (Math.abs(normal.dot(up)) > 0.9) {
        up = new THREE.Vector3(0, 0, 1);
    }
    const tangent = new THREE.Vector3().crossVectors(normal, up).normalize();
    const bitangent = new THREE.Vector3().crossVectors(normal, tangent).normalize();

    const uvs: number[] = [];
    faceIndices.forEach(idx => {
        const vArr = vertices[idx];
        if (vArr) {
            const v = new THREE.Vector3(...vArr);
            const rel = new THREE.Vector3().subVectors(v, centroid);
            const u = rel.dot(tangent) * 0.55 + 0.5;
            const vProj = rel.dot(bitangent) * 0.55 + 0.5;
            uvs.push(u, vProj);
        } else {
            uvs.push(0.5, 0.5);
        }
    });

    return uvs;
};

const SkinsModal: React.FC<SkinsModalProps> = ({ isOpen, onClose }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const sceneRef = useRef<THREE.Scene | null>(null);
    const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
    const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
    const diceRef = useRef<THREE.Mesh | null>(null);
    const animationIdRef = useRef<number | null>(null);
    const isDraggingRef = useRef(false);
    const previousMouseRef = useRef({ x: 0, y: 0 });

    const [selectedDiceType, setSelectedDiceType] = useState<DiceType>('d20');
    const [selectedKit, setSelectedKit] = useState<string>('white');
    const [zoom, setZoom] = useState(5);

    // Owned skins (free skins are owned by default)
    const [ownedSkins, setOwnedSkins] = useState<Set<string>>(() => {
        const stored = localStorage.getItem('ownedDiceSkins');
        if (stored) {
            return new Set(JSON.parse(stored));
        }
        // Free skins owned by default
        return new Set(SKIN_KITS.filter(k => k.price === 0).map(k => k.id));
    });

    // Active skin per dice type
    const [activeSkins, setActiveSkins] = useState<Record<DiceType, string>>(() => {
        const stored = localStorage.getItem('activeDiceSkins');
        if (stored) {
            return JSON.parse(stored);
        }
        // Default all to white
        return {
            'd2': 'white',
            'd4': 'white',
            'd6': 'white',
            'd8': 'white',
            'd10': 'white',
            'd12': 'white',
            'd20': 'white',
        };
    });

    // Save to localStorage when changed
    useEffect(() => {
        localStorage.setItem('ownedDiceSkins', JSON.stringify(Array.from(ownedSkins)));
    }, [ownedSkins]);

    useEffect(() => {
        localStorage.setItem('activeDiceSkins', JSON.stringify(activeSkins));
    }, [activeSkins]);

    const handlePurchase = (skinId: string) => {
        // Mock purchase - in real app, would integrate with payment system
        setOwnedSkins(prev => new Set([...prev, skinId]));
    };

    const handleApply = (skinId: string) => {
        setActiveSkins(prev => ({
            ...prev,
            [selectedDiceType]: skinId
        }));
        // Also select the skin to show in preview
        setSelectedKit(skinId);
    };

    // Auto-select active skin when switching dice types
    useEffect(() => {
        const activeSkin = activeSkins[selectedDiceType];
        if (activeSkin) {
            setSelectedKit(activeSkin);
        }
    }, [selectedDiceType, activeSkins]);




    // Get colorset from selected skin
    const colorset = useMemo<Colorset>(() => {
        const skin = SKIN_KITS.find(k => k.id === selectedKit) || SKIN_KITS[0]!;
        return {
            foreground: `#${skin.colors.text.toString(16).padStart(6, '0')}`,
            background: `#${skin.colors.base.toString(16).padStart(6, '0')}`,
            outline: '#000000',
            edge: '#000000',
            material: 'standard'
        };
    }, [selectedKit]);

    // Load PBR textures for preview
    const [pbrTextures, setPbrTextures] = useState<{ map?: THREE.Texture, normalMap?: THREE.Texture, roughnessMap?: THREE.Texture, aoMap?: THREE.Texture } | null>(null);

    useEffect(() => {
        const skin = SKIN_KITS.find(k => k.id === selectedKit);
        if (skin && skin.textureType === 'pbr' && skin.texturePath) {
            const loader = new THREE.TextureLoader();

            const loadTex = (type: 'map' | 'normalMap' | 'roughnessMap' | 'aoMap', filename: string) => {
                loader.load(`${skin.texturePath}/${filename}`, (tex) => {
                    tex.wrapS = THREE.RepeatWrapping;
                    tex.wrapT = THREE.RepeatWrapping;
                    tex.flipY = false; // Disable flipY to avoid WebGL errors with certain formats/drivers
                    if (skin.textureScale) {
                        tex.repeat.set(skin.textureScale, skin.textureScale);
                    }
                    setPbrTextures(prev => ({ ...prev, [type]: tex }));
                }, undefined, (err) => {
                    console.warn(`[SkinsModal] Failed to load texture ${filename}`, err);
                });
            };

            const maps = skin.textureMaps || {};

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
    }, [selectedKit]);

    // Create geometry and materials for dice
    const { geometry, materials } = useMemo(() => {
        const geometryData = DICE_GEOMETRY[selectedDiceType];

        if (!geometryData || geometryData.type === 'Cylinder') {
            // D2: Create cylinder with textures on both round faces
            const geometry = new THREE.CylinderGeometry(1.5, 1.5, 0.3, 32);

            // Create materials for d2: side (plain), top (2), bottom (1)
            const sideMaterialParams: THREE.MeshStandardMaterialParameters = {
                color: colorset.background,
                roughness: 0.5,
                metalness: 0.1,
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

            // Helper for cap materials
            const createCapMaterial = (value: number) => {
                const bgImage = pbrTextures?.map?.image;
                const texture = createNumberTexture(value, colorset, bgImage, 0.55);

                const params: THREE.MeshStandardMaterialParameters = {
                    map: texture,
                    color: 0xffffff,
                    roughness: 0.5,
                    metalness: 0.1,
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

            const topMaterial = createCapMaterial(2);
            const bottomMaterial = createCapMaterial(1);

            return {
                geometry,
                materials: [sideMaterial, topMaterial, bottomMaterial]
            };
        }

        const { vertices = [], faces = [], faceValues = [] } = geometryData;

        // Generate materials (one texture per face)
        const mats = faceValues.map((value: number) => {
            // If PBR, blend the number with the base map
            const bgImage = pbrTextures?.map?.image;
            // Use fontScale 0.55 to compensate for the UV zoom (0.55)
            const texture = createNumberTexture(value, colorset, bgImage, 0.55);

            const materialParams: THREE.MeshStandardMaterialParameters = {
                map: texture,
                color: 0xffffff,
                roughness: 0.5,
                metalness: 0.1,
            };

            if (pbrTextures?.normalMap) materialParams.normalMap = pbrTextures.normalMap;
            if (pbrTextures?.roughnessMap) {
                materialParams.roughnessMap = pbrTextures.roughnessMap;
                materialParams.roughness = 1.0;
            }
            if (pbrTextures?.aoMap) {
                materialParams.aoMap = pbrTextures.aoMap;
                materialParams.aoMapIntensity = 1.0;
            }

            return new THREE.MeshStandardMaterial(materialParams);
        });

        // Build custom BufferGeometry with groups
        const geo = new THREE.BufferGeometry();
        const positionArray: number[] = [];
        const uvArray: number[] = [];
        const indicesArray: number[] = [];

        let vertexOffset = 0;

        faces.forEach((face: number[]) => {
            const rawMaterialIndex = face[face.length - 1];
            const materialIndex = rawMaterialIndex !== undefined && rawMaterialIndex > 0
                ? rawMaterialIndex - 1
                : 0;

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

        return { geometry: geo, materials: mats };
    }, [selectedDiceType, colorset, pbrTextures]);

    // Initialize scene
    useEffect(() => {
        if (!canvasRef.current || !isOpen) return;

        const scene = new THREE.Scene();
        scene.background = null;
        sceneRef.current = scene;

        const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 1000);
        camera.position.z = zoom;
        cameraRef.current = camera;

        const renderer = new THREE.WebGLRenderer({ canvas: canvasRef.current, alpha: true, antialias: true });
        renderer.setSize(400, 400);
        renderer.setPixelRatio(window.devicePixelRatio);
        rendererRef.current = renderer;

        // Lighting
        scene.add(new THREE.AmbientLight(0xffffff, 0.8));
        const light1 = new THREE.PointLight(0xffd700, 1.2, 100);
        light1.position.set(5, 5, 5);
        scene.add(light1);
        const light2 = new THREE.PointLight(0x4a90e2, 0.8, 100);
        light2.position.set(-5, -5, 5);
        scene.add(light2);

        // Create dice
        const dice = new THREE.Mesh(geometry, materials);
        dice.scale.set(0.75, 0.75, 0.75);
        diceRef.current = dice;
        scene.add(dice);

        // Animation
        const animate = () => {
            animationIdRef.current = requestAnimationFrame(animate);
            if (diceRef.current && !isDraggingRef.current) {
                diceRef.current.rotation.y += 0.005;
            }
            renderer.render(scene, camera);
        };
        animate();

        return () => {
            if (animationIdRef.current) cancelAnimationFrame(animationIdRef.current);
            renderer.dispose();
            geometry.dispose();
            if (Array.isArray(materials)) {
                materials.forEach((m: THREE.MeshStandardMaterial) => {
                    if (m.map) m.map.dispose();
                    m.dispose();
                });
            } else if (materials instanceof THREE.Material) {
                materials.dispose();
            }
        };
    }, [isOpen, geometry, materials, zoom]);

    // Update zoom
    useEffect(() => {
        if (cameraRef.current) cameraRef.current.position.z = zoom;
    }, [zoom]);

    const handleMouseDown = (e: React.MouseEvent) => {
        isDraggingRef.current = true;
        previousMouseRef.current = { x: e.clientX, y: e.clientY };
    };

    const handleMouseMove = (e: React.MouseEvent) => {
        if (!isDraggingRef.current || !diceRef.current) return;
        const deltaX = e.clientX - previousMouseRef.current.x;
        const deltaY = e.clientY - previousMouseRef.current.y;
        diceRef.current.rotation.y += deltaX * 0.01;
        diceRef.current.rotation.x += deltaY * 0.01;
        previousMouseRef.current = { x: e.clientX, y: e.clientY };
    };

    const handleMouseUp = () => { isDraggingRef.current = false; };

    const handleWheel = (e: React.WheelEvent) => {
        e.preventDefault();
        setZoom(prev => Math.max(3, Math.min(10, prev + e.deltaY * 0.01)));
    };

    const selectedSkin = SKIN_KITS.find(k => k.id === selectedKit) || SKIN_KITS[0]!;

    return (
        <Modal isOpen={isOpen} onClose={onClose} title="Dice Skins" maxWidth="1200px">
            <div style={{ display: 'flex', flexDirection: 'column', gap: '24px', height: '70vh' }}>
                {/* Dice Type Selector */}
                <div style={{ display: 'flex', gap: '12px', justifyContent: 'center', flexWrap: 'wrap', borderBottom: '1px solid rgba(212, 175, 55, 0.3)', paddingBottom: '16px' }}>
                    {DICE_TYPES.map(type => (
                        <button key={type} onClick={() => setSelectedDiceType(type)}
                            style={{
                                padding: '12px 20px',
                                background: selectedDiceType === type ? 'linear-gradient(135deg, rgba(212, 175, 55, 0.3), rgba(74, 144, 226, 0.3))' : 'rgba(0, 0, 0, 0.3)',
                                border: `2px solid ${selectedDiceType === type ? '#D4AF37' : 'rgba(212, 175, 55, 0.3)'}`,
                                borderRadius: '12px',
                                color: selectedDiceType === type ? '#D4AF37' : '#888',
                                fontSize: '18px',
                                fontWeight: 'bold',
                                cursor: 'pointer',
                                transition: 'all 0.3s ease',
                                textTransform: 'uppercase',
                                fontFamily: 'monospace',
                            }}>
                            {type.toUpperCase()}
                        </button>
                    ))}
                </div>

                {/* Main Content */}
                <div style={{ display: 'flex', gap: '24px', flex: 1, overflow: 'hidden' }}>
                    {/* 3D Preview */}
                    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: '24px' }}>
                        <canvas ref={canvasRef}
                            onMouseDown={handleMouseDown}
                            onMouseMove={handleMouseMove}
                            onMouseUp={handleMouseUp}
                            onMouseLeave={handleMouseUp}
                            onWheel={handleWheel}
                            style={{
                                width: '400px',
                                height: '400px',
                                cursor: isDraggingRef.current ? 'grabbing' : 'grab',
                                border: '2px solid rgba(212, 175, 55, 0.3)',
                                borderRadius: '16px',
                                background: 'rgba(0, 0, 0, 0.2)',
                            }} />
                        <div style={{ textAlign: 'center' }}>
                            <div style={{ fontSize: '28px', fontWeight: 'bold', marginBottom: '4px', color: '#D4AF37' }}>
                                {selectedSkin.name}
                            </div>
                            <div style={{ fontSize: '24px', fontWeight: 'bold', color: selectedSkin.price === 0 ? '#4a90e2' : '#D4AF37', textShadow: '0 2px 8px rgba(212, 175, 55, 0.6)', marginBottom: '8px' }}>
                                {selectedSkin.price === 0 ? 'FREE' : `$${selectedSkin.price.toFixed(2)}`}
                            </div>
                            <div style={{ fontSize: '14px', color: '#888', fontStyle: 'italic' }}>
                                {selectedSkin.description}
                            </div>
                        </div>
                    </div>

                    {/* Skin List */}
                    <div style={{ width: '320px', overflowY: 'auto', paddingRight: '8px' }} className="modal-content-scroll">
                        <h3 style={{ margin: '0 0 16px 0', fontSize: '20px', color: '#D4AF37', fontFamily: '"Crimson Text", serif' }}>
                            Available Skins ({SKIN_KITS.length})
                        </h3>
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                            {SKIN_KITS.map(kit => {
                                const owned = ownedSkins.has(kit.id);
                                const active = activeSkins[selectedDiceType] === kit.id;

                                return (
                                    <div key={kit.id}
                                        style={{
                                            padding: '16px',
                                            background: selectedKit === kit.id ? 'linear-gradient(135deg, rgba(212, 175, 55, 0.2), rgba(74, 144, 226, 0.2))' : 'rgba(0, 0, 0, 0.3)',
                                            border: `2px solid ${active ? '#4a90e2' : selectedKit === kit.id ? '#D4AF37' : 'rgba(212, 175, 55, 0.2)'}`,
                                            borderRadius: '12px',
                                            transition: 'all 0.3s ease',
                                            position: 'relative',
                                        }}>
                                        {active && (
                                            <div style={{
                                                position: 'absolute',
                                                top: '8px',
                                                right: '8px',
                                                background: '#4a90e2',
                                                color: '#fff',
                                                padding: '4px 8px',
                                                borderRadius: '6px',
                                                fontSize: '10px',
                                                fontWeight: 'bold',
                                            }}>
                                                ACTIVE
                                            </div>
                                        )}
                                        <div onClick={() => setSelectedKit(kit.id)} style={{ cursor: 'pointer' }}>
                                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', marginBottom: '8px' }}>
                                                <span style={{ fontSize: '16px', fontWeight: 'bold', color: selectedKit === kit.id ? '#D4AF37' : '#ccc' }}>
                                                    {kit.name}
                                                </span>
                                                <span style={{ fontSize: '14px', color: kit.price === 0 ? '#4a90e2' : '#D4AF37', fontWeight: 'bold' }}>
                                                    {kit.price === 0 ? 'FREE' : `$${kit.price.toFixed(2)}`}
                                                </span>
                                            </div>
                                            <div style={{ fontSize: '12px', color: '#888', fontStyle: 'italic', marginBottom: '12px' }}>
                                                {kit.description}
                                            </div>
                                        </div>
                                        {owned ? (
                                            <button
                                                onClick={() => handleApply(kit.id)}
                                                disabled={active}
                                                style={{
                                                    width: '100%',
                                                    padding: '10px',
                                                    background: active ? '#333' : 'linear-gradient(135deg, #4a90e2, #357abd)',
                                                    border: 'none',
                                                    borderRadius: '8px',
                                                    color: active ? '#666' : '#fff',
                                                    fontSize: '14px',
                                                    fontWeight: 'bold',
                                                    cursor: 'default',
                                                    transition: 'all 0.3s ease',
                                                }}>
                                                {active ? '✓ Applied' : 'Apply to ' + selectedDiceType.toUpperCase()}
                                            </button>
                                        ) : (
                                            <button
                                                onClick={() => handlePurchase(kit.id)}
                                                style={{
                                                    width: '100%',
                                                    padding: '10px',
                                                    background: 'linear-gradient(135deg, #D4AF37, #b8941f)',
                                                    border: 'none',
                                                    borderRadius: '8px',
                                                    color: '#000',
                                                    fontSize: '14px',
                                                    fontWeight: 'bold',
                                                    cursor: 'pointer',
                                                    transition: 'all 0.3s ease',
                                                }}>
                                                Purchase ${kit.price.toFixed(2)}
                                            </button>
                                        )}
                                    </div>
                                );
                            })}
                        </div>
                    </div>
                </div>
            </div>
        </Modal>
    );
};

export default SkinsModal;
