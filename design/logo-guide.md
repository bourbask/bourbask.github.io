# Fougère + Botte — Guide de conception du logo

**Projet** : Logo de marque personnelle — Kevin Bourbasquet  
**Designer/Client** : Kevin Bourbasquet  
**Version du document** : 0.1 (document de travail)  
**Date** : 2026-05-18  
**Statut** : En définition — aucun fichier final produit

---

## Sommaire

1. [Narration de marque](#1-narration-de-marque)
2. [Système à trois variantes](#2-système-à-trois-variantes)
3. [Règles couleur](#3-règles-couleur)
4. [Architecture SVG](#4-architecture-svg)
5. [Chorégraphie d'animation](#5-chorégraphie-danimation)
6. [Principes de construction formelle](#6-principes-de-construction-formelle)
7. [Intégration au système CSS](#7-intégration-au-système-css)
8. [Livrables attendus](#8-livrables-attendus)
9. [Ce qu'il ne faut pas faire](#9-ce-quil-ne-faut-pas-faire)

---

## 1. Narration de marque

### Le concept

Une fougère qui pousse depuis l'intérieur d'une botte de randonnée. Ce n'est pas une illustration — c'est un signe. La différence est fondamentale et conditionne toutes les décisions qui suivent.

### Les trois niveaux de lecture

Un logo efficace se lit à plusieurs vitesses. Celui-ci en a trois.

**Niveau 1 — Littéral** (0 à 2 secondes)  
Une plante sort d'une botte. Lisible par quiconque. Pas besoin de contexte.

**Niveau 2 — Interprétatif** (après quelques secondes)  
La nature reprend possession d'un objet humain. La botte n'est pas abandonnée — elle est habitée. Ce n'est pas une ruine, c'est une cohabitation. Ce niveau parle de la relation entre l'humain et le vivant : l'outil au service du milieu naturel, pas l'inverse.

**Niveau 3 — Référentiel** (pour qui connaît la référence)  
WALL-E. La plante qui survit dans l'inhospitalier. La croissance comme acte de résistance. La vie qui trouve un chemin dans ce qui n'était pas prévu pour elle. Ce niveau encode les valeurs de Kevin : souveraineté numérique, responsabilité écologique, résilience.

Ces trois lectures coexistent dans le même signe. Aucune ne s'impose à l'autre. C'est ce qui fait que le logo vieillit bien : on peut le regarder des dizaines de fois et y trouver quelque chose de nouveau selon son humeur ou son niveau d'attention.

### Pourquoi ce signe fonctionne comme logo

- **Mémorable** : la combinaison plante + botte est inattendue. L'inattendu se mémorise.
- **Personnel** : la fougère est la plante préférée de Kevin. Ce n'est pas un choix "nature générique".
- **Scalable narrativement** : le signe peut s'accompagner d'un texte court ou exister seul. Il n'a pas besoin d'explication mais peut en avoir une.
- **Visuellement asymétrique** : la botte donne de l'ancrage et de la masse, la fougère donne de la légèreté et de la direction verticale. L'ensemble ne sera jamais confondu avec un logo symétrique, ce qui aide à la reconnaissance.

### Ce que ce logo n'est pas

Il n'est pas :
- Un logo "outdoor" ou "randonnée"
- Un logo "jardinage" ou "nature lifestyle"
- Un logo illustratif générique au style gravure

Il est un signe d'identité personnelle avec une profondeur narrative que le regardeur peut choisir d'aller chercher ou non.

---

## 2. Système à trois variantes

Un seul concept, trois niveaux de détail. La contrainte est claire : une seule identité, pas trois logos différents. Chaque variante est une résolution différente du même signe.

### Matrice d'usage

| Variante | Taille cible | Contextes | Niveau de détail |
|---|---|---|---|
| **Mark** | 16 – 64 px | Favicon, app icon, avatar | Silhouette pure |
| **Standard** | 64 – 256 px | Header du site, watermark, signature | Outline + couleur |
| **Display** | 256 px → ∞ | Hero, about, impression, animation | Illustration structurée |

---

### Variante 1 — Mark (la plus petite)

**Principe** : tout sacrifice est autorisé au bénéfice de la reconnaissance.

À 16 pixels, un SVG ne peut véhiculer qu'une silhouette. La Mark est cette silhouette : la forme globale de la botte + l'émergence de la fougère. Un seul path, une seule couleur de fond, pas de contours internes.

**Ce qui reste** :
- La silhouette de la botte vue de côté (profil droit) : tige, col, semelle
- L'émergence de la fougère : 2 à 3 frondes schématiques au-dessus du col
- La tension entre la forme lourde (botte) et la forme légère (fougère)

**Ce qui disparaît** :
- Les détails de la semelle
- Les lacets (à moins qu'ils deviennent une ligne graphique forte — à tester)
- Les frondaisons individuelles de la fougère : tout se fond en silhouette

**Forme** : remplie / filled. Monochrome.  
**Fond** : carré ou cercle, selon contexte (favicon = carré, avatar = cercle).

**Adaptation au thème** :  
Le fond est la variable. La silhouette est fixe.

| Thème | Fond du conteneur | Couleur de la silhouette |
|---|---|---|
| Clair | `#ffffff` ou `#f8fafc` | Vert forêt `#52b788` |
| Sombre | `#0f172a` | Vert forêt `#52b788` |
| Couleur forte | Vert forêt `#52b788` | `#ffffff` |

La couleur de la silhouette peut aussi être le charcoal `#1e293b` en mode clair si le fond est blanc — à tester sur le contexte réel du favicon.

---

### Variante 2 — Standard (défaut)

**Principe** : l'équilibre entre lisibilité fonctionnelle et identité de marque.

C'est la variante qui apparaît dans le header du site, dans les signatures, dans les watermarks. Elle est vue quotidiennement. Elle doit être immédiatement reconnaissable et jamais fatigante.

**Structure formelle** :
- **La botte** : contour en stroke uniquement. Pas de remplissage. L'intérieur de la botte est transparent — on voit le fond de page derrière. Épaisseur de trait : 1,5 à 2 px à 1x (ajuster selon le viewport).
- **La fougère** : remplie en vert forêt `#52b788`. Toujours. Indépendamment du thème.

**C'est la règle la plus importante de cette variante** : la fougère ne change pas de couleur. Elle est l'ancre identitaire. La botte s'adapte au contexte — la fougère ne s'adapte pas.

**Adaptation au thème** :

| Thème | Stroke de la botte | Remplissage de la fougère |
|---|---|---|
| Clair | Charcoal `#1e293b` | Vert forêt `#52b788` |
| Sombre | Blanc cassé `#f8fafc` | Vert forêt `#52b788` |

La fougère est identique dans les deux lignes. C'est voulu.

**Proportions** :  
La fougère doit dépasser du col de la botte d'environ 40 à 50 % de la hauteur totale du logo. Trop peu et le logo semble "étouffé". Trop et la botte perd son rôle d'ancrage.

**Simplification requise** :  
À cette taille, les frondaisons doivent être lisibles mais pas hyper-détaillées. Objectif : 3 à 5 frondes principales reconnaissables comme une fougère — pas comme un feuillage générique.

---

### Variante 3 — Display (grande taille)

**Principe** : le moment cinématique de la marque. C'est ici que la profondeur narrative est pleinement visible.

**Ce qui est visible** :
- La botte avec ses éléments anatomiques distincts : silhouette, semelle, col, lacets ou coutures (traités comme des paths séparés)
- La fougère avec ses frondes individuelles, un pétiole visible, une suggestion de sol ou de terre au niveau du col de la botte
- Une suggestion de texture ou de matière (légère — ne pas basculer vers l'illustration réaliste)
- L'ambre `#e8971a` peut apparaître ici comme couleur secondaire sur des éléments de la botte (oeillets de lacets, coutures, détails de semelle)

**Structure SVG** : voir section [Architecture SVG](#4-architecture-svg).

**Animation** : cette variante est la seule qui soit animée. Voir section [Chorégraphie d'animation](#5-chorégraphie-danimation).

**Usages** :
- Section hero de la page d'accueil
- Page about
- En-tête d'article de blog (variante statique du SVG)
- Impression : carte de visite, papier à lettres, affiche

---

## 3. Règles couleur

### Les deux couleurs d'identité

Le logo introduit deux nouvelles couleurs qui complètent la palette existante du design system :

**Vert forêt — `#52b788`**
- Rôle : couleur primaire d'identité du logo
- Appartient exclusivement à la fougère dans toutes les variantes Standard et Display
- C'est la seule couleur qui ne change jamais, quelle que soit le thème
- Ne pas la confondre avec `--accent-primary: #3dba7c` (vert forêt vivace) ou `--accent-secondary: #40916c` (vert forêt profond) du design system — c'est une troisième nuance délibérément différente

**Ambre miel — `#e8971a`**
- Rôle : couleur secondaire, détails et accents de la botte dans la variante Display
- Identique à `--accent-tertiary: #e8971a` du design system — c'est la même valeur, utilisée ici pour les détails de la botte
- Utilisée avec parcimonie — elle ne doit pas rivaliser avec le vert forêt

### Ce qui change et ce qui ne change pas

| Élément | Mode clair | Mode sombre | Règle |
|---|---|---|---|
| Fougère (remplissage) | `#52b788` | `#52b788` | **Invariant absolu** |
| Botte (stroke — Standard) | `#1e293b` | `#f8fafc` | Adaptatif |
| Détails botte (Display) | `#e8971a` | `#e8971a` | Invariant |
| Contenu de la botte (fond) | Transparent | Transparent | Toujours transparent |
| Silhouette Mark | `#52b788` | `#52b788` | Invariant |

### Valeurs CSS custom properties à ajouter

Ces propriétés s'ajoutent à `variables.css` sans modifier les existantes :

```css
:root {
  /* Logo identity colors — never change with theme */
  --logo-fern-green: #52b788;
  --logo-fern-green-rgb: 82, 183, 136;
  --logo-amber: #e8971a;
  --logo-amber-rgb: 232, 151, 26;
  
  /* Logo boot stroke — adapts to theme */
  --logo-boot-stroke: #1e293b;
  --logo-boot-stroke-width: 1.5px;
}

[data-theme="dark"] {
  --logo-boot-stroke: #f8fafc;
}
```

### Hiérarchie d'usage de la couleur verte

Trois verts coexistent dans le design system, chacun avec un rôle distinct. Ne pas les interchanger :

| Couleur | Hex | Rôle |
|---|---|---|
| `--accent-primary` | `#3dba7c` (light) / `#52c98a` (dark) | Accentuation UI, boutons, liens |
| `--accent-secondary` | `#40916c` (identique clair/sombre) | Hover states, fond secondaire |
| `--logo-fern-green` | `#52b788` | La fougère du logo uniquement |

Le vert forêt du logo est délibérément entre les deux verts du design system. Il ne se confond pas avec l'UI mais reste dans le registre végétal.

---

## 4. Architecture SVG

### Principe fondamental

"Fragmentation" ne signifie pas déconstruction visuelle. Cela signifie : chaque élément anatomique est son propre `<path>` SVG. La fragmentation est structurelle, invisible à l'œil. Elle permet l'animation couche par couche.

### Structure de calques pour la variante Display

```
<svg id="logo-display">
  
  <!-- Groupe 1 : Botte -->
  <g id="boot">
    <path id="boot-silhouette" />     <!-- contour extérieur de la botte -->
    <path id="boot-sole" />           <!-- semelle + bord inférieur -->
    <path id="boot-collar" />         <!-- col/tige de la botte -->
    <path id="boot-detail-laces" />   <!-- lacets ou coutures (optionnel) -->
    <path id="boot-detail-eyelets" /> <!-- oeillets (optionnel, ambre) -->
  </g>
  
  <!-- Groupe 2 : Sol / Transition -->
  <g id="ground">
    <path id="soil-line" />           <!-- ligne de terre, transition botte/fougère -->
  </g>
  
  <!-- Groupe 3 : Fougère -->
  <g id="fern">
    <path id="fern-stem" />           <!-- pétiole / rachis principal -->
    <path id="frond-1" />             <!-- fronde centrale, la plus haute -->
    <path id="frond-2-left" />        <!-- fronde gauche niveau 1 -->
    <path id="frond-2-right" />       <!-- fronde droite niveau 1 -->
    <path id="frond-3-left" />        <!-- fronde gauche niveau 2 -->
    <path id="frond-3-right" />       <!-- fronde droite niveau 2 -->
    <!-- Ajouter frond-4-* si la forme le permet à grande taille -->
  </g>
  
</svg>
```

### Règles de nommage des paths

- IDs en kebab-case, préfixés par le groupe parent
- Chaque `<path>` anatomiquement distinct = un `<path>` SVG distinct
- Pas de `<group>` inutiles qui compliquent les sélecteurs CSS
- Les paths d'animation (fougère) doivent avoir une `stroke-dasharray` calculable : pas de formes fermées pour les frondes — des courbes ouvertes qui se "dessinent"

### Optimisation et nettoyage SVG

Avant l'intégration, appliquer ces règles :

1. **Supprimer tous les attributs d'éditeur** : `inkscape:*`, `sodipodi:*`, `id` générés automatiquement sur les groupes sans animation
2. **Aplatir les transforms** : pas de `transform="translate()"` ou `transform="matrix()"` imbriqués — toutes les coordonnées doivent être absolues dans le viewport final
3. **Simplifier les paths** : réduire le nombre de points de Bézier au strict minimum. Un contour de botte simple n'a pas besoin de 200 points.
4. **Viewbox normalisée** : `viewBox="0 0 100 100"` ou `viewBox="0 0 200 200"` selon les proportions finales. Éviter les viewboxes issues de l'export direct d'Illustrator (ex: `0 0 1247.38 983.14`).
5. **Pas de `<defs>` inutiles** : uniquement si des dégradés ou des clipPaths sont vraiment nécessaires

### SVG pour la variante Standard

Structure plus simple :

```
<svg id="logo-standard">
  <g id="boot">
    <path id="boot-outline" />    <!-- stroke only, pas de fill -->
  </g>
  <g id="fern">
    <path id="fern-body" />       <!-- forme unifiée de la fougère, fill uniquement -->
  </g>
</svg>
```

La variante Standard peut être construite comme une simplification de la variante Display — mêmes proportions, mêmes angles, mais moins de paths internes.

### SVG pour la variante Mark

Un seul path possible :

```
<svg id="logo-mark">
  <path id="mark-silhouette" />   <!-- union de la botte et de la fougère -->
</svg>
```

Ou deux paths si la lisibilité à 16 px nécessite de distinguer visuellement la botte et la fougère par leurs couleurs respectives.

---

## 5. Chorégraphie d'animation

### Contexte d'usage

L'animation s'applique uniquement à la variante Display, et uniquement dans des contextes où elle est justifiée : chargement d'une page, section hero, page about. Elle ne doit jamais jouer en boucle — une seule lecture à l'entrée, puis le logo reste statique.

Respecter `prefers-reduced-motion` : si l'utilisateur a activé cette préférence système, le logo s'affiche directement dans son état final sans animation.

### Principe technique : stroke-dashoffset

Chaque fronde de la fougère est un `<path>` ouvert (courbe, pas shape fermée). On utilise `stroke-dasharray` égal à la longueur totale du path, puis on anime `stroke-dashoffset` de cette valeur vers 0. Effet : le trait se dessine progressivement.

Pour les éléments filled (remplissage), l'animation est une transition d'opacité (`opacity: 0 → 1`) ou un clip-path animé.

### Séquence d'animation

La narration de l'animation suit l'ordre de croissance logique : d'abord le contenant, puis ce qui pousse dedans.

| Étape | Élément | Type d'animation | Durée | Délai depuis le début |
|---|---|---|---|---|
| 1 | `#boot-silhouette` | stroke-dashoffset draw | 0.5 s | 0 s |
| 2 | `#boot-sole` | stroke-dashoffset draw | 0.25 s | 0.4 s |
| 3 | `#boot-collar` | stroke-dashoffset draw | 0.2 s | 0.6 s |
| 4 | `#boot-detail-*` | opacity fade-in | 0.2 s | 0.8 s |
| 5 | `#soil-line` | stroke-dashoffset draw | 0.15 s | 0.9 s |
| 6 | `#fern-stem` | stroke-dashoffset draw | 0.3 s | 1.0 s |
| 7 | `#frond-1` | stroke-dashoffset draw | 0.25 s | 1.2 s |
| 8 | `#frond-2-left`, `#frond-2-right` | stroke-dashoffset draw (simultané) | 0.25 s | 1.4 s |
| 9 | `#frond-3-left`, `#frond-3-right` | stroke-dashoffset draw (simultané) | 0.25 s | 1.6 s |
| 10 | Remplissage de `#fern` | opacity fade-in | 0.4 s | 1.7 s |

**Durée totale** : environ 2,1 secondes. Ne pas dépasser 2,5 s — au-delà, l'animation devient un obstacle à l'expérience.

### Fonctions d'easing recommandées

| Élément | Easing | Justification |
|---|---|---|
| Contour de la botte | `cubic-bezier(0.4, 0, 0.2, 1)` (ease-in-out standard) | Construction mécanique, précise |
| Tige de la fougère | `cubic-bezier(0.0, 0, 0.2, 1)` (ease-out) | Poussée depuis le sol, accélération puis ralentissement |
| Frondes | `cubic-bezier(0.34, 1.56, 0.64, 1)` (spring léger) | Déroulement organique avec légère élasticité finale |
| Remplissage final | `ease-in-out` | Apparition douce, pas de pop |

### Calcul de stroke-dasharray

La longueur d'un path SVG se calcule en JavaScript :

```javascript
const path = document.getElementById('frond-1');
const length = path.getTotalLength();
path.style.strokeDasharray = length;
path.style.strokeDashoffset = length;
```

Cette valeur change si le SVG est redimensionné via `transform: scale()` mais pas via les attributs `width`/`height`. Utiliser des `@keyframes` avec des valeurs fixes calculées une fois, ou recalculer en JS au mount.

### CSS d'animation (structure)

```css
/* Désactivation globale si prefers-reduced-motion */
@media (prefers-reduced-motion: reduce) {
  #logo-display * {
    animation: none !important;
    stroke-dashoffset: 0 !important;
    opacity: 1 !important;
  }
}

/* Exemple : animation d'une fronde */
#frond-1 {
  stroke-dasharray: var(--frond-1-length, 150);
  stroke-dashoffset: var(--frond-1-length, 150);
  animation: draw-frond 0.25s cubic-bezier(0.34, 1.56, 0.64, 1) 1.2s forwards;
}

@keyframes draw-frond {
  to {
    stroke-dashoffset: 0;
  }
}

/* Remplissage de la fougère */
#fern {
  opacity: 0;
  animation: fern-fill 0.4s ease-in-out 1.7s forwards;
}

@keyframes fern-fill {
  to {
    opacity: 1;
  }
}
```

---

## 6. Principes de construction formelle

### Objectif visuel général

**Pas de l'illustration réaliste. Pas du pictogramme générique.**

Le point d'arrivée est entre les deux : une forme géométriquement simplifiée qui préserve la précision botanique de la fougère et la reconnaissance immédiate d'une botte. On parle d'icône de qualité, pas de logo plat sans personnalité.

Référence mentale : la qualité de trait d'un bon jeu d'icônes système (SF Symbols, Phosphor Icons) — mais avec une personnalité visible.

### La botte

**Profil à utiliser** : vue de côté, pied droit, regardant vers la droite. La tige de la botte pointe vers le haut-gauche (position naturelle d'une botte posée sur le sol).

**Simplification requise** :
- Semelle : une forme plate avec un léger profil de talon. Pas de motif de semelle, pas de laçage de semelle.
- Tige : une forme simple avec une légère courbure au niveau de la cheville. Le col de la botte est l'ouverture où sort la fougère.
- Lacets : soit omis dans toutes les variantes, soit traités comme 2 lignes courtes croisées dans la variante Display uniquement. Ne pas dessiner 6 rangées de laçage.
- Forme globale : elle doit être reconnaissable comme "botte de randonnée" (tige haute, semelle épaisse) et non comme une botte de pluie (trop fine, trop lisse) ou une sneaker.

**Test de la botte à 16 px** :  
Si la silhouette de la botte seule n'est pas reconnaissable à 16 px, elle est trop complexe. Simplifier.

### La fougère

**Espèce de référence** : *Dryopteris filix-mas* (fougère mâle) — même référence que pour le projet typographique Fougère. Cohérence d'univers.

**Ce qui définit une fougère vs une feuille générique** :
- Les frondes s'attachent de chaque côté d'un rachis central visible
- Les pinnules (sous-divisions des frondes) sont de taille décroissante vers l'apex
- La forme d'ensemble est lancéolée (plus large au milieu, effilée en haut et en bas)
- Les frondes ont une légère courbure vers l'extérieur et vers le bas pour les plus basses

**Abstraction à chaque variante** :

| Variante | Niveau de détail de la fougère |
|---|---|
| Mark | Silhouette lancéolée avec 2-3 "dents" pour suggérer les frondes |
| Standard | 3 à 5 frondes reconnaissables, rachis visible, pas de pinnules |
| Display | Frondes avec pinnules visibles, rachis détaillé, texture légère |

**Ce qu'il ne faut jamais faire avec la fougère** :
- La dessiner comme une feuille de chêne ou de lierre
- Lui donner une symétrie bilatérale parfaite (la vraie fougère est légèrement asymétrique)
- La raccourcir pour qu'elle "tienne" dans le cadre — la fougère doit sortir de la botte avec assurance

### La relation botte/fougère

La fougère ne "sort de" la botte comme d'un vase — elle en émerge avec énergie. La distinction est dans la direction et la posture : une fougère dans un vase tombe mollement, une fougère qui pousse monte et s'ouvre.

**Composition** :
- La botte occupe environ 60 % de la hauteur totale du logo
- La fougère occupe environ 60 % (avec chevauchement au niveau du col)
- Le centre de masse visuel est au col de la botte — c'est le point de tension
- Le logo ne doit pas avoir de fond : il s'intègre dans n'importe quel fond de page

**Proportions à tester absolument** :
- Ratio hauteur/largeur global : environ 1:0.7 (plus haut que large) — ne pas créer un logo carré ou horizontal
- À 16 px, le ratio doit être lisible sans zoom

---

## 7. Intégration au système CSS

### Variables CSS à ajouter à `variables.css`

Placer ces déclarations à la fin du bloc `:root` existant, dans une section clairement séparée :

```css
/* ===== LOGO IDENTITY TOKENS ===== */
/* Ces valeurs ne changent JAMAIS avec le thème */

/* Fern color — invariant across all themes */
--logo-fern-green: #52b788;
--logo-fern-green-rgb: 82, 183, 136;

/* Boot detail accent */
--logo-amber: #e8971a;
--logo-amber-rgb: 232, 151, 26;

/* Boot stroke — adapts to theme */
--logo-boot-stroke: #1e293b;
--logo-boot-stroke-width: 1.5px;

/* Animation timing */
--logo-animation-total: 2.1s;
--logo-animation-easing-organic: cubic-bezier(0.34, 1.56, 0.64, 1);
--logo-animation-easing-mechanical: cubic-bezier(0.4, 0, 0.2, 1);
```

Dans le bloc `[data-theme="dark"]` :

```css
/* Only the boot stroke changes */
--logo-boot-stroke: #f8fafc;
/* --logo-fern-green stays the same — do NOT override */
```

### Classe CSS pour le conteneur du logo

```css
.logo {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

.logo-mark {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
}

.logo-standard {
  width: 48px;
  height: auto;
}

.logo-display {
  width: 100%;
  max-width: 300px;
  height: auto;
}
```

### Application des tokens dans le SVG inline

Pour les SVGs inline dans les templates Leptos :

```rust
// Dans le composant header par exemple
view! {
    <svg class="logo-standard" viewBox="0 0 100 140" aria-label="Logo Kevin Bourbasquet">
        <g id="boot">
            <path id="boot-outline" 
                  stroke="var(--logo-boot-stroke)" 
                  stroke-width="var(--logo-boot-stroke-width)"
                  fill="none" />
        </g>
        <g id="fern">
            <path id="fern-body" 
                  fill="var(--logo-fern-green)" />
        </g>
    </svg>
}
```

**Important** : utiliser `currentColor` uniquement si la couleur doit suivre la couleur de texte. Pour le logo, utiliser explicitement les custom properties pour garder le contrôle total.

### Accessibilité

- Toujours inclure `aria-label` sur le `<svg>` racine
- Si le logo est un lien, mettre le `aria-label` sur le `<a>` parent et `aria-hidden="true"` sur le SVG
- Pour les animations : le `@media (prefers-reduced-motion: reduce)` est obligatoire

---

## 8. Livrables attendus

### Fichiers sources

| Fichier | Description | Outil |
|---|---|---|
| `logo-mark.svg` | Variante Mark, viewBox 100×100, path unique | Inkscape / Illustrator |
| `logo-standard.svg` | Variante Standard, viewBox 100×140, 2 paths | Inkscape / Illustrator |
| `logo-display.svg` | Variante Display, viewBox 200×280, paths complets | Inkscape / Illustrator |
| `logo-display-animated.svg` | Variante Display avec CSS d'animation inline | Export depuis le fichier display |

### Exports PNG

| Fichier | Résolution | Usage |
|---|---|---|
| `logo-mark-16.png` | 16×16 | Favicon legacy |
| `logo-mark-32.png` | 32×32 | Favicon standard |
| `logo-mark-64.png` | 64×64 | App icon / avatar |
| `logo-mark-180.png` | 180×180 | Apple touch icon |
| `logo-standard-2x.png` | 256×358 | Fallback PNG |
| `logo-display.png` | 600×840 | Impression / haute résolution |

### Favicon set

```
favicon.ico           — multi-résolution (16, 32, 48px)
favicon-16x16.png
favicon-32x32.png
apple-touch-icon.png  — 180×180
android-chrome-192.png
android-chrome-512.png
site.webmanifest      — à créer ou mettre à jour
```

### Emplacement dans le projet

```
/public/
  images/
    logo/
      logo-mark.svg
      logo-mark-16.png
      logo-mark-32.png
      logo-mark-64.png
      logo-mark-180.png
      logo-standard.svg
      logo-standard-2x.png
      logo-display.svg
      logo-display.png
  favicon.ico
  favicon-16x16.png
  favicon-32x32.png
  apple-touch-icon.png
  android-chrome-192.png
  android-chrome-512.png
  site.webmanifest
```

---

## 9. Ce qu'il ne faut pas faire

Cette section est aussi importante que les règles positives. Ces erreurs sont fréquentes dans ce type de logo.

### Erreurs conceptuelles

**Ne pas viser le réalisme**  
Un logo est un signe, pas une illustration. La tentation de "bien rendre" les détails de la botte (texture du cuir, lacets détaillés, profil de semelle) ou de la fougère (sores, nervures fines) produit un résultat qui ne passe pas à petite taille et qui n'est pas mémorable. Chaque détail ajouté doit avoir une raison fonctionnelle ou narrative — pas une raison esthétique.

**Ne pas créer de symétrie artificielle**  
Une botte est asymétrique. Une fougère est quasi-symétrique mais pas parfaitement. La tentation de symétriser le logo pour qu'il "fasse plus propre" lui fait perdre sa vitalité. L'asymétrie contrôlée est une signature.

**Ne pas "équilibrer" la couleur**  
La botte en outline et la fougère en fill vert : cela semble déséquilibré à première vue. C'est voulu. Le déséquilibre est la règle. Ne pas ajouter de fill à la botte pour "équilibrer" les deux éléments — cela dilue le principe identitaire.

**Ne pas multiplier les couleurs**  
Le vert forêt pour la fougère, l'ambre pour les détails de botte dans la variante Display, et le stroke de la botte. C'est le maximum. Tout ajout de couleur supplémentaire (dégradé sur la fougère, ombre portée colorée, fond coloré dans la variante Standard) est interdit.

### Erreurs d'exécution SVG

**Ne pas utiliser de filtres SVG pour les ombres ou les flous**  
Les filtres SVG (`<feDropShadow>`, `<feGaussianBlur>`) sont lourds, mal rendus à petite taille, et inanimables proprement. Toute profondeur visuelle doit venir de la forme et de la couleur, pas des effets.

**Ne pas aplatir les paths d'animation**  
Si les paths de la variante Display sont aplatis en un seul path dans l'export, l'animation devient impossible. Vérifier que l'export SVG préserve la structure de calques.

**Ne pas utiliser de coordonnées absolues issues d'un export direct**  
Un SVG exporté d'Illustrator avec `viewBox="0 0 1247.38 983.14"` est inutilisable en production sans nettoyage. Normaliser systématiquement.

**Ne pas oublier les transforms résiduels**  
Inkscape en particulier génère des `transform="translate(-x,-y)"` sur les groupes. Ces transforms doivent être aplatis avant livraison.

### Erreurs d'intégration

**Ne pas mettre une couleur hardcodée dans le SVG pour la botte**  
Le stroke de la botte doit utiliser `var(--logo-boot-stroke)` pour changer avec le thème. Un `stroke="#1e293b"` hardcodé ne fonctionnera pas en mode sombre.

**Ne pas mettre `var(--logo-fern-green)` dans le SVG pour la fougère**  
Paradoxe apparent : la fougère ne change pas de couleur, donc elle n'a *pas besoin* de la custom property. Mais utiliser `fill="var(--logo-fern-green)"` est une bonne pratique car elle documente l'intention. L'erreur à éviter est d'utiliser `fill="currentColor"` ou `fill="var(--accent-primary)"` — ce serait la mauvaise variable et la couleur changerait.

**Ne pas oublier `prefers-reduced-motion`**  
L'animation sans cette media query est une faute d'accessibilité. Il ne s'agit pas d'une option.

**Ne pas jouer l'animation en boucle**  
L'animation est un accueil, pas un spectacle permanent. `animation-iteration-count: 1` avec `animation-fill-mode: forwards`. C'est tout.

### Ce que ce logo n'est pas censé faire

- Apparaître partout sur le site à chaque scroll (ce serait un easter egg, pas une identité)
- Servir de bouton ou d'élément interactif autre que le retour à l'accueil
- Être décliné en version "couleur fond vert" avec la fougère blanche — la botte en stroke disparaîtrait sur fond vert
- Être accompagné d'un slogan ou d'une tagline dans sa version Mark ou Standard

---

*Document de travail — Kevin Bourbasquet — v0.1 — 2026-05-18*  
*Ce document évolue avec la création du logo. Mettre à jour les specs SVG et les valeurs de timing une fois les premiers tracés disponibles.*
