# UI Design System — Neubrutalism

The Claude Sentinel desktop app uses a strict neubrutalism design language: pure black and white, thick borders, offset shadows, and monospace type throughout.

## Palette

| Variable | Value | Usage |
|----------|-------|-------|
| `--black` | `#000000` | Borders, text, shadows |
| `--white` | `#ffffff` | Backgrounds, button labels (active) |
| `--border` | `2px solid #000` | All element borders |
| `--shadow` | `4px 4px 0px #000` | Default offset drop shadow |
| `--shadow-sm` | `2px 2px 0px #000` | Compact elements |
| `--font-mono` | `"JetBrains Mono", "Courier New", monospace` | Everything |

No grays. No color. No gradients. No border-radius.

## Typography

- Font family: monospace throughout (`JetBrains Mono` → `Courier New` → system monospace)
- Headings: ALL-CAPS
- Body: 14px / 1.4 line-height
- Labels: 12px, ALL-CAPS, letter-spacing 0.05em

## Borders and Shadows

Every interactive element has:
- `border: 2px solid #000`
- `box-shadow: 4px 4px 0px #000`

On `:hover`:
- `box-shadow: 2px 2px 0px #000` (shadow shrinks inward)

On `:active` / selected:
- `background: #000`, `color: #fff`
- `box-shadow: none`

## Components

### Buttons

```css
.btn {
  background: #fff;
  color: #000;
  border: 2px solid #000;
  box-shadow: 4px 4px 0px #000;
  font-family: var(--font-mono);
  padding: 8px 16px;
  cursor: pointer;
  border-radius: 0;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
.btn:hover { box-shadow: 2px 2px 0px #000; }
.btn:active, .btn.active { background: #000; color: #fff; box-shadow: none; }
```

### Cards

```css
.card {
  background: #fff;
  border: 2px solid #000;
  box-shadow: 4px 4px 0px #000;
  padding: 16px;
  border-radius: 0;
}
```

### Tabs

Active tab: black background, white text, no bottom border.
Inactive tab: white background, black text, bottom border.

### Quota Bar

```css
.quota-bar-fill {
  background: #000;
  height: 100%;
  transition: width 0.3s ease;
}
```

No color coding — pure black fill against white track. Percentage shown as ALL-CAPS text label.

### Tables

```css
.data-table th {
  background: #000;
  color: #fff;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
.data-table tr:nth-child(even) { background: #f5f5f5; }
.data-table td, .data-table th {
  border: 1px solid #000;
  padding: 8px 12px;
  font-family: var(--font-mono);
}
```

## Tray Menu (macOS / Linux / Windows)

```
[🛡 WORK:BACKEND ▼]
┌────────────────────────────┐
│ WORK                       │
│  ▶ BACKEND  [ACTIVE]       │
│    FRONTEND                │
│                            │
│ PERSONAL                   │
│    DEFAULT                 │
├────────────────────────────┤
│ AUTO-SWITCH  [ON ■]        │
│ QUOTA: ████░░  67%         │
│ RESETS IN: 3H 42M          │
├────────────────────────────┤
│ + NEW SESSION              │
│ ⊞ OPEN SENTINEL            │
│ ✕ QUIT                     │
└────────────────────────────┘
```

## App Window — Tab Structure

1. **PROFILES** — create/edit/delete profiles with auth type badge
2. **SESSIONS** — card grid per profile, last-used timestamp, token count
3. **AUTO-SWITCH** — fallback chain config, live rate-limit timers, switch log
4. **STATS** — token usage table, cost estimate, rate-limit history

## Spacing Scale

| Name | Value |
|------|-------|
| xs | 4px |
| sm | 8px |
| md | 16px |
| lg | 24px |
| xl | 32px |

Margins and paddings are multiples of 4px. No sub-pixel values.

## Icons

Pixel-art or geometric SVG icons only. No SVG gradients. No rounded paths.
Badge icons use ALL-CAPS text labels instead of icon glyphs where possible.

## CSS File

The full design system is implemented in `apps/desktop/src/styles/neubrutalism.css`.
Import it at the top of `App.tsx`:

```tsx
import "./styles/neubrutalism.css";
```
