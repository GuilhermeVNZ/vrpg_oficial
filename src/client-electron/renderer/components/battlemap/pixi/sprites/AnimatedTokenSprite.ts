import * as PIXI from 'pixi.js';
import { SpriteLoader } from './SpriteLoader';
import { gridToIso } from '../../grid/CoordinateConverter';

export interface AnimatedTokenOptions {
    texturePath: string;
    gridX: number;
    gridY: number;
    frameWidth?: number;
    frameHeight?: number;
    animationSpeed?: number;
}

export type MovementMode = 'Walk' | 'Fly' | 'Swim' | 'Burrow' | 'Climb';

export class AnimatedTokenSprite extends PIXI.Container {
    private sprite: PIXI.AnimatedSprite | null = null;
    private shadow: PIXI.Graphics;
    private options: AnimatedTokenOptions;
    private loader: SpriteLoader;

    // Stats
    private movementSpeeds: Record<MovementMode, number> = {
        'Walk': 60,
        'Fly': 80,
        'Swim': 0,
        'Burrow': 0,
        'Climb': 40
    };
    private currentMovementMode: MovementMode;
    private distanceMoved: number = 0; // Distance moved this turn

    private maxHealth: number = 200;
    private currentHealth: number = 200;

    // Stats Overlay
    private statsContainer: PIXI.Container;
    private statsBg: PIXI.Graphics;
    private statsText: PIXI.Text;

    // UI
    private barsContainer: PIXI.Container;
    private healthBar: PIXI.Graphics;
    private movementBar: PIXI.Graphics;

    constructor(options: AnimatedTokenOptions) {
        super();
        this.options = {
            frameWidth: 341, // 1024 / 3 approx
            frameHeight: 341,
            animationSpeed: 0.15,
            ...options
        };
        this.loader = SpriteLoader.getInstance();

        // Initialize current movement mode to the fastest available speed
        const speeds = Object.entries(this.movementSpeeds) as [MovementMode, number][];
        const fastestMode = speeds
            .filter(([_, speed]) => speed > 0)
            .sort((a, b) => b[1] - a[1])[0];
        this.currentMovementMode = fastestMode ? fastestMode[0] : 'Walk';


        // Create shadow
        this.shadow = new PIXI.Graphics();
        this.drawShadow();
        this.addChild(this.shadow);

        // Set interactivity
        this.eventMode = 'static';
        this.cursor = 'pointer';
        // Define hit area (larger for easier clicking)
        this.hitArea = new PIXI.Rectangle(-75, -75, 150, 150);

        // Load and create sprite
        this.loadSprite();

        // Create UI Bars
        this.barsContainer = new PIXI.Container();
        this.barsContainer.visible = false; // Hidden by default
        this.healthBar = new PIXI.Graphics();
        this.movementBar = new PIXI.Graphics();
        this.barsContainer.addChild(this.healthBar);
        this.barsContainer.addChild(this.movementBar);
        this.addChild(this.barsContainer);

        // Create Stats Overlay
        this.statsContainer = new PIXI.Container();
        this.statsContainer.visible = false;
        this.statsBg = new PIXI.Graphics();
        this.statsText = new PIXI.Text({
            text: '',
            style: {
                fontFamily: 'Arial',
                fontSize: 14,
                fill: 0xFFFFFF,
                align: 'left',
                dropShadow: {
                    alpha: 0.5,
                    angle: 0.5,
                    blur: 2,
                    color: '#000000',
                    distance: 1,
                },
            }
        });
        this.statsContainer.addChild(this.statsBg);
        this.statsContainer.addChild(this.statsText);
        this.addChild(this.statsContainer);

        // Hover events for stats
        this.on('pointerover', () => {
            this.drawStats();
            this.statsContainer.visible = true;
        });
        this.on('pointerout', () => {
            this.statsContainer.visible = false;
        });

        // Set initial position
        this.updatePosition();
    }

    private drawShadow(): void {
        this.shadow.clear();
        this.shadow.ellipse(0, 0, 40, 20); // Isometric shadow
        this.shadow.fill({ color: 0x000000, alpha: 0.4 });
        this.shadow.position.set(0, 50); // Aligned with selection ring
    }

