# Design System — quick reference

> Last updated: 2026-06-18
> **Full source of truth:** `design/design-system-guide.md`

This document is a quick-lookup reference. For full specs (rationale, detailed rules, implementation checklist), see the files in `design/`.

---

## Source documents

| File | Contents |
|------|----------|
| `design/design-system-guide.md` | Single source of truth — colors, typography, spacing, components, accessibility |
| `design/logo-guide.md` | Fern + Boot logo specs (3 variants, SVG, animation) |
| `design/typo-fougere-guide.md` | Custom Fougère typeface — design, process, rules |

---

## CSS tokens (`public/css/variables.css`)

### Colors

#### Light theme (`:root`)

| Token | Value | Role |
|-------|-------|------|
| `--bg-primary` | `#ffffff` | Main background |
| `--bg-secondary` | `#f8fafc` | Alternate background, TOC, callouts |
| `--bg-tertiary` | `#f1f5f9` | Tags, inputs, inactive |
| `--text-primary` | `#0f172a` | Body, headings |
| `--text-secondary` | `#475569` | Descriptions |
| `--text-tertiary` | `#64748b` | Metadata, labels |
| `--accent-primary` | `#3dba7c` | CTA, active links, focus |
| `--accent-secondary` | `#40916c` | Hover, active |
| `--accent-tertiary` | `#e8971a` | Badges, warm accents |
| `--border-color` | `#e2e8f0` | Neutral borders |

#### Dark theme (`[data-theme="dark"]`)

| Token | Value |
|-------|-------|
| `--bg-primary` | `#0f172a` |
| `--bg-secondary` | `#1e293b` |
| `--bg-tertiary` | `#334155` |
| `--text-primary` | `#f8fafc` |
| `--text-secondary` | `#cbd5e1` |
| `--text-tertiary` | `#94a3b8` |
| `--accent-primary` | `#52c98a` |
| `--accent-secondary` | `#40916c` |
| `--accent-tertiary` | `#e8971a` ← invariant |
| `--border-color` | `#334155` |

#### Absolute invariants (NEVER change between themes)

```
--accent-tertiary    #e8971a   Amber is amber
--accent-secondary   #40916c   Constant deep green
--logo-fern-green    #52b788   The logo's fern
--logo-amber         #e8971a   Boot details
```

#### Three greens — do not mix up

| Token | Hex (light) | Usage |
|-------|-------------|-------|
| `--accent-primary` | `#3dba7c` | Buttons, focus, links |
| `--accent-secondary` | `#40916c` | Hover states |
| `--logo-fern-green` | `#52b788` | Logo only — never in the UI |

---

### Typography

| Token | Font | Usage |
|-------|------|-------|
| `--font-display` | Fougère (→ Literata fallback) | H1, H2, logotype |
| `--font-body` | Literata (variable, self-hosted WOFF2) | Body, H3–H6, prose |
| `--font-ui` | `system-ui` | Nav, buttons, tags, labels |
| `--font-code` | Fira Code | Inline code, blocks |

**Rule:** never mix `--font-body` and `--font-ui` in the same element.

#### Scale (excerpt)

| Level | rem | px | Font | Weight |
|-------|-----|----|------|--------|
| H1 Hero | 3.5rem | 56px | `--font-display` | 800 |
| H2 Section | 2.5rem | 40px | `--font-display` | 700 |
| H3 | 1.75rem | 28px | `--font-body` | 700 |
| Body | 1rem | 16px | `--font-body` | 400 |
| Nav label | 0.9rem | 14.4px | `--font-ui` | 500 |
| Tag/caption | 0.75rem | 12px | `--font-ui` | 500 |

---

### Radius

| Token | Value | Usage |
|-------|-------|-------|
| `--radius-xs` | 4px | Inline code |
| `--radius-sm` | 8px | Buttons, inputs, tags |
| `--radius-md` | 12px | Medium containers |
| `--radius-lg` | 16px | Cards |
| `--radius-xl` | 20px | Large panels |
| `--radius-2xl` | 32px | Profile image, decorative |
| `--radius-circle` | 50% | FAB, circular elements |

