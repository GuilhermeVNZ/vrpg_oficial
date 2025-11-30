import * as PIXI from 'pixi.js';
import { gridToIso, ISO_TILE_WIDTH, ISO_TILE_HEIGHT } from '../../grid/CoordinateConverter';

export interface GridLayerOptions {
    gridWidth: number;   // Number of cells horizontally
    gridHeight: number;  // Number of cells vertically
    lineColor: number;   // Grid line color
    lineAlpha: number;   // Grid line opacity
    visible: boolean;    // Show/hide grid
}

export class GridLayer extends PIXI.Container {
    private graphics: PIXI.Graphics;
    private options: GridLayerOptions;

    constructor(options: Partial<GridLayerOptions> = {}) {
        super();

        this.options = {
            gridWidth: 20,
            gridHeight: 20,
            lineColor: 0xFFFFFF,
            lineAlpha: 0.2,
            visible: true,
            ...options
        };

        this.graphics = new PIXI.Graphics();
        this.addChild(this.graphics);
        this.visible = this.options.visible;

        this.render();
    }

    /**
     * Render isometric grid
     */
    private render(): void {
        const { gridWidth, gridHeight, lineColor, lineAlpha } = this.options;

        this.graphics.clear();

        // Draw horizontal lines
        for (let y = 0; y <= gridHeight; y++) {
            const start = gridToIso(0, y);
            const end = gridToIso(gridWidth, y);
            this.graphics.moveTo(start.x, start.y);
            this.graphics.lineTo(end.x, end.y);
        }

        // Draw vertical lines
        for (let x = 0; x <= gridWidth; x++) {
            const start = gridToIso(x, 0);
            const end = gridToIso(x, gridHeight);
            this.graphics.moveTo(start.x, start.y);
            this.graphics.lineTo(end.x, end.y);
        }

        this.graphics.stroke({ width: 1, color: lineColor, alpha: lineAlpha });
    }

    /**
     * Toggle grid visibility
     */
    toggleVisibility(): void {
        this.visible = !this.visible;
    }

    /**
     * Highlight specific cell
     */
    highlightCell(gridX: number, gridY: number, color: number = 0xFFFF00, alpha: number = 0.3): void {
        const topLeft = gridToIso(gridX, gridY);
        const topRight = gridToIso(gridX + 1, gridY);
        const bottomRight = gridToIso(gridX + 1, gridY + 1);
        const bottomLeft = gridToIso(gridX, gridY + 1);

        this.graphics.poly([
            topLeft.x, topLeft.y,
            topRight.x, topRight.y,
            bottomRight.x, bottomRight.y,
            bottomLeft.x, bottomLeft.y
        ]);
        this.graphics.fill({ color, alpha });

        // Redraw grid lines on top
        this.render();
    }

    /**
     * Clear all highlights
     */
    clearHighlights(): void {
        this.render();
    }
}
