import * as PIXI from 'pixi.js';
import { AnimatedTokenSprite } from './sprites/AnimatedTokenSprite';
import { HighlightManager } from './HighlightManager';
import { GridCoords } from '../state/types';
import { PathFinder } from '../grid/PathFinder';

export enum MoveResult {
    SUCCESS = 'SUCCESS',
    INVALID = 'INVALID',
    PENDING_SELECTION = 'PENDING_SELECTION'
}

export class InteractionManager {
    private selectedToken: AnimatedTokenSprite | null = null;
    private selectionRing: PIXI.Graphics;
    private stage: PIXI.Container;
    private highlightManager: HighlightManager;
    private pathFinder: PathFinder;
    private movementRange: number = 6; // example range in cells

    public onMovementSelect?: (token: AnimatedTokenSprite, x: number, y: number, modes: string[]) => void;

    constructor(stage: PIXI.Container, gridWidth: number, gridHeight: number) {
        this.stage = stage;
        this.selectionRing = new PIXI.Graphics();
        this.selectionRing.visible = false;
        this.stage.addChild(this.selectionRing);
        // Initialize HighlightManager with same container
        this.highlightManager = new HighlightManager(this.stage);
        this.pathFinder = new PathFinder(gridWidth, gridHeight);
    }

    /**
     * Select a token (Force select)
     */
    public selectToken(token: AnimatedTokenSprite): void {
        if (this.selectedToken === token) return;

        // Deselect previous
        if (this.selectedToken) {
            this.selectedToken.setBarsVisible(false);
        }

        this.selectedToken = token;
        this.selectedToken.setBarsVisible(true);
        this.updateSelectionRing();
        console.log('[InteractionManager] Selected token');
    }

    /**
     * Toggle token selection
     */
    public toggleTokenSelection(token: AnimatedTokenSprite): void {
        if (this.selectedToken === token) {
            this.deselectToken();
        } else {
            this.selectToken(token);
        }
    }

    /**
     * Deselect current token
     */
    public deselectToken(): void {
        if (this.selectedToken) {
            this.selectedToken.setBarsVisible(false);
        }
        this.selectedToken = null;
        this.selectionRing.visible = false;
        console.log('[InteractionManager] Deselected token');
        this.highlightManager.clearAll();
    }

    /**
     * Validate and move token after drag-and-drop
     */
    public async validateAndMove(token: AnimatedTokenSprite, targetGridX: number, targetGridY: number): Promise<MoveResult> {
        const start: GridCoords = { x: token.gridX, y: token.gridY };
        const end: GridCoords = { x: targetGridX, y: targetGridY };

        // If dropped on same spot, just return true (no move needed)
        if (start.x === end.x && start.y === end.y) return MoveResult.SUCCESS;

        const path = this.pathFinder.findPath(start, end);

        if (path) {
            const steps = path.length - 1;
            const distance = steps * 5; // Distance in feet

            console.log('[validateAndMove] Path analysis:', {
                pathLength: path.length,
                steps,
                distance,
                from: start,
                to: end
            });

            // Check for multiple movement modes
            const availableModes = token.getAvailableMovementModes();

            // Filter modes that have enough remaining movement for this specific path
            // IMPORTANT: Use D&D 5e cost calculation (Climb/Swim = 2×)
            const validModes = availableModes.filter(mode => {
                const cost = token.getMovementCost(distance, mode as any);
                const remaining = token.getRemainingMovement(mode as any);
                const isValid = remaining >= cost;

                console.log(`[validateAndMove] Mode ${mode}:`, {
                    distance,
                    cost,
                    remaining,
                    isValid
                });

                return isValid;
            });

            if (validModes.length >= 1) {
                // Show modal if we have any valid modes (including just one)
                if (this.onMovementSelect) {
                    this.onMovementSelect(token, targetGridX, targetGridY, validModes);
                    return MoveResult.PENDING_SELECTION;
                }
            } else {
                // No modes can cover this distance
                return MoveResult.INVALID;
            }
        }

        // Invalid move
        return MoveResult.INVALID;
    }

    public async executeMove(token: AnimatedTokenSprite, targetGridX: number, targetGridY: number, mode: string): Promise<void> {
        const start: GridCoords = { x: token.gridX, y: token.gridY };
        const end: GridCoords = { x: targetGridX, y: targetGridY };
        const path = this.pathFinder.findPath(start, end);

        if (path) {
            const steps = path.length - 1;
            const distance = steps * 5; // Distance in feet

            // Set movement mode first
            (token as any).setMovementMode(mode);

            // Calculate cost with D&D 5e rules (Climb/Swim = 2×)
            const cost = (token as any).getMovementCost(distance, mode);

            // Consume movement and execute
            token.consumeMovement(cost);
            token.moveTo(targetGridX, targetGridY);

            if (this.selectedToken === token) {
                this.updateSelectionRing();
            }
        }
    }

    /**
     * Get currently selected token
     */
    public getSelectedToken(): AnimatedTokenSprite | null {
        return this.selectedToken;
    }

    /**
     * Update selection ring position and visibility
     */
    public update(): void {
        if (this.selectedToken) {
            this.updateSelectionRing();
        }
    }

    private updateSelectionRing(): void {
        if (!this.selectedToken) return;

        const { x, y } = this.selectedToken.position;

        this.selectionRing.clear();
        this.selectionRing.ellipse(0, 0, 47, 23.5); // Increased diameter by 5px (was 42x21)
        this.selectionRing.stroke({ width: 4, color: 0x00FF00, alpha: 0.8 }); // Thicker green ring
        this.selectionRing.position.set(x, y + 50); // Moved down 2px (was +48)
        this.selectionRing.visible = true;
        this.selectionRing.zIndex = this.selectedToken.zIndex - 0.1;
    }

    public getPathFinder(): PathFinder {
        return this.pathFinder;
    }

    /**
     * Handle hover over grid cell
     */
    public handleHover(gridX: number, gridY: number): void {
        if (!this.selectedToken) return;

        const start: GridCoords = { x: this.selectedToken.gridX, y: this.selectedToken.gridY };
        const end: GridCoords = { x: gridX, y: gridY };

        // Don't calculate if hovering over start
        if (start.x === end.x && start.y === end.y) {
            this.highlightManager.clearPath();
            return;
        }

        // Calculate path
        const path = this.pathFinder.findPath(start, end);

        if (path) {
            // Calculate cost (5ft per step)
            const steps = path.length - 1;
            const cost = steps * 5;
            const remaining = this.selectedToken.getRemainingMovement();

            if (cost <= remaining) {
                this.highlightManager.drawPath(path);
            } else {
                this.highlightManager.clearPath();
            }
        } else {
            this.highlightManager.clearPath();
        }
    }
}
