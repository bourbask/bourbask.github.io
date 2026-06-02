# Design System — Référence rapide

> Dernière mise à jour : 2026-06-01  
> **Source de vérité complète :** `design/design-system-guide.md`

Ce document est une référence de consultation rapide. Pour les spécifications complètes (justifications, règles détaillées, checklist implémentation), voir les fichiers dans `design/`.

---

## Documents source

| Fichier | Contenu |
|---------|---------|
| `design/design-system-guide.md` | Source de vérité unique — couleurs, typo, espacement, composants, accessibilité |
| `design/logo-guide.md` | Spécifications logo Fougère + Botte (3 variantes, SVG, animation) |
| `design/typo-fougere-guide.md` | Police custom Fougère — conception, processus, règles |

---

## Tokens CSS (`public/css/variables.css`)

### Couleurs

#### Thème clair (`:root`)

| Token | Valeur | Rôle |
|-------|--------|------|
| `--bg-primary` | `#ffffff` | Fond principal |
| `--bg-secondary` | `#f8fafc` | Fond alterné, TOC, callouts |
| `--bg-tertiary` | `#f1f5f9` | Tags, inputs, inactifs |
| `--text-primary` | `#0f172a` | Corps, titres |
| `--text-secondary` | `#475569` | Descriptions |
| `--text-tertiary` | `#64748b` | Métadonnées, labels |
| `--accent-primary` | `#3dba7c` | CTA, liens actifs, focus |
| `--accent-secondary` | `#40916c` | Hover, actif |
| `--accent-tertiary` | `#e8971a` | Badges, accents chauds |
| `--border-color` | `#e2e8f0` | Bordures neutres |

#### Thème sombre (`[data-theme="dark"]`)

| Token | Valeur |
|-------|--------|
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

#### Invariants absolus (ne changent JAMAIS entre thèmes)

```
--accent-tertiary    #e8971a   L'ambre est l'ambre
--accent-secondary   #40916c   Vert profond constant
--logo-fern-green    #52b788   La fougère du logo
--logo-amber         #e8971a   Détails de la botte
```

#### Trois verts — ne pas confondre

| Token | Hex (clair) | Usage |
|-------|-------------|-------|
| `--accent-primary` | `#3dba7c` | Boutons, focus, liens |
| `--accent-secondary` | `#40916c` | Hover states |
| `--logo-fern-green` | `#52b788` | Logo uniquement — jamais en UI |

---

### Typographie

| Token | Police | Usage |
|-------|--------|-------|
| `--font-display` | Fougère (→ Literata fallback) | H1, H2, logotype |
| `--font-body` | Literata (variable, WOFF2 auto-hébergé) | Corps, H3–H6, prose |
| `--font-ui` | `system-ui` | Nav, boutons, tags, labels |
| `--font-code` | Fira Code | Code inline, blocs |

**Règle :** ne jamais mélanger `--font-body` et `--font-ui` dans le même élément.

#### Échelle (extrait)

| Niveau | rem | px | Police | Poids |
|--------|-----|----|--------|-------|
| H1 Hero | 3.5rem | 56px | `--font-display` | 800 |
| H2 Section | 2.5rem | 40px | `--font-display` | 700 |
| H3 | 1.75rem | 28px | `--font-body` | 700 |
| Body | 1rem | 16px | `--font-body` | 400 |
| Label nav | 0.9rem | 14.4px | `--font-ui` | 500 |
| Tag/caption | 0.75rem | 12px | `--font-ui` | 500 |

---

### Radius

| Token | Valeur | Usage |
|-------|--------|-------|
| `--radius-xs` | 4px | Code inline |
| `--radius-sm` | 8px | Boutons, inputs, tags |
| `--radius-md` | 12px | Conteneurs moyens |
| `--radius-lg` | 16px | Cartes |
| `--radius-xl` | 20px | Grands panneaux |
| `--radius-2xl` | 32px | Image profil, décoratifs |
| `--radius-circle` | 50% | FAB, éléments circulaires |

