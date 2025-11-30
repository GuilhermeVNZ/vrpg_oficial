import { GridCoords } from '../state/types';

interface PathNode {
    x: number;
    y: number;
    g: number; // Cost from start
    h: number; // Heuristic to end
    f: number; // g + h
    parent: PathNode | null;
}

export class PathFinder {
    private gridWidth: number;
    private gridHeight: number;
    private obstacles: Set<string>;

    constructor(gridWidth: number, gridHeight: number) {
        this.gridWidth = gridWidth;
        this.gridHeight = gridHeight;
        this.obstacles = new Set();
    }

    /**
     * Add obstacle cells that block movement
     */
    public addObstacle(x: number, y: number): void {
        this.obstacles.add(`${x},${y}`);
    }

    /**
     * Remove obstacle
     */
    public removeObstacle(x: number, y: number): void {
        this.obstacles.delete(`${x},${y}`);
    }

    /**
     * Check if cell is walkable
     */
    private isWalkable(x: number, y: number): boolean {
        // Out of bounds
        if (x < 0 || x >= this.gridWidth || y < 0 || y >= this.gridHeight) {
            return false;
        }

        // Blocked by obstacle
        if (this.obstacles.has(`${x},${y}`)) {
            return false;
        }

        return true;
    }

    /**
     * Chebyshev distance heuristic (for 8-way movement where diagonal cost = 1)
     */
    private heuristic(a: GridCoords, b: GridCoords): number {
        return Math.max(Math.abs(a.x - b.x), Math.abs(a.y - b.y));
    }

    /**
     * Get neighbor cells (4-directional movement)
     */
    private getNeighbors(node: PathNode): PathNode[] {
        const neighbors: PathNode[] = [];
        const directions = [
            { x: 0, y: -1 }, // North
            { x: 1, y: 0 },  // East
            { x: 0, y: 1 },  // South
            { x: -1, y: 0 }, // West
            { x: 1, y: -1 }, // North-East
            { x: 1, y: 1 },  // South-East
            { x: -1, y: 1 }, // South-West
            { x: -1, y: -1 } // North-West
        ];

        for (const dir of directions) {
            const x = node.x + dir.x;
            const y = node.y + dir.y;

            if (this.isWalkable(x, y)) {
                neighbors.push({
                    x,
                    y,
                    g: 0,
                    h: 0,
                    f: 0,
                    parent: null
                });
            }
        }

        return neighbors;
    }

    /**
     * Find path from start to end using A*
     */
    public findPath(start: GridCoords, end: GridCoords): GridCoords[] | null {
        // If start or end is not walkable, return null
        if (!this.isWalkable(start.x, start.y) || !this.isWalkable(end.x, end.y)) {
            return null;
        }

        const openList: PathNode[] = [];
        const closedList: Set<string> = new Set();

        // Create start node
        const startNode: PathNode = {
            x: start.x,
            y: start.y,
            g: 0,
            h: this.heuristic(start, end),
            f: 0,
            parent: null
        };
        startNode.f = startNode.g + startNode.h;
        openList.push(startNode);

        while (openList.length > 0) {
            // Get node with lowest f score
            let currentIndex = 0;
            for (let i = 1; i < openList.length; i++) {
                const current = openList[currentIndex];
                const next = openList[i];
                if (current && next && next.f < current.f) {
                    currentIndex = i;
                }
            }

            const current = openList[currentIndex];
            if (!current) break; // Should not happen

            // Found the goal
            if (current.x === end.x && current.y === end.y) {
                return this.reconstructPath(current);
            }

            // Move current from open to closed
            openList.splice(currentIndex, 1);
            closedList.add(`${current.x},${current.y}`);

            // Check all neighbors
            const neighbors = this.getNeighbors(current);
            for (const neighbor of neighbors) {
                const neighborKey = `${neighbor.x},${neighbor.y}`;

                // Skip if already evaluated
                if (closedList.has(neighborKey)) {
                    continue;
                }

                // Calculate g score
                neighbor.g = current.g + 1; // Cost to move to neighbor
                neighbor.h = this.heuristic(neighbor, end);
                neighbor.f = neighbor.g + neighbor.h;
                neighbor.parent = current;

                // Check if neighbor is already in open list with better score
                const existingIndex = openList.findIndex(n => n.x === neighbor.x && n.y === neighbor.y);
                if (existingIndex !== -1) {
                    if (neighbor.g < openList[existingIndex].g) {
                        openList[existingIndex] = neighbor;
                    }
                } else {
                    openList.push(neighbor);
                }
            }
        }

        // No path found
        return null;
    }

    /**
     * Reconstruct path from end node to start
     */
    private reconstructPath(endNode: PathNode): GridCoords[] {
        const path: GridCoords[] = [];
        let current: PathNode | null = endNode;

        while (current !== null) {
            path.unshift({ x: current.x, y: current.y });
            current = current.parent;
        }

        return path;
    }

    /**
     * Calculate all cells within movement range (BFS)
     */
    public getMovementRange(start: GridCoords, maxDistance: number): GridCoords[] {
        const reachable: GridCoords[] = [];
        const visited: Set<string> = new Set();
        const queue: Array<{ x: number; y: number; distance: number }> = [];

        queue.push({ x: start.x, y: start.y, distance: 0 });
        visited.add(`${start.x},${start.y}`);

        while (queue.length > 0) {
            const current = queue.shift()!;

            if (current.distance <= maxDistance) {
                reachable.push({ x: current.x, y: current.y });
            }

            if (current.distance < maxDistance) {
                const directions = [
                    { x: 0, y: -1 },
                    { x: 1, y: 0 },
                    { x: 0, y: 1 },
                    { x: -1, y: 0 },
                ];

                for (const dir of directions) {
                    const nx = current.x + dir.x;
                    const ny = current.y + dir.y;
                    const key = `${nx},${ny}`;

                    if (!visited.has(key) && this.isWalkable(nx, ny)) {
                        visited.add(key);
                        queue.push({ x: nx, y: ny, distance: current.distance + 1 });
                    }
                }
            }
        }

        return reachable;
    }
}
