// Grid coordinate types
export interface GridCoords {
    x: number;
    y: number;
}

export interface ScreenCoords {
    x: number;
    y: number;
}

// Isometric grid constants
export const ISO_TILE_WIDTH = 128;  // Width of isometric tile
export const ISO_TILE_HEIGHT = 64;  // Height of isometric tile

/**
 * Converts grid coordinates to isometric screen coordinates
 * Origin (0,0) in grid space maps to screen center
 * UPDATED: Returns the CENTER of the tile (offset by height/2)
 */
export function gridToIso(gridX: number, gridY: number): ScreenCoords {
    const isoX = (gridX - gridY) * (ISO_TILE_WIDTH / 2);
    const isoY = (gridX + gridY) * (ISO_TILE_HEIGHT / 2) + (ISO_TILE_HEIGHT / 2);

    return { x: isoX, y: isoY };
}

/**
 * Converts isometric screen coordinates to grid coordinates
 */
export function isoToGrid(isoX: number, isoY: number): GridCoords {
    // Adjust for the center offset before converting
    const adjustedY = isoY - (ISO_TILE_HEIGHT / 2);

    const gridX = (isoX / (ISO_TILE_WIDTH / 2) + adjustedY / (ISO_TILE_HEIGHT / 2)) / 2;
    const gridY = (adjustedY / (ISO_TILE_HEIGHT / 2) - isoX / (ISO_TILE_WIDTH / 2)) / 2;

    return {
        x: Math.floor(gridX),
        y: Math.floor(gridY)
    };
}

/**
 * Converts screen mouse coordinates to grid coordinates
 * Accounts for camera offset and zoom
 */
export function screenToGrid(
    screenX: number,
    screenY: number,
    cameraX: number,
    cameraY: number,
    zoom: number
): GridCoords {
    // Adjust for camera position and zoom
    const worldX = (screenX - cameraX) / zoom;
    const worldY = (screenY - cameraY) / zoom;

    return isoToGrid(worldX, worldY);
}

/**
 * Converts grid coordinates to screen coordinates
 * Accounts for camera offset and zoom
 */
export function gridToScreen(
    gridX: number,
    gridY: number,
    cameraX: number,
    cameraY: number,
    zoom: number
): ScreenCoords {
    const iso = gridToIso(gridX, gridY);

    return {
        x: iso.x * zoom + cameraX,
        y: iso.y * zoom + cameraY
    };
}
