# Frontend architecture — Rust/Leptos/WASM

> Last updated: 2026-06-18

---

## Overview

Single-page app compiled to WebAssembly via [Trunk](https://trunkrs.dev/). CSR (Client-Side Rendering) — everything runs in the browser once the WASM bundle loads.

```
src/                         Rust source
├── lib.rs                   WASM entry point (mounts App)
├── app.rs                   Router + context providers
├── components/              UI components
│   ├── mod.rs               Public re-exports
│   ├── navigation.rs        Desktop navigation bar
│   ├── mobile_nav.rs        Mobile navigation FAB
│   ├── hero.rs              Hero section
│   ├── about.rs             About section
│   ├── skills.rs            Skills section
│   ├── projects.rs          Projects section
│   ├── interests.rs         Interests section
│   ├── contact.rs           Contact section
│   ├── footer.rs            Footer
│   ├── not_found.rs         404 page
│   ├── veille.rs            /veille page (tech watch)
│   └── blog/
│       ├── mod.rs           /blog route + article resolver
│       ├── blog_index.rs    Article list
│       └── blog_article.rs  Single-article rendering
├── services/                Shared logic (global context)
│   ├── mod.rs
│   ├── i18n.rs              Internationalization service (FR/EN)
│   ├── theme.rs             Theme service (light/dark)
│   ├── storage.rs           LocalStorage persistence
│   └── blog.rs              Article loading and indexing
└── data/
    ├── cv.rs                CV data (static structs)
    ├── articles/            Blog articles (inline Markdown)
    └── translations/        i18n strings
        ├── fr.rs
        └── en.rs
```

---

## Routing

Handled by `leptos_router`. Three static routes:

```
/           → HomePage      (Navigation + Hero + About + Skills + Projects + Interests + Contact + Footer)
/blog       → BlogPage      (resolves /blog and /blog/:slug)
/veille     → VeillePage    (+ implicit sub-route /veille?synthesis=:id)
```

Fallback: `NotFound404`.

```rust
// app.rs — simplified structure
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
  MobileFloatingNav  // outside Routes, present on every page
}
```

---

## Services (global context)

Provided via `provide_context` in `App`, consumed with `use_context` in any descendant component.

### I18nService (`services/i18n.rs`)

```
State : Signal<Lang>   (Lang::Fr | Lang::En)
API   : lang()         → current Lang
        set_lang(l)    → switch language
        t(key)         → translated String in the active language
```

Translations are `&'static str` in `data/translations/fr.rs` and `en.rs`. No external JSON file — everything is embedded at compile time.

### ThemeService (`services/theme.rs`)

```
State : Signal<Theme>   (Theme::Light | Theme::Dark)
API   : theme()         → current Theme
        toggle()        → switch
        apply()         → writes data-theme="dark|light" on <html>
Persistence : reads/writes via StorageService (key "theme")
```

On mount, reads the stored preference, otherwise the OS `prefers-color-scheme`.

### StorageService (`services/storage.rs`)

`LocalStorage` wrapper via `gloo-storage`. Serializes/deserializes to JSON. Keys used:

| Key | Type | Used by |
|-----|------|---------|
| `"theme"` | `String` (`"dark"` / `"light"`) | ThemeService |
| `"lang"` | `String` (`"fr"` / `"en"`) | I18nService |

### BlogService (`services/blog.rs`)

```
State : &'static [Article]   (static, compiled in)
API   : articles()    → full list
        get(slug)     → Option<&Article>
```

Articles are Rust structs with inline Markdown — no network fetch.

---

## Key components

### Navigation (`navigation.rs`)

- Fixed top, transparent at rest → glassmorphism on scroll
- Consumes `I18nService` (language toggle) + `ThemeService` (theme toggle)
- Detects the active section via `IntersectionObserver` (JS interop through `web-sys`)

### MobileFloatingNav (`mobile_nav.rs`)

- Fixed bottom-right FAB, 5 items in an arc
- Drag-to-navigate: `pointermove` events, magnetic zones, floating labels
- `prefers-reduced-motion`: animations disabled, transitions `none`

### VeillePage (`veille.rs`)

See [veille-pipeline.md](veille-pipeline.md) for the full documentation.

Summary:
- `create_resource` → `fetch("public/news.json")` on mount
- `serde_json` deserialization → `NewsData { items: Vec<NewsItem> }`
- Reactive filters: category (incl. dedicated `ai` tab), language, text
- Conditional rendering: `WeeklySynthesisCard` (synthesis type) vs article card
- Markdown → HTML via `pulldown-cmark`

### BlogPage (`blog/`)

- `/blog`: article list (`BlogIndexPage`)
- `/blog/:slug`: resolves the slug in `BlogService`, renders `BlogArticlePage`
- Articles are inline Markdown compiled into the binary

---

## Embedded static data

```
src/data/cv.rs          CV struct (experience, education, skills)
src/data/articles/      One .rs file per article, holding the Markdown
src/data/translations/  FR/EN i18n strings
```

Everything is `&'static str` or `const` — zero runtime allocation for this data.

---

## Asset handling (`public/`)

Trunk copies `public/` into `dist/` as-is. The frontend accesses assets via absolute URLs from the root:

```
/public/fonts/Literata[opsz,wght].woff2  → fonts.css @font-face
/public/css/*.css                         → loaded via index.html <link>
/public/news.json                         → fetch() in veille.rs
/public/images/*, /public/icons/*        → <img> tags or <link rel="icon">
```

---

## Rust dependencies (Cargo.toml)

| Crate | Role |
|-------|------|
| `leptos` 0.8 (CSR) | Reactive UI framework |
| `leptos_meta` 0.8 | Dynamic `<head>` (title, meta) |
| `leptos_router` 0.8 | SPA router |
| `serde` / `serde_json` | `news.json` deserialization |
| `gloo-net` | WASM `fetch()` |
| `gloo-storage` | `LocalStorage` |
| `pulldown-cmark` 0.12 | Markdown → HTML |
| `web-sys` / `js-sys` | DOM/JS interop |
| `wasm-bindgen` | WASM ↔ JS glue |
| `chrono` | Date parsing and formatting |

---

## Build (`Trunk.toml`)

```toml
[build]
target = "index.html"   # SPA template
dist   = "dist"         # Output directory

[watch]
watch = ["src", "public"]  # Hot reload in dev
```

Commands:

```bash
trunk serve            # Dev server http://127.0.0.1:9999 (hot reload)
trunk build --release  # Production build → dist/
```

The production build minifies the WASM and applies `wasm-opt` automatically.

---

## WASM compilation

```
rustc → wasm32-unknown-unknown target
     → wasm-bindgen (generates the JS glue)
     → wasm-opt (size optimization — release mode)
     → dist/
         index.html
         *.wasm
         *.js (loader)
         [public/* copied]
```

Typical WASM bundle size: ~2–4 MB uncompressed, ~600–900 KB gzipped.

---

## Adding a component

1. Create `src/components/my_component.rs`
2. Expose it via `src/components/mod.rs` (`pub mod my_component; pub use my_component::*;`)
3. Use it in `app.rs` or another parent component
4. Consume services via `use_context::<MyService>()`

No global registry — Leptos decouples components through the tree context.
