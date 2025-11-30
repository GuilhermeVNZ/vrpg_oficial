// src/client-electron/renderer/dice/DiceFactory.ts
import * as THREE from 'three';
import * as CANNON from 'cannon-es';
import { DiceType, DiceConfig, Colorset } from './DiceConfig';
import { createGeometry, createConvexPolyhedron } from './DiceGeometry';
import { createNumberTexture } from './NumberTexture';
import { diceAppearanceService } from './DiceAppearanceService';

/**
 * Simple default colorset used when none is provided.
 */
const defaultColorset: Colorset = {
    foreground: '#FFFFFF', // white numbers
    background: '#D4AF37', // golden dice body
    outline: '#000000',   // black outline for numbers
    edge: '#8B5A00',      // dark edge color
    material: 'metal',
};

/**
 * DiceFactory registers dice configurations and creates dice meshes with physics bodies.
 */
export class DiceFactory {
    private configs: Map<DiceType, DiceConfig> = new Map();

    constructor() {
        // Register built‑in dice types with default configs
        this.register('d4');
        this.register('d6');
        this.register('d8');
        this.register('d10');
        this.register('d12');
        this.register('d20');
        this.register('d100');
    }

    /** Register a dice type with optional overrides */
    register(type: DiceType, overrides?: Partial<DiceConfig>) {
        const geometry = createGeometry(type);
        const physicsShape = createConvexPolyhedron(geometry);
        const faces = geometry.index ? geometry.index.count / 3 : geometry.getAttribute('position').count / 3;
        const faceValues = Array.from({ length: faces }, (_, i) => i + 1);

        const baseConfig: DiceConfig = {
            type,
            faces,
            mass: 1,
            scale: 1,
            colorset: defaultColorset,
            geometry,
            physicsShape,
            faceValues,
        };
        const config = { ...baseConfig, ...overrides } as DiceConfig;
        this.configs.set(type, config);
    }

    /** Load PBR textures for a dice type */
    private async loadPbrTextures(type: DiceType): Promise<{ map?: THREE.Texture, normalMap?: THREE.Texture, roughnessMap?: THREE.Texture, aoMap?: THREE.Texture } | null> {
        const appearance = diceAppearanceService.getAppearanceForDice(type);

        if (appearance.textureType === 'pbr' && appearance.texturePath) {
            const loader = new THREE.TextureLoader();
            const textures: { map?: THREE.Texture, normalMap?: THREE.Texture, roughnessMap?: THREE.Texture, aoMap?: THREE.Texture } = {};

            const loadTex = (type: 'map' | 'normalMap' | 'roughnessMap' | 'aoMap', filename: string): Promise<THREE.Texture | null> => {
                return new Promise((resolve) => {
                    loader.load(
                        `${appearance.texturePath}/${filename}`,
                        (tex) => {
                            tex.wrapS = THREE.RepeatWrapping;
                            tex.wrapT = THREE.RepeatWrapping;
                            tex.flipY = false;
                            if (appearance.textureScale) {
                                tex.repeat.set(appearance.textureScale, appearance.textureScale);
                            }
                            resolve(tex);
                        },
                        undefined,
                        (err) => {
                            console.warn(`[DiceFactory] Failed to load texture ${filename}`, err);
                            resolve(null);
                        }
                    );
                });
            };

            const maps = appearance.textureMaps || {};

            // Load all textures in parallel
            const [baseMap, normalMap, roughnessMap, aoMap] = await Promise.all([
                loadTex('map', maps.map || 'base.png'),
                loadTex('normalMap', maps.normalMap || 'normal.png'),
                loadTex('roughnessMap', maps.roughnessMap || 'roughness.png'),
                maps.aoMap ? loadTex('aoMap', maps.aoMap) : Promise.resolve(null)
            ]);

            if (baseMap) textures.map = baseMap;
            if (normalMap) textures.normalMap = normalMap;
            if (roughnessMap) textures.roughnessMap = roughnessMap;
            if (aoMap) textures.aoMap = aoMap;

            return textures;
        }

        return null;
    }

    /** Create a dice mesh and physics body ready to be added to a scene/world */
    async createDice(type: DiceType, scene: THREE.Scene, world: CANNON.World, customColorset?: Colorset) {
        const config = this.configs.get(type);
        if (!config) throw new Error(`Dice type ${type} not registered`);

        // Get appearance from service
        const appearance = diceAppearanceService.getAppearanceForDice(type);
        const colorset: Colorset = customColorset ?? {
            foreground: appearance.labelColor,
            background: appearance.diceColor,
            outline: appearance.outlineColor,
            edge: appearance.edgeColor,
            material: appearance.material
        };

        // Load PBR textures
        const pbrTextures = await this.loadPbrTextures(type);

        // Generate textures for each face with PBR support
        const materials: THREE.Material[] = [];
        for (let i = 0; i < config.faces; i++) {
            const bgImage = pbrTextures?.map?.image;
            const tex = createNumberTexture(config.faceValues[i]!, colorset, bgImage, 0.55);

            // Create material with enhanced properties for reflections
            const materialParams: THREE.MeshStandardMaterialParameters = {
                map: tex,
                color: 0xffffff,
                roughness: 0.2, // Lower for more reflection
                metalness: 0.7, // Higher for metallic look
                envMapIntensity: 1.0,
            };

            // Apply PBR maps if available
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

            materials.push(new THREE.MeshStandardMaterial(materialParams));
        }

        const mesh = new THREE.Mesh(config.geometry, materials);
        mesh.scale.setScalar(config.scale);
        mesh.castShadow = true;
        mesh.receiveShadow = true;
        scene.add(mesh);

        const body = new CANNON.Body({
            mass: config.mass,
            shape: config.physicsShape,
            material: new CANNON.Material(),
            linearDamping: 0.1,
            angularDamping: 0.1,
            allowSleep: true,
            sleepSpeedLimit: 1.5, // DSN uses 75 for scale 50 -> 1.5 for scale 1
            sleepTimeLimit: 0.5,  // DSN uses 0.9, we can be a bit more aggressive
        });
        world.addBody(body);

        return { mesh, body, config };
    }
}