---

### Shadows

| Token | Usage |
|-------|-------|
| `--shadow-sm` | Cards at rest |
| `--shadow-md` | Primary button, floating elements |
| `--shadow-lg` | Cards on hover |
| `--shadow-xl` | Hero floating cards, profile hover |

---

### Spacing (`--space-*`)

Base 4px (0.25rem). Tokens from `--space-1` (4px) to `--space-32` (128px).

Commonly used: `--space-4` (16px standard padding), `--space-6` (24px gap), `--space-8` (32px section padding), `--space-16` (64px section header margin).

---

## CSS — file structure

```
public/css/
├── fonts.css              @font-face Literata (load first)
├── variables.css          All tokens
├── base.css               Reset, typography, containers
├── animations.css         Keyframes, transitions
├── navigation/
│   ├── nav-base.css
│   ├── nav-layout.css
│   ├── nav-responsive.css
│   └── nav-fab.css        Mobile FAB + animations
├── components/
│   ├── buttons.css
│   ├── cards.css
│   ├── forms.css
│   └── ui.css             Tags, badges, toggles
├── sections/              hero, about, skills, projects…
└── pages/
    ├── blog-index.css
    ├── blog-article.css
    └── veille.css
```

Import order in `style.css`: fonts → variables → base → animations → navigation → components → sections → pages → responsive.

---

## Logo system

3 variants — same mark, 3 resolutions:

| Variant | Target size | Usage |
|---------|-------------|-------|
| **Mark** | 16–64px | Favicon, avatar — pure silhouette, 1–2 paths |
| **Standard** | 64–256px | Header nav — boot outline + fern fill |
| **Display** | 256px+ | Hero, About — multi-path, animatable |

**Key rule:** fern is always `--logo-fern-green` (#52b788) — never `--accent-primary`. Boot stroke via `--logo-boot-stroke` (theme-adaptive).

Animation (Display only): 2.1s sequence, boot → fern, plays once. `prefers-reduced-motion`: jumps to the final state.

**Status:** logo in design phase (see `design/logo-guide.md`). Roadmap phase 3.

---

## Fougère typeface

Custom typeface derived from the morphology of *Dryopteris filix-mas*.

- **Uppercase**: frond seen from above (structure, stability)
- **Lowercase**: crozier seen from the side (dynamism, growth)
- **Usage**: display only (H1/H2 ≥ 48px, logotype)
- **Status**: in design (see `design/typo-fougere-guide.md`). Roadmap phase 2.

Currently: `--font-display` → Literata (placeholder).

---

## Accessibility

**Non-negotiable rules:**

- `focus-visible` on every interactive element — never remove without a replacement
- `prefers-reduced-motion` on any animation > 300ms or looping
- WCAG AA contrast: `--accent-primary` (#3dba7c light) = 3.1:1 — use only for large text/icons, never body text. Prefer `--accent-secondary` (#40916c = 4.5:1) for colored text
- Decorative SVGs: `aria-hidden="true"`. Functional SVGs: `aria-label` on the parent `<a>`

**Post-deploy validation:** pa11y WCAG 2.1 AA on `/`, `/blog`, `/veille`.

---

## New-feature checklist

- [ ] Colors via `var(--...)` only — zero hardcoded hex in components
- [ ] Font: `--font-ui` (interface) or `--font-body`/`--font-display` (content) — no mixing
- [ ] Radius from `--radius-*`, shadows from `--shadow-*`
- [ ] Explicit transitions (not `all`) — except during an in-progress migration
- [ ] `prefers-reduced-motion` if animation > 300ms or looping
- [ ] `focus-visible` on every interactive element
- [ ] WCAG AA contrast verified
- [ ] Works in both themes
