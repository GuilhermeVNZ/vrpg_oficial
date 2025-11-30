import * as PIXI from 'pixi.js';
import { ISO_TILE_WIDTH, ISO_TILE_HEIGHT } from '../../grid/CoordinateConverter';

export interface TerrainLayerOptions {
    imagePath: string;
    gridWidth: number;
    gridHeight: number;
    onLoaded?: () => void;
}

export class TerrainLayer extends PIXI.Container {
    private sprite: PIXI.Sprite | null = null;
    private gridWidth: number;
    private gridHeight: number;
    private battlemapWidth: number = 0;
    private battlemapHeight: number = 0;
    private onLoaded?: () => void;

    constructor(options: TerrainLayerOptions) {
        super();
        this.gridWidth = options.gridWidth;
        this.gridHeight = options.gridHeight;
        this.onLoaded = options.onLoaded;
        this.sortableChildren = true;
        this.zIndex = -1; // Behind everything

        this.loadTerrain(options.imagePath);
    }

    private async loadTerrain(imagePath: string): Promise<void> {
        try {
            const texture = await PIXI.Assets.load(imagePath);

            this.sprite = new PIXI.Sprite(texture);
            this.sprite.anchor.set(0.5, 0.5);

            // FIXED SCALE: We want to preserve the "validated" cell size relative to the map.
            // In the 60x60 version:
            // Grid Width (Iso) = (60+60) * 64 = 7680
            // Image Width = 4096
            // Scale = 7680 / 4096 = 1.875
            // User requested 25% reduction in "approximation" (zoom/scale):
            // 1.875 * 0.75 = 1.40625
            const scale = 1.40625;
            this.sprite.scale.set(scale);

            // Store actual battlemap dimensions after scaling
            this.battlemapWidth = texture.width * scale;
            this.battlemapHeight = texture.height * scale;

            // Center the image on the grid
            // Grid center in ISO coords is at x=0, y=(total_grid_height_iso / 2)
            // Total grid height in iso = (gridWidth + gridHeight) * (ISO_TILE_HEIGHT / 2)
            const gridCenterY = (this.gridWidth + this.gridHeight) * (ISO_TILE_HEIGHT / 4);

            this.sprite.position.set(0, gridCenterY);

            this.addChild(this.sprite);

            console.log('[TerrainLayer] Loaded:', imagePath);
            console.log('[TerrainLayer] Scale fixed to:', scale);
            console.log('[TerrainLayer] Battlemap size:', this.battlemapWidth, 'x', this.battlemapHeight);

            if (this.onLoaded) {
                this.onLoaded();
            }
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

    /**
     * Get the local bounds of the map sprite relative to this container
     * Used for camera clamping
     */
    public getMapLocalBounds(): { x: number; y: number; width: number; height: number } {
        if (!this.sprite) return { x: 0, y: 0, width: 0, height: 0 };

        // Sprite is anchored at 0.5, 0.5
        // Position is (0, gridCenterY)
        const gridCenterY = (this.gridWidth + this.gridHeight) * (ISO_TILE_HEIGHT / 4);

        return {
            x: -this.battlemapWidth / 2,
            y: gridCenterY - this.battlemapHeight / 2,
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
