# Dice Engine – Cinematic Roll Overlay

## Purpose
`DiceEngine` is the React‑Three‑Fiber (R3F) component that renders a **cinematic view** of the dice when the user triggers the roll overlay (press **D**). It provides a fixed 45° lighting setup, environment map reflections, and a smooth animation loop.

## Key Implementations
| Implementation | Details |
|----------------|---------|
| **Lighting** | Ambient (0.25), Directional (0.5) and Hemisphere (0.25) lights – all reduced by 50 % from the original values to avoid blown‑out highlights.
| **Environment** | `@react-three/drei` `Environment` with `preset="studio"` supplies an HDRI for realistic reflections.
| **Material Settings** | Dice faces use `MeshStandardMaterial` with `metalness: 0.7`, `roughness: 0.2`, `envMapIntensity: 1.0` for a shiny, reflective look.
| **Shadows** | Disabled – dice never cast shadows, matching the design requirement.
| **Animation** | Simple rotation on the Y‑axis (`dice.rotation.y += 0.005`) for a continuous spin.

## Component Hierarchy (simplified)
```tsx
<CinematicRollOverlay>
  <Canvas>
    <ambientLight intensity={0.25} />
    <directionalLight intensity={0.5} position={[5,10,5]} />
    <hemisphereLight intensity={0.25} />
    <Environment preset="studio" />
    <Dice geometry={...} materials={...} />
  </Canvas>
</CinematicRollOverlay>
```

## Interaction Flow
1. User presses **D** → `CinematicRollOverlay` mounts.
2. `DiceEngine` creates the Three.js scene and adds the dice mesh.
3. Lighting and environment are applied, then the dice spins until the roll resolves.
4. The overlay unmounts, returning control to the main UI.

---
*All visual choices follow the aesthetic guidelines in `RULEBOOK.md` (dark mode, vibrant gradients, micro‑animations).*
