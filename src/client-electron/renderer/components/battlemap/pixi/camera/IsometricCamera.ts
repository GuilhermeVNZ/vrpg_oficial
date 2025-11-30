import { GridCoords } from '../../grid/CoordinateConverter';

/**
 * Camera state for isometric view
 */
export interface CameraState {
    x: number;        // Camera position X (world space)
    y: number;        // Camera position Y (world space)
    zoom: number;     // Zoom level (1.0 = 100%)
    minZoom: number;  // Minimum zoom allowed
    maxZoom: number;  // Maximum zoom allowed
}

export class IsometricCamera {
    private state: CameraState;

    constructor() {
        this.state = {
            x: 0,
            y: 0,
            zoom: 1.0,
            minZoom: 0.5,
            maxZoom: 2.0
        };
    }

    /**
     * Get current camera state
     */
    getState(): Readonly<CameraState> {
        return { ...this.state };
    }

    /**
     * Set camera position
     */
    setPosition(x: number, y: number): void {
        this.state.x = x;
        this.state.y = y;
    }

    private bounds: {
        map: { x: number; y: number; width: number; height: number };
        viewport: { width: number; height: number };
    } | null = null;

    /**
     * Set bounds for clamping
     */
    setBounds(
        mapBounds: { x: number; y: number; width: number; height: number },
        viewport: { width: number; height: number }
    ): void {
        this.bounds = { map: mapBounds, viewport };
        this.clampPosition();
    }

    /**
     * Clamp camera position to keep viewport inside map
     */
    private clampPosition(): void {
        if (!this.bounds) return;

        const { map, viewport } = this.bounds;
        const zoom = this.state.zoom;

        // Calculate min/max x/y for the camera (worldContainer position)
        // We want:
        // 1. mapLeft <= 0 (screen space) -> worldX + map.x * zoom <= 0 -> worldX <= -map.x * zoom
        // 2. mapRight >= viewportWidth -> worldX + (map.x + map.width) * zoom >= viewportWidth 
        //    -> worldX >= viewportWidth - (map.x + map.width) * zoom

        const minX = viewport.width - (map.x + map.width) * zoom;
        const maxX = -map.x * zoom;

        const minY = viewport.height - (map.y + map.height) * zoom;
        const maxY = -map.y * zoom;

        // Apply clamping
        // If map is smaller than viewport (shouldn't happen with correct minZoom), center it
        if (minX > maxX) {
            this.state.x = (viewport.width - map.width * zoom) / 2 - map.x * zoom;
        } else {
            this.state.x = Math.max(minX, Math.min(maxX, this.state.x));
        }

        if (minY > maxY) {
            this.state.y = (viewport.height - map.height * zoom) / 2 - map.y * zoom;
        } else {
            this.state.y = Math.max(minY, Math.min(maxY, this.state.y));
        }
    }

    /**
     * Pan camera by delta
     */
    pan(dx: number, dy: number): void {
        this.state.x += dx;
        this.state.y += dy;
        this.clampPosition();
    }

    /**
     * Set zoom level (clamped to min/max)
     */
    setZoom(zoom: number): void {
        this.state.zoom = Math.max(
            this.state.minZoom,
            Math.min(this.state.maxZoom, zoom)
        );
        this.clampPosition();
    }

    /**
     * Zoom by delta (multiplicative) centered on a specific screen point
     */
    zoom(delta: number, centerX: number = 0, centerY: number = 0): void {
        const oldZoom = this.state.zoom;
        const newZoom = Math.max(
            this.state.minZoom,
            Math.min(this.state.maxZoom, oldZoom * delta)
        );

        // If zoom didn't change (hit limit), don't move camera
        if (oldZoom === newZoom) return;

        // Calculate world position of the center point
        const worldX = (centerX - this.state.x) / oldZoom;
        const worldY = (centerY - this.state.y) / oldZoom;

        // Update zoom
        this.state.zoom = newZoom;

        // Adjust camera position to keep the world point at the same screen position
        this.state.x = centerX - worldX * newZoom;
        this.state.y = centerY - worldY * newZoom;

        this.clampPosition();
    }

    /**
     * Set zoom limits dynamically based on battlemap bounds
     * Ensures that zoom out never shows black areas outside the battlemap
     */
    setZoomLimits(battlemapWidth: number, battlemapHeight: number, viewportWidth: number, viewportHeight: number): void {
        // Calculate minimum zoom that fits the entire battlemap in viewport
        const minZoomX = viewportWidth / battlemapWidth;
        const minZoomY = viewportHeight / battlemapHeight;

        // Use the smaller value to ensure battlemap fills viewport without black areas
        this.state.minZoom = Math.max(minZoomX, minZoomY);

        // Clamp current zoom if it's below new minimum
        if (this.state.zoom < this.state.minZoom) {
            this.state.zoom = this.state.minZoom;
        }

        console.log('[Camera] Zoom limits set:', {
            minZoom: this.state.minZoom,
            maxZoom: this.state.maxZoom,
            battlemapSize: { width: battlemapWidth, height: battlemapHeight },
            viewportSize: { width: viewportWidth, height: viewportHeight }
        });
    }

    /**
     * Center camera on specific grid cell
     */
    centerOnCell(gridX: number, gridY: number, viewportWidth: number, viewportHeight: number): void {
        // Calculate isometric position of cell
        const isoX = (gridX - gridY) * 64;
        const isoY = (gridX + gridY) * 32;

        // Center on viewport
        this.state.x = viewportWidth / 2 - isoX * this.state.zoom;
        this.state.y = viewportHeight / 2 - isoY * this.state.zoom;
    }
}
