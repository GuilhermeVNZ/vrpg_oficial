# Inventory Modal – VRPG Client

## Purpose
The `InventoryModal` provides the player with a full view of their equipment, items, and companions. It is opened from the main UI (e.g., pressing **I**).

## Main Features Implemented
| Feature | Description |
|---------|-------------|
| **SplitLayout** | Restored the `SplitLayout` component to separate the left‑hand navigation from the right‑hand content area. Adjusted `backgroundSize` from `cover` → `contain` so background images are fully visible.
| **Slot Glow** | Added four teal radial glow points per equipment slot. The glow is rendered *behind* the slot image and offset inward (5‑10 px) to create a subtle halo.
| **Icon Replacement** | Imported `quarterstaff.png` and `robe.png` and replaced textual item names with these icons for a cleaner UI.
| **Attuned Tab Fixes** | Fixed syntax errors and closing braces, re‑added the `AttunedTab` component.
| **Tooltips** | Added tooltip placeholders for future contextual help on each slot.

## Component Structure (simplified)
```tsx
<Modal title="Inventory">
  <SplitLayout>
    <Sidebar> {/* tabs: Equipped, Attuned, Landholdings, Companions */} </Sidebar>
    <ContentArea>
      {/* Each tab renders a grid of `SlotBox` components */}
    </ContentArea>
  </SplitLayout>
</Modal>
```

## Interaction Flow
1. User opens the modal → `InventoryModal` mounts.
2. `SplitLayout` arranges navigation and content.
3. Each slot renders a `SlotBox` which now includes the glow effect and optional icon.
4. State is kept locally; no external API calls are required.

---
*All visual tweaks follow the design system defined in `RULEBOOK.md` (dark palette, glass‑morphism, micro‑animations).*
