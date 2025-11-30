// src/client-electron/renderer/dice/NumberTexture.ts
import * as THREE from 'three';
import { Colorset } from './DiceConfig';

/**
 * Blends two hex colors together
 */
function blendColors(color1: string, color2: string, ratio: number): string {
    const hex = (c: string) => parseInt(c.replace('#', ''), 16);
    const r1 = (hex(color1) >> 16) & 0xff;
    const g1 = (hex(color1) >> 8) & 0xff;
    const b1 = hex(color1) & 0xff;
    const r2 = (hex(color2) >> 16) & 0xff;
    const g2 = (hex(color2) >> 8) & 0xff;
    const b2 = hex(color2) & 0xff;

    const r = Math.round(r1 + (r2 - r1) * ratio);
    const g = Math.round(g1 + (g2 - g1) * ratio);
    const b = Math.round(b1 + (b2 - b1) * ratio);

    return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
}

/**
 * Creates a 512×512 canvas texture with a number rendered in the center.
 * The texture matches the style used by Dice So Nice with Arial font.
 */
export function createNumberTexture(value: number, colorset: Colorset, backgroundImage?: HTMLImageElement | HTMLCanvasElement | ImageBitmap, fontScale: number = 1.0, backgroundScale: number = 1.0): THREE.CanvasTexture {
    const size = 512;
    const canvas = document.createElement('canvas');
    canvas.width = size;
    canvas.height = size;
    const ctx = canvas.getContext('2d')!;

    if (backgroundImage) {
        // Draw background image
        try {
            if (backgroundScale !== 1.0) {
                // Draw scaled/tiled background
                // For marble, we want to center it and scale it
                const scaledSize = size * backgroundScale;
                const offset = (size - scaledSize) / 2;
                ctx.drawImage(backgroundImage as any, offset, offset, scaledSize, scaledSize);
            } else {
                ctx.drawImage(backgroundImage as any, 0, 0, size, size);
            }
        } catch (e) {
            console.warn('Failed to draw background image on texture', e);
            // Fallback to gradient
            drawGradientBackground(ctx, size, colorset);
        }
    } else {
        drawGradientBackground(ctx, size, colorset);
    }

    // Draw centered number
    ctx.strokeStyle = colorset.outline || '#000000';
    ctx.lineWidth = 20 * fontScale;
    ctx.font = `bold ${320 * fontScale}px Arial`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    const text = value.toString();

    // Draw outline multiple times for thickness
    for (let i = 0; i < 4; i++) {
        ctx.strokeText(text, size / 2, size / 2);
    }

    // Draw fill
    ctx.fillStyle = colorset.foreground;
    ctx.fillText(text, size / 2, size / 2);

    // Draw underscore for 6 and 9 to differentiate
    if (value === 6 || value === 9) {
        const textMetrics = ctx.measureText(text);
        const textWidth = textMetrics.width;
        const lineY = (size / 2) + (160 * fontScale); // Position below the number
        const lineWidth = textWidth * 0.6; // Slightly shorter than the number width

        ctx.beginPath();
        ctx.moveTo((size / 2) - (lineWidth / 2), lineY);
        ctx.lineTo((size / 2) + (lineWidth / 2), lineY);
        ctx.lineCap = 'round';
        ctx.stroke(); // Use the existing stroke style (outline color)

        // Fill the line with the foreground color, slightly thinner
        ctx.lineWidth = 10 * fontScale;
        ctx.strokeStyle = colorset.foreground;
        ctx.stroke();
    }

    const texture = new THREE.CanvasTexture(canvas);
    texture.anisotropy = 16;
    texture.needsUpdate = true;
    return texture;
}

function drawGradientBackground(ctx: CanvasRenderingContext2D, size: number, colorset: Colorset) {
    // Background with subtle radial gradient for edge definition
    const gradient = ctx.createRadialGradient(size / 2, size / 2, size / 2.5, size / 2, size / 2, size / 1.5);
    gradient.addColorStop(0, colorset.background);

    // Mix edge color with background for subtle effect
    const edgeColor = colorset.edge || '#000000';
    const bgColor = colorset.background;
    const mixedColor = blendColors(bgColor, edgeColor, 0.15);
    gradient.addColorStop(1, mixedColor);

    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, size, size);
}