    private drawStats(): void {
        // Mock data - in real app this would come from props/state
        const name = "Red Dragon";
        const hp = `${this.currentHealth}/${this.maxHealth}`;
        const ac = "19";

        // Sort speeds descending
        const speedText = (Object.entries(this.movementSpeeds) as [MovementMode, number][])
            .filter(([_, speed]) => speed > 0)
            .sort((a, b) => b[1] - a[1])
            .map(([mode, speed]) => `${speed}ft ${mode}`)
            .join(', ');

        const buffs = "Frightful Presence";
        const debuffs = "None";

        const text = `Name: ${name}\nHP: ${hp}\nAC: ${ac}\nSpeed: ${speedText}\nBuffs: ${buffs}\nDebuffs: ${debuffs}`;

        this.statsText.text = text;
        this.statsText.position.set(10, 10);

        // Background
        const padding = 10;
        const width = this.statsText.width + padding * 2;
        const height = this.statsText.height + padding * 2;

        this.statsBg.clear();
        this.statsBg.roundRect(0, 0, width, height, 8);
        this.statsBg.fill({ color: 0x000000, alpha: 0.8 });
        this.statsBg.stroke({ width: 2, color: 0xFFFFFF, alpha: 0.5 });

        // Position above token
        this.statsContainer.position.set(-width / 2, -250);
        this.statsContainer.zIndex = 1000; // Always on top
    }

    public drawBars(): void {
        this.healthBar.clear();
        this.movementBar.clear();

        const barWidth = 80;
        const barHeight = 6;
        const spacing = 4;
        const yOffset = -100; // Above the sprite

        // Health Bar (Lime Green)
        const healthPercent = this.currentHealth / this.maxHealth;
        this.healthBar.rect(-barWidth / 2, yOffset, barWidth, barHeight);
        this.healthBar.fill({ color: 0x333333, alpha: 0.8 }); // Background
        this.healthBar.rect(-barWidth / 2, yOffset, barWidth * healthPercent, barHeight);
        this.healthBar.fill({ color: 0x00FF00 }); // Lime Green

        // Movement Bar (Yellow Ticks)
        // 1 tick = 5 feet
        // Always draw ticks based on MAX speed (e.g., 80ft = 16 ticks)
        const maxSpeed = Math.max(...Object.values(this.movementSpeeds));
        const currentSpeed = this.movementSpeeds[this.currentMovementMode];
        const remaining = Math.max(0, currentSpeed - this.distanceMoved);

        const maxTicks = Math.ceil(maxSpeed / 5);
        const currentModeTicks = Math.ceil(currentSpeed / 5);
        const activeTicks = Math.ceil(remaining / 5);

        console.log('[drawBars]', {
            currentMode: this.currentMovementMode,
            maxSpeed,
            currentSpeed,
            distanceMoved: this.distanceMoved,
            remaining,
            maxTicks,
            currentModeTicks,
            activeTicks
        });

        // Tick width is calculated by dividing the total bar width by the max possible ticks
        const tickWidth = (barWidth - (maxTicks - 1)) / maxTicks;

        const startX = -barWidth / 2;
        const movementY = yOffset + barHeight + spacing;

        // Always draw ALL max ticks, but color them based on current mode availability
        for (let i = 0; i < maxTicks; i++) {
            const x = startX + i * (tickWidth + 1);

            this.movementBar.rect(x, movementY, tickWidth, barHeight);

            if (i < activeTicks) {
                // Active: within remaining movement for current mode
                this.movementBar.fill({ color: 0xFFFF00 }); // Yellow
            } else if (i < currentModeTicks) {
                // Used: within current mode's total, but already consumed
                this.movementBar.fill({ color: 0x333333, alpha: 0.5 }); // Dark (used)
            } else {
                // Beyond current mode's capability
                this.movementBar.fill({ color: 0x222222, alpha: 0.3 }); // Very dark (not available)
            }
        }
    }

    public setBarsVisible(visible: boolean): void {
        this.barsContainer.visible = visible;
        if (visible) {
            // Just redraw the bars without resetting movement
            this.drawBars();
        }
    }

    public consumeMovement(feet: number): void {
        console.log('[consumeMovement]', {
            consuming: feet,
            before: this.distanceMoved,
            after: this.distanceMoved + feet,
            currentMode: this.currentMovementMode
        });
        this.distanceMoved += feet;
        this.drawBars();
    }

    /**
     * Calculate movement cost based on D&D 5e rules
     * @param distance Distance in feet
     * @param mode Movement mode
     * @returns Actual movement cost in feet
     */
    public getMovementCost(distance: number, mode: MovementMode): number {
        // D&D 5e Rule: Climbing and Swimming cost 2× movement ONLY if you don't have that speed
        // If you HAVE climb/swim speed, it costs 1:1 (normal)
        const hasThisSpeed = this.movementSpeeds[mode] > 0;

        if (!hasThisSpeed && (mode === 'Climb' || mode === 'Swim')) {
            // Trying to climb/swim without having that speed type = 2× cost
            return distance * 2;
        }

        // Normal cost (1:1) for:
        // - Walk, Fly, Burrow (always 1:1)
        // - Climb/Swim if creature HAS that speed
        return distance;
    }

