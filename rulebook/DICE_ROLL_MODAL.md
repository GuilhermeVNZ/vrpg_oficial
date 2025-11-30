# Dice Roll Modal – VRPG Client

## Purpose
`DiceRollModal` is the **interactive roll screen** that appears when the player initiates a dice roll from the UI. Unlike the cinematic overlay, this modal uses a plain Three.js scene (no R3F) and gives the user direct control over the dice preview.

## Core Features Implemented
| Feature | Implementation |
|---------|----------------|
| **Lighting** | Ambient (0.1), Directional (0.2) and two Point lights (0.25 & 0.2) – intensity reduced to ~10 % of the original values to prevent blown‑out highlights.
| **PBR Support** | The dice meshes are created by `DiceFactory` which now loads normal, roughness and AO maps for each skin via `DiceAppearanceService`.
| **Material Settings** | `MeshStandardMaterial` with `metalness: 0.7`, `roughness: 0.2`, `envMapIntensity: 1.0` – consistent with the cinematic view.
| **Interaction** | Mouse drag rotates the dice, mouse wheel zooms, and the dice is animated while idle.
| **Shadow Handling** | Shadows are disabled (`castShadow = false`) to keep the visual style clean.

## Component Flow (simplified)
```tsx
<DiceRollModal isOpen={...} onClose={...}>
  <div ref={mountRef} />   {/* Three.js canvas */}
</DiceRollModal>
```
Inside the modal:
1. `RollManager` creates a new `DiceFactory` instance.
2. Calls `await factory.createDice(type, scene, world)` – the factory loads PBR textures and returns a mesh.
3. The mesh is added to the Three.js scene, lighting is applied, and the render loop starts.
4. User can rotate/zoom the dice; when the roll resolves the modal closes.

---
*All visual tweaks respect the design system defined in `RULEBOOK.md` (dark palette, subtle glow, micro‑animations).*
