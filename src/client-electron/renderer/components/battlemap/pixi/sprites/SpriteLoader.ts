import * as PIXI from 'pixi.js';

export class SpriteLoader {
    private static instance: SpriteLoader;
    private textureCache: Map<string, PIXI.Texture>;
    private loadingPromises: Map<string, Promise<PIXI.Texture>>;

    private constructor() {
        this.textureCache = new Map();
        this.loadingPromises = new Map();
    }

    public static getInstance(): SpriteLoader {
        if (!SpriteLoader.instance) {
            SpriteLoader.instance = new SpriteLoader();
        }
        return SpriteLoader.instance;
    }

    /**
     * Load a texture from a URL or path
     */
    public async loadTexture(url: string): Promise<PIXI.Texture> {
        // Check cache first
        if (this.textureCache.has(url)) {
            return this.textureCache.get(url)!;
        }

        // Check if already loading
        if (this.loadingPromises.has(url)) {
            return this.loadingPromises.get(url)!;
        }

        // Start loading
        const loadPromise = (async () => {
            try {
                const texture = await PIXI.Assets.load(url);
                this.textureCache.set(url, texture);
                this.loadingPromises.delete(url);
                return texture;
            } catch (error) {
                console.error(`[SpriteLoader] Failed to load texture: ${url}`, error);
                this.loadingPromises.delete(url);
                throw error;
            }
        })();

        this.loadingPromises.set(url, loadPromise);
        return loadPromise;
    }

    /**
     * Preload a list of textures
     */
    public async preloadTextures(urls: string[]): Promise<void> {
        await Promise.all(urls.map(url => this.loadTexture(url)));
    }

    /**
     * Get a texture from cache (synchronous)
     * Returns undefined if not loaded
     */
    public getTexture(url: string): PIXI.Texture | undefined {
        return this.textureCache.get(url);
    }

    /**
     * Clear cache
     */
    public clearCache(): void {
        this.textureCache.forEach(texture => texture.destroy(true));
        this.textureCache.clear();
        this.loadingPromises.clear();
    }
}
