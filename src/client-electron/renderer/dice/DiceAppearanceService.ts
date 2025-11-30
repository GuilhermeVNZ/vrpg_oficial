/**
 * Dice Appearance Service
 * 
 * Resolves dice appearance configuration from user preferences and settings
 */

import { userFlagsManager } from '../store/diceSettings';
import { SKIN_KITS } from './DiceConfig';

export interface DiceAppearance {
    system: string;
    colorset: string;
    material: string;
    labelColor: string;
    diceColor: string;
    outlineColor: string;
    edgeColor: string;
    isGhost?: boolean;
    textureType?: 'none' | 'pbr';
    texturePath?: string;
    textureScale?: number;
    textureMaps?: {
        map?: string;
        normalMap?: string;
        roughnessMap?: string;
        metalnessMap?: string;
        aoMap?: string;
    };
}

export class DiceAppearanceService {
    /**
     * Get active skin from localStorage for a specific dice type
     */
    private getActiveSkin(diceType: string): string | null {
        try {
            const stored = localStorage.getItem('activeDiceSkins');
            if (stored) {
                const activeSkins = JSON.parse(stored);
                return activeSkins[diceType] || null;
            }
        } catch (error) {
            console.error('[DiceAppearanceService] Error reading active skins:', error);
        }
        return null;
    }

    /**
     * Get skin kit colors from skin ID
     */
    private getSkinColors(skinId: string | null) {
        if (!skinId) return null;

        const skin = SKIN_KITS.find(k => k.id === skinId);

        if (skin) {
            return {
                foreground: `#${skin.colors.text.toString(16).padStart(6, '0')}`,
                background: `#${skin.colors.base.toString(16).padStart(6, '0')}`,
                outline: '#000000',
                edge: '#000000',
                textureType: skin.textureType,
                texturePath: skin.texturePath,
                textureScale: skin.textureScale,
                textureMaps: skin.textureMaps
            };
        }

        return null;
    }

    /**
     * Get appearance configuration for a specific dice type
     * Priority: active skin from localStorage > dice-specific settings > user global settings > system defaults
     */
    getAppearanceForDice(diceType: string, options: { isGhost?: boolean } = {}): DiceAppearance {
        const userAppearance = userFlagsManager.getFlag('dice-so-nice', 'appearance');

        console.log('[DiceAppearanceService] User appearance from flags:', userAppearance);

        // Check for active skin from localStorage
        const activeSkinId = this.getActiveSkin(diceType);
        const skinColors = this.getSkinColors(activeSkinId);

        // Check for dice-specific override
        const diceSpecific = userAppearance?.[diceType];
        const global = userAppearance?.global || {};

        return {
            system: diceSpecific?.system || global.system || 'standard',
            colorset: diceSpecific?.colorset || global.colorset || 'custom',
            material: diceSpecific?.material || global.material || 'plastic',
            labelColor: skinColors?.foreground || global.labelColor || '#000000',
            diceColor: skinColors?.background || global.diceColor || '#ffffff',
            outlineColor: skinColors?.outline || global.outlineColor || '#000000',
            edgeColor: skinColors?.edge || global.edgeColor || '#000000',
            isGhost: options.isGhost || false,
            textureType: skinColors?.textureType,
            texturePath: skinColors?.texturePath,
            textureScale: skinColors?.textureScale,
            textureMaps: skinColors?.textureMaps
        };
    }

    /**
     * Build complete dsnConfig object for dice rolls
     */
    buildDsnConfig(diceType: string): any {
        const appearance = this.getAppearanceForDice(diceType);

        console.log('[DiceAppearanceService] Building dsnConfig for', diceType, appearance);

        const config = {
            appearance: {
                system: appearance.system,
                colorset: appearance.colorset,
                material: appearance.material,
                labelColor: appearance.labelColor,
                diceColor: appearance.diceColor,
                outlineColor: appearance.outlineColor,
                edgeColor: appearance.edgeColor,
                isGhost: appearance.isGhost || false
            },
            systemSettings: {},
            result: {
                isVisible: true,
                rollOrder: 0
            }
        };

        console.log('[DiceAppearanceService] Final dsnConfig:', config);

        return config;
    }

    /**
     * Update user's global appearance settings
     */
    async setGlobalAppearance(appearance: Partial<DiceAppearance>): Promise<void> {
        const current = userFlagsManager.getFlag('dice-so-nice', 'appearance') || { global: {} };

        current.global = {
            ...current.global,
            ...appearance
        };

        await userFlagsManager.setFlag('dice-so-nice', 'appearance', current);
    }

    /**
     * Update dice-specific appearance settings
     */
    async setDiceAppearance(diceType: string, appearance: Partial<DiceAppearance>): Promise<void> {
        const current = userFlagsManager.getFlag('dice-so-nice', 'appearance') || { global: {} };

        current[diceType] = {
            ...(current[diceType] || {}),
            ...appearance
        };

        await userFlagsManager.setFlag('dice-so-nice', 'appearance', current);
    }

    /**
     * Reset to default appearance
     */
    async resetAppearance(diceType?: string): Promise<void> {
        if (diceType) {
            const current = userFlagsManager.getFlag('dice-so-nice', 'appearance') || { global: {} };
            delete current[diceType];
            await userFlagsManager.setFlag('dice-so-nice', 'appearance', current);
        } else {
            await userFlagsManager.setFlag('dice-so-nice', 'appearance', {
                global: {
                    colorset: 'custom',
                    system: 'standard',
                    material: 'plastic',
                    labelColor: '#000000',
                    diceColor: '#ffffff',
                    outlineColor: '#000000',
                    edgeColor: '#000000'
                }
            });
        }
    }
}

export const diceAppearanceService = new DiceAppearanceService();
