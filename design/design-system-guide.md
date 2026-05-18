# Guide du Design System — bourbask.github.io

**Auteur** : Kevin Bourbasquet  
**Version** : 1.0 — Source de vérité unique  
**Date** : 2026-05-18  
**Statut** : Référence active — tous les choix visuels se justifient par ce document

---

## Avant-propos

Ce document est la source de vérité de l'identité visuelle de bourbask.github.io. Il synthétise trois artéfacts de design préexistants — le système couleur (implémenté dans `variables.css`), le guide typographique (Fougère), et le guide logo (Fougère + Botte) — en un système cohérent et opérationnel.

**Règle fondamentale** : toute décision d'interface qui ne trouve pas de justification dans ce document doit être traitée comme une question ouverte, pas comme une liberté créative.

### Piliers de marque

Trois valeurs structurent toutes les décisions visuelles :

1. **Souveraineté numérique** — Contrôle, clarté, absence de dépendances opaques. Se traduit par : typographie auto-hébergée, tokens CSS explicites, pas de CDN pour les ressources d'identité.
2. **Sécurité web** — Précision, rigueur, lisibilité sous pression. Se traduit par : contrastes accessibles, hiérarchies claires, pas d'effets qui masquent l'information.
3. **Responsabilité écologique** — Frugalité, pérennité, ancrage dans le vivant. Se traduit par : palette botanique, typographie végétale, animations légères et non-intrusives.

### Architecture du système

```
design/
  design-system-guide.md     ← ce document
  logo-guide.md              ← spécifications logo
  typo-fougere-guide.md      ← spécifications Fougère

public/css/
  variables.css              ← tokens (couleurs, radius, ombres)
  fonts.css                  ← @font-face Literata
  base.css                   ← reset, typographie, conteneurs
  animations.css             ← keyframes, transitions
  components/                ← buttons, cards, forms, ui
  navigation/                ← nav desktop + FAB mobile
  sections/                  ← hero, about, skills, projects…
  pages/                     ← blog, veille
```

---

## Sommaire

