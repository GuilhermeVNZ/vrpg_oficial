// src/client-electron/renderer/dice/DiceConfig.ts
import * as THREE from 'three';
import * as CANNON from 'cannon-es';
export type DiceType = 'd2' | 'd4' | 'd6' | 'd8' | 'd10' | 'd12' | 'd20' | 'd100';
export const DICE_TYPES: DiceType[] = ['d2', 'd4', 'd6', 'd8', 'd10', 'd12', 'd20'];

export interface Colorset {
    foreground: string; // number color
    background: string; // dice body color
    outline: string;   // number outline
    edge: string;      // dice edge color
    material: string;  // material type identifier
    font?: string;     // optional font name
    texture?: string;  // optional texture name
}

export interface DiceConfig {
    type: DiceType;
    faces: number;
    mass: number;
    scale: number;
    colorset: Colorset;
    geometry: THREE.BufferGeometry;
    physicsShape: CANNON.Shape;
    faceValues: number[]; // mapping from face index to value
}
export interface SkinKit {
    id: string;
    name: string;
    description: string;
    price: number;
    colors: {
        base: number;
        text: number;
        roughness: number;
        metalness: number;
        emissive?: number;
        emissiveIntensity?: number;
    };
    textureType?: 'none' | 'pbr';
    texturePath?: string; // Path to folder containing base.png, normal.png, etc.
    textureScale?: number;
    textureMaps?: {
        map?: string;
        normalMap?: string;
        roughnessMap?: string;
        metalnessMap?: string;
        aoMap?: string;
    };
}

