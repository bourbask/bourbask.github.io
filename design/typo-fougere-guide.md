# Fougère — Guide de conception typographique

**Projet** : Police de caractères custom basée sur la morphologie des fougères  
**Designer** : Kevin Bourbasquet  
**Version du document** : 0.1 (document de travail)  
**Date** : 2026-05-18  
**Statut** : En développement — phase de définition

---

## Sommaire

1. [Vision et narration](#1-vision-et-narration)
2. [Le système à deux sources](#2-le-système-à-deux-sources)
3. [Table de correspondance anatomique](#3-table-de-correspondance-anatomique)
4. [Processus de création étape par étape](#4-processus-de-création-étape-par-étape)
5. [Contraintes et règles de cohérence](#5-contraintes-et-règles-de-cohérence)
6. [Guide d'utilisation](#6-guide-dutilisation)
7. [Trajectoire d'évolution](#7-trajectoire-dévolution)
8. [Références et ressources](#8-références-et-ressources)

---

## 1. Vision et narration

### Pourquoi la fougère

La fougère est l'une des plantes les plus anciennes encore vivantes sur Terre — elle précède les fleurs, les graines, les oiseaux. Sa forme est mathématique sans être mécanique : la pinnule reproduit l'architecture du frondon, le frondon reproduit celle de la touffe. C'est une fractalité organique, lisible à toutes les échelles.

Ce projet s'inscrit dans une tradition de polices extraites du monde réel plutôt que construites par convention. Comme *Amazonia* a tiré ses formes des tracés de rivières vus depuis l'espace, Fougère tire les siennes de la botanique macro-photographiée — une discipline d'observation avant d'être une discipline de dessin.

### Ce que cette police raconte

Fougère est une police d'identité. Elle n'est pas destinée à faciliter la lecture de corps de texte : elle est destinée à inscrire une présence. Elle dit : *quelqu'un regarde le monde naturel et en tire une langue visuelle cohérente*.

Le choix de la fougère est aussi une prise de position sur les valeurs du designer : ancrage dans le vivant, attention à la croissance organique plutôt qu'à l'expansion mécanique, refus de l'arbitraire décoratif. Chaque trait a une raison qui vient de la plante.

### Audience et registre

Cette police s'adresse d'abord à elle-même : elle est l'identité visuelle de Kevin Bourbasquet. En tant que telle, elle doit fonctionner dans des contextes de présentation professionnelle (portfolio, articles, signature), sans jamais verser dans l'ornement gratuit ou le "design nature" générique.

Le registre visé est : **botanique scientifique rencontre typographie contemporaine**. Pas l'aquarelle décorative, pas le minimalisme stérile — quelque chose entre les illustrations de planches botaniques du 19e siècle et un design web qui respire.

---

## 2. Le système à deux sources

### Principe fondamental

La cohérence de Fougère repose sur une règle absolue : **chaque casse provient d'une seule source visuelle et d'un seul point de vue**. Ce principe élimine l'arbitraire et garantit que l'alphabet entier forme un système reconnaissable, et non une collection de variations stylisées.

### Source 1 — Le frondon mature (majuscules)

**Point de vue** : vue de dessus, axe vertical, fronde étalée à plat  
**Espèce de référence** : *Dryopteris filix-mas* (fougère mâle commune) — symétrie bilatérale claire, pinnules bien séparées, rachis visible  
**Métaphore typographique** : stabilité, ouverture, structure

Le frondon vu de dessus est symétrique, dense au centre, aéré aux extrémités. Cette géométrie se traduit en majuscules ouvertes, dont les empattements et les terminaisons évoquent des pinnules déployées. Les lettres majuscules sont des formes **accomplies** : elles ont atteint leur état final, elles occupent l'espace avec assurance.

Caractéristiques attendues des majuscules :
- Axe de symétrie central fort (surtout sur A, H, M, W)
- Terminaisons en crossettes organiques (courbes douces, pas rectangulaires)
- Contre-formes internes aérées, proportionnelles aux espaces entre pinnules
- Empattement modéré — semi-empattée, pas slab

### Source 2 — La crosse / crozier (minuscules)

**Point de vue** : vue de côté, plan vertical, crosse en cours de déroulement  
**Stades documentés** : 5 stades minimum — de la crosse fermée (sphérique) à la fronde à moitié déployée (spirale visible mais encore enroulée)  
**Métaphore typographique** : dynamisme, direction, croissance

La crosse est un spirale vivante. Elle indique une direction — vers le haut, vers la lumière — et un mouvement arrêté en plein déroulement. Cette énergie se traduit en minuscules dont les courbes sont *tendues*, dont les terminaisons *suggèrent une continuation*. Les lettres minuscules sont des formes **en devenir** : elles poussent.

Caractéristiques attendues des minuscules :
- Courbes tendues plutôt que rondes (éviter le cercle parfait)
- Terminaisons en spirale atténuée — la queue d'un `a`, le haut d'un `f` se referment légèrement vers l'intérieur
- Direction de lecture incorporée dans la logique de croissance : base à gauche, pointe à droite
- Ascendants et descendants traités comme des jets en élongation, pas comme des prolongements mécaniques

### Pourquoi deux sources et non une

Une seule source (le frondon seul, par exemple) produirait soit des majuscules et minuscules trop semblables (perte de hiérarchie), soit une cohérence interne cassée par les contraintes différentes de chaque casse. Le mapping casse/état de croissance est une *solution structurelle*, pas un choix esthétique.

---

## 3. Table de correspondance anatomique

Cette table est la règle de traduction entre botanique et typographie. Toute décision de dessin doit pouvoir être justifiée en référence à cette table.

| Élément botanique | Description botanique | Élément typographique | Notes de traduction |
|---|---|---|---|
| **Rachis** | Axe central du frondon, le plus épais | Fût principal (*main stroke*) | Trait le plus épais ; épaisseur de 3–5× le trait fin |
| **Pétiole** | Base du rachis, attache à la souche | Empattement de pied | Zone de contact avec la ligne de base, légèrement évasée |
| **Pinnule** | Foliole latérale, attachée au rachis | Trait secondaire, serif, crossette | S'attache au fût comme la pinnule au rachis ; s'affine à l'extrémité |
| **Nervure de pinnule** | Veine centrale de chaque foliole | Délié (trait fin) | Contraste marqué avec le rachis |
| **Apex du frondon** | Extrémité effilée du frondon | Terminal de stroke | Terminaison par amincissement progressif, pas par coupe |
| **Sore** | Groupe de sporanges sur la face inférieure | Accent, point, signe diacritique | Forme ovale légèrement asymétrique — ne pas utiliser de cercle parfait |
| **Crosse / crozier** | Fronde en cours de déroulement, enroulée | Courbe tendue des minuscules | La tension interne de la spirale = tension du tracé de Bézier |
| **Indusie** | Membrane recouvrant le sore | Optionnel : couverture de certains accents composés | Usage rare, si cohérence demande une forme couverte |
| **Rachis secondaire (costa)** | Axe de chaque pinnule | Barre transversale (H, A, e, f…) | Épaisseur intermédiaire entre rachis et nervure |

### Règle d'épaisseur

La logique d'épaisseur suit le sens de croissance de la plante : **épaisse à la base, fine à la pointe**. En typographie, cela se traduit par :

- Transition d'épaisseur dans les courbes : épaisse là où la courbe est convexe vers la tige (équivalent de la base de la pinnule), fine là où elle s'en éloigne
- Le contraste global épaisseur/finesse se situe dans un ratio approximatif de **4:1** (à affiner sur les épreuves) — assez marqué pour être lisible en grand format, pas assez pour se briser en petit format
- Aucun trait ne doit être parfaitement uniforme sur toute sa longueur

---

## 4. Processus de création étape par étape

### Étape 0 — Préparation du corpus photographique

**Objectif** : constituer une banque de sources visuelles suffisante pour extraire des formes, pas pour les inventer.

**À photographier** :
- 10 à 15 photos de frondes de *Dryopteris filix-mas* en vue de dessus stricte (téléphone ou appareil en position zénithale, fronde posée à plat sur fond neutre blanc ou noir mat)
- 10 photos de crosses à 5 stades distincts : 1 = totalement enroulée, 2 = légèrement ouverte (10°), 3 = quart déployée, 4 = moitié déployée, 5 = trois-quarts déployée
- Éclairage : lumière directionnelle rasante (lumière naturelle de côté), qui fait ressortir les nervures et la texture
- Format : RAW ou JPEG haute résolution (au moins 3000 px sur le plus grand côté)

**Ce qu'on cherche dans les photos** :
- Clarté des contours du rachis
- Visibilité des points d'attache rachis/pinnule
- Lisibilité des nervures de pinnule
- Variété de poses de crosses (pas deux fois le même stade)

**Ce qu'on ne cherche pas** :
- La perfection de la photo en tant que photo
- La couleur (tout sera traité en noir et blanc pour l'extraction)

**Outils** : appareil photo ou smartphone, tripode ou bras rigide pour la vue de dessus, boîte à lumière ou lumière naturelle diffuse.

---

### Étape 1 — Extraction des contours (Inkscape)

**Objectif** : extraire des courbes vectorielles fidèles à l'anatomie, sans interprétation prématurée.

**Méthode** :

1. Importer la photo en fond de calque (calque verrouillé, opacité 50 %)
2. Créer un calque "contours anatomiques" par-dessus
3. Tracer avec l'outil Plume les contours du rachis principal en premier — **ne jamais commencer par les détails**
4. Tracer les silhouettes de 3 à 5 pinnules représentatives (pas toutes — on cherche le patron, pas la plante entière)
5. Tracer 2 à 3 crosses complètes, en suivant la spirale de l'intérieur vers l'extérieur

**Règles de tracé** :

- Utiliser le minimum de points d'ancrage qui permettent de rendre la courbe fidèlement (principe des courbes de Bézier propres)
- Ne pas "lisser" les irrégularités naturelles à ce stade — elles seront jugées plus tard, intentionnellement gardées ou éliminées
- Annoter chaque tracé avec l'élément anatomique correspondant (calques nommés)

**Livrables de l'étape** :
- Fichier `.svg` avec calques nommés par élément anatomique
- Aucune lettre à ce stade — seulement des formes botaniques

---

### Étape 2 — Identification des formes génératives

**Objectif** : extraire des formes-types (pas des lettres) qui seront les briques du système.

À partir des contours tracés à l'étape 1, identifier et isoler :

1. **Le module de courbure du rachis** : la courbe caractéristique du rachis quand il est légèrement incliné — ce sera la base des fûts courbes (C, G, J, S, U pour les majuscules ; b, d, p, q pour les minuscules)
2. **Le module d'attache pinnule/rachis** : la jonction entre un trait vertical et un trait latéral — base de tous les empattements et des crossettes
3. **La spirale de crosse** : la courbe de Bézier caractéristique de la crosse — base des courbes fermées des minuscules (a, e, g, o, s)
4. **Le terminal d'apex** : la terminaison effilée de l'extrémité du frondon — base de tous les terminals de traits

Ces 4 modules sont le vocabulaire minimal du système. Toute lettre doit être composable à partir de combinaisons de ces modules.

**Point de décision** : si les 4 modules semblent incompatibles entre eux (proportions, angles, épaisseurs incohérents), revenir à l'étape 0 et revoir la sélection photographique avant de continuer.

---

### Étape 3 — Alphabet pilote (8 lettres)

**Objectif** : vérifier que le système tient sur un ensemble minimal représentatif avant d'investir dans l'alphabet complet.

**Lettres pilotes** :

| Lettre | Raison du choix |
|---|---|
| **A** | Majuscule avec symétrie axiale + barre transversale — teste rachis et costa |
| **E** | Majuscule avec 3 barres horizontales — teste les terminaisons multiples |
| **O** | Majuscule ovale fermée — teste la courbe de rachis en boucle |
| **H** | Majuscule avec deux fûts et barre — teste les proportions et la barre centrale |
| **n** | Minuscule archétypale — teste l'arche et les attaches |
| **a** | Minuscule à double compartiment — teste la spirale fermée |
| **e** | Minuscule avec œil — teste la contre-forme et le terminal ouvert |
| **o** | Minuscule ovale — teste la courbe de crosse en boucle |

**Protocole de validation des pilotes** :

Pour chaque lettre, répondre à ces questions avant de passer à la suivante :
- Peut-on identifier l'origine botanique de chaque trait de cette lettre ?
- Les épaisseurs respectent-elles la table de correspondance anatomique ?
- La lettre est-elle lisible à 200 pt ? à 60 pt ? à 36 pt ?
- Posée à côté des 7 autres pilotes, forme-t-elle une famille reconnaissable ?

Si une lettre échoue à ces questions, retravailler uniquement cette lettre, pas le système entier.

**Outil principal** : FontForge (libre, multiplateforme — gestion des métriques, kerning, épreuves). FontForge utilise le format `.sfd` natif ou le format ouvert `.ufo` (recommandé pour le versionnement git — fichiers texte, diffables). Le scripting Python intégré permet d'automatiser les opérations répétitives (espacements, corrections de courbes). Les tracés d'exploration peuvent rester dans Inkscape jusqu'à cette étape.

---

### Étape 4 — Test d'identité (logo signature)

**Objectif** : valider le système dans son usage primaire avant de l'étendre.

Composer "Kevin Bourbasquet" en version display (grande taille, 100–200 pt) et en version signature (30–50 pt) avec les 8 lettres pilotes complétées des lettres manquantes (v, i, k, u, r, b, g, s, t — à tracer rapidement en cohérence avec le système pilote).

**Ce qu'on teste** :
- L'espacement (tracking et kerning naturels)
- L'équilibre entre majuscule K et les minuscules
- La lisibilité du nom complet comme identité, pas comme test technique
- La tenue à différentes tailles et sur fond sombre (vert profond type Floema)

**Point de décision** : si le rendu du nom complet ne semble pas "juste" à ce stade, identifier si le problème vient d'une lettre isolée, d'un problème de spacing systématique, ou d'une incohérence de registre entre majuscules et minuscules. Ne pas continuer vers l'alphabet complet si le problème est systémique.

---

### Étape 5 — Complétion de l'alphabet

**Ordre recommandé** :

1. **Minuscules complètes** (26 lettres) — la voix principale de la police pour les textes d'identité
2. **Majuscules complètes** (26 lettres) — la voix des titres et logos
3. **Chiffres** (0–9) — traités selon la logique des majuscules (chiffres elzéviriens ou alignés, à décider)
4. **Ponctuation de base** : . , : ; ! ? ' " ( ) — et le tiret long —
5. **Caractères accentués** indispensables pour le français : é è ê ë à â ù û ü ô î ï ç

**Pour chaque lettre nouvelle** :
- Identifier le ou les modules anatomiques utilisés
- Vérifier la cohérence avec les lettres déjà dessinées les plus proches (exemple : `b` après `p` et `d`)
- Tester dans un mot réel avant de valider

---

### Étape 6 — Métriques et espacement

**À définir une fois l'alphabet pilote validé** :

- **UPM** (Units Per Em) : 1000 (standard)
- **Cap height** : à mesurer sur H, I — cible approximative 700
- **x-height** : à mesurer sur x, o — cible approximative 500 (ratio cap/x d'environ 0,71)
- **Ascendants** : à définir sur b, d, h, k, l — dépasser légèrement la cap height (cible 750–780)
- **Descendants** : à définir sur g, j, p, q, y — cible −220 à −250
- **Espacement latéral** (*sidebearings*) : commencer avec n et o comme références, étendre aux autres classes

**Kerning** : traiter uniquement les paires problématiques (AV, AT, Av, WA, etc.) — ne pas surkerntner, la police est display et une légère irrégularité "vit" mieux qu'une uniformité artificielle.

---

## 5. Contraintes et règles de cohérence

Cette section liste ce qu'il ne faut **jamais** faire pour préserver la cohérence du système. Ces règles valent autant que les règles positives.

### Règles absolues (ne jamais enfreindre)

**R1 — Un seul point de vue par casse.**  
Les majuscules viennent toujours de la vue de dessus du frondon. Les minuscules viennent toujours de la vue de côté de la crosse. Mélanger les deux sources sur une même casse brise immédiatement la cohérence.

**R2 — Aucune terminaison plate ou coupe droite.**  
Toute terminaison de trait doit s'amincir progressivement (comme l'apex d'une pinnule) ou se recourber légèrement (comme la pointe d'une crosse). Une coupe droite n'existe pas dans la plante source et ne doit pas exister dans la police.

**R3 — Le rachis est toujours le trait le plus épais.**  
Le fût principal d'une lettre est toujours plus épais que ses barres, crossettes et déliés. Inverser ce rapport rompt la logique anatomique et déstabilise la hiérarchie visuelle.

**R4 — Pas de cercle parfait.**  
Les courbes fermées (O, o, 0, sores/points) ne sont jamais des cercles géométriquement parfaits. Elles doivent présenter une légère tension asymétrique, comme une fronde qui pousse légèrement d'un côté. Tolérance : déformation d'au moins 3–5% par rapport au cercle parfait.

**R5 — La direction de croissance est invariable.**  
Dans toute la police, la logique base → pointe correspond à gauche → droite dans le sens de lecture. Un trait qui "pousserait" de droite à gauche serait une contradiction du système.

### Règles importantes (enfreindre seulement avec justification documentée)

**R6 — Le module d'attache pinnule/rachis est cohérent.**  
Toutes les jonctions trait/crossette ou fût/empattement utilisent la même courbure caractéristique. Si une lettre exige une jonction différente pour être lisible, le documenter dans le fichier de travail avec la justification.

**R7 — Le ratio épaisseur/finesse reste dans la plage 3:1 – 5:1.**  
En dessous de 3:1, la police perd son caractère organique. Au-dessus de 5:1, les déliés ne tiennent plus en usage digital et à petite taille. Si un usage physique (gravure, impression sérigraphique) justifie un ratio plus extrême, créer une variante dédiée sans modifier les fichiers maîtres.

**R8 — L'espacement doit être cohérent avec l'espace négatif des pinnules.**  
L'espace entre les lettres doit rappeler l'espace entre les pinnules sur un frondon : régulier, aéré, jamais étouffant. Ne pas serrer l'espacement pour des raisons d'économie de place — la police est display, elle ne doit pas être serrée.

### Signaux d'alarme

Si l'un de ces symptômes apparaît, stopper et reévaluer la direction avant de continuer :

- Une lettre semble appartenir à une autre famille que les autres (style différent, poids différent, terminaisons différentes)
- Les majuscules et les minuscules semblent être de polices différentes (pas juste différentes de registre — réellement incohérentes)
- La logique botanique n'est plus reconnaissable dans les tracés (on a dérivé vers de la "stylisation nature" générique)
- Un test de lisibilité à 36 pt échoue sur plus de 3 lettres pilotes

---

## 6. Guide d'utilisation

### Usages autorisés

Fougère est une police **display uniquement**. Elle est conçue pour :

| Contexte | Taille minimale recommandée | Notes |
|---|---|---|
| Titres H1 | 48 pt / 64 px | Idéalement au-dessus de 60 px |
| Titres H2 | 36 pt / 48 px | Vérifier la lisibilité des lettres à crossettes fines |
| Logotype / signature | Pas de minimum — à évaluer visuellement | Tester à toutes les tailles prévues dès le début |
| Mise en avant d'un mot-clé dans un article | 36 pt minimum | Mot isolé ou expression courte (3–4 mots max) |
| En-tête imprimé (carte de visite, papier à lettres) | 14 pt minimum en impression haute résolution | Uniquement si la résolution d'impression le permet |

### Usages interdits

- **Corps de texte** (toute taille inférieure à 24 pt) — les terminaisons organiques et les déliés fins ne tiennent pas à petite taille
- **Texte long** (plus de 2 lignes consécutives en Fougère) — la forte personnalité de la police fatigue la lecture sur la durée
- **Sous-titres de navigation** (menu, fil d'Ariane, labels d'interface) — trop de caractère pour des éléments fonctionnels
- **Texte sur fond complexe ou photographique** sans contraste suffisant — les déliés disparaissent
- **Usage en gras** ou **en italic** (sauf si une variante est explicitement créée) — ne pas simuler le gras par épaississement logiciel, qui casserait les proportions anatomiques

### Associations typographiques recommandées

Fougère étant très marquée, la police qui lui est associée pour le corps de texte doit être neutre et fonctionnelle. Elle ne doit pas "répondre" stylistiquement à Fougère — elle doit la laisser exister seule sur ses titres.

Pistes à tester :
- **Source Serif** (Google Fonts, open source) — sérif humaniste discret, bon ancrage dans la page
- **Literata** (Google Fonts) — sérif optimisé pour la lecture longue, bonne neutralité
- **iA Writer Quattro** — fonctionnel et sobre, bon contrepoint à l'organique
- Éviter toute police à "effet naturel", calligraphique ou handwritten — il ne faut pas doubler le registre botanique sur le texte courant

### Palettes de couleur d'usage

Fougère a été conceptualisée en référence à l'esthétique de Floema (vert sombre organique). Les combinaisons qui préservent l'intention :

| Fond | Couleur Fougère | Contexte |
|---|---|---|
| `#1a2e1a` (vert nuit) | `#e8e0c8` (ivoire chaud) | Usage principal, fort, solennel |
| `#f5f0e8` (parchemin) | `#2d4a2d` (vert foncé) | Impression, version lumineuse |
| `#0f1f0f` (noir vert) | `#c8d4a0` (vert pâle) | Écran sombre, technique |
| `#ffffff` (blanc pur) | `#1a3a1a` (vert profond) | À éviter si possible — trop peu d'ancrage organique |

Ne jamais utiliser Fougère en rouge, orange ou jaune vif — ces couleurs sont en contradiction avec le registre végétal sobre voulu.

---

## 7. Trajectoire d'évolution

### Phase 0 (actuelle) — Document de définition

Ce document. Aucune lettre n'existe encore.

### Phase 1 — Corpus photographique + extraction des modules

Durée estimée : 2 à 4 semaines (dépend du temps de séance photo et de la discipline d'extraction)  
Livrable : fichier `.svg` avec les 4 modules anatomiques vectorisés et annotés

### Phase 2 — Alphabet pilote (8 lettres)

Durée estimée : 4 à 6 semaines  
Livrable : fichier `.ufo` avec les 8 pilotes, métriques provisoires, premier test d'identité

### Phase 3 — Alphabet complet + métriques

Durée estimée : 8 à 16 semaines (selon intensité du travail)  
Livrable : fichier `.ufo` complet, export `.otf` bêta

### Phase 4 — Test d'usage réel et itération

Implémenter Fougère sur le site bourbask.github.io en remplacement de la police de titre actuelle. Observer le rendu sur :
- Différents systèmes d'exploitation (antialiasing variable entre macOS, Windows, Linux)
- Différentes tailles d'écran (mobile : grande taille uniquement)
- Différentes résolutions (retina vs standard)

Corriger les problèmes identifiés, stabiliser les métriques, versionner.

### Évolution future — variantes possibles

Ces évolutions ne sont à envisager qu'une fois la version display complète et stable.

**Variante Display Light** : ratio d'épaisseur réduit (2:1), pour des titres plus aériens sur fond sombre. Conserver les mêmes formes, uniquement modifier les épaisseurs.

**Variante Text** : refonte profonde pour la lisibilité à 14–18 pt. Les terminaisons organiques devront être simplifiées. C'est en réalité une police distincte partageant le même vocabulaire formel, pas une interpolation simple.

**Variante Monospaced** : pour l'usage dans des contextes de code ou de données — intéressant en cohérence avec le profil de développeur du designer. Contrainte forte : chaque glyphe occupe la même largeur, ce qui impose des compromis importants sur les formes asymétriques.

**Usage physique** : gravure laser, tampons, sérigraphie. Ces usages imposent des contraintes de finesse minimale différentes du digital. Prévoir une variante "physique" avec déliés élargis et terminaisons légèrement renforcées.

### Ce qui ne changera jamais

Quelles que soient les variantes produites, ces éléments sont l'ADN permanent de Fougère :
- La distinction de source entre majuscules (frondon) et minuscules (crosse)
- La logique d'épaisseur base → pointe
- L'absence de terminaisons droites
- Le module d'attache pinnule/rachis comme signature des jonctions

---

## 8. Références et ressources

### Références typographiques directes

- **Amazonia** — projet de référence pour la méthode d'extraction de formes naturelles. Lire le processus de création pour la discipline d'observation.
- **Get Plants** (sur fontsuse.com) — registre illustration botanique. À observer pour les terminaisons et la qualité de ligne, pas pour les formes letterforms.
- **Floema** (sur awwwards.com) — référence d'usage web. À observer pour la palette, le rythme typographique, et comment une police organique peut fonctionner dans un design web sérieux.

### Références botaniques

- *Dryopteris filix-mas* : espèce de référence principale — commune, robuste, symétrie bilatérale claire
- *Matteuccia struthiopteris* (fougère autruche) : frondes plus grandes, utile pour les photos en vue de dessus si l'espèce principale est difficile à photographier à plat
- Planches botaniques du *Köhler's Medizinal-Pflanzen* (domaine public) — référence pour le style illustration, disponibles sur Wikimedia Commons
- *The Fern Guide*, Edgar T. Wherry (domaine public, Archive.org) — description anatomique précise des espèces nord-américaines, utile pour le vocabulaire exact

### Outils

- **FontForge** (multiplateforme, libre) — outil principal pour le dessin de police. Gestion des métriques, du kerning, des classes et de l'export OTF/TTF. Utiliser le format `.ufo` pour le stockage (versionnable en git) plutôt que le `.sfd` natif. Note pratique : la gestion des nœuds de Bézier est plus manuelle et moins assistée — préférer des courbes propres importées depuis Inkscape plutôt que de tout tracer directement dans FontForge.
- **Inkscape** (libre, multiplateforme) — pour l'extraction des contours botaniques (étapes 1 et 2) et les explorations de formes. Export SVG propre, compatible avec l'import FontForge.
- **FontTools** (Python, CLI) — pour les opérations de post-traitement sur les fichiers .otf/.ttf (sous-ensemble, métadonnées, vérification)
- **Wakamai Fondue** (web) — pour inspecter un fichier de police et vérifier que les métriques sont correctement exportées

### Lectures sur la conception typographique

- *Designing Type*, Karen Cheng — déconstruction anatomique de familles typographiques classiques, utile pour comprendre les conventions que Fougère choisit de respecter ou de briser
- *The Anatomy of Type*, Stephen Coles — vocabulaire de référence illustré
- *How to Create Typefaces: from Sketch to Screen*, Cristóbal Henestrosa et al. — guide pratique complet du processus de création

---

*Document de travail — Kevin Bourbasquet — v0.1 — 2026-05-18*  
*Ce document évolue avec le projet. Versionner les modifications significatives.*