1. [Fondations — Couleurs](#1-fondations--couleurs)
2. [Fondations — Typographie](#2-fondations--typographie)
3. [Fondations — Espacement](#3-fondations--espacement)
4. [Fondations — Radius et bordures](#4-fondations--radius-et-bordures)
5. [Fondations — Ombres et lueurs](#5-fondations--ombres-et-lueurs)
6. [Fondations — Mouvement](#6-fondations--mouvement)
7. [Système logo](#7-système-logo)
8. [Système typographique intégré](#8-système-typographique-intégré)
9. [Composants](#9-composants)
10. [Système de thème](#10-système-de-thème)
11. [Principes d'animation](#11-principes-danimation)
12. [Accessibilité](#12-accessibilité)
13. [Feuille de route d'implémentation](#13-feuille-de-route-dimplémentation)

---

## 1. Fondations — Couleurs

### 1.1 Palette complète — tokens CSS

Tous les tokens sont déclarés dans `public/css/variables.css`. Ne jamais utiliser une valeur hexadécimale directement dans le CSS des composants — toujours passer par un token.

#### Thème clair (`:root`)

| Token | Valeur | Rôle |
|---|---|---|
| `--bg-primary` | `#ffffff` | Fond principal, arrière-plan des cartes |
| `--bg-secondary` | `#f8fafc` | Fond alterné, TOC, callouts |
| `--bg-tertiary` | `#f1f5f9` | Tags, inputs, éléments inactifs |
| `--text-primary` | `#0f172a` | Corps de texte, titres |
| `--text-secondary` | `#475569` | Texte secondaire, descriptions |
| `--text-tertiary` | `#64748b` | Métadonnées, labels, captions |
| `--accent-primary` | `#3dba7c` | Vert forêt vivace — CTA, liens actifs, focus |
| `--accent-secondary` | `#40916c` | Vert forêt profond — hover, états actifs |
| `--accent-tertiary` | `#e8971a` | Ambre miel — badges, accents chauds |
| `--border-color` | `#e2e8f0` | Bordures neutres |
| `--success-color` | `#3dba7c` | Alias de `--accent-primary` |
| `--warning-color` | `#e8971a` | Alias de `--accent-tertiary` |

#### Thème sombre (`[data-theme="dark"]`)

| Token | Valeur | Rôle |
|---|---|---|
| `--bg-primary` | `#0f172a` | Fond principal (bleu-nuit profond) |
| `--bg-secondary` | `#1e293b` | Fond alterné |
| `--bg-tertiary` | `#334155` | Tags, inputs, éléments inactifs |
| `--text-primary` | `#f8fafc` | Corps de texte |
| `--text-secondary` | `#cbd5e1` | Texte secondaire |
| `--text-tertiary` | `#94a3b8` | Métadonnées, labels |
| `--accent-primary` | `#52c98a` | Vert forêt éclairci (lisible sur fond sombre) |
| `--accent-secondary` | `#40916c` | Identique au thème clair |
| `--accent-tertiary` | `#e8971a` | Identique au thème clair — invariant |
| `--border-color` | `#334155` | Bordures discrètes sur fond sombre |

#### Gradients

| Token | Valeur | Usage |
|---|---|---|
| `--accent-gradient` | `135deg, #3dba7c → #40916c` | Bouton primaire, profil |
| `--accent-gradient-warm` | `135deg, #3dba7c → #e8971a` | Bouton secondaire, accents chauds |

> En mode sombre : les gradients utilisent `#52c98a` comme point de départ à la place de `#3dba7c`.

### 1.2 Tokens de couleur logo (à ajouter à `variables.css`)

Ces tokens ne sont pas encore dans le fichier — ils font partie de la Phase 1.

```css
/* ===== LOGO IDENTITY TOKENS ===== */
/* Ces valeurs ne changent JAMAIS avec le thème */
:root {
  --logo-fern-green: #52b788;       /* Fougère — couleur invariante */
  --logo-fern-green-rgb: 82, 183, 136;
  --logo-amber: #e8971a;            /* Détails botte (variante Display) */
  --logo-amber-rgb: 232, 151, 26;
  --logo-boot-stroke: #1e293b;      /* Contour botte — s'adapte au thème */
  --logo-boot-stroke-width: 1.5px;
  --logo-animation-total: 2.1s;
  --logo-animation-easing-organic: cubic-bezier(0.34, 1.56, 0.64, 1);
  --logo-animation-easing-mechanical: cubic-bezier(0.4, 0, 0.2, 1);
}

[data-theme="dark"] {
  --logo-boot-stroke: #f8fafc;
  /* --logo-fern-green NE PAS surcharger — c'est intentionnel */
}
```

### 1.3 Hiérarchie des verts

Trois verts coexistent dans le système. Ils ne sont pas interchangeables :

| Couleur | Hex (clair) | Hex (sombre) | Rôle |
|---|---|---|---|
| `--accent-primary` | `#3dba7c` | `#52c98a` | Interface — boutons, focus, liens |
| `--accent-secondary` | `#40916c` | `#40916c` | Interface — hover, actif |
| `--logo-fern-green` | `#52b788` | `#52b788` | Logo uniquement — jamais en UI |

La fougère du logo a sa propre nuance, délibérément distincte des verts d'interface. Ne jamais utiliser `--accent-primary` pour colorier la fougère du logo.

### 1.4 Invariants de couleur

Certaines couleurs ne changent **jamais** entre les thèmes. C'est une règle absolue :

- `--accent-tertiary` (`#e8971a`) — l'ambre est l'ambre, jour et nuit
- `--logo-fern-green` (`#52b788`) — la fougère est verte, toujours
- `--logo-amber` (`#e8971a`) — les détails de la botte restent ambrés

### 1.5 Ratios de contraste WCAG AA

Valeurs calculées selon WCAG 2.1 — le minimum requis est 4.5:1 pour le texte normal, 3:1 pour le grand texte (18pt+ ou 14pt+ gras).

#### Thème clair

| Combinaison | Ratio | AA Normal | AA Grand |
|---|---|---|---|
| `--text-primary` sur `--bg-primary` | 16.1:1 | Passe | Passe |
| `--text-primary` sur `--bg-secondary` | 14.7:1 | Passe | Passe |
| `--text-secondary` sur `--bg-primary` | 7.2:1 | Passe | Passe |
| `--text-secondary` sur `--bg-secondary` | 6.5:1 | Passe | Passe |
| `--text-tertiary` sur `--bg-primary` | 4.6:1 | Passe | Passe |
| `--accent-primary` (`#3dba7c`) sur `--bg-primary` | 3.1:1 | Échec | Passe |
| `--accent-primary` sur `--bg-secondary` | 2.9:1 | Échec | Échec |
| Blanc sur `--accent-primary` | 3.1:1 | Échec | Passe |
| `--accent-secondary` (`#40916c`) sur `--bg-primary` | 4.5:1 | Passe | Passe |
| `--accent-tertiary` (`#e8971a`) sur `--bg-primary` | 2.8:1 | Échec | Échec |
| Blanc sur `--accent-tertiary` | 2.8:1 | Échec | Passe |

#### Thème sombre

| Combinaison | Ratio | AA Normal | AA Grand |
|---|---|---|---|
| `--text-primary` (`#f8fafc`) sur `--bg-primary` | 15.9:1 | Passe | Passe |
| `--text-secondary` (`#cbd5e1`) sur `--bg-primary` | 9.3:1 | Passe | Passe |
| `--text-tertiary` (`#94a3b8`) sur `--bg-primary` | 5.1:1 | Passe | Passe |
| `--accent-primary` (`#52c98a`) sur `--bg-primary` | 6.3:1 | Passe | Passe |
| `--accent-primary` sur `--bg-secondary` | 5.4:1 | Passe | Passe |

**Conséquence importante** : `--accent-primary` en thème clair (`#3dba7c`) **ne passe pas** le ratio AA pour le texte normal sur fond blanc. Ne jamais l'utiliser pour du texte courant — uniquement pour les éléments de grande taille (titres, badges), les bordures actives, les indicateurs de focus, et les icônes. En texte, préférer `--accent-secondary` (`#40916c`, 4.5:1) qui passe AA.

---

## 2. Fondations — Typographie

### 2.1 Tokens de police (à créer dans `variables.css`)

Ces tokens de police n'existent pas encore dans le code — c'est la Phase 1.

```css
:root {
  /* Couche contenu — Literata (serif variable auto-hébergée) */
  --font-body: "Literata", Georgia, "Times New Roman", serif;

  /* Couche display — Fougère (futur) ; actuellement Literata en attendant */
  --font-display: "Fougère", "Literata", Georgia, serif;

  /* Couche interface — system-ui sans-serif */
  --font-ui: system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;

  /* Couche code */
  --font-code: "Fira Code", "Cascadia Code", ui-monospace, SFMono-Regular,
               Menlo, Monaco, Consolas, "Liberation Mono", monospace;
}
```

### 2.2 Rôles des polices

| Token | Police actuelle | Futur | Usages |
|---|---|---|---|
| `--font-body` | Literata (variable, WASM) | Inchangé | Corps de texte, paragraphes, H3–H6, citations |
| `--font-display` | Literata (placeholder) | Fougère | H1 hero, H2 sections, logotype texte |
| `--font-ui` | `system-ui` | Inchangé | Navigation, boutons, tags, toggles, labels de formulaire, métadonnées |
| `--font-code` | Fira Code | Inchangé | Blocs de code, inline code, kbd |

**Règle de séparation** : Literata/Fougère = contenu et identité. `system-ui` = interface fonctionnelle. Ne jamais mélanger les deux dans le même élément.

> La navigation, les boutons, les tags et tous les éléments d'interface utilisent `system-ui`. L'implémentation actuelle dans `base.css` utilise Literata pour `body` et tous les headings. La migration vers `--font-ui` pour les éléments de navigation est prévue en Phase 1.

### 2.3 Literata — Configuration

```css
/* Déclaré dans fonts.css */
@font-face {
  font-family: "Literata";
  src: url("../fonts/Literata[opsz,wght].woff2") format("woff2 supports variations"),
       url("../fonts/Literata[opsz,wght].woff2") format("woff2");
  font-weight: 300 900;   /* Axe wght */
  font-style: normal;
  font-display: swap;
}
```

Axes variables Literata :
- `opsz` : 7–72 (optical size — activé automatiquement via `font-optical-sizing: auto`)
- `wght` : 300–900

Toujours appliquer `font-optical-sizing: auto` aux éléments utilisant Literata. Le navigateur sélectionne automatiquement l'instance optique adaptée à la taille affichée.

### 2.4 Échelle typographique

L'échelle est basée en `rem` (1 rem = 16 px navigateur par défaut).

| Niveau | Token CSS (futur) | rem | px | Poids | Police | Usage |
|---|---|---|---|---|---|---|
| Display XL | `--text-display-xl` | 5rem | 80 px | 800 | `--font-display` | Titres hero ultra-wide (≥1920 px) |
| H1 Hero | `--text-h1` | 3.5rem | 56 px | 800 | `--font-display` | Titre principal homepage |
| H1 Mobile | `--text-h1-mobile` | 2.5rem | 40 px | 800 | `--font-display` | H1 sur mobile |
| H2 Section | `--text-h2` | 2.5rem | 40 px | 700 | `--font-display` | Titres de sections (`.section-title`) |
| H3 Sous-titre | `--text-h3` | 1.75rem | 28 px | 700 | `--font-body` | Sous-titres d'articles |
| H4 / Card title | `--text-h4` | 1.25rem | 20 px | 600 | `--font-body` | Titres de cartes |
| Lead | `--text-lead` | 1.3rem | 21 px | 400 italic | `--font-body` | Chapô d'article |
| Body | `--text-body` | 1rem | 16 px | 400 | `--font-body` | Corps de texte standard |
| Body large | `--text-body-lg` | 1.25rem | 20 px | 400 | `--font-body` | Description hero, sous-titres de sections |
| Label | `--text-label` | 0.9rem | 14.4 px | 500 | `--font-ui` | Liens de navigation |
| Small | `--text-small` | 0.875rem | 14 px | 400/500 | `--font-ui` ou `--font-body` | Métadonnées, breadcrumbs |
| Caption | `--text-caption` | 0.75rem | 12 px | 500 | `--font-ui` | Tags, labels de formulaire, tooltips |
| Code inline | `--text-code` | 0.875em | relatif | 400 | `--font-code` | Code inline dans un paragraphe |

### 2.5 Hauteurs de ligne et espacement lettres

| Contexte | `line-height` | `letter-spacing` | Justification |
|---|---|---|---|
| Corps de texte | 1.8 | 0 | Lisibilité longue durée (optimisation Literata) |
| Titres H1/H2 | 1.1 | −0.01em à 0 | Titres display doivent être serrés |
| Titres H3/H4 | 1.3 | 0 | Sous-titres — entre display et prose |
| Navigation | 1.4 | 0 | Compact, lisible |
| Labels uppercase | 1.4 | 0.05em–0.1em | Convention pour les petites caps/uppercase |
| Code | 1.6 | 0 | Standard pour la lecture de code |
| `body` global | 1.6 | 0 | Base, surchargée par les contextes ci-dessus |

### 2.6 Swap Fougère — procédure

Quand Fougère sera prête (Phase 2), un seul changement suffit :

```css
/* Dans variables.css — une seule ligne change */
:root {
  --font-display: "Fougère", "Literata", Georgia, serif;
}
```

Et dans `fonts.css`, ajouter le `@font-face` de Fougère. Aucun autre fichier CSS n'est à modifier si tous les éléments display utilisent correctement `var(--font-display)`.

**Prérequis** : que `--font-display` soit effectivement utilisé dans les sélecteurs concernés. C'est l'objectif de la Phase 1.

---

## 3. Fondations — Espacement

### 3.1 Échelle de tokens d'espacement (à créer — Phase 1)

L'espacement est actuellement défini en valeurs en dur dans chaque fichier CSS. La Phase 1 introduit une échelle de tokens.

```css
:root {
  /* Base : 4px (0.25rem) */
  --space-px:  1px;        /* Borders, dividers */
  --space-1:   0.25rem;    /* 4px  — micro-espacement */
  --space-2:   0.5rem;     /* 8px  — padding compact (tags, chips) */
  --space-3:   0.75rem;    /* 12px — padding réduit */
  --space-4:   1rem;       /* 16px — padding standard */
  --space-5:   1.25rem;    /* 20px */
  --space-6:   1.5rem;     /* 24px — gap entre éléments liés */
  --space-8:   2rem;       /* 32px — padding section horizontal */
  --space-10:  2.5rem;     /* 40px */
  --space-12:  3rem;       /* 48px — gap vertical moyen */
  --space-16:  4rem;       /* 64px — section-header margin */
  --space-20:  5rem;       /* 80px — padding section vertical */
  --space-24:  6rem;       /* 96px */
  --space-32:  8rem;       /* 128px */
}
```

### 3.2 Correspondances actuelles → futurs tokens

| Valeur actuelle | Token futur | Contexte |
|---|---|---|
| `0.25rem` | `--space-1` | Code dots gap |
| `0.5rem` | `--space-2` | Tags padding vertical, article-tags gap |
| `0.75rem` | `--space-3` | Tags padding horizontal |
| `1rem` | `--space-4` | Form input padding, body padding |
| `1.5rem` | `--space-6` | Card padding |
| `2rem` | `--space-8` | Container padding horizontal |
| `3rem` | `--space-12` | Article section margin-bottom |
| `4rem` | `--space-16` | Section header margin-bottom |
| `5rem` | `--space-20` | Section padding vertical |

### 3.3 Conteneurs

| Classe | `max-width` | `padding` | Usage |
|---|---|---|---|
| `.container` | 1200 px | `0 2rem` | Sections homepage |
| `.hero-container` | 1200 px | `0 2rem` | Hero |
| `.blog-article` | 1200 px | `2rem` + top 100 px | Articles |
| `.article-body` | 800 px | 0 | Corps d'article (dans la grille) |

---

## 4. Fondations — Radius et bordures

### 4.1 Échelle de radius

Déclarée dans `variables.css`. Complète et stable.

| Token | Valeur | Usage canonique |
|---|---|---|
| `--radius-xs` | `4px` | `inline code` (`.article-section p code`), micro-chips |
| `--radius-sm` | `8px` | Boutons, inputs, tags, toggles, nav items, tooltips |
| `--radius-md` | `12px` | CV download button, conteneurs moyens |
| `--radius-lg` | `16px` | Cartes génériques (`.card`), TOC, article hero image, stat cards |
| `--radius-xl` | `20px` | Grands panneaux, labels flottants FAB |
| `--radius-2xl` | `2rem` (32 px) | Image de profil, conteneurs décoratifs larges, glassy label |
| `--radius-circle` | `50%` | FAB, nav items mobiles, éléments purement circulaires |

**Règle** : augmenter le radius avec l'importance visuelle de l'élément. Les micro-éléments (`xs`), les composants d'action (`sm`), les cartes et conteneurs (`lg`), les éléments d'identité et décoratifs (`2xl`, `circle`).

### 4.2 Bordures

| Contexte | Largeur | Couleur | Token |
|---|---|---|---|
| Cartes | `1px solid` | `--border-color` | |
| Inputs | `1px solid` | `--border-color` → `--accent-primary` au focus | |
| Nav (scrolled) | `1px solid` | `--border-color` | |
| Article TLDR | `4px solid` (gauche) | `--accent-primary` | |
| Article blockquote | `4px solid` (gauche) | `--accent-primary` | |
| Article H2 | `2px solid` (bas) | `--border-color` | |
| TOC item hover | `2px solid` (gauche) | `--accent-primary` | |
| CV download btn | `2px solid` | `--border-color` → `--accent-primary` au hover | |

---

## 5. Fondations — Ombres et lueurs

### 5.1 Système d'ombres

Déclaré dans `variables.css`. L'opacité augmente en mode sombre (de 0.05–0.10 à 0.30).

| Token | Thème clair | Usage |
|---|---|---|
| `--shadow-sm` | `0 1px 2px rgb(0 0 0 / 0.05)` | État repos des cartes, lang-slider |
| `--shadow-md` | `0 4px 6px -1px + 0 2px 4px -2px / 0.10` | Bouton primaire, flottants |
| `--shadow-lg` | `0 10px 15px -3px + 0 4px 6px -4px / 0.10` | Hover des cartes, bouton hover |
| `--shadow-xl` | `0 20px 25px -5px + 0 8px 10px -6px / 0.10` | Floating cards hero, hover profil |

**Convention** : progression sm → md → lg → xl avec l'élévation perçue. Les éléments au repos utilisent `sm`, les éléments en interaction utilsient `lg` ou `xl`.

### 5.2 Lueurs (glow)

| Token | Valeur (clair) | Valeur (sombre) | Usage |
|---|---|---|---|
| `--glow-green` | `0 0 20px rgba(61,186,124,0.35)` | `0 0 25px rgba(82,201,138,0.4)` | Hover skill-tag, états actifs |
| `--glow-amber` | `0 0 20px rgba(232,151,26,0.35)` | `0 0 25px rgba(232,151,26,0.4)` | Hover btn-secondary |

Les glow ne s'utilisent **qu'au hover ou à l'état actif**, jamais au repos. Sur mobile (`prefers-reduced-motion`), les glow doivent être réduits ou absents.

---

## 6. Fondations — Mouvement

### 6.1 Échelle de durées (à ajouter aux tokens — Phase 1)

```css
:root {
  --duration-instant:    0ms;    /* Changements d'état sans transition */
  --duration-fast:     150ms;    /* Micro-interactions rapides */
  --duration-normal:   300ms;    /* Transitions standard (utilisée partout actuellement) */
  --duration-slow:     500ms;    /* Entrées de page, thème icon transition */
  --duration-cinematic: 600ms+;  /* Animations d'entrée, logo draw */
}
```

### 6.2 Fonctions d'easing

| Token (futur) | Valeur | Usage |
|---|---|---|
| `--ease-standard` | `ease` | Transitions génériques |
| `--ease-enter` | `ease-out` | Éléments qui entrent à l'écran |
| `--ease-exit` | `ease-in` | Éléments qui quittent l'écran |
| `--ease-spring` | `cubic-bezier(0.34, 1.56, 0.64, 1)` | FAB, arc items, interactions magnétiques |
| `--ease-mechanical` | `cubic-bezier(0.4, 0, 0.2, 1)` | Éléments UI précis (contour botte dans l'animation logo) |
| `--ease-smooth` | `cubic-bezier(0.4, 0, 0.2, 1)` | Transitions de thème, nav scroll |

### 6.3 Règle `prefers-reduced-motion`

**Obligatoire sur toute animation non-triviale.** Le pattern systématique :

```css
@media (prefers-reduced-motion: reduce) {
  /* Annuler les animations */
  .element { animation: none; transition: none; }

  /* Pour le logo — afficher directement l'état final */
  #logo-display * {
    animation: none !important;
    stroke-dashoffset: 0 !important;
    opacity: 1 !important;
  }
}
```

L'implémentation actuelle dans `nav-fab.css` respecte déjà cette règle pour les animations du FAB et des items de navigation mobile. Ce pattern est à reproduire sur toute nouvelle animation.

---

## 7. Système logo

### 7.1 Vue d'ensemble

Le logo "Fougère + Botte" existe en trois variantes. C'est un système, pas trois logos distincts. Chaque variante est une résolution du même signe.

| Variante | Taille cible | Complexité SVG | Usage sur le site |
|---|---|---|---|
| **Mark** | 16–64 px | 1 path (silhouette unie) | Favicon, `apple-touch-icon`, avatar |
| **Standard** | 64–256 px | 2 paths (botte outline + fougère fill) | Header navigation (`logo-standard`), watermark |
| **Display** | 256 px → ∞ | Multi-paths animables | Hero homepage, page About, og:image |

### 7.2 Mapping usage → contexte site

| Contexte | Variante | Classe CSS | Comportement |
|---|---|---|---|
| Favicon `favicon.ico` | Mark | n/a | Statique, monochrome |
| `apple-touch-icon.png` | Mark | n/a | Fond vert forêt, silhouette blanche |
| Header desktop (`<nav>`) | Standard | `.logo-standard` | Stroke adaptatif, fougère fixe |
| Header mobile (FAB icon non applicable) | Standard | `.logo-standard` | Identique |
| Hero (`<section class="hero">`) | Display | `.logo-display` | Animé au premier chargement |
| Page About | Display | `.logo-display` | Statique (pas d'animation répétée) |
| og:image (OpenGraph) | Display | n/a | Export PNG statique |
| Blog article header | Display (statique) | `.logo-display` | Sans animation |

### 7.3 Tokens couleur logo

Voir section 1.2 pour les tokens CSS. Résumé :

| Élément | Token | Valeur | Varie avec le thème ? |
|---|---|---|---|
| Fougère (fill) | `--logo-fern-green` | `#52b788` | **Non — invariant absolu** |
| Botte (stroke) | `--logo-boot-stroke` | `#1e293b` clair / `#f8fafc` sombre | Oui |
| Détails botte Display | `--logo-amber` | `#e8971a` | Non |
| Fond silhouette Mark | Fond de page | `--bg-primary` | Oui (transparent sur SVG) |

### 7.4 Classes CSS logo

```css
/* Wrapper commun */
.logo {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2); /* 0.5rem */
}

/* Variante Mark */
.logo-mark {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
}

/* Variante Standard (header) */
.logo-standard {
  width: 48px;
  height: auto;
}

/* Variante Display */
.logo-display {
  width: 100%;
  max-width: 300px;
  height: auto;
}
```

### 7.5 Relation `--logo-fern-green` / `--accent-primary`

Ces deux verts sont délibérément différents :

- `--accent-primary` (`#3dba7c`) : vert UI vivace, légèrement plus saturé
- `--logo-fern-green` (`#52b788`) : vert forêt doux, entre les deux verts UI

La fougère du logo ne doit **jamais** utiliser `--accent-primary`. Elle a sa propre couleur parce qu'elle est une ancre d'identité, pas un élément d'interface. Si elle prenait la couleur des boutons, elle se diluerait dans l'UI.

### 7.6 Principes d'animation SVG (Display seulement)

L'animation est un accueil, pas un spectacle. Elle joue **une seule fois** au chargement de la page qui contient le logo Display. Jamais en boucle.

**Séquence** : botte d'abord (contenant), puis fougère (contenu qui émerge).

| Étape | Élément SVG | Technique | Durée | Délai |
|---|---|---|---|---|
| 1 | `#boot-silhouette` | `stroke-dashoffset` draw | 500 ms | 0 ms |
| 2 | `#boot-sole` | `stroke-dashoffset` draw | 250 ms | 400 ms |
| 3 | `#boot-collar` | `stroke-dashoffset` draw | 200 ms | 600 ms |
| 4 | `#boot-detail-*` | `opacity` fade-in | 200 ms | 800 ms |
| 5 | `#soil-line` | `stroke-dashoffset` draw | 150 ms | 900 ms |
| 6 | `#fern-stem` | `stroke-dashoffset` draw | 300 ms | 1000 ms |
| 7 | `#frond-1` | `stroke-dashoffset` draw | 250 ms | 1200 ms |
| 8 | `#frond-2-left`, `#frond-2-right` | simultané | 250 ms | 1400 ms |
| 9 | `#frond-3-left`, `#frond-3-right` | simultané | 250 ms | 1600 ms |
| 10 | Remplissage `#fern` | `opacity` fade-in | 400 ms | 1700 ms |

**Durée totale** : ~2,1 s. Ne pas dépasser 2,5 s.

**`prefers-reduced-motion`** : affichage direct de l'état final, aucune animation.

---

## 8. Système typographique intégré

### 8.1 Les deux couches

Le site utilise une architecture typographique en deux couches distinctes, non mélangées :

**Couche contenu** (`--font-body` / `--font-display`) — Literata → Fougère  
Tout ce qui est lu : titres, corps de texte, articles, descriptions. Caractère, personnalité botanique, identité.

**Couche interface** (`--font-ui`) — `system-ui`  
Tout ce qui est actionné : navigation, boutons, tags, toggles, labels de formulaire, métadonnées d'articles. Neutre, rapide, fonctionnel.

### 8.2 Règles d'attribution par élément

| Élément | Police | Poids | Taille |
|---|---|---|---|
| `h1.hero-title` | `--font-display` | 800 | 3.5rem |
| `h2.section-title` | `--font-display` | 700 | 2.5rem |
| `h2.article-section h2` | `--font-display` | 700 | 1.75rem |
| `h3.article-section h3` | `--font-body` | 600 | 1.25rem |
| `p` (body, article) | `--font-body` | 400 | 1rem |
| `.hero-description` | `--font-body` | 400 | 1.25rem |
| `.lead-text` | `--font-body` | 400 italic | 1.3rem |
| `.section-subtitle` | `--font-body` | 400 | 1.25rem |
| `.nav-link` | `--font-ui` | 500 | 0.9rem |
| `.btn` | `--font-ui` | 600 | 1rem |
| `.tag` | `--font-ui` | 500 | 0.75rem |
| `.skill-tag` | `--font-ui` | 500 | 0.875rem |
| `.hero-badge` | `--font-ui` | 500 | 0.875rem |
| `.stat-label` | `--font-ui` | 500 uppercase | 0.875rem |
| `.article-toc h3` | `--font-ui` | 700 uppercase | 0.875rem |
| `code`, `pre` | `--font-code` | 400 | 0.875rem/em |

### 8.3 Règles prose (articles de blog)

Les articles `.article-section` ont des règles typographiques spéciales pour la lecture longue :

- `line-height: 1.8` — espacement généreux pour la prose Literata
- `max-width: 800px` sur `.article-body` — limite de la colonne de lecture
- Liens : pas de `text-decoration` native, remplacement par `border-bottom: 1px solid transparent` → `border-bottom-color: --accent-primary` au hover
- Code inline dans la prose : `background: --bg-tertiary`, `border-radius: --radius-xs`, couleur `--accent-primary`
- Blockquotes : `border-left: 4px solid --accent-primary`, fond `--bg-secondary`, italique

### 8.4 `font-optical-sizing`

Toujours activer pour Literata :

```css
/* Dans base.css — déjà implémenté */
body, h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-body); /* ou --font-display */
  font-optical-sizing: auto;
}
```

`font-optical-sizing: auto` est le seul réglage nécessaire — le navigateur et la police variable s'occupent du reste. Ne pas forcer `font-variation-settings: "opsz" X` manuellement sauf cas exceptionnel documenté.

---

## 9. Composants

Pour chaque composant : classes CSS, variantes, tokens utilisés, règles à respecter.

---

### 9.1 Boutons

**Fichier** : `public/css/components/buttons.css`

**Base `.btn`**
- Police : `--font-ui`, poids 600
- `border-radius: --radius-sm` (8 px)
- Padding : `0.75rem 2rem`
- Transition : `transform 0.3s ease, box-shadow 0.3s ease`
- Focus : `outline: 2px solid --accent-primary; outline-offset: 3px`

| Variante | Classe | Fond | Texte | Hover |
|---|---|---|---|---|
| Primaire | `.btn-primary` | `--accent-gradient` (vert → vert profond) | blanc | `translateY(-2px)`, `--shadow-lg` |
| Secondaire | `.btn-secondary` | Gradient vert → ambre (animated 200%) | blanc | `background-position: 100%`, `--glow-amber` |
| Accent | `.btn-accent` | `--accent-tertiary` | blanc | Non défini — à ajouter |

**Do**
- Utiliser `.btn-primary` pour l'action principale de chaque section (1 seul par zone)
- Utiliser `.btn-secondary` pour une alternative ou un CTA de second ordre
- Toujours inclure `focus-visible` dans les tests

**Don't**
- Ne jamais supprimer le `focus-visible` pour des raisons esthétiques
- Ne pas utiliser `.btn-accent` pour des actions destructives — son ambre est associé à un contexte positif (badge, veille)

**Mobile** : largeur 100%, max 300 px, police réduite à `0.875rem`

---

### 9.2 Tags et badges

**Fichier** : `public/css/components/ui.css`

| Composant | Classe | Fond | Texte | Taille | Border-radius |
|---|---|---|---|---|---|
| Tag générique | `.tag` | `--bg-tertiary` | `--text-secondary` | 0.75rem | `--radius-sm` |
| Tag d'article | `.article-tags .tag` | `--bg-secondary` | `--text-secondary` | 0.75rem | `--radius-lg` |
| Skill tag | `.skill-tag` | `--bg-tertiary`, bordure | `--text-secondary` | 0.875rem | `--radius-sm` |
| Badge hero | `.hero-badge` | `--accent-tertiary` | blanc | 0.875rem | `--radius-sm` |

**Hover `.skill-tag`** : fond → `--accent-primary`, couleur → blanc, `--glow-green`, `translateY(-2px)`

**Do** : Les tags sont en `--font-ui`. Ils ne portent pas d'identité de contenu, seulement une fonction de classification.

**Don't** : Ne pas mettre de tags en Literata/Fougère — ce serait mélanger les deux couches.

---

### 9.3 Cartes

**Fichier** : `public/css/components/cards.css`

**Base `.card`**
- `background: --bg-primary`
- `border-radius: --radius-lg` (16 px)
- `border: 1px solid --border-color`
- `box-shadow: --shadow-sm`
- Hover : `translateY(-5px)`, `--shadow-lg`

Les cartes contextuelles (projets, intérêts, statistiques) héritent de `.card` et surchargent les propriétés spécifiques à leur section.

**Stat card** (`.stat`) : même structure que `.card`, avec `stat-number` en 3rem/800, couleur alternée `--accent-primary` / `--accent-tertiary` / `--accent-secondary` selon `:nth-child`.

**Floating card** (hero) : `box-shadow: --shadow-xl`, animation `float` 6s.

---

### 9.4 Navigation

**Fichiers** : `public/css/navigation/`

#### Desktop (`nav`)

- Position : fixed top, `z-index: 1000`
- Fond transparent au repos → `rgba(--bg-primary-rgb, 0.45)` + `backdrop-filter: blur(20px)` au scroll (classe `.scrolled`)
- `.nav-link` : `--font-ui`, 500, 0.9rem — `--text-secondary` → `--text-primary` + `--bg-tertiary` au hover
- `.nav-link-active` : `--accent-secondary`, fond `color-mix(--accent-primary 12%, transparent)`
- Toggles langue et thème : `--bg-tertiary`, `--radius-sm`, `--font-ui`

#### Mobile FAB

- Position : fixed bottom-right, `z-index: 1002`
- FAB : 72 px, `--radius-circle`, gradient `--accent-primary → --accent-secondary`
- Animation d'entrée : `fabEntrance` — `cubic-bezier(0.34, 1.56, 0.64, 1)`
- Ouvert : rotation 45°, fond rouge (`#ff4757`) — signal de fermeture
- Items arc : 48 px cercles, `--bg-primary`, `--border-color`, 5 items en arc 45° (bas-droit → haut-droit)
- Labels flottants : `--radius-xl`, glassmorphism en mode drag

**`prefers-reduced-motion`** : toutes animations FAB désactivées, transitions à `none`.

---

### 9.5 Toggles

#### Toggle langue

Structure : `.lang-toggle` > `.lang-option` × 2 + `.lang-slider` (indicateur glissant)

- Fond : `--bg-tertiary`, `--border-color`, `--radius-sm`
- Slider : `--bg-primary`, `--shadow-sm`, transition `transform 0.3s ease`
- Position du slider contrôlée via `style:transform` inline depuis `navigation.rs`

#### Toggle thème

Structure : `.theme-toggle` > `.theme-icon-container` > `.sun-icon` + `.moon-icon`

- Light : sun visible (`translateX(-0.5rem)`), moon caché (`translateX(0.5rem)`)
- Dark : moon visible, sun caché — transition `0.5s ease`

---

### 9.6 Formulaires

**Fichier** : `public/css/components/forms.css`

**`.form-input`**
- Padding : `1rem`
- `border: 1px solid --border-color`
- `border-radius: --radius-sm`
- `background: --bg-primary` — suit le thème
- `font-family: inherit` — Literata (contexte body)
- Focus : `border-color: --accent-primary`, `box-shadow: 0 0 0 3px rgba(61,186,124,0.1)` (ring vert discret)

**Do** : Le label doit toujours être visible au-dessus du champ, pas seulement comme placeholder. Les formulaires de contact utilisent un label explicite.

**Don't** : Ne pas utiliser `outline: none` sur le focus input sans le remplacer par le border-color. La règle actuelle est correcte — ne pas la retirer.

---

### 9.7 Typographie d'article (prose)

**Fichier** : `public/css/pages/blog-article.css`

L'article est une zone de lecture longue avec des règles spéciales qui surchargent le comportement global :

| Élément | Règle spéciale | Justification |
|---|---|---|
| `.article-section p` | `line-height: 1.8` | Prose longue — plus aéré que le 1.6 global |
| `.article-section a` | Pas de `text-decoration`, `border-bottom: transparent` → couleur au hover | Liens discrets qui s'activent |
| `.article-section blockquote` | `border-left: 4px solid --accent-primary`, fond `--bg-secondary`, italique | Citation visiblement distincte |
| `.article-tldr` | Identique blockquote + `--border-color` bordure basse | Résumé en avant du contenu |
| `.article-toc li a` | `border-left: 2px solid transparent` → `--accent-primary` au hover | Progression de lecture visible |
| `.lead-text` | `font-size: 1.3rem`, italique, `border-left: 4px solid --accent-primary` | Chapô — entre titre et corps |
| Callouts | `.tip` (vert), `.warning` (ambre), `.info` (bleu) | Couleurs sémantiques hors palette principale |

> Les callouts utilisent des couleurs hors palette (`#22c55e`, `#f59e0b`, `#3b82f6`) avec un fond en `rgba()` très discret. Ces couleurs sont sémantiques — elles ne font pas partie de l'identité de marque mais suivent les conventions UI universelles.

---

## 10. Système de thème

### 10.1 Implémentation

Le thème est piloté par l'attribut `data-theme` sur `<html>` (ou `<body>`) :

```css
/* Thème clair — :root (défaut) */
:root { --bg-primary: #ffffff; ... }

/* Thème sombre — override */
[data-theme="dark"] { --bg-primary: #0f172a; ... }
```

Côté Rust/Leptos : `src/services/theme.rs` gère l'état du thème. Le toggle dans `navigation.rs` bascule l'attribut. Le stockage persiste via `services/storage.rs`.

### 10.2 Table de mapping complet

| Token | Thème clair | Thème sombre |
|---|---|---|
| `--bg-primary` | `#ffffff` | `#0f172a` |
| `--bg-secondary` | `#f8fafc` | `#1e293b` |
| `--bg-tertiary` | `#f1f5f9` | `#334155` |
| `--text-primary` | `#0f172a` | `#f8fafc` |
| `--text-secondary` | `#475569` | `#cbd5e1` |
| `--text-tertiary` | `#64748b` | `#94a3b8` |
| `--accent-primary` | `#3dba7c` | `#52c98a` |
| `--accent-secondary` | `#40916c` | `#40916c` |
| `--accent-tertiary` | `#e8971a` | `#e8971a` |
| `--border-color` | `#e2e8f0` | `#334155` |
| `--shadow-sm` | `rgb(0 0 0 / 0.05)` | `rgb(0 0 0 / 0.3)` |
| `--shadow-md/lg/xl` | `/ 0.1` | `/ 0.3` |
| `--glow-green` | `rgba(61,186,124,0.35)` | `rgba(82,201,138,0.4)` |
| `--glow-amber` | `rgba(232,151,26,0.35)` | `rgba(232,151,26,0.4)` |
| `--logo-boot-stroke` | `#1e293b` | `#f8fafc` |

### 10.3 Ce qui ne change JAMAIS

Ces valeurs sont définies dans `:root` et **ne sont pas surchargées** dans `[data-theme="dark"]` :

- `--accent-tertiary: #e8971a` — l'ambre est une constante
- `--accent-secondary: #40916c` — le vert profond est une constante
- `--logo-fern-green: #52b788` — la fougère ne change pas
- `--logo-amber: #e8971a` — les détails de botte ne changent pas
- `--logo-animation-*` — les timings d'animation sont indépendants du thème

### 10.4 Transition de thème

```css
/* Dans base.css — déjà implémenté */
body, .nav, .card, input, textarea {
  transition: all 0.3s ease;
}
```

La transition `all` est pratique mais coûteuse. Migration future recommandée :

```css
body {
  transition:
    background-color var(--duration-normal) var(--ease-standard),
    color var(--duration-normal) var(--ease-standard);
}
```

---

## 11. Principes d'animation

### 11.1 Hiérarchie des animations

Les animations sont classées par intention, de la plus fonctionnelle à la plus narrative.

**Niveau 1 — Feedback immédiat** (0–150 ms)
Réponse aux interactions. L'utilisateur doit sentir que l'interface réagit.
- Touch/click states
- `fabPulse` (FAB touché)

**Niveau 2 — Transitions d'état** (150–350 ms)
Passage d'un état à l'autre. Continuité visuelle.
- Hover sur boutons (`translateY(-2px)` + shadow)
- Hover sur cartes (`translateY(-5px)`)
- Toggle thème/langue
- Nav link color changes

**Niveau 3 — Entrées d'écran** (350–700 ms)
Éléments qui arrivent dans le viewport.
- `fadeInUp` sur les sections
- `fabEntrance` (FAB mobile)

**Niveau 4 — Narratif** (700 ms–2,5 s)
Animations qui racontent quelque chose.
- Logo draw animation
- `float` sur les floating cards hero

### 11.2 Catalogue des animations actuelles

| Animation | Durée | Easing | Élément | Déclencheur |
|---|---|---|---|---|
| `float` | 6s ∞ | `ease-in-out` | `.floating-card` | Auto |
| `fadeInUp` | 0.6s | `ease` | `.section` | Scroll (via `prefers-reduced-motion: no-preference`) |
| `cvFillProgress` | 1.5s ∞ | `ease-in-out` | CV btn loading | État `loading` |
| `cvIconPulse` | 1.2s ∞ | `ease-in-out` | CV icon | État `loading` |
| `cvSuccessBounce` | 0.6s | `ease-out` | CV icon | État `success` |
| `fabEntrance` | 0.6s | spring | `.mobile-fab` | Mount |
| `fabBreath` | 3s ∞ | `ease-in-out` | `.mobile-fab::before` | Auto (halo) |
| `fabPulse` | 0.3s | `ease` | `.mobile-fab` | Touché |
| `magneticPulse` | 0.5s ∞ alt | — | `.mobile-fab.near-target` | Drag |
| `readyPulse` | 0.3s ∞ alt | — | `.mobile-fab.ready-trigger` | Drag proche |
| `dragHintRipple` | 0.65s × 2 | `ease-out` | `.mobile-fab.drag-open::after` | Post drag-open |
| `itemGlowHint` | 0.45s × 2 alt | `ease-in-out` | items drag-hint | Post drag-open |
| `magneticGlow` | 0.5s ∞ alt | — | `.mobile-nav-item.near-target` | Drag |
| `readyGlow` | 0.3s ∞ alt | — | `.mobile-nav-item.ready-trigger` | Drag proche |
| `iconBounce` | 0.5s ∞ alt | — | icône near/ready | Drag |
| `activationPulse` | 0.3s | `ease-out` | item activé | Sélection |

### 11.3 Micro-interaction hover — le pattern `translateY(-2px)`

C'est le pattern d'élévation standard du site. Il simule le soulèvement d'un élément sous le curseur.

```css
/* Pattern standard — implémenté dans animations.css */
.element {
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}
.element:hover {
  transform: translateY(-2px);
}
```

Variante pour les cartes (soulèvement plus prononcé) :
```css
.card:hover { transform: translateY(-5px); }
```

Ne jamais utiliser `translateY()` > `10px` sur des éléments inline ou des éléments de navigation — le mouvement devient désorientant.

---

## 12. Accessibilité

### 12.1 Focus visible

Règle globale dans `base.css` :

```css
:focus-visible {
  outline: 2px solid var(--accent-primary);
  outline-offset: 3px;
}
```

**Ne jamais supprimer `focus-visible`.** La règle `outline: none` sans remplacement est une faute d'accessibilité grave. Les formulaires remplacent `outline: none` par `border-color + box-shadow` — c'est le seul pattern acceptable.

### 12.2 Contrastes — règles pratiques

Déduites du tableau de contraste section 1.5 :

| Situation | Règle |
|---|---|
| Texte courant sur fond | Utiliser `--text-primary` (16:1) ou `--text-secondary` (7:1). Jamais `--text-tertiary` en-dessous de 18 px. |
| Texte en couleur accent | `--accent-secondary` (#40916c, 4.5:1) pour le texte. `--accent-primary` uniquement pour grand texte ou non-texte. |
| Texte blanc sur fond coloré | Vérifier le ratio. Blanc sur `--accent-primary` clair (3.1:1) : acceptable uniquement pour grand texte (>18pt) ou gras (>14pt). |
| Ambre `--accent-tertiary` | Ratio faible — ne pas l'utiliser pour du texte. Uniquement pour les badges, bordures, icônes. |

### 12.3 Tailles minimales de police

| Contexte | Taille minimale | Règle |
|---|---|---|
| Corps de texte | `1rem` (16 px) | Jamais en-dessous |
| Labels d'interface | `0.75rem` (12 px) | Minimum absolu, poids 500+ requis |
| Code inline | `0.875em` | Relative au contexte — s'adapte |
| Captions/métadonnées | `0.75rem` | Poids 500+ pour compenser |

### 12.4 Navigation clavier

- Tous les liens, boutons et éléments interactifs sont dans l'ordre du DOM
- Le FAB mobile est accessible via `tabindex` et `aria-label`
- Les toggles langue/thème ont des `aria-label` ou des contenus visibles
- Les SVGs décoratifs ont `aria-hidden="true"`
- Les SVGs fonctionnels (logo lien) ont un `aria-label` sur le `<a>` parent

### 12.5 Mouvement — utilisateurs sensibles

Les conditions médicales (épilepsie, vertiges, nausées) peuvent être aggravées par les animations. Règles non négociables :

1. `prefers-reduced-motion: reduce` sur toute animation `> 300ms` ou en boucle
2. L'animation du logo (narrative, 2,1 s) doit s'afficher directement en état final si `prefers-reduced-motion`
3. `float` (6s, infini) sur les floating cards : désactiver sous `prefers-reduced-motion`
4. `fabBreath` (3s, infini) : déjà désactivé dans `nav-fab.css` — pattern à suivre

### 12.6 Contenu alternatif

- Toute image fonctionnelle a un `alt` descriptif
- Les icônes emoji utilisées comme décoration ont `aria-hidden="true"` ou `role="img" aria-label="..."`
- Les SVGs logo en inline Leptos : `aria-label` sur le `<svg>`, ou sur le `<a>` parent avec `aria-hidden` sur le SVG

---

## 13. Feuille de route d'implémentation

### Phase 0 — Fait (état actuel)

- [x] Tokens couleurs dans `variables.css` (palette complète, thème clair/sombre)
- [x] Tokens radius dans `variables.css`
- [x] Tokens ombres dans `variables.css`
- [x] Tokens glow dans `variables.css`
- [x] `@font-face` Literata auto-hébergée dans `fonts.css`
- [x] `font-optical-sizing: auto` sur `body` et headings
- [x] Composants construits : buttons, cards, forms, ui, navigation desktop, FAB mobile
- [x] `prefers-reduced-motion` sur FAB
- [x] `focus-visible` global
- [x] Guide logo (`logo-guide.md`)
- [x] Guide typographique Fougère (`typo-fougere-guide.md`)
- [x] Ce document (`design-system-guide.md`)

### Phase 1 — Fait

- [x] Ajouter `--font-display`, `--font-body`, `--font-ui`, `--font-code` dans `variables.css`
- [x] Ajouter l'échelle `--space-*` dans `variables.css`
- [x] Ajouter les tokens de durée `--duration-*` dans `variables.css`
- [x] Ajouter les tokens d'easing `--ease-*` dans `variables.css`
- [x] Ajouter les tokens logo `--logo-fern-green`, `--logo-boot-stroke`, etc. dans `variables.css`
- [x] Migrer `body` vers `font-family: var(--font-body)`
- [x] Migrer `.nav`, `.btn`, `.tag`, `.skill-tag` et tous les éléments UI vers `font-family: var(--font-ui)`
- [x] Créer `.logo`, `.logo-mark`, `.logo-standard`, `.logo-display` dans `components/logo.css`
- [ ] Migrer les `transition: all 0.3s ease` restants vers des transitions explicites (en cours)

### Phase 2 — Intégration Fougère

- [ ] Photographier le corpus botanique (*Dryopteris filix-mas*)
- [ ] Extraire les modules anatomiques (Inkscape)
- [ ] Créer l'alphabet pilote 8 lettres (FontForge)
- [ ] Tester "Kevin Bourbasquet" en display
- [ ] Compléter l'alphabet latin + accents français
- [ ] Export `.woff2` variable ou statique
- [ ] Ajouter `@font-face` Fougère dans `fonts.css`
- [ ] Mettre à jour `--font-display: "Fougère", "Literata", Georgia, serif;`

### Phase 3 — Création logo SVG

- [ ] Dessiner `logo-mark.svg` (viewBox 100×100, 1–2 paths)
- [ ] Dessiner `logo-standard.svg` (viewBox 100×140, 2 paths)
- [ ] Dessiner `logo-display.svg` (viewBox 200×280, paths nommés)
- [ ] Exports PNG favicon set
- [ ] Intégration dans le composant header Leptos
- [ ] Remplacer le texte `.logo-text` actuel par le SVG Standard

### Phase 4 — Animation logo

- [ ] Créer `logo-display-animated.svg` avec CSS draw animation
- [ ] Calculer les `stroke-dasharray` valeurs en JS au mount
- [ ] Implémenter la séquence (voir section 7.6)
- [ ] Tester `prefers-reduced-motion`
- [ ] Intégrer dans la section hero (1 seule lecture au chargement)

---

## Annexe A — Import CSS

Ordre d'import dans `style.css` (déjà correct, documenté ici pour référence) :

```css
/* 1. Fonts — @font-face avant tout usage */
@import url("/public/css/fonts.css");

/* 2. Foundation */
@import url("/public/css/variables.css");
@import url("/public/css/base.css");
@import url("/public/css/animations.css");

/* 3. Navigation */
@import url("/public/css/navigation/nav-base.css");
@import url("/public/css/navigation/nav-layout.css");
@import url("/public/css/navigation/nav-responsive.css");
@import url("/public/css/navigation/nav-fab.css");

/* 4. Composants partagés */
@import url("/public/css/components/buttons.css");
@import url("/public/css/components/cards.css");
@import url("/public/css/components/forms.css");
@import url("/public/css/components/ui.css");
/* Phase 1 : ajouter components/logo.css ici */

/* 5. Sections homepage */
@import url("/public/css/sections/hero.css");
/* ... */

/* 6. Pages */
@import url("/public/css/pages/blog-index.css");
@import url("/public/css/pages/blog-article.css");
/* ... */

/* 7. Responsive global — dernier = plus haute spécificité */
@import url("/public/css/responsive.css");
```

## Annexe B — Checklist nouvelle fonctionnalité

Avant tout nouveau composant ou section :

- [ ] Les couleurs utilisent exclusivement des tokens `var(--...)`
- [ ] Les tailles de police respectent l'échelle (section 2.4)
- [ ] La police est `--font-ui` (interface) ou `--font-body`/`--font-display` (contenu) — pas de mélange
- [ ] Le radius vient de l'échelle `--radius-*`
- [ ] Les ombres viennent de `--shadow-*`
- [ ] Les transitions spécifient les propriétés affectées (pas `all`) — sauf pendant la migration
- [ ] `prefers-reduced-motion` est géré si la transition > 300 ms ou en boucle
- [ ] `focus-visible` est présent sur tout élément interactif
- [ ] Le contraste texte/fond respecte WCAG AA
- [ ] Le composant fonctionne dans les deux thèmes

---

*Source de vérité du design system — Kevin Bourbasquet — v1.0 — 2026-05-18*  
*Ce document évolue avec le projet. Toute décision qui s'en écarte doit être documentée ici avant implémentation.*
