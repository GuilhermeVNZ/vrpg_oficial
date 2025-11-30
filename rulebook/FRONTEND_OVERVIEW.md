# Frontend Overview – VRPG Client

## High‑Level Architecture
- **Renderer**: Electron + React‑Three‑Fiber (R3F) runs inside `src/client-electron/renderer`. This is the only UI process; it hosts all React components and Three.js scenes.
- **UI Layer**: All UI components live under `src/client-electron/renderer/components`. The primary entry points are **modals** that overlay the main game view.
- **Dice System**:
  - **DiceEngine** – R3F component used for the cinematic roll overlay (fixed 45° lighting, environment map, no shadows).
  - **DiceRollModal** – a custom Three.js scene for the interactive roll screen, with user‑controlled rotation/zoom.
  - **DiceFactory** – central factory that creates dice meshes with Cannon‑ES physics bodies and now fully supports PBR texture loading (base, normal, roughness, AO maps).
- **State Management**: Simple React state + `localStorage` for persistence of owned skins, active skins, inventory selections, etc.
- **Appearance Service**: `DiceAppearanceService` supplies skin metadata (colors, texture paths, material type) used by `DiceFactory`.

## Core UI Modules
| Module | Purpose | Key Files |
|--------|---------|-----------|
| **Modals** | Dialog windows for inventory, skins, dice roll, settings, etc. | `InventoryModal.tsx`, `SkinsModal.tsx`, `DiceRollModal.tsx`, `SettingsModal.tsx` |
| **Left Navigation Menu** | Persistent vertical menu on the left side of the main UI, giving quick access to inventory, character sheet, map, quests, etc. | `LeftMenu.tsx`, `MenuItem.tsx` |
| **Dashboard / Home Screen** | Shows a grid of **cards** (e.g., recent quests, party status, recent rolls) that act as shortcuts to deeper screens. | `HomeScreen.tsx`, `Card.tsx` |
| **Dice Engine** | Cinematic dice view used by the roll overlay. | `dice/engine/DiceEngine.tsx`, `dice/engine/Dice.tsx` |
| **Dice Factory** | Generates dice meshes with PBR textures and physics bodies. | `dice/DiceFactory.ts` |
| **Appearance Service** | Provides skin configuration and texture paths for each dice type. | `dice/DiceAppearanceService.ts` |

## Left‑Side Menu – Structure & Functions
- **Menu Items** (vertical list, icons + tooltip):
  1. **Inventory** – Opens `InventoryModal` where the player can view equipment, attuned items, landholdings, companions.
  2. **Character Sheet** – Opens a detailed sheet with stats, abilities, and equipment summary.
  3. **Map** – Shows the world map with current location, markers for quests and points of interest.
  4. **Quests** – Lists active and completed quests, with progress bars.
  5. **Dice Roller** – Directly opens `DiceRollModal` for a quick roll without cinematic overlay.
  6. **Settings** – Opens `SettingsModal` for audio, video, and UI preferences.
- **Implementation Details**:
  - Each item is a `<MenuItem>` component receiving `icon`, `label`, and an `onClick` handler.
  - Hover shows a tooltip (implemented with `react-tooltip`).
  - The menu state (which item is active) is stored in a React context `MenuContext` for easy access across the app.

## Dashboard / Home Screen – Card Layout
- The home screen uses a **responsive CSS grid** (`display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));`) to lay out cards.
- **Card Types**:
  - **Quest Card** – Shows quest title, short description, and a progress bar.
  - **Party Card** – Displays party member avatars, health bars, and quick‑action buttons (e.g., “Heal”).
  - **Recent Roll Card** – Shows the last dice roll result with a miniature dice preview (uses the same material settings as the roll modal).
  - **News / Updates Card** – Pulls data from the server to display patch notes or announcements.
- Each card is a reusable `<Card>` component that accepts `title`, `icon`, `content` and optional `action` callbacks.
- Cards are **clickable**; clicking navigates to the detailed view (e.g., clicking a Quest Card opens the full quest screen).

## Interaction Flow (Expanded)
1. **App Startup** – Electron loads the React root (`src/client-electron/renderer/index.tsx`). Global providers (`MenuContext`, `ThemeProvider`) are mounted.
2. **User opens a modal** (e.g., presses `i` → `InventoryModal`). The left‑side menu highlights the active item.
3. **Inventory Modal** renders a `SplitLayout` with a vertical tab bar (Equipped, Attuned, Landholdings, Companions). Each tab displays a grid of `SlotBox` components, each with a teal glow effect and optional item icon.
4. **Skins Modal** (opened from Inventory) shows a 3‑D preview canvas. The preview uses the same lighting configuration as the cinematic overlay and loads PBR textures via `DiceFactory`.
5. **Dice Roll** – When the user clicks the Dice Roller menu item or a “Roll” button on a card, `DiceRollModal` mounts:
   - `RollManager` creates a dice via `DiceFactory` (async load of textures).
   - The dice mesh is added to a Three.js scene with low‑intensity ambient, directional, and point lights (≈10 % of original intensity).
   - User can drag to rotate, scroll to zoom, and watch the dice animate until the roll resolves.
6. **Cinematic Roll Overlay** – Pressing **D** triggers `CinematicRollOverlay`, which mounts `DiceEngine`. This uses R3F, the same material parameters (`metalness: 0.7`, `roughness: 0.2`, `envMapIntensity: 1.0`) and a fixed 45° directional light (intensity 0.5 after reduction). No shadows are cast.
7. **State Persistence** – Selections (active skin, owned skins) are saved to `localStorage` via `useEffect` hooks in the modals, ensuring the data survives page reloads.

## Visual & Design Guidelines
- **Dark‑mode palette** with gold accent (`#D4AF37`) for highlights.
- **Glass‑morphism**: semi‑transparent panels with backdrop‑blur, used throughout modals.
- **Micro‑animations**: hover glows, button transitions (`transition: all 0.3s ease`), and dice spin.
- **Typography**: Google Font *Inter* for body text, *Crimson Text* for headings.
- **Responsiveness**: All modals and the dashboard adapt to window size; cards re‑flow using CSS grid.

---
*All components adhere to the standards defined in `RULEBOOK.md` (linting, TypeScript strictness, testing, and build pipelines).*
