---
status: DRAFT
---

# UI/UX 01 — Design System

The visual language for Smith. The goal (per [01-vision.md](../01-vision.md) design north
star 5): a UI pleasant for multi-hour sessions — restrained color, generous whitespace,
predictable motion, never visually loud. Color is semantic (element kind, relationship kind,
defect severity), never decorative.

Notation shapes, line styles, and arrowheads are fixed by UML — see
[research/04-uml-visual-notation.md](../research/04-uml-visual-notation.md). This document
owns everything the spec leaves open: palette, typography, spacing, motion, theming.

Harmonize with (do not copy): Material 3 color system, Apple HIG color, WCAG 2.1 contrast,
Okabe–Ito color-safe palette (see
[research/04-uml-visual-notation.md](../research/04-uml-visual-notation.md) §Color guidance).

## Design tokens

`[REQ-DS-001]` The design system MUST be expressed as a set of named **tokens** (not raw
values), themeable for light and dark mode. Tokens are the single source of truth; components
never hardcode colors/sizes.

Token categories: `color`, `font`, `size`, `space`, `radius`, `stroke`, `shadow`, `motion`,
`z`.

## Color

### Principles
`[REQ-DS-002]` Color usage rules:
- **Meaning never depends on color alone** (WCAG 2.1 SC 1.4.1; UML helps — line style and
  arrowheads carry meaning shape-based).
- **Semantic only** — color encodes element kind, relationship kind, defect severity, or
  interaction state (selected/hover/dragging). No decorative color.
- **One accent** — a single accent hue for primary actions / focus rings. Everything else is
  neutral + semantic category tints.
- **Calm canvas** — the diagram canvas is a near-neutral surface (low saturation, mid
  lightness in light mode; deep-but-not-pure-black in dark mode). Elements are subtle
  containers; the eye is drawn to text and connectors, not fills.

### Palette structure (Material 3-inspired tonal roles)
`[REQ-DS-003]` Smith defines tonal roles (each with light + dark values, all ≥ WCAG AA):