export const SKIN_KITS: SkinKit[] = [
    // Free Skins
    { id: 'white', name: 'Classic White', description: 'Traditional white plastic', price: 0, colors: { base: 0xeeeeee, text: 0x000000, roughness: 0.6, metalness: 0 } },
    { id: 'black', name: 'Basic Black', description: 'Sleek black finish', price: 0, colors: { base: 0x1a1a1a, text: 0xffffff, roughness: 0.5, metalness: 0.1 } },
    {
        id: 'wood',
        name: 'Natural Wood',
        description: 'Rustic wooden texture',
        price: 0,
        colors: { base: 0x8B4513, text: 0x2f1f0f, roughness: 1, metalness: 0 },
        textureType: 'pbr',
        texturePath: '/skins/natural_wood',
        textureScale: 1.0,
        textureMaps: {
            map: 'Walnut_Veneer_tfdoebqc_1K_BaseColor.jpg',
            normalMap: 'Walnut_Veneer_tfdoebqc_1K_Normal.jpg',
            roughnessMap: 'Walnut_Veneer_tfdoebqc_1K_Roughness.jpg',
            aoMap: 'Walnut_Veneer_tfdoebqc_1K_Cavity.jpg'
        }
    },
    {
        id: 'rosewood',
        name: 'Rosewood Parquet',
        description: 'Elegant parquet pattern',
        price: 0,
        colors: { base: 0x654321, text: 0xffd700, roughness: 0.8, metalness: 0.1 },
        textureType: 'pbr',
        texturePath: '/skins/rosewood_parquet',
        textureScale: 1.0,
        textureMaps: {
            map: 'Dutch_Rosewood_Parquet_tibkfgyl_1K_BaseColor.jpg',
            normalMap: 'Dutch_Rosewood_Parquet_tibkfgyl_1K_Normal.jpg',
            roughnessMap: 'Dutch_Rosewood_Parquet_tibkfgyl_1K_Roughness.jpg',
            aoMap: 'Dutch_Rosewood_Parquet_tibkfgyl_1K_AO.jpg'
        }
    },

    // PBR Skins
    {
        id: 'marble_white',
        name: 'Carrara Marble',
        description: 'Italian white marble',
        price: 2.99,
        colors: { base: 0xffffff, text: 0x000000, roughness: 0.1, metalness: 0.0 },
        textureType: 'pbr',
        texturePath: '/skins/marble_white',
        textureScale: 1.0,
        textureMaps: {
            map: 'Calacatta_Viola_Marble_tgglddvc_1K_BaseColor.jpg',
            normalMap: 'Calacatta_Viola_Marble_tgglddvc_1K_Normal.jpg',
            roughnessMap: 'Calacatta_Viola_Marble_tgglddvc_1K_Roughness.jpg',
            // metalnessMap: 'Calacatta_Viola_Marble_tgglddvc_1K_Specular.jpg', // Specular is not exactly metalness, but we can try mapping it or ignore
            aoMap: 'Calacatta_Viola_Marble_tgglddvc_1K_Cavity.jpg'
        }
    },

    // Paid Premium Skins
    {
        id: 'golden',
        name: 'Golden Luxury',
        description: 'Opulent gold finish',
        price: 1.99,
        colors: { base: 0xFFD700, text: 0x000000, roughness: 0.3, metalness: 1 },
        textureType: 'pbr',
        texturePath: '/skins/golden_luxury',
        textureScale: 1.0,
        textureMaps: {
            map: 'Spider_Black_and_Gold_Marble_tjglaflv_1K_BaseColor.jpg',
            normalMap: 'Spider_Black_and_Gold_Marble_tjglaflv_1K_Normal.jpg',
            roughnessMap: 'Spider_Black_and_Gold_Marble_tjglaflv_1K_Roughness.jpg',
            aoMap: 'Spider_Black_and_Gold_Marble_tjglaflv_1K_Cavity.jpg'
        }
    },
    {
        id: 'obsidian',
        name: 'Obsidian Night',
        description: 'Black with gold accents',
        price: 1.99,
        colors: { base: 0x0a0a0a, text: 0xFFD700, roughness: 0.2, metalness: 0.8 },
        textureType: 'pbr',
        texturePath: '/skins/obsidian_night',
        textureScale: 1.0,
        textureMaps: {
            map: 'obsidian_albedo.png',
            normalMap: 'obsidian_normal-ogl.png',
            aoMap: 'obsidian_ao.png'
        }
    },
    {
        id: 'crystal',
        name: 'Crystal Clear',
        description: 'Transparent ice blue',
        price: 1.99,
        colors: { base: 0x88ccff, text: 0x003366, roughness: 0.1, metalness: 0.3 },
        textureType: 'pbr',
        texturePath: '/skins/crystal_clear',
        textureScale: 1.0,
        textureMaps: {
            map: 'Crystal_basecolor.png',
            normalMap: 'Crystal_normal.png',
            roughnessMap: 'Crystal_roughness.png',
            aoMap: 'Crystal_ambientocclusion.png'
        }
    },
    {
        id: 'fire',
        name: 'Fire Ember',
        description: 'Burning red with glow',
        price: 1.99,
        colors: { base: 0xff4500, text: 0xffff00, roughness: 0.4, metalness: 0.6, emissive: 0xff4500, emissiveIntensity: 0.3 },
        textureType: 'pbr',
        texturePath: '/skins/fire_ember',
        textureScale: 1.0,
        textureMaps: {
            map: 'ground_0027_color_1k.jpg',
            normalMap: 'ground_0027_normal_opengl_1k.png',
            roughnessMap: 'ground_0027_roughness_1k.jpg',
            aoMap: 'ground_0027_ao_1k.jpg'
        }
    },
    {
        id: 'ice',
        name: 'Ice Frost',
        description: 'Frozen cyan shimmer',
        price: 1.99,
        colors: { base: 0x00ffff, text: 0x000033, roughness: 0.2, metalness: 0.7 },
        textureType: 'pbr',
        texturePath: '/skins/ice_frost',
        textureScale: 1.0,
        textureMaps: {
            map: 'Fresh_Windswept_Snow_ugjichedy_1K_BaseColor.jpg',
            normalMap: 'Fresh_Windswept_Snow_ugjichedy_1K_Normal.jpg',
            roughnessMap: 'Fresh_Windswept_Snow_ugjichedy_1K_Roughness.jpg',
            aoMap: 'Fresh_Windswept_Snow_ugjichedy_1K_AO.jpg'
        }
    },
    { id: 'nature', name: 'Nature\'s Wisdom', description: 'Earthen green', price: 1.99, colors: { base: 0x228b22, text: 0xffffff, roughness: 0.8, metalness: 0.1 } },
    {
        id: 'galactic',
        name: 'Galactic Purple',
        description: 'Deep space purple',
        price: 1.99,
        colors: { base: 0x8b00ff, text: 0xffffff, roughness: 0.3, metalness: 0.8, emissive: 0x4b0082, emissiveIntensity: 0.2 },
        textureType: 'pbr',
        texturePath: '/skins/galactic_purple',
        textureScale: 1.0,
        textureMaps: {
            map: 'A23DTEX_Albedo.jpg',
            normalMap: 'A23DTEX_Normal.jpg',
            roughnessMap: 'A23DTEX_Roughness.jpg',
            aoMap: 'A23DTEX_Ambient Occlusion.jpg'
        }
    },
    {
        id: 'ruby',
        name: 'Blood Ruby',
        description: 'Deep crimson metallic',
        price: 1.99,
        colors: { base: 0x8b0000, text: 0xffd700, roughness: 0.3, metalness: 0.9 },
        textureType: 'pbr',
        texturePath: '/skins/blood_ruby',
        textureScale: 1.0,
        textureMaps: {
            map: 'ground_0025_color_1k.jpg',
            normalMap: 'ground_0025_normal_opengl_1k.png',
            roughnessMap: 'ground_0025_roughness_1k.jpg',
            aoMap: 'ground_0025_ao_1k.jpg'
        }
    },
    { id: 'silver', name: 'Stormy Silver', description: 'Brushed silver', price: 1.99, colors: { base: 0xc0c0c0, text: 0x1e90ff, roughness: 0.4, metalness: 0.95 } },
    { id: 'toxic', name: 'Toxic Slime', description: 'Radioactive green', price: 1.99, colors: { base: 0x00ff00, text: 0x000000, roughness: 0.3, metalness: 0.5, emissive: 0x00ff00, emissiveIntensity: 0.4 } },
    { id: 'bronze', name: 'Ancient Bronze', description: 'Weathered bronze', price: 1.99, colors: { base: 0xcd7f32, text: 0x000000, roughness: 0.7, metalness: 0.8 } },
    { id: 'astral', name: 'Astral Sea', description: 'Deep blue cosmos', price: 1.99, colors: { base: 0x1e3a8a, text: 0xfbbf24, roughness: 0.4, metalness: 0.6 } },
    { id: 'bloodmoon', name: 'Blood Moon', description: 'Crimson eclipse', price: 1.99, colors: { base: 0x7f1d1d, text: 0xfcd34d, roughness: 0.5, metalness: 0.4 } },
    { id: 'lightning', name: 'Lightning Strike', description: 'Electric blue energy', price: 1.99, colors: { base: 0x3b82f6, text: 0xfef08a, roughness: 0.3, metalness: 0.7, emissive: 0x60a5fa, emissiveIntensity: 0.3 } },
    { id: 'radiant', name: 'Radiant', description: 'Holy white glow', price: 1.99, colors: { base: 0xffffff, text: 0xfbbf24, roughness: 0.2, metalness: 0.4, emissive: 0xfef3c7, emissiveIntensity: 0.25 } },
    {
        id: 'necrotic',
        name: 'Necrotic',
        description: 'Dark purple death',
        price: 1.99,
        colors: { base: 0x4c1d95, text: 0x86efac, roughness: 0.6, metalness: 0.3 },
        textureType: 'pbr',
        texturePath: '/skins/necrotic',
        textureScale: 1.0,
        textureMaps: {
            map: 'Swamp_Water_tgmjffbqx_1K_BaseColor.jpg',
            normalMap: 'Swamp_Water_tgmjffbqx_1K_Normal.jpg',
            roughnessMap: 'Swamp_Water_tgmjffbqx_1K_Roughness.jpg',
            aoMap: 'Swamp_Water_tgmjffbqx_1K_AO.jpg'
        }
    },
    { id: 'poison', name: 'Poison', description: 'Sickly green toxin', price: 1.99, colors: { base: 0x15803d, text: 0x000000, roughness: 0.5, metalness: 0.4 } },
    { id: 'glitter', name: 'Glitter Party', description: 'Sparkling rainbow', price: 1.99, colors: { base: 0xff69b4, text: 0xffffff, roughness: 0.2, metalness: 0.9 } },
];
