import * as PIXI from 'pixi.js';
import { ISO_TILE_WIDTH, ISO_TILE_HEIGHT } from '../../grid/CoordinateConverter';

export interface TerrainLayerOptions {
    imagePath: string;
    gridWidth: number;
    gridHeight: number;
}

export class TerrainLayer extends PIXI.Container {
    private sprite: PIXI.Sprite | null = null;
    private gridWidth: number;
    private gridHeight: number;
    private battlemapWidth: number = 0;
    private battlemapHeight: number = 0;

    constructor(options: TerrainLayerOptions) {
        super();
        this.gridWidth = options.gridWidth;
        this.gridHeight = options.gridHeight;
        this.sortableChildren = true;
        this.zIndex = -1; // Behind everything

        this.loadTerrain(options.imagePath);
    }

    private async loadTerrain(imagePath: string): Promise<void> {
        try {
            const texture = await PIXI.Assets.load(imagePath);

            this.sprite = new PIXI.Sprite(texture);
            this.sprite.anchor.set(0.5, 0.5);

            // Calculate isometric grid bounds
            const totalWidth = (this.gridWidth + this.gridHeight) * (ISO_TILE_WIDTH / 2);
            const totalHeight = (this.gridWidth + this.gridHeight) * (ISO_TILE_HEIGHT / 2);

            // Scale to cover the entire grid
            const scaleX = totalWidth / texture.width;
            const scaleY = totalHeight / texture.height;
            const scale = Math.max(scaleX, scaleY);

            this.sprite.scale.set(scale);

            // Store actual battlemap dimensions after scaling
            this.battlemapWidth = texture.width * scale;
            this.battlemapHeight = texture.height * scale;

            // Position at center
            this.sprite.position.set(0, totalHeight / 2);

            this.addChild(this.sprite);

            console.log('[TerrainLayer] Loaded:', imagePath);
            console.log('[TerrainLayer] Battlemap size:', this.battlemapWidth, 'x', this.battlemapHeight);
        } catch (error) {
            console.error('[TerrainLayer] Load error:', error);
        }
    }

    /**
     * Get battlemap dimensions (after scaling)
     */
    public getBattlemapBounds(): { width: number; height: number } {
        return {
            width: this.battlemapWidth,
            height: this.battlemapHeight
        };
    }

    public destroy(): void {
        if (this.sprite) {
            this.sprite.destroy();
            this.sprite = null;
        }
        super.destroy();
    }
}
