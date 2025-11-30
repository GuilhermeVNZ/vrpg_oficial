import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src/client-electron/renderer'),
        },
    },
    base: './', // Ensure relative paths for Electron
    assetsInclude: ['**/*.wasm', '**/*.wasm.wasm'], // Include WASM files as assets
    optimizeDeps: {
        exclude: ['@3d-dice/dice-box'], // Don't pre-bundle dice-box
    },
});
