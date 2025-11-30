import React, { useEffect, useRef } from 'react';
import * as PIXI from 'pixi.js';
import { GridLayer } from './pixi/layers/GridLayer';
import { TerrainLayer } from './pixi/layers/TerrainLayer';
import { IsometricCamera } from './pixi/camera/IsometricCamera';
import { AnimatedTokenSprite } from './pixi/sprites/AnimatedTokenSprite';
import { isoToGrid } from './grid/CoordinateConverter';
import { InteractionManager, MoveResult } from './pixi/InteractionManager';
import redDragonSheetImg from '../../assets/red_dragon_sheet.png';

export interface IsometricRendererProps {
    width: number;
    height: number;
    gridWidth?: number;
    gridHeight?: number;
}

export const IsometricRenderer: React.FC<IsometricRendererProps> = ({
    width,
    height,
    gridWidth = 200,
    gridHeight = 200
}) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const appRef = useRef<PIXI.Application | null>(null);
    const cameraRef = useRef<IsometricCamera>(new IsometricCamera());
    const isDraggingRef = useRef(false);
    const lastMousePosRef = useRef({ x: 0, y: 0 });
    const interactionManagerRef = useRef<InteractionManager | null>(null);
    const modalVisibleRef = useRef(false); // Track modal visibility for event handlers

    // Modal State
    const [movementModal, setMovementModal] = React.useState<{
        visible: boolean;
        token: AnimatedTokenSprite | null;
        x: number;
        y: number;
        modes: string[];
        originalPosition?: { gridX: number; gridY: number };
    }>({ visible: false, token: null, x: 0, y: 0, modes: [] });

    const [hoveredMode, setHoveredMode] = React.useState<string | null>(null);

    // Keep ref in sync with modal visibility
    useEffect(() => {
        modalVisibleRef.current = movementModal.visible;
    }, [movementModal.visible]);

    useEffect(() => {
        if (!containerRef.current) return;

        let app: PIXI.Application | null = null;
        let canvas: HTMLCanvasElement | null = null;
        let isMounted = true;

        const handlers = {
            wheel: null as ((e: WheelEvent) => void) | null,
            mousedown: null as ((e: MouseEvent) => void) | null,
            mousemove: null as ((e: MouseEvent) => void) | null,
            mouseup: null as ((e: MouseEvent) => void) | null,
            keydown: null as ((e: KeyboardEvent) => void) | null,
            contextmenu: null as ((e: Event) => void) | null,
        };

        const initPixi = async () => {
            try {
                const newApp = new PIXI.Application();

                await newApp.init({
                    width,
                    height,
                    backgroundColor: 0x1a1a1a,
                    antialias: true,
                    resolution: window.devicePixelRatio || 1,
                    autoDensity: true
                });

                if (!isMounted || !containerRef.current || !newApp.canvas) {
                    newApp.destroy(true);
                    return;
                }

                app = newApp;
                canvas = newApp.canvas;
                containerRef.current.appendChild(canvas);
                appRef.current = app;

                const worldContainer = new PIXI.Container();
                worldContainer.sortableChildren = true;
                app.stage.addChild(worldContainer);

                const interactionManager = new InteractionManager(worldContainer, gridWidth, gridHeight);
                interactionManager.onMovementSelect = (token, x, y, modes) => {
                    setMovementModal({
                        visible: true,
                        token,
                        x,
                        y,
                        modes,
                        originalPosition: { gridX: token.gridX, gridY: token.gridY }
                    });
                };
                interactionManagerRef.current = interactionManager;

                const camera = cameraRef.current;

                const updateCameraTransform = () => {
                    const state = camera.getState();
                    worldContainer.position.set(state.x, state.y);
                    worldContainer.scale.set(state.zoom, state.zoom);
                };

                // Add terrain background with onLoaded callback
                const terrainLayer = new TerrainLayer({
                    imagePath: '/battlemaps/volcanic.jpeg',
                    gridWidth,
                    gridHeight,
                    onLoaded: () => {
                        // Configure zoom and pan limits after terrain is loaded
                        const bounds = terrainLayer.getBattlemapBounds();
                        const localBounds = terrainLayer.getMapLocalBounds();

                        if (bounds.width > 0 && bounds.height > 0) {
                            camera.setZoomLimits(bounds.width, bounds.height, width, height);
                            camera.setBounds(localBounds, { width, height });

                            // Start with a zoomed out view (minZoom)
                            const state = camera.getState();
                            camera.setZoom(state.minZoom);

                            updateCameraTransform();
                            console.log('[IsometricRenderer] Terrain loaded, limits configured');
                        }
                    }
                });
                worldContainer.addChild(terrainLayer);

                const gridLayer = new GridLayer({
                    gridWidth,
                    gridHeight,
                    lineColor: 0xD4AF37,
                    lineAlpha: 0.3
                });
                worldContainer.addChild(gridLayer);

                worldContainer.eventMode = 'static';

                const bgGraphic = new PIXI.Graphics();
                bgGraphic.rect(-10000, -10000, 20000, 20000);
                bgGraphic.fill({ color: 0x000000, alpha: 0 });
                worldContainer.addChild(bgGraphic);

                // Background click deselects
                worldContainer.on('pointertap', (event) => {
                    if (isDraggingRef.current) return;
                    if (event.pointerType === 'mouse' && event.button !== 0) return;

                    interactionManager.deselectToken();
                });

                let isDraggingToken = false;
                let dragOffset = { x: 0, y: 0 };
                let startDragPos = { x: 0, y: 0 };
                let wasSelectedAtStart = false;

                const token = new AnimatedTokenSprite({
                    texturePath: redDragonSheetImg,
                    gridX: Math.floor(gridWidth / 2),
                    gridY: Math.floor(gridHeight / 2),
                    animationSpeed: 0.15
                });

                token.eventMode = 'static';
                token.cursor = 'grab';
                token.zIndex = 1;

                app.stage.eventMode = 'static';

                token.on('pointerdown', (e) => {
                    if (e.button === 0) {
                        e.stopPropagation(); // Prevent background click

                        // Block dragging if movement modal is open
                        if (modalVisibleRef.current) {
                            console.log('Drag blocked - movement modal is open');
                            return;
                        }

                        // Check if movement is available (max possible speed)
                        const maxRemaining = Math.max(...token.getAvailableMovementModes().map(m => token.getRemainingMovement(m as any)));

                        if (maxRemaining <= 0) {
                            console.log('No movement remaining');
                            return; // Lock token
                        }

                        const localPos = worldContainer.toLocal(e.global);
                        dragOffset.x = token.position.x - localPos.x;
                        dragOffset.y = token.position.y - localPos.y;
                        startDragPos = { x: token.position.x, y: token.position.y };

                        isDraggingToken = true;
                        token.cursor = 'grabbing';

                        // Track if it was selected at the start of drag
                        wasSelectedAtStart = interactionManager.getSelectedToken() === token;

                        // If it's NOT selected, select it immediately
                        if (!wasSelectedAtStart) {
                            interactionManager.selectToken(token);
                        }
                    }
                });

                // Explicitly stop propagation on tap to ensure background doesn't catch it
                token.on('pointertap', (e) => {
                    e.stopPropagation();
                });

                app.stage.on('pointermove', (e) => {
                    const localPos = worldContainer.toLocal(e.global);
                    const gridCoords = isoToGrid(localPos.x, localPos.y);

                    interactionManager.handleHover(gridCoords.x, gridCoords.y);

                    if (isDraggingToken) {
                        token.position.set(
                            localPos.x + dragOffset.x,
                            localPos.y + dragOffset.y
                        );
                    }
                });

                app.stage.on('pointerup', async (e) => {
                    if (isDraggingToken && e.button === 0) {
                        isDraggingToken = false;
                        token.cursor = 'grab';

                        // Snap based on MOUSE position, not token position
                        const localPos = worldContainer.toLocal(e.global);
                        const gridCoords = isoToGrid(localPos.x, localPos.y);

                        // Check distance for click vs drag
                        const dx = token.position.x - startDragPos.x;
                        const dy = token.position.y - startDragPos.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);
                        const isClick = dist < 10; // Increased threshold to 10

                        if (isClick) {
                            // If it was a click
                            // If it WAS selected at start, toggle it off
                            if (wasSelectedAtStart) {
                                interactionManager.deselectToken();
                            }
                            // If it wasn't selected, we already selected it in pointerdown, so keep it.

                            // Snap back to original grid pos (since we didn't really move)
                            token.moveTo(token.gridX, token.gridY);
                        } else {
                            // Dragged
                            // Validate and move
                            const result = await interactionManager.validateAndMove(token, gridCoords.x, gridCoords.y);

                            if (result === MoveResult.INVALID) {
                                // Revert to original position if invalid
                                token.moveTo(token.gridX, token.gridY);
                            }
                            // If PENDING_SELECTION, token stays at dropped position until user confirms/cancels
                        }
                    }
                });

                worldContainer.addChild(token);

                camera.centerOnCell(gridWidth / 2, gridHeight / 2, width, height);

                app.ticker.add(() => {
                    interactionManager.update();
                });

                // Use native event listener for wheel to ensure we can preventDefault (non-passive)
                handlers.wheel = (event: WheelEvent) => {
                    event.preventDefault();
                    event.stopPropagation();
                    const delta = event.deltaY > 0 ? 0.9 : 1.1;
                    // Zoom centered on the screen center
                    camera.zoom(delta, width / 2, height / 2);
                    updateCameraTransform();
                };

                handlers.mousedown = (event: MouseEvent) => {
                    if (event.button === 2) {
                        isDraggingRef.current = true;
                        lastMousePosRef.current = { x: event.clientX, y: event.clientY };
                    }
                };

                handlers.mousemove = (event: MouseEvent) => {
                    if (isDraggingRef.current) {
                        const dx = event.clientX - lastMousePosRef.current.x;
                        const dy = event.clientY - lastMousePosRef.current.y;

                        camera.pan(dx, dy);
                        updateCameraTransform();

                        lastMousePosRef.current = { x: event.clientX, y: event.clientY };
                    }
                };

                handlers.mouseup = (event: MouseEvent) => {
                    if (event.button === 2) {
                        isDraggingRef.current = false;
                    }
                };

                canvas.addEventListener('wheel', handlers.wheel, { passive: false });
                canvas.addEventListener('mousedown', handlers.mousedown);
                canvas.addEventListener('mousemove', handlers.mousemove);
                canvas.addEventListener('mouseup', handlers.mouseup);

                const handleKeyDown = (e: KeyboardEvent) => {
                    if (e.key.toLowerCase() === 'g' && !e.ctrlKey && !e.altKey && !e.metaKey) {
                        const target = e.target as HTMLElement;
                        if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return;

                        gridLayer.visible = !gridLayer.visible;
                    }
                };
                handlers.keydown = handleKeyDown;
                window.addEventListener('keydown', handlers.keydown);

                const handleContextMenu = (e: Event) => {
                    e.preventDefault();
                    e.stopPropagation();
                    return false;
                };
                handlers.contextmenu = handleContextMenu;
                canvas.addEventListener('contextmenu', handlers.contextmenu);
                window.addEventListener('contextmenu', handleContextMenu);

                updateCameraTransform();
            } catch (error) {
                console.error('[BattleMap] Init error:', error);
                if (app) {
                    try {
                        app.destroy(true);
                    } catch (e) {
                        console.error('Error destroying failed app:', e);
                    }
                    app = null;
                }
            }
        };

        initPixi();

        return () => {
            isMounted = false;
            if (canvas) {
                if (handlers.wheel) canvas.removeEventListener('wheel', handlers.wheel);
                if (handlers.mousedown) canvas.removeEventListener('mousedown', handlers.mousedown);
                if (handlers.mousemove) canvas.removeEventListener('mousemove', handlers.mousemove);
                if (handlers.mouseup) canvas.removeEventListener('mouseup', handlers.mouseup);
                if (handlers.keydown) window.removeEventListener('keydown', handlers.keydown);
                if (handlers.contextmenu) {
                    canvas.removeEventListener('contextmenu', handlers.contextmenu);
                    window.removeEventListener('contextmenu', handlers.contextmenu);
                }
            }

            if (app) {
                try {
                    app.destroy(true);
                } catch (e) {
                    console.error('Error destroying app during cleanup:', e);
                }
                app = null;
            }
        };
    }, [width, height, gridWidth, gridHeight]);

    const getRuleText = (mode: string | null) => {
        if (!mode) return "Select a movement type to proceed.";
        return `Switching to ${mode}: Subtract the distance you've already moved from your ${mode} speed. The result determines how much farther you can move.`;
    };

    return (
        <div
            ref={containerRef}
            style={{
                width: '100%',
                height: '100%',
                cursor: isDraggingRef.current ? 'grabbing' : 'grab',
                position: 'relative'
            }}
            onContextMenu={(e) => {
                e.preventDefault();
                e.stopPropagation();
            }}
        >
            {/* Movement Selection Modal */}
            {movementModal.visible && (
                <div style={{
                    position: 'absolute',
                    top: '50%',
                    left: '50%',
                    transform: 'translate(-50%, -50%)',
                    backgroundColor: 'rgba(0, 0, 0, 0.95)',
                    padding: '20px',
                    borderRadius: '12px',
                    border: '2px solid #D4AF37',
                    zIndex: 1000,
                    color: 'white',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '12px',
                    width: '280px',
                    boxShadow: '0 10px 25px rgba(0,0,0,0.8)'
                }}>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        {movementModal.modes.map(mode => (
                            <button
                                key={mode}
                                onClick={() => {
                                    if (interactionManagerRef.current && movementModal.token) {
                                        interactionManagerRef.current.executeMove(
                                            movementModal.token,
                                            movementModal.x,
                                            movementModal.y,
                                            mode
                                        );
                                    }
                                    setMovementModal({ ...movementModal, visible: false });
                                    setHoveredMode(null);
                                }}
                                style={{
                                    padding: '12px',
                                    backgroundColor: '#333',
                                    color: 'white',
                                    border: '1px solid #666',
                                    borderRadius: '6px',
                                    cursor: 'pointer',
                                    fontSize: '16px',
                                    fontWeight: 'bold',
                                    transition: 'all 0.2s'
                                }}
                                onMouseOver={(e) => {
                                    e.currentTarget.style.backgroundColor = '#444';
                                    e.currentTarget.style.borderColor = '#D4AF37';
                                    setHoveredMode(mode);
                                }}
                                onMouseOut={(e) => {
                                    e.currentTarget.style.backgroundColor = '#333';
                                    e.currentTarget.style.borderColor = '#666';
                                    setHoveredMode(null);
                                }}
                            >
                                {mode}
                            </button>
                        ))}
                    </div>

                    <button
                        onClick={() => {
                            // Snap back to original position on cancel
                            if (movementModal.token && movementModal.originalPosition) {
                                movementModal.token.moveTo(
                                    movementModal.originalPosition.gridX,
                                    movementModal.originalPosition.gridY
                                );
                            }
                            setMovementModal({ ...movementModal, visible: false });
                            setHoveredMode(null);
                        }}
                        style={{
                            padding: '12px',
                            backgroundColor: '#d32f2f',
                            color: 'white',
                            border: '1px solid #b71c1c',
                            borderRadius: '6px',
                            cursor: 'pointer',
                            fontSize: '16px',
                            fontWeight: 'bold',
                            marginTop: '4px',
                            transition: 'all 0.2s'
                        }}
                        onMouseOver={(e) => e.currentTarget.style.backgroundColor = '#b71c1c'}
                        onMouseOut={(e) => e.currentTarget.style.backgroundColor = '#d32f2f'}
                    >
                        Cancel
                    </button>

                    {/* Rules Text Area */}
                    <div style={{
                        marginTop: '10px',
                        padding: '10px',
                        backgroundColor: 'rgba(255, 255, 255, 0.1)',
                        borderRadius: '6px',
                        fontSize: '13px',
                        lineHeight: '1.4',
                        color: '#ddd',
                        height: '80px',
                        fontStyle: 'italic',
                        whiteSpace: 'normal',
                        overflow: 'hidden'
                    }}>
                        {getRuleText(hoveredMode)}
                    </div>
                </div>
            )}

            {/* Reset Movement Button (Debug/Dash) */}
            <button
                onClick={() => {
                    const token = interactionManagerRef.current?.getSelectedToken();
                    if (token) {
                        token.resetMovement();
                    }
                }}
                style={{
                    position: 'absolute',
                    bottom: '20px',
                    right: '20px',
                    padding: '10px 20px',
                    backgroundColor: '#4CAF50',
                    color: 'white',
                    border: 'none',
                    borderRadius: '5px',
                    cursor: 'pointer',
                    zIndex: 100,
                    fontWeight: 'bold',
                    boxShadow: '0 2px 5px rgba(0,0,0,0.5)'
                }}
            >
                Dash / New Turn
            </button>
        </div>
    );
};