| Role | Light | Dark | Used for |
|------|-------|------|----------|
| `surface.canvas` | warm off-white (#FAFAF7-ish) | deep neutral (#1A1B1E-ish, not pure black) | diagram background |
| `surface.panel` | slightly darker than canvas | slightly lighter than canvas | side panels (explorer, inspector, issues) |
| `surface.element` | white | #242629-ish | element shape fill (class box, etc.) |
| `surface.elementAlt` | tinted by category (see below) | tinted by category | alternate fill for category emphasis (optional, subtle) |
| `stroke.default` | #3A3D42 | #C8CACE | element borders, connectors (≥ 4.5:1 on their fill) |
| `stroke.muted` | #8A8E95 | #6A6E75 | grid, guides, suppressed compartments |
| `text.primary` | #1A1B1E | #F0F1F3 | element names, labels |
| `text.secondary` | #5A5E65 | #9A9EA5 | multiplicities, role names, meta |
| `accent.primary` | a single hue (e.g., indigo #4F5BD5) | same hue, lighter shade | focus rings, primary buttons, active selection |
| `accent.primaryContainer` | light tint of accent | dark tint of accent | selected-element fill (subtle) |
| `semantic.structure` | blue tint | blue tint | structure elements (class, interface, component) |
| `semantic.behavior` | amber tint | amber tint | behavior elements (use case, activity, state) |
| `semantic.requirement` | green tint | green tint | requirements |
| `semantic.test` | teal tint | teal tint | test cases |
| `severity.high` | red | red | HIGH defects |
| `severity.medium` | amber | amber | MEDIUM defects |
| `severity.low` | blue (informational) | blue | LOW defects |

`[REQ-DS-004]` Category tints are **low-saturation, pastel-range** fills/borders (not vivid),
so a diagram with 50 elements isn't a rainbow. Default: elements use neutral fill; category
tint appears only as a thin top-bar or icon color (configurable to full-fill for users who
prefer it).

### Contrast (WCAG 2.1)
`[REQ-DS-005]` Contrast minimums (verified for BOTH light and dark themes):
- Text on its background: ≥ 4.5:1 (SC 1.4.3).
- Large text (≥ 18pt or 14pt bold): ≥ 3:1.
- Non-text graphical objects that convey identity (element borders, connector strokes,
  arrowheads): ≥ 3:1 against their fill/background (SC 1.4.11).
- Focus rings: ≥ 3:1 against adjacent colors (SC 1.4.11).

`[REQ-DS-006]` The design system MUST include a contrast-verification test (automated, in the
build) that asserts every token pair meets its minimum. Failing contrast fails the build.

### Color-vision safety
`[REQ-DS-007]` Categorical hues (structure/behavior/requirement/test, severity) MUST be
distinguishable under common color-vision deficiencies (deuteranopia, protanopia,
tritanopia). Use the Okabe–Ito palette as the basis; verify with a simulation tool.

## Typography

`[REQ-DS-008]` Typography uses a single type family for UI and canvas (a neutral humanist
sans, e.g. Inter or system-ui stack). Monospace for code-like fields (feature signatures,
constraint expressions).

Type scale (modular, ratio ~1.2):

| Token | Size | Weight | Use |
|-------|------|--------|-----|
| `font.display` | 20px | 600 | diagram title |
| `font.heading` | 16px | 600 | panel headers |
| `font.body` | 14px | 400 | element names, labels, default text |
| `font.meta` | 12px | 400 | multiplicity, role names, stereotypes |
| `font.code` | 13px | 400 | feature signatures (monospace) |
| `font.caption` | 11px | 400 | issue-panel breadcrumbs, tooltips |

`[REQ-DS-009]` Element-name font weight: regular for concrete, **italic** for abstract (UML
convention). Static features: **underlined**. Derived features: leading `/`.

`[REQ-DS-010]` Canvas text MUST be crisp at all zoom levels — render via Canvas 2D `fillText`
(re-rasterized per zoom), not pre-rendered textures. See
[research/06-canvas-and-frontend-stack.md](../research/06-canvas-and-frontend-stack.md).

## Spacing & sizing

8px base grid. Tokens: `space.1` = 4px (half-grid for fine adjustments), `space.2` = 8px,
`space.3` = 12px, `space.4` = 16px, `space.6` = 24px, `space.8` = 32px.

`[REQ-DS-011]` Element internals follow the grid: compartment padding = `space.2` (8px);
feature line-height = 20px; element min-width = 120px; connector label offset = `space.2`.

## Stroke & shape

`[REQ-DS-012]` Stroke weights:
- Element border: 1.5px (≥ 3:1 contrast against fill).
- Connector line: 1.5px.
- Selected/hover ring: 2px accent.
- Grid dots: 1px, `stroke.muted`.
- Thick bar (activity fork/join): 6px.

`[REQ-DS-013]` Corner radius: 6px for element rectangles (class, component); 12px for
rounded-rectangle shapes (activity action, state); 0 for diamonds/bars/folders (spec-fixed
topology — see [research/04](../research/04-uml-visual-notation.md)).

`[REQ-DS-014]` Shadow: subtle, single-direction (top-down light), small blur. Default
`shadow.subtle` on elements; `shadow.drag` (larger) when dragging. No shadow on flat panels
— they use border + slight tint to separate from canvas.

## Motion

`[REQ-DS-015]` Motion principles:
- **Predictable and short.** Transitions 120–200ms, ease-out. No bounce, no spring on
  routine UI.
- **Motion explains, doesn't decorate.** Animate state changes (selection, panel collapse),
  not idle elements. No ambient animation on the canvas.
- **Respect reduced motion.** Honor `prefers-reduced-motion` (disable non-essential motion).

Motion tokens: `motion.fast` = 120ms, `motion.base` = 180ms, `motion.slow` = 240ms.

## Theming

`[REQ-DS-016]` Smith ships **light** and **dark** themes, auto-switching with the OS by
default, manually overridable in settings. Both themes MUST pass all contrast checks.

`[REQ-DS-017]` The token system MUST be data-driven (JSON/TS) so custom themes are a future
option (not in v1 UI, but the architecture allows it).

## Iconography

`[REQ-DS-018]` Icons: a single consistent icon set (e.g., Lucide or Phosphor — both MIT,
regular-weight, 1.5px stroke to match element borders). No mixed icon sets. Icons are
monochrome `stroke.default`, tinted by semantic role when they encode category.

`[REQ-DS-019]` Tool-bar icons have text tooltips (≥ `font.caption`); no icon-only actions
without a tooltip or label.

## Z-order & layering

`[REQ-DS-020]` Canvas layer order (back to front): grid → connectors/edges → element shapes →
element labels → selection handles → gizmo → marquee → tooltips. Selection handles and the
gizmo are always on top.

## Accessibility (beyond color)

`[REQ-DS-021]` Smith MUST meet WCAG 2.1 AA for the chrome (panels, menus, inspector):
keyboard navigation, visible focus, ARIA labels on icon buttons, sufficient target size
(≥ 44×44px hit area for pointer; ≥ 24×24px with spacing for dense toolbars).

`[REQ-DS-022]` The canvas itself (a `<canvas>`) cannot be screen-reader-navigated natively;
Smith MUST provide an alternative: the model explorer tree (DOM-based) is the accessible
view of the model, fully keyboard-navigable, mirroring selection with the canvas.

## Open questions (resolve before STABLE)

- [ ] Exact accent hue — indigo vs. a warmer hue. Tentative: indigo (#4F5BD5 family) for a
      calm, professional feel. Confirm with a mock.
- [ ] Full category fill vs. top-bar-only tint as the default. Tentative: top-bar-only
      (calmer); expose a setting.
- [ ] Font family: bundle Inter, or use system stack (`-apple-system, Segoe UI, Roboto, ...`)?
      Tentative: system stack for chrome (zero bundle, native feel); Inter for canvas text
      (consistency across webviews). Reconsider if canvas/system mismatch is jarring.