    public getRemainingMovement(mode?: MovementMode): number {
        const targetMode = mode || this.currentMovementMode;
        const speed = this.movementSpeeds[targetMode];
        // D&D 5e Rule: Subtract distance already moved from new speed
        return Math.max(0, speed - this.distanceMoved);
    }

    public resetMovement(): void {
        this.distanceMoved = 0;
        this.drawBars();
    }

    public setMovementMode(mode: MovementMode): void {
        if (this.movementSpeeds[mode] > 0) {
            this.currentMovementMode = mode;
            this.drawBars();
        }
    }

    public getAvailableMovementModes(): MovementMode[] {
        return (Object.keys(this.movementSpeeds) as MovementMode[])
            .filter(mode => this.movementSpeeds[mode] > 0);
    }

    private async loadSprite(): Promise<void> {
        try {
            const texture = await this.loader.loadTexture(this.options.texturePath);

            // Create frames (3x3 grid)
            const frames: PIXI.Texture[] = [];
            const cols = 3;
            const rows = 3;
            const width = texture.width / cols;
            const height = texture.height / rows;

            for (let i = 0; i < rows * cols; i++) {
                const col = i % cols;
                const row = Math.floor(i / cols);

                const frame = new PIXI.Texture({
                    source: texture.source,
                    frame: new PIXI.Rectangle(
                        col * width,
                        row * height,
                        width,
                        height
                    )
                });
                frames.push(frame);
            }

            // Create AnimatedSprite
            this.sprite = new PIXI.AnimatedSprite(frames);
            this.sprite.anchor.set(0.5, 0.5); // Center anchor for grid alignment
            this.sprite.animationSpeed = this.options.animationSpeed!;
            this.sprite.play();

            // Scale down to fit grid (adjust as needed)
            const scale = 0.5;
            this.sprite.scale.set(scale);

            this.addChild(this.sprite);
        } catch (error) {
            console.error('[AnimatedTokenSprite] Failed to load sprite:', error);
        }
    }

    private updatePosition(): void {
        const iso = gridToIso(this.options.gridX, this.options.gridY);
        // Add vertical offset to visually center the sprite in the isometric cell
        // This is needed because the sprite's visual mass is typically in the lower portion
        const verticalOffset = -15; // Negative moves up (adjusted from -20)
        this.position.set(iso.x, iso.y + verticalOffset);
        // Z-index sorting based on Y position
        this.zIndex = this.options.gridY + this.options.gridX;
    }

    public get gridX(): number {
        return this.options.gridX;
    }

    public set gridX(value: number) {
        this.options.gridX = value;
    }

    public get gridY(): number {
        return this.options.gridY;
    }

    public set gridY(value: number) {
        this.options.gridY = value;
    }

    /**
     * Move token along a path of grid coordinates
     */
    public async moveAlongPath(path: { x: number, y: number }[]): Promise<void> {
        if (path.length === 0) return;

        // Skip the first node if it's the current position
        const startNode = path[0];
        let startIndex = 0;
        if (startNode && startNode.x === this.options.gridX && startNode.y === this.options.gridY) {
            startIndex = 1;
        }

        for (let i = startIndex; i < path.length; i++) {
            const node = path[i];
            if (node) {
                await new Promise<void>(resolve => {
                    this.moveTo(node.x, node.y, resolve);
                });
            }
        }
    }

    /**
     * Move token to new grid coordinates with animation
     */
    public moveTo(gridX: number, gridY: number, onComplete?: () => void): void {
        const startIso = { x: this.position.x, y: this.position.y };
        const endIso = gridToIso(gridX, gridY);

        // Apply the same vertical offset as in updatePosition
        const verticalOffset = -15;
        const endX = endIso.x;
        const endY = endIso.y + verticalOffset;

        // Update logical position immediately
        this.options.gridX = gridX;
        this.options.gridY = gridY;
        this.zIndex = gridY + gridX; // Update Z-index for depth sorting

        // Animation variables
        const startTime = Date.now();
        const duration = 200; // Faster for path movement

        const animate = () => {
            const now = Date.now();
            const progress = Math.min((now - startTime) / duration, 1);

            // Ease out cubic
            const ease = 1 - Math.pow(1 - progress, 3);

            const currentX = startIso.x + (endX - startIso.x) * ease;
            const currentY = startIso.y + (endY - startIso.y) * ease;

            this.position.set(currentX, currentY);

            if (progress < 1) {
                requestAnimationFrame(animate);
            } else {
                if (onComplete) onComplete();
            }
        };

        requestAnimationFrame(animate);
    }
}