---

### Ombres

| Token | Usage |
|-------|-------|
| `--shadow-sm` | Cartes au repos |
| `--shadow-md` | Bouton primaire, flottants |
| `--shadow-lg` | Cartes hover |
| `--shadow-xl` | Floating cards hero, profil hover |

---

### Espacement (`--space-*`)

Base 4px (0.25rem). Tokens de `--space-1` (4px) à `--space-32` (128px).

Couramment utilisés : `--space-4` (16px padding standard), `--space-6` (24px gap), `--space-8` (32px padding section), `--space-16` (64px section header margin).

---

## CSS — structure des fichiers

```
public/css/
├── fonts.css              @font-face Literata (charger en premier)
├── variables.css          Tous les tokens
├── base.css               Reset, typographie, conteneurs
├── animations.css         Keyframes, transitions
├── navigation/
│   ├── nav-base.css
│   ├── nav-layout.css
│   ├── nav-responsive.css
│   └── nav-fab.css        FAB mobile + animations
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

Ordre d'import dans `style.css` : fonts → variables → base → animations → navigation → components → sections → pages → responsive.

---

## Système logo

3 variantes — même signe, 3 résolutions :

| Variante | Taille cible | Usage |
|----------|-------------|-------|
| **Mark** | 16–64px | Favicon, avatar — silhouette pure, 1–2 paths |
| **Standard** | 64–256px | Header nav — botte outline + fougère fill |
| **Display** | 256px+ | Hero, About — multi-paths animables |

**Règle clé :** fougère toujours `--logo-fern-green` (#52b788) — jamais `--accent-primary`. Botte stroke via `--logo-boot-stroke` (adaptatif au thème).

Animation (Display uniquement) : séquence 2.1s, botte → fougère, une seule lecture. `prefers-reduced-motion` : état final direct.

**Statut :** logo en phase de conception (voir `design/logo-guide.md`). Phase 3 du roadmap.

---

## Police Fougère

Police custom extraite de la morphologie des *Dryopteris filix-mas*.

- **Majuscules** : frondon vu de dessus (structure, stabilité)
- **Minuscules** : crosse/crozier vu de côté (dynamisme, croissance)
- **Usage** : display uniquement (H1/H2 ≥ 48px, logotype)
- **Statut** : en conception (voir `design/typo-fougere-guide.md`). Phase 2 du roadmap.

Actuellement : `--font-display` → Literata (placeholder).

---

## Accessibilité

**Règles non négociables :**

- `focus-visible` sur tout élément interactif — jamais supprimer sans remplacement
- `prefers-reduced-motion` sur toute animation > 300ms ou en boucle
- Contrastes WCAG AA : `--accent-primary` (#3dba7c clair) = 3.1:1 — utiliser uniquement pour grand texte/icônes, jamais corps de texte. Préférer `--accent-secondary` (#40916c = 4.5:1) pour le texte coloré
- SVGs décoratifs : `aria-hidden="true"`. SVGs fonctionnels : `aria-label` sur le `<a>` parent

**Validation post-déploiement :** pa11y WCAG 2.1 AA sur `/`, `/blog`, `/veille`.

---

## Checklist nouvelle fonctionnalité

- [ ] Couleurs via `var(--...)` uniquement — zéro hex en dur dans les composants
- [ ] Police : `--font-ui` (interface) ou `--font-body`/`--font-display` (contenu) — pas de mélange
- [ ] Radius depuis `--radius-*`, ombres depuis `--shadow-*`
- [ ] Transitions explicites (pas `all`) — sauf migration en cours
- [ ] `prefers-reduced-motion` si animation > 300ms ou en boucle
- [ ] `focus-visible` sur tout élément interactif
- [ ] Contraste WCAG AA vérifié
- [ ] Fonctionne dans les deux thèmes
