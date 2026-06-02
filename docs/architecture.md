# Architecture frontend — Rust/Leptos/WASM

> Dernière mise à jour : 2026-06-01

---

## Vue d'ensemble

Application SPA compilée en WebAssembly via [Trunk](https://trunkrs.dev/). Mode CSR (Client-Side Rendering) — tout tourne dans le navigateur après le premier chargement du bundle WASM.

```
src/                         Rust source
├── lib.rs                   Point d'entrée WASM (mount App)
├── app.rs                   Routeur + fournisseurs de contexte
├── components/              Composants UI
│   ├── mod.rs               Re-exports publics
│   ├── navigation.rs        Barre de navigation desktop
│   ├── mobile_nav.rs        FAB navigation mobile
│   ├── hero.rs              Section Hero
│   ├── about.rs             Section About
│   ├── skills.rs            Section Skills
│   ├── projects.rs          Section Projects
│   ├── interests.rs         Section Interests
│   ├── contact.rs           Section Contact
│   ├── footer.rs            Pied de page
│   ├── not_found.rs         Page 404
│   ├── veille.rs            Page /veille (tech watch)
│   └── blog/
│       ├── mod.rs           Route /blog + article resolver
│       ├── blog_index.rs    Liste des articles
│       └── blog_article.rs  Rendu d'un article
├── services/                Logique partagée (contexte global)
│   ├── mod.rs
│   ├── i18n.rs              Service internationalisation (FR/EN)
│   ├── theme.rs             Service thème (clair/sombre)
│   ├── storage.rs           Persistance LocalStorage
│   └── blog.rs              Chargement et indexation des articles
└── data/
    ├── cv.rs                Données CV (struct statiques)
    ├── articles/            Articles de blog (Markdown inline)
    └── translations/        Chaînes i18n
        ├── fr.rs
        └── en.rs
```

---

## Routage

Géré par `leptos_router`. 3 routes statiques :

```
/           → HomePage      (Navigation + Hero + About + Skills + Projects + Interests + Contact + Footer)
/blog       → BlogPage      (résout /blog et /blog/:slug)
/veille     → VeillePage    (+ sous-route implicite /veille/synthesis/:id)
```

Fallback : `NotFound404`.

```rust
// app.rs — structure simplifiée
provide_context(I18nService::new());
provide_context(ThemeService::new());
provide_context(BlogService::new());

Router {
  Routes {
    Route "" → HomePage
    Route "/blog" → BlogPage
    Route "/veille" → VeillePage
    fallback → NotFound404
  }
  MobileFloatingNav  // hors des Routes, présent sur toutes les pages
}
```

---

## Services (contexte global)

Fournis via `provide_context` dans `App`, consommés avec `use_context` dans n'importe quel composant descendant.

### I18nService (`services/i18n.rs`)

```
État : Signal<Lang>   (Lang::Fr | Lang::En)
API  : lang()         → Lang actuel
       set_lang(l)    → bascule la langue
       t(key)         → String traduite dans la langue active
```

Les traductions sont des `&'static str` dans `data/translations/fr.rs` et `en.rs`. Pas de fichier JSON externe — tout est embarqué à la compilation.

### ThemeService (`services/theme.rs`)

```
État : Signal<Theme>   (Theme::Light | Theme::Dark)
API  : theme()         → Theme actuel
       toggle()        → bascule
       apply()         → écrit data-theme="dark|light" sur <html>
Persistance : lit/écrit via StorageService (clé "theme")
```

Au montage, lit la préférence stockée, sinon `prefers-color-scheme` OS.

### StorageService (`services/storage.rs`)

Wrapper `LocalStorage` via `gloo-storage`. Sérialise/désérialise en JSON. Clés utilisées :

| Clé | Type | Utilisé par |
|-----|------|-------------|
| `"theme"` | `String` (`"dark"` / `"light"`) | ThemeService |
| `"lang"` | `String` (`"fr"` / `"en"`) | I18nService |

### BlogService (`services/blog.rs`)

```
État : &'static [Article]   (statique, compilé dedans)
API  : articles()    → liste complète
       get(slug)     → Option<&Article>
```

Les articles sont des structs Rust avec le Markdown inline — aucun fetch réseau.

---

## Composants clés

### Navigation (`navigation.rs`)

- Fixed top, transparente au repos → glassmorphism au scroll
- Consomme `I18nService` (toggle langue) + `ThemeService` (toggle thème)
- Détecte la section active via `IntersectionObserver` (JS interop `web-sys`)

### MobileFloatingNav (`mobile_nav.rs`)

- FAB fixe bas-droite, 5 items en arc
- Drag-to-navigate : `pointermove` events, zones magnétiques, labels flottants
- `prefers-reduced-motion` : animations désactivées, transitions `none`

### VeillePage (`veille.rs`)

Voir [veille-pipeline.md](veille-pipeline.md) pour la documentation complète.

Résumé :
- `create_resource` → `fetch("public/news.json")` au montage
- Désérialisation `serde_json` → `NewsData { items: Vec<NewsItem> }`
- Filtres réactifs : catégorie, langue, texte
- Rendu conditionnel : `WeeklySynthesisCard` (type synthesis) vs `NewsItemCard` (type article)
- Markdown → HTML via `pulldown-cmark`

### BlogPage (`blog/`)

- `/blog` : liste des articles (`BlogIndexPage`)
- `/blog/:slug` : résout le slug dans `BlogService`, rend `BlogArticlePage`
- Articles en Markdown inline compilé dans le binaire

---

## Données statiques embarquées

```
src/data/cv.rs          Struct CV (expériences, formations, compétences)
src/data/articles/      Un fichier .rs par article, contenant le Markdown
src/data/translations/  Chaînes i18n FR/EN
```

Tout est `&'static str` ou `const` — zéro allocation au runtime pour ces données.

---

## Gestion des assets (`public/`)

Trunk copie `public/` vers `dist/` tels quels. Le frontend accède aux assets via des URLs absolues depuis la racine :

```
/fonts/Literata[opsz,wght].woff2    → fonts.css @font-face
/css/*.css                           → chargés via index.html <link>
/news.json                           → fetch() dans veille.rs
/images/*, /icons/*                 → balises <img> ou <link rel="icon">
```

---

## Dépendances Rust (Cargo.toml)

| Crate | Rôle |
|-------|------|
| `leptos` 0.8 (CSR) | Framework UI réactif |
| `leptos_meta` 0.8 | `<head>` dynamique (title, meta) |
| `leptos_router` 0.8 | Routeur SPA |
| `serde` / `serde_json` | Désérialisation `news.json` |
| `gloo-net` | `fetch()` WASM |
| `gloo-storage` | `LocalStorage` |
| `pulldown-cmark` 0.12 | Markdown → HTML |
| `web-sys` / `js-sys` | Interop DOM/JS |
| `wasm-bindgen` | Glue WASM ↔ JS |
| `chrono` | Parsing et formatage de dates |

---

## Build (`Trunk.toml`)

```toml
[build]
target = "index.html"   # Template SPA
dist   = "dist"         # Répertoire de sortie

[watch]
watch = ["src", "public"]  # Hot reload en dev
```

Commandes :

```bash
trunk serve          # Dev server http://127.0.0.1:9999 (hot reload)
trunk build --release  # Build de production → dist/
```

Le build de production minifie le WASM et applique `wasm-opt` automatiquement.

---

## Compilation WASM

```
rustc → wasm32-unknown-unknown target
     → wasm-bindgen (génère le JS glue)
     → wasm-opt (optimisation taille — mode release)
     → dist/
         index.html
         *.wasm
         *.js (loader)
         [public/* copié]
```

Taille typique du bundle WASM : ~2–4 MB non compressé, ~600–900 KB gzip.

---

## Ajout d'un composant

1. Créer `src/components/mon_composant.rs`
2. Exposer via `src/components/mod.rs` (`pub mod mon_composant; pub use mon_composant::*;`)
3. Utiliser dans `app.rs` ou un autre composant parent
4. Consommer les services via `use_context::<MonService>()`

Aucun registre global — Leptos découple les composants via le contexte d'arbre.
