# Système d'icônes — Guide de conception et de migration

**Projet** : bourbask.github.io — Design system personnel  
**Designer/Client** : Kevin Bourbasquet  
**Version du document** : 0.1 (document de travail)  
**Date** : 2026-05-18  
**Statut** : En définition — aucun fichier final produit

---

## Sommaire

1. [Pourquoi remplacer les émojis](#1-pourquoi-remplacer-les-émojis)
2. [Philosophie du set](#2-philosophie-du-set)
3. [Grille et géométrie](#3-grille-et-géométrie)
4. [Règles de style](#4-règles-de-style)
5. [Catalogue complet des icônes](#5-catalogue-complet-des-icônes)
6. [Système d'illustrations botaniques](#6-système-dillustrations-botaniques)
7. [Logos sociaux revisités](#7-logos-sociaux-revisités)
8. [Implémentation dans Leptos/WASM](#8-implémentation-dans-leptoswasm)
9. [Carte de migration — émojis vers SVG](#9-carte-de-migration--émojis-vers-svg)
10. [Ce qu'il ne faut pas faire](#10-ce-quil-ne-faut-pas-faire)

---

## 1. Pourquoi remplacer les émojis

### Le problème de fond

Un émoji est un caractère Unicode dont le rendu est délégué au système d'exploitation. Sur macOS, il sera rond et brillant. Sur Android, légèrement différent. Sur Linux, ça dépend de la distribution et de la police installée. Il n'y a aucune garantie de consistance visuelle, aucune possibilité de thématisation, aucun contrôle sur la taille, les couleurs, les proportions.

Ce problème est tolérable dans un contexte de communication (messagerie, commentaires). Il est rédhibitoire dans un design system où chaque pixel exprime une intention.

### Les trois raisons concrètes

**Identité incoherente.** Un 🌿 d'Apple et un 🌿 de Google sont deux objets visuels différents. Le design system qu'on construit a des tokens de couleur précis, une palette définie, un style de tracé. Un émoji ne peut pas respecter ces tokens. Il vit hors du système.

**Accessibilité dégradée.** Les lecteurs d'écran annoncent un émoji par son nom Unicode complet — « herb emoji », « robot face emoji ». Ce n'est pas ce qu'on veut dire à un utilisateur aveugle qui navigue sur le portfolio. Un SVG avec `aria-label` ou `aria-hidden` (selon le contexte) permet de contrôler exactement ce qui est annoncé, ou de ne rien annoncer quand l'émoji est purement décoratif.

**Positionnement professionnel.** Un émoji dans un titre principal ou une carte de projet envoie un signal ambigu — entre le playful et le peu soigné. Les icônes SVG faites à la main, cohérentes entre elles, avec les même traits et les mêmes proportions, envoient un signal de maîtrise. C'est un détail qui ne se voit pas directement, mais qui se ressent.

### Ce qu'on garde

Les émojis de drapeaux 🇫🇷 🇬🇧 dans la navigation mobile restent — ils sont le seul moyen sémantiquement unambiguë de représenter une langue à cette taille, et leur contexte (navigation) justifie le registre fonctionnel.

Les émojis dans le *contenu* des articles Markdown (callouts ℹ️ ⚠️ 💡) restent également — ils appartiennent au corpus éditorial, pas au design system.

---

## 2. Philosophie du set

### Le trait qui relie tout

Les icônes existantes du site (soleil, lune, flèche, téléchargement) sont déjà stroke-based — des lignes, pas des formes remplies. C'est le bon point de départ. On formalise et on étend ce choix.

Un stroke icon, c'est une icône construite sur un trait continu, comme un dessin au crayon. À l'opposé, un fill icon est une silhouette pleine. Le stroke convient mieux à l'identité botanicale qu'on construit — les gravures de botanique du 19e siècle sont des dessins à la plume, des tracés précis et fins, jamais des aplats.

### Deux familles, une cohérence

**Icônes UI** (24×24px)  
Icônes fonctionnelles et navigables : toggle thème, toggle langue, téléchargement, liens, filtres. Trait fin (1.75px), tracé pur, aucune décoration. L'objectif est la lisibilité parfaite à petite taille, dans les deux thèmes.

**Illustrations botaniques** (48–96px)  
Illustrations décoratives pour les sections hero, projets, intérêts, contact. Plus évocatrices, avec plus de détail. Elles peuvent mélanger stroke et fill local, utiliser les accents de couleur. Elles sont le lien visible entre le contenu du site et l'identité visuelle.

### L'influence botanique

Les icônes UI sont épurées, mais leur style de tracé s'inspire des instruments de dessin scientifique : précision des angles, régularité du trait, économie des formes. Aucune icône n'a de forme qui ne soit pas justifiée fonctionnellement.

Les illustrations, elles, peuvent citer la botanique directement : une feuille de fougère dans une illustration de projet, une courbe organique dans un fond de section. Ce n'est pas un motif imposé sur chaque élément — c'est une présence discrète qui crée la cohérence.

---

## 3. Grille et géométrie

### Icônes UI — grille 24×24

```
┌────────────────────────────────┐
│  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·│  24px
│  ·  ╔══════════════════╗  ·  ·│
│  ·  ║                  ║  ·  ·│
│  ·  ║   Zone de dessin ║  ·  ·│  20×20px
│  ·  ║   (1px marge)    ║  ·  ·│
│  ·  ║                  ║  ·  ·│
│  ·  ╚══════════════════╝  ·  ·│
│  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·│
└────────────────────────────────┘
  Marge : 2px de chaque côté
  Viewport SVG : 0 0 24 24
```

- **Marge optique** : 2px (certains éléments ronds peuvent déborder légèrement de 1px si la forme le justifie — ex : cercle soleil)
- **Taille d'export** : 24×24px base, scalable
- **Points d'ancrage** : préférer les demi-valeurs (0.5, 1.5, 11.5...) pour les tracés qui ne tombent pas sur la grille — cela évite le flou subpixel sur les écrans non-retina

### Illustrations botaniques — grille 64×64

```
┌──────────────────────────────────────────┐
│  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·│  64px
│  ·  ╔══════════════════════════╗  ·  ·  │
│  ·  ║                          ║  ·  ·  │
│  ·  ║    Zone de dessin        ║  ·  ·  │
│  ·  ║    (8px marge)           ║  ·  ·  │  48×48px
│  ·  ║                          ║  ·  ·  │
│  ·  ╚══════════════════════════╝  ·  ·  │
│  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·│
└──────────────────────────────────────────┘
  Viewport SVG : 0 0 64 64
  Marge optique : 8px
```

### Logos sociaux — grille 20×20

GitHub et LinkedIn utilisent leurs propres proportions officielles. On les centre dans une grille 20×20 et on leur applique notre `fill="currentColor"`. Viewport : `0 0 20 20`.

---

## 4. Règles de style

### Attributs SVG communs (icônes UI)

```svg
<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 24 24"
  width="24"
  height="24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.75"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <!-- paths -->
</svg>
```

**`stroke-linecap: round`** — les extrémités des traits sont arrondies. C'est l'une des deux règles les plus importantes. Sans ça, le set a l'air rigide et technique. Avec ça, il y a de la chaleur sans être soft.

**`stroke-linejoin: round`** — les angles des tracés sont arrondis. Même impact que le linecap sur la personnalité globale.

**`stroke-width: 1.75`** — entre 1.5 (trop fin, disparaît en dark mode) et 2 (trop lourd pour des petits éléments). 1.75 est le point d'équilibre à 24px.

**`fill="none"`** — règle par défaut. Seuls les éléments fermés et intentionnellement pleins (ex : le dot de l'icône LinkedIn, certains éléments d'illustration) reçoivent `fill="currentColor"` ou une valeur de la palette.

**`aria-hidden="true"`** — toujours, sauf si l'icône est le seul label de l'élément interactif (cf. section 8).

### Modifications pour les icônes existantes

Les icônes actuelles (soleil, lune, téléchargement, flèche) manquent de `stroke-linecap` et `stroke-linejoin`. Lors de leur migration vers des composants SVG dédiés, on ajoute ces deux attributs.

### Attributs SVG communs (illustrations botaniques)

```svg
<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 64 64"
  width="64"
  height="64"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <!-- paths -->
</svg>
```

Les illustrations peuvent avoir des éléments avec `fill` local (ex : une feuille de fougère avec `fill="var(--accent-primary)"` à 20% d'opacité). Ce fill local est toujours déclaré sur l'élément, jamais hérité.

### Couleur et thématisation

| Contexte | Valeur |
|----------|--------|
| Icône monochrome adaptative | `stroke="currentColor"` |
| Accent primaire (vert forêt) | `stroke="var(--accent-primary)"` |
| Accent tertiaire (ambre) | `stroke="var(--accent-tertiary)"` |
| Fill léger décoratif | `fill="var(--accent-primary)" fill-opacity="0.12"` |
| Logo social (solide) | `fill="currentColor"` |

Ne jamais utiliser de valeur hexadécimale directe dans un SVG destiné au site — toujours passer par `currentColor` ou les custom properties CSS pour garantir la réactivité au thème.

---

## 5. Catalogue complet des icônes

### 5.1 Icônes UI — contrôles de navigation

#### Soleil (toggle thème → mode clair)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="5"/>
  <line x1="12" y1="2"    x2="12" y2="4"/>
  <line x1="12" y1="20"   x2="12" y2="22"/>
  <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
  <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
  <line x1="2"  y1="12" x2="4"  y2="12"/>
  <line x1="20" y1="12" x2="22" y2="12"/>
  <line x1="4.22"  y1="19.78" x2="5.64"  y2="18.36"/>
  <line x1="18.36" y1="5.64"  x2="19.78" y2="4.22"/>
</svg>
```

*Note : les rayons commencent à y=2 (pas y=1) pour respecter la marge de 2px.*

#### Lune (toggle thème → mode sombre)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
</svg>
```

#### Flèche droite (CTA, liens projet, blog)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <line x1="5" y1="12" x2="19" y2="12"/>
  <polyline points="12 5 19 12 12 19"/>
</svg>
```

*Sépare la ligne du chevron en deux éléments distincts pour faciliter les animations éventuelles.*

### 5.2 Icônes UI — actions

#### Télécharger (bouton CV)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
  <polyline points="7 10 12 15 17 10"/>
  <line x1="12" y1="15" x2="12" y2="3"/>
</svg>
```

#### Cadenas fermé (venir bientôt / PGP / E2E)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <rect x="5" y="11" width="14" height="10" rx="2" ry="2"/>
  <path d="M8 11V7a4 4 0 0 1 8 0v4"/>
</svg>
```

*Usage dual : badge "coming soon" dans les projets (disabled state) + note de chiffrement PGP dans le formulaire de contact.*

#### Email / enveloppe

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
  <polyline points="22,6 12,13 2,6"/>
</svg>
```

#### Feuille botanique (footer — remplace ❤️)

Le cœur dans "Conçu avec ❤️" n'est ni sémantique ni identitaire — c'est une convention héritée. On le remplace par un motif cohérent avec l'identité : une petite feuille de fougère, référence discrète au logo et au reste du système.

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <!-- Nervure centrale -->
  <path d="M12 20 C12 14, 12 10, 12 4"/>
  <!-- Folioles gauches -->
  <path d="M12 16 C10 14, 7 13, 5 14"/>
  <path d="M12 13 C10 11, 7 10, 5 10"/>
  <path d="M12 10 C10 8, 8 7, 6 7"/>
  <!-- Folioles droites -->
  <path d="M12 16 C14 14, 17 13, 19 14"/>
  <path d="M12 13 C14 11, 17 10, 19 10"/>
  <path d="M12 10 C14 8, 16 7, 18 7"/>
</svg>
```

*Ce motif est la version 24px de l'illustration botanique — même ADN, format contraint.*

### 5.3 Icônes de navigation mobile

#### Blog / édition

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 20h9"/>
  <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
</svg>
```

#### Veille / signal

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
  <path d="M1.42 9a16 16 0 0 1 21.16 0"/>
  <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
  <circle cx="12" cy="20" r="1" fill="currentColor" stroke="none"/>
</svg>
```

*Le point plein en bas ancre l'icône — c'est le récepteur, là où le signal atterrit.*

#### CV / document

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <line x1="16" y1="13" x2="8" y2="13"/>
  <line x1="16" y1="17" x2="8" y2="17"/>
  <line x1="10" y1="9"  x2="8" y2="9"/>
</svg>
```

### 5.4 Icônes de catégorie — Blog et articles

Ces icônes remplacent les émojis retournés par `get_category_emoji()` dans `src/data/articles/mod.rs`.

| Catégorie | Icône actuelle | Remplacement SVG |
|-----------|---------------|------------------|
| `project` | 🚀 | Pièces d'assemblage (cube en perspective axo) |
| `devops` | 🔧 | Clé plate |
| `tools` | 🛠️ | Marteau + clé croisés |
| `hardware` | ⚙️ | Engrenage |
| `system` | 🐧 | Terminal / ligne de commande |
| `default` | 📝 | Plume / édition |

#### Clé plate (devops)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77
           a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91
           a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
</svg>
```

#### Terminal / ligne de commande (system/Linux)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <polyline points="4 17 10 11 4 5"/>
  <line x1="12" y1="19" x2="20" y2="19"/>
</svg>
```

#### Engrenage (hardware)

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="3"/>
  <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83
           l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21
           a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33
           l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15
           a1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9
           a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06
           A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0
           v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06
           a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9
           a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09
           a1.65 1.65 0 0 0-1.51 1z"/>
</svg>
```

---

## 6. Système d'illustrations botaniques

### 6.1 Philosophie des illustrations

Les illustrations ne sont pas des icônes agrandies. Ce sont des compositions à part entière, conçues pour occuper un espace plus large (48–96px) avec plus de détail et d'expression. Elles peuvent citer le répertoire botanique : fougères, frondaisons, nervures, croziers.

La règle d'or : **une illustration doit être lisible en niveaux de gris**. La couleur est un plus, jamais un vecteur de sens.

### 6.2 Hero — trois cartes flottantes

Les trois flottants du hero représentent les trois pôles de l'identité professionnelle de Kevin : technique (🔧), naturel (🌿), terrain (🥾).

#### Carte "Outil" — remplace 🔧

**Concept** : Un circuit de PCB avec une petite fougère qui pousse depuis un via (trou de connexion). La rencontre du monde technique et du monde naturel, en miniature.

**Construction (64×64px) :**
- Fond : rectangle arrondi 56×56, `rx="8"`, `fill="var(--bg-secondary)"`
- Lignes de circuit : 3-4 segments orthogonaux avec des angles à 90°, épaisseur 2px, `stroke="var(--accent-primary)" stroke-opacity="0.4"`
- Vias (jonctions) : cercles de r=2, `fill="var(--accent-primary)" fill-opacity="0.6"`
- Fronde de fougère : 1 nervure centrale en courbe Bézier partant du centre, 4-5 paires de folioles latérales, `stroke="var(--accent-primary)"`, épaisseur 1.5

#### Carte "Nature" — remplace 🌿

**Concept** : Une fronde de fougère détaillée, représentation directe de l'identité botanique. Pas stylisée à l'excès — assez précise pour évoquer une gravure de botanique.

**Construction (64×64px) :**
- Nervure centrale : courbe de Bézier cubique de (32, 58) à (32, 8), légèrement inclinée à 5° pour rompre la symétrie parfaite
- Folioles : 8 paires de petites feuilles ovales sur la nervure, décroissantes vers la pointe
- Chaque foliole : `<ellipse>` orientée par `transform="rotate()"`, épaisseur 1.5, pas de fill
- Crozier visible en haut (pointe enroulée) pour l'authenticité botanique
- Couleur : `stroke="var(--accent-primary)"`

#### Carte "Terrain" — remplace 🥾

**Concept** : La botte de randonnée du logo (Phase 3), en version illustrative. Même ADN mais rendu en contexte de carte — plus proche d'une esquisse rapide que du mark rigoureux du logo.

**Construction (64×64px) :**
- Profil de botte simplifié, vu de côté gauche
- 3-4 lacets horizontaux suggérés par des tirets
- Semelle épaisse, légèrement débordante
- Une petite feuille de fougère sortant de l'ouverture supérieure (citation directe du logo)
- Épaisseur de trait : 2px pour la silhouette, 1.25px pour les détails intérieurs

### 6.3 Projets — illustrations de placeholder

Ces illustrations remplacent les émojis dans `.project-placeholder` (`aria-hidden="true"`).

#### EzPrint3D — remplace 🖨️

**Concept** : Tête d'imprimante 3D en train de déposer une ligne, vue isométrique simplifiée. La ligne déposée forme une petite feuille de fougère (pour citer l'identité tout en restant sémantique du projet).

**Construction (48×48px) :**
- Buse : rectangle étroit `6×12px`, arrondi en bas
- Portique (frame) : 3 lignes formant un U inversé
- Ligne déposée : chemin en courbe organique partant de la buse vers le bas, épaisseur croissante pour simuler le dépôt
- Couleur : `currentColor`, simple

#### LeKickerFou — remplace 🤖

**Concept** : Pas un robot générique — une IA de football. Une table de baby-foot vue de dessus, stylisée. Les joueurs sont des rectangles sur une tige, le ballon est un cercle.

**Construction (48×48px) :**
- Rectangle extérieur (table) avec `rx="4"`
- 2 tiges horizontales (lignes) avec 3 petits rectangles sur chacune
- Cercle central (ballon) de r=4
- Couleur : `currentColor`

#### Custom Keyboards — remplace ⌨️

**Concept** : Vue de dessus d'un clavier 40% (format compact, cohérent avec la culture keyboard). Touches représentées par des petits rectangles arrondis en grille irrégulière.

**Construction (48×48px) :**
- 3 rangées de touches de tailles variées
- Barre espace bien visible (touche large)
- Touches dessinées comme des `<rect rx="2">` avec un léger gap entre elles
- Pas de texte sur les touches
- Couleur : `currentColor`

### 6.4 Intérêts — icônes de section

Ces icônes remplacent les émojis dans les cartes de la section "Quand je ne code pas".

#### Photographie — remplace 📸

```svg
<svg viewBox="0 0 48 48" fill="none" stroke="currentColor"
     stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
  <!-- Corps appareil -->
  <path d="M8 18h32a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H8
           a2 2 0 0 1-2-2V20a2 2 0 0 1 2-2z"/>
  <!-- Sabot supérieur (avec encoche pour l'objectif) -->
  <path d="M16 18v-4a2 2 0 0 1 2-2h4l3 4"/>
  <!-- Objectif — cercles concentriques -->
  <circle cx="24" cy="28" r="7"/>
  <circle cx="24" cy="28" r="4"/>
  <!-- Flash / voyant -->
  <circle cx="38" cy="22" r="1.5" fill="currentColor" stroke="none"/>
</svg>
```

#### Musique — remplace 🎵

```svg
<svg viewBox="0 0 48 48" fill="none" stroke="currentColor"
     stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
  <!-- Portée — 4 lignes -->
  <line x1="8"  y1="16" x2="40" y2="16"/>
  <line x1="8"  y1="21" x2="40" y2="21"/>
  <line x1="8"  y1="26" x2="40" y2="26"/>
  <line x1="8"  y1="31" x2="40" y2="31"/>
  <!-- Note noire (croche) -->
  <ellipse cx="20" cy="32" rx="4" ry="3" transform="rotate(-15 20 32)"
           fill="currentColor" stroke="none"/>
  <line x1="24" y1="31" x2="24" y2="14"/>
  <!-- Crochet de la croche -->
  <path d="M24 14 C28 16, 28 20, 26 22"/>
  <!-- Deuxième note -->
  <ellipse cx="32" cy="30" rx="4" ry="3" transform="rotate(-15 32 30)"
           fill="currentColor" stroke="none"/>
  <line x1="36" y1="29" x2="36" y2="12"/>
  <path d="M36 12 C40 14, 40 18, 38 20"/>
</svg>
```

#### Randonnée / Trekking — remplace 🥾

Réutilise l'illustration de la carte hero "Terrain" (§6.2), en version 48px. Même construction, proportions adaptées. Cohérence maximale entre les deux occurrences du même motif.

#### Linux / Open Source — remplace 🐧

Ne pas représenter Tux — le manchot est une marque déposée gérée par la Linux Foundation. Représenter l'idée à la place : un terminal, symbole universel de Linux et de la culture open source.

Réutilise l'icône Terminal du catalogue UI (§5.4), en version 48px avec `stroke-width="1.5"`.

### 6.5 Section Contact — "Construisons quelque chose"

La section contact a trois liens de contact : email, GitHub, LinkedIn. Les émojis actuels (📧 🐙 💼) sont remplacés par les icônes email (§5.2) et les logos sociaux revisités (§7).

---

## 7. Logos sociaux revisités

### Principes communs

GitHub et LinkedIn sont des marques déposées avec des guidelines d'usage. On respecte leurs formes officielles — pas de distorsion, pas de fusion avec d'autres éléments. Ce qu'on adapte :

1. **La couleur** : on retire leurs couleurs de marque pour passer à `fill="currentColor"`, ce qui les intègre au système de thèmes du site
2. **La pondération** : on s'assure que leur épaisseur optique est cohérente avec les autres icônes du set au même format
3. **Le conteneur** : on leur ajoute un contexte visuel (cercle ou carré arrondi) qui les ancre dans notre grille de 24px

### 7.1 GitHub

Le mark officiel de GitHub est l'Invertocat — la silhouette de l'octocat sans les détails du visage. C'est le seul mark approuvé par GitHub pour les contextes techniques (liens vers des dépôts, profils GitHub).

**Implémentation** :

```svg
<svg viewBox="0 0 24 24" fill="currentColor" stroke="none"
     xmlns="http://www.w3.org/2000/svg" aria-label="GitHub">
  <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504
           .5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703
           -2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466
           -.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032
           .892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338
           -2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688
           -.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026
           A9.564 9.564 0 0 1 12 6.844a9.59 9.59 0 0 1 2.504.337
           c1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651
           .64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943
           .359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747
           0 .268.18.58.688.482A10.019 10.019 0 0 0 22 12.017
           C22 6.484 17.522 2 12 2z"/>
</svg>
```

**Revisitation design system :** dans le contexte des cartes de contact, on encadre le mark dans un cercle stroke :

```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="1.75" stroke-linecap="round">
  <circle cx="12" cy="12" r="10"/>
  <!-- Invertocat simplifié, intérieur, fill="currentColor" -->
  <path fill="currentColor" stroke="none"
        d="M12 7C9.24 7 7 9.24 7 12c0 2.21 1.43 4.09 3.42 4.76
           .25.05.34-.11.34-.24v-.85c-1.39.3-1.68-.67-1.68-.67
           -.23-.58-.55-.73-.55-.73-.45-.31.03-.3.03-.3
           .5.03.76.51.76.51.44.76 1.17.54 1.45.41
           .05-.32.17-.54.31-.67-1.11-.13-2.28-.56-2.28-2.48
           0-.55.19-.99.51-1.34-.05-.13-.22-.64.05-1.33
           0 0 .42-.13 1.37.51.4-.11.82-.17 1.25-.17
           .43 0 .85.06 1.25.17.95-.64 1.37-.51 1.37-.51
           .27.69.1 1.2.05 1.33.32.35.51.79.51 1.34
           0 1.92-1.17 2.35-2.28 2.47.18.15.34.46.34.92
           v1.37c0 .13.09.29.34.24A5 5 0 0 0 17 12c0-2.76-2.24-5-5-5z"/>
</svg>
```

### 7.2 LinkedIn

Le mark officiel LinkedIn est le logotype "in" dans un carré avec coins arrondis. La couleur de marque est le bleu `#0077B5`, qu'on remplace par `currentColor`.

```svg
<svg viewBox="0 0 24 24" fill="currentColor" stroke="none"
     xmlns="http://www.w3.org/2000/svg" aria-label="LinkedIn">
  <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037
           -1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046
           c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286z
           M5.337 7.433a2.062 2.062 0 0 1-2.063-2.065 2.064 2.064 0 1 1 2.063 2.065zm
           1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542
           C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729
           C24 .774 23.2 0 22.222 0h.003z"/>
</svg>
```

**Revisitation** : même traitement que GitHub — le mark officiel sur fond `currentColor` pour l'usage en contact card. Pas de modification de forme.

### 7.3 Règles d'usage partagées

- Ne jamais modifier les proportions des marks officiels
- Toujours inclure `aria-label="GitHub"` ou `aria-label="LinkedIn"` si l'icône est le seul contenu d'un lien
- En usage monochrome (notre cas), respecter les guidelines de zone d'exclusion des deux marques : minimum 4px de marge autour du mark dans son conteneur

---

## 8. Implémentation dans Leptos/WASM

### 8.1 Où vivre les SVG

Trois stratégies selon le type d'icône :

**A — Inline dans le composant Rust** (icônes UI interactives)

Pour les icônes dans des éléments interactifs (boutons, toggles) ou qui ont besoin de réactivité. Le SVG est écrit directement dans le `view!` macro.

```rust
// Exemple : bouton avec icône inline
view! {
    <button
        class="theme-toggle"
        aria-label={move || i18n.t("toggleTheme")}
        on:click=move |_| theme.toggle_theme()
    >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"
             aria-hidden="true">
            <circle cx="12" cy="12" r="5"/>
            // ...
        </svg>
    </button>
}
```

**B — Composant Leptos dédié** (icônes réutilisées dans plusieurs endroits)

Quand la même icône apparaît en plusieurs endroits, la factoriser en composant :

```rust
// src/components/ui/icons.rs
#[component]
pub fn IconDownload(#[prop(default = 24)] size: u32) -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            width={size}
            height={size}
            fill="none"
            stroke="currentColor"
            stroke-width="1.75"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
    }
}
```

**C — Fichiers SVG statiques** (illustrations décoratives, social logos)

Pour les illustrations complexes et les logos sociaux, stocker en fichiers SVG dans `public/images/icons/`. Référencer via `<img>` avec `alt=""` (décoratif) ou `alt="GitHub"` (informatif).

```
public/
└── images/
    └── icons/
        ├── ui/
        │   ├── sun.svg
        │   ├── moon.svg
        │   ├── download.svg
        │   ├── arrow-right.svg
        │   ├── lock.svg
        │   ├── email.svg
        │   └── leaf.svg
        ├── nav/
        │   ├── blog.svg
        │   ├── signal.svg
        │   └── document.svg
        ├── social/
        │   ├── github.svg
        │   └── linkedin.svg
        └── illustrations/
            ├── hero-tool.svg
            ├── hero-nature.svg
            ├── hero-terrain.svg
            ├── project-ezprint.svg
            ├── project-kicker.svg
            ├── project-keyboard.svg
            ├── interest-camera.svg
            ├── interest-music.svg
            ├── interest-boot.svg
            └── interest-terminal.svg
```

### 8.2 Accessibilité — la règle des deux cas

**Cas 1 — Icône purement décorative** (accompagne un texte qui dit la même chose)

```rust
// Le texte "Télécharger" est présent → l'icône est décorative
<button>
    <IconDownload/>  // aria-hidden="true" inclus dans le composant
    "Télécharger"
</button>
```

**Cas 2 — Icône seule, sans texte visible** (l'icône EST le label)

```rust
// Pas de texte → l'icône porte le sens
<button aria-label={move || i18n.t("downloadCV")}>
    <IconDownload/>  // aria-hidden="true" sur le SVG
</button>
```

La règle simple : si supprimer l'icône ferait perdre de l'information → `aria-label` sur l'élément parent + `aria-hidden="true"` sur le SVG. Si supprimer l'icône ne change pas la compréhension → `aria-hidden="true"` suffit.

### 8.3 Adaptation au thème

Les icônes `stroke="currentColor"` et `fill="currentColor"` héritent automatiquement de la couleur du texte de leur contexte CSS (`color: var(--text-primary)`). Aucune logique Rust spécifique n'est nécessaire.

Pour les illustrations avec couleurs d'accent (ex : feuille verte dans hero card) :

```css
/* La couleur est définie en CSS, pas dans le SVG */
.hero-illustration-nature {
    color: var(--accent-primary); /* stroke/fill héritent */
}
```

---

## 9. Carte de migration — émojis vers SVG

Inventaire complet avec statut de priorité.

| Emplacement | Fichier | Émoji actuel | Remplacement | Priorité |
|-------------|---------|--------------|--------------|----------|
| Hero flottant 1 | `hero.rs` | 🔧 | Illustration "Outil" (§6.2) | Phase 4 |
| Hero flottant 2 | `hero.rs` | 🌿 | Illustration "Nature" (§6.2) | Phase 4 |
| Hero flottant 3 | `hero.rs` | 🥾 | Illustration "Terrain" (§6.2) | Phase 4 |
| Toggle thème (nav) | `navigation.rs` | SVG sans linecap | SVG avec linecap round | Phase 3 |
| Toggle thème (mobile) | `mobile_nav.rs` | SVG sans linecap | SVG avec linecap round | Phase 3 |
| Bouton CV | `cv_download.rs` | SVG sans linecap | SVG avec linecap round | Phase 3 |
| Flèches CTA | `projects.rs`, `blog_page.rs` | SVG sans linecap | SVG avec linecap round | Phase 3 |
| Placeholder EzPrint3D | `projects.rs` | 🖨️ | Illustration imprimante (§6.3) | Phase 4 |
| Placeholder LeKickerFou | `projects.rs` | 🤖 | Illustration table de foot (§6.3) | Phase 4 |
| Placeholder Keyboards | `projects.rs` | ⌨️ | Illustration clavier 40% (§6.3) | Phase 4 |
| Cadenas coming-soon | `projects.rs` | 🔒 | IconLock (§5.2) | Phase 3 |
| Lien blog (mobile) | `mobile_nav.rs` | 📝 | IconBlog (§5.3) | Phase 3 |
| Lien veille (mobile) | `mobile_nav.rs` | 📡 | IconSignal (§5.3) | Phase 3 |
| Bouton CV (mobile) | `mobile_nav.rs` | 📄 | IconDocument (§5.3) | Phase 3 |
| Contact email | `contact.rs` | 📧 | IconEmail (§5.2) | Phase 3 |
| Contact GitHub | `contact.rs` | 🐙 | Logo GitHub (§7.1) | Phase 3 |
| Contact LinkedIn | `contact.rs` | 💼 | Logo LinkedIn (§7.2) | Phase 3 |
| Cadenas PGP | `contact.rs` | 🔐 | IconLock (§5.2) | Phase 3 |
| Intérêt photo | `interests.rs` | 📸 | IconCamera (§6.4) | Phase 4 |
| Intérêt musique | `interests.rs` | 🎵 | IconMusic (§6.4) | Phase 4 |
| Intérêt trekking | `interests.rs` | 🥾 | Illustration Boot (§6.4) | Phase 4 |
| Intérêt Linux | `interests.rs` | 🐧 | IconTerminal (§5.4) | Phase 4 |
| Cœur footer | `footer.rs` | ❤️ | IconLeaf (§5.2) | Phase 3 |
| Catégories blog | `data/articles/mod.rs` | 🚀🔧🛠️⚙️🐧📝 | Set icônes catégories (§5.4) | Phase 3 |
| 404 page | `not_found.rs` | 🔍🤖🏠📝 | IconSearch, robot illustr., home, blog | Phase 4 |

### Phasage

**Phase 3 — Icônes UI** (peut commencer sans les illustrations)  
Toutes les icônes de contrôle et de navigation. Ce sont des SVGs simples, faisables sans outil de dessin. Mise à jour des SVG existants (linecap), création des icônes manquantes, migration des émojis dans les composants fonctionnels.

**Phase 4 — Illustrations** (après le logo SVG de Phase 3)  
Les illustrations botaniques et les placeholders de projet. Nécessitent Inkscape et du temps de dessin. Commencer après avoir validé le style du logo (les illustrations partagent le même vocabulaire formel).

---

## 10. Ce qu'il ne faut pas faire

**Ne pas mélanger les styles dans un même composant.** Si une icône du toggle thème a `stroke-linecap: round`, toutes les icônes de la navigation doivent avoir `stroke-linecap: round`. L'incohérence se voit immédiatement.

**Ne pas utiliser des SVGs issus de bibliothèques sans vérification.** Heroicons, Feather Icons, Lucide — ce sont d'excellentes sources d'inspiration, mais leurs icônes ont parfois des `stroke-width` différents (2px plutôt que 1.75px), pas de `stroke-linecap`, ou des formes qui ne s'accordent pas avec notre identité botanique. Adapter ou recréer, ne pas copier-coller.

**Ne pas animer les icônes UI fonctionnelles.** Le toggle thème peut avoir une transition CSS `opacity` ou `transform`. Il ne doit pas avoir d'animation élaborée — ce sont des contrôles, pas des éléments expressifs. Garder les animations pour les illustrations et le logo.

**Ne pas mettre de fill coloré sur les icônes UI.** Toutes les icônes UI sont `stroke="currentColor"`. Un fill coloré est réservé aux illustrations et aux éléments décoratifs. Une icône UI avec du vert accent ressemble à une notification, pas à un contrôle.

**Ne pas utiliser des valeurs hexadécimales directes dans les SVG du site.** Aucun `stroke="#3dba7c"` dans le code. Toujours `stroke="currentColor"` ou `stroke="var(--accent-primary)"`. Sans ça, les icônes ne réagissent pas au thème et les valeurs se désynchronisent de la palette à la prochaine mise à jour.

**Ne pas modifier les marks de marque (GitHub, LinkedIn).** Pas de rotation, pas de distorsion, pas de fusion avec d'autres formes. La revisitation se limite à la couleur et au contexte (cercle de fond). Toute autre modification viole les guidelines de marque.

**Ne pas oublier `aria-hidden="true"` sur les SVG.** Un SVG sans cet attribut est lu par les lecteurs d'écran, qui annoncent son contenu interne — les paths, les cercles, les lines — ce qui est absurde. Toujours soit `aria-hidden="true"` (décoratif), soit `aria-label="..."` sur l'élément parent + `aria-hidden="true"` sur le SVG lui-même.
