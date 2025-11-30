import * as PIXI from 'pixi.js';
import { GridCoords } from '../state/types';
import { gridToIso } from '../grid/CoordinateConverter';

export class HighlightManager {
    private container: PIXI.Container;
    private cellSize: number;
    private highlights: Map<string, PIXI.Graphics>;
    private pathLine: PIXI.Graphics | null;

    constructor(container: PIXI.Container, cellSize: number = 64) {
        this.container = container;
        this.cellSize = cellSize;
        this.highlights = new Map();
        this.pathLine = null;
    }

    /**
     * Highlight a single cell
     */
    public highlightCell(coords: GridCoords, color: number = 0x00FF00, alpha: number = 0.3): void {
        const key = `${coords.x},${coords.y}`;

        // Remove existing highlight
        if (this.highlights.has(key)) {
            this.container.removeChild(this.highlights.get(key)!);
            this.highlights.delete(key);
        }

        const highlight = new PIXI.Graphics();
        const iso = gridToIso(coords.x, coords.y);

        // Draw isometric diamond
        highlight.poly([
            { x: iso.x, y: iso.y - this.cellSize / 2 },       // Top
            { x: iso.x + this.cellSize / 2, y: iso.y },       // Right
            { x: iso.x, y: iso.y + this.cellSize / 2 },       // Bottom
            { x: iso.x - this.cellSize / 2, y: iso.y },       // Left
        ]);
        highlight.fill({ color, alpha });
        highlight.zIndex = 1; // Above grid, below tokens

        this.highlights.set(key, highlight);
        this.container.addChild(highlight);
    }   /**
     * Highlight multiple cells (e.g., movement range)
     */
    public highlightCells(cells: GridCoords[], color: number = 0x0088FF, alpha: number = 0.2): void {
        for (const cell of cells) {
            this.highlightCell(cell, color, alpha);
        }
    }

    /**
     * Clear specific cell highlight
     */
    public clearCell(coords: GridCoords): void {
        const key = `${coords.x},${coords.y}`;
        if (this.highlights.has(key)) {
            this.container.removeChild(this.highlights.get(key)!);
            this.highlights.delete(key);
        }
    }

    /**
     * Clear all highlights
     */
    public clearAll(): void {
        for (const highlight of this.highlights.values()) {
            this.container.removeChild(highlight);
        }
        this.highlights.clear();

        if (this.pathLine) {
            this.container.removeChild(this.pathLine);
            this.pathLine = null;
        }
    }

    /**
     * Draw a path line through cells
     */
    public drawPath(path: GridCoords[], color: number = 0xFFFF00, width: number = 3): void {
        // Clear existing path
        if (this.pathLine) {
            this.container.removeChild(this.pathLine);
        }

        if (path.length < 2) return;

        this.pathLine = new PIXI.Graphics();

        // Convert to isometric coordinates
        // Add vertical offset to match selection ring (iso.y + 35)
        const verticalOffset = 35;
        const isoPath = path.map(p => {
            const iso = gridToIso(p.x, p.y);
            return { x: iso.x, y: iso.y + verticalOffset };
        });

        // Draw line
        if (isoPath[0]) {
            this.pathLine.moveTo(isoPath[0].x, isoPath[0].y);
        }

        for (let i = 1; i < isoPath.length; i++) {
            if (isoPath[i]) {
                this.pathLine.lineTo(isoPath[i].x, isoPath[i].y);
            }
        }
        this.pathLine.stroke({ width, color, alpha: 0.8 });

        // Draw dots and numbers at each waypoint (excluding the last one)
        for (let i = 0; i < isoPath.length - 1; i++) {
            const iso = isoPath[i];

            // Draw dot
            this.pathLine.circle(iso.x, iso.y, 8); // Slightly larger for text
            this.pathLine.fill({ color, alpha: 1 });

            // Draw cost number
            const cost = i * 5;
            const text = new PIXI.Text({
                text: cost.toString(),
                style: {
                    fontFamily: 'Arial',
                    fontSize: 10,
                    fill: 0x000000,
                    fontWeight: 'bold',
                    align: 'center'
                }
            });
            text.anchor.set(0.5);
            text.position.set(iso.x, iso.y);
            this.pathLine.addChild(text);
        }

        this.pathLine.zIndex = 2; // Above highlights
        this.container.addChild(this.pathLine);
    }

    /**
     * Clear the path line
     */
    public clearPath(): void {
        if (this.pathLine) {
            this.container.removeChild(this.pathLine);
            this.pathLine = null;
        }
    }

    /**
     * Cleanup
     */
    public destroy(): void {
        this.clearAll();
    }
}
