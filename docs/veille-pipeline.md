# Pipeline Veille Technologique — Documentation complète

> Dernière mise à jour : 2026-06-01

---

## Vue d'ensemble

Pipeline **entièrement automatisé** en 3 étapes :

1. **Fetch** (quotidien, sans IA) — collecte ~10 articles/domaine depuis 35+ sources
2. **Score** (quotidien, IA Haiku) — compétition inter-domaines → top 5 articles du jour
3. **Synthèse** (hebdomadaire, IA Sonnet) — article journalistique avec ton éditorial de Kevin

---

## Architecture globale

```
╔══════════════════════════════════════════════════════════════════════════╗
║                        SOURCES EXTERNES (6 domaines)                    ║
║                                                                          ║
║  dev_stack      ai_emerging    security       health_science             ║
║  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐           ║
║  │Rust Blog │  │ArXiv AI  │  │CISA      │  │ArXiv CS.HC   │           ║
║  │This Week │  │ArXiv PL  │  │NIST      │  │ArXiv q-bio   │           ║
║  │Go Blog   │  │Papers    │  │ENISA     │  │eLife         │           ║
║  │Mozilla   │  │With Code │  │OpenSSF   │  │PLOS ONE      │           ║
║  │WebKit    │  │IEEE      │  │Krebs     │  └──────────────┘           ║
║  │GitHub Eng│  │ACM       │  │Schneier  │                              ║
║  │LWN       │  └──────────┘  │PortSwigger                             ║
║  │Lobste.rs │                │SANS ISC  │                              ║
║  │LinuxFr   │                └──────────┘                              ║
║  └──────────┘                                                           ║
║                                                                          ║
║  business_market              architecture                               ║
║  ┌────────────────┐           ┌─────────────────────┐                  ║
║  │HackerNews≥150  │           │Dezeen Sustainable   │                  ║
║  │The Register    │           │Low-tech Magazine    │                  ║
║  │InfoQ           │           │ArchDaily            │                  ║
║  │CNCF Blog       │           │TreeHugger           │                  ║
║  │W3C / IETF      │           │Resilient Design     │                  ║
║  └────────────────┘           └─────────────────────┘                  ║
╚══════════════════════════════════════════════════════════════════════════╝
                                    │
                                    ▼
╔══════════════════════════════════════════════════════════════════════════╗
║          ÉTAPE 1 — FETCH (quotidien 06:00 UTC, ZÉRO IA)                ║
║          fetch_news.py                                                   ║
║                                                                          ║
║   1. Fetch RSS/Atom via feedparser (max 10 items/source)                ║
║   2. Fetch HackerNews top 80 → score ≥ 150 → max 12 stories            ║
║   3. Filtre bruit (regex NOISE_PATTERNS)                                ║
║   4. Déduplication (hash MD5 12 chars sur l'URL)                        ║
║   5. Classification par mots-clés (catégories sémantiques)              ║
║   6. Ajout champs domain + status="raw" + fetched_at                    ║
║   7. Merge avec news.json existant (rétention par status)               ║
║   8. Auto-commit "chore: update tech news feed"                         ║
╚══════════════════════════════════════════════════════════════════════════╝
                                    │
                              ~50-80 articles "raw"
                                    │
                                    ▼
╔══════════════════════════════════════════════════════════════════════════╗
║          ÉTAPE 2 — SCORE (quotidien 06:30 UTC, IA Haiku)               ║
║          score_articles.py                                               ║
║                                                                          ║
║   Pour chaque domaine (6 domaines) :                                    ║
║     - Prend les articles "raw" du jour (max 15 candidats)               ║
║     - Appel Claude Haiku : scoring 0–10 selon critères domaine          ║
║     - Sélectionne top 3 par domaine                                     ║
║                                                                          ║
║   Compétition globale (18 candidats max) :                              ║
║     - Tri par score décroissant                                         ║
║     - Contrainte : max 1 article architecture                           ║
║     - Sélection finale : top 5                                          ║
║                                                                          ║
║   Résultat :                                                            ║
║     - 5 articles → status="selected"                                    ║
║     - Reste → status="archived"                                         ║
║   Auto-commit "chore: score articles — YYYY-MM-DD"                     ║
╚══════════════════════════════════════════════════════════════════════════╝
                                    │
                              5 articles/jour
                              → 35 articles/semaine (max)
                                    │
                                    ▼
╔══════════════════════════════════════════════════════════════════════════╗
║          ÉTAPE 3 — SYNTHÈSE (lundi 07:00 UTC, IA Sonnet)               ║
║          synthesize_news.py                                              ║
║                                                                          ║
║   1. Charge articles status="selected" des 7 derniers jours             ║
║   2. Appel Claude Sonnet (claude-sonnet-4-6)                            ║
║      - Ton éditorial de Kevin (direct, technique, souveraineté)        ║
║      - Article journalistique bilingue FR + EN (600-900 mots)          ║
║      - Si sécurité → section "Actions concrètes" obligatoire           ║
║      - Si architecture → extraction nom architecte + projet             ║
║   3. Si architecture : fetch Wikipedia → bio + image                    ║
║   4. Création carte synthesis avec architecture_visual                  ║
║   5. Tag des articles sources avec synthesis_id                         ║
║   Auto-commit "chore: weekly tech synthesis — YYYY-WNN"                ║
╚══════════════════════════════════════════════════════════════════════════╝
                                    │
                                    ▼
╔══════════════════════════════════════════════════════════════════════════╗
║               DÉPLOIEMENT (push sur main)                               ║
║               deploy.yml — trunk build --release → GitHub Pages         ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

## Planification (GitHub Actions cron)

```
Heure UTC  │ Lun  Mar  Mer  Jeu  Ven  Sam  Dim
───────────┼────────────────────────────────────
06:00      │  F    F    F    F    F    F    F    ← fetch-news.yml  (sans IA)
06:30      │  S    S    S    S    S    S    S    ← score-news.yml  (Haiku)
07:00      │  SY   ·    ·    ·    ·    ·    ·   ← synthesize-news.yml (Sonnet, lundi)

F  = fetch_news.py    S = score_articles.py    SY = synthesize_news.py
```

---

## Critères de scoring par domaine

### `dev_stack`
Stack relevance Rust/WASM/web/open source **40%** · Impact développeur **40%** · Signal vs hype **20%**

### `ai_emerging`
Applicabilité pratique réelle **40%** · Rigueur scientifique / vrai breakthrough **40%** · Nouveauté vs hype **20%**
Malus fort pour les pièces hype LLM sans substance technique.

### `security`
Sévérité/urgence pour dev web **50%** · Actionnabilité (patch/mitigation concret) **30%** · Ampleur de l'impact **20%**
Bonus : articles avec CVE ID + plage de versions affectées.

### `health_science`
Avancée pour la santé humaine **40%** · Niveau d'innovation scientifique **40%** · Accessibilité des résultats **20%**
Préférence pour publications peer-reviewed vs communiqués de presse.

### `business_market`
Signal long-terme (pas bruit court-terme) **40%** · Impact écosystème dev / open source **30%** · Insight contrarian **30%**
Malus pour annonces de funding sans substance technique.

### `architecture`
Éco-responsabilité / adaptation climatique **40%** · Applicabilité auto-construction matériaux locaux **40%** · Innovation technique/matériaux **20%**
Focus : passivhaus, matériaux naturels, gestion eau, autonomie énergétique, intégration alimentaire.

---

## Contrainte globale top 5

Sur les 18 candidats (top 3 × 6 domaines), le top 5 global respecte :
- **Max 1 article architecture** — intérêt personnel hors contexte principal
- Tri par score Claude Haiku décroissant

---

## Synthèse hebdomadaire — spécificités

### Ton éditorial (Kevin)
Direct, techniquement précis, légèrement opinionné. Zéro bullshit corporate. Valeurs : souveraineté numérique, sécurité web as first-class, responsabilité écologique. Stack : Rust, WebAssembly, open web.

### Traitement sécurité
Si au moins un article `security` ou catégorie `urgent` dans la semaine → section `## Actions concrètes` **obligatoire** dans les deux langues. Contenu : 3-5 étapes précises et immédiatement réalisables par le lecteur (numéros de version, commandes, config snippets).

### Traitement architecture
Si un article `architecture` dans la semaine :
1. Claude extrait le nom de l'architecte/studio + requête Wikipedia
2. `fetch_wikipedia_info()` → Wikipedia REST API `/page/summary/{query}`
3. Récupère : bio (600 chars max) + thumbnail URL + lien Wikipedia
4. Stocké dans `architecture_visual` de la carte synthesis

---

## Structure de données : `public/news.json`

```
public/news.json
│
├── generated_at     ISO timestamp
├── period           YYYY-MM-DD
├── count            Nb total items
├── synthesis        null (plus utilisé en quotidien — champ gardé pour compat)
│
└── items[]
    ├── [type="article"]
    │   ├── id              hash MD5 12 chars
    │   ├── type            "article"
    │   ├── title
    │   ├── url
    │   ├── source          nom du flux
    │   ├── domain          dev_stack|ai_emerging|security|health_science|business_market|architecture
    │   ├── categories[]    [urgent|good_news|future_watch|stack_alt|general]
    │   ├── published_at    ISO timestamp
    │   ├── fetched_at      ISO timestamp (ajouté par fetch_news.py)
    │   ├── lang            en|fr|de|ja
    │   ├── status          raw|selected|archived
    │   ├── score           float 0–10 (set par score_articles.py)
    │   ├── score_reason    string courte (set par score_articles.py)
    │   └── synthesis_id    "synthesis_YYYY_WNN" (après synthèse hebdo)
    │
    └── [type="synthesis"]
        ├── id              "synthesis_YYYY_WNN"
        ├── type            "synthesis"
        ├── title_fr
        ├── title_en
        ├── content_fr      article Markdown 600-900 mots
        ├── content_en      article Markdown 600-900 mots
        ├── security_actions[]  actions concrètes si sécurité
        ├── architecture_visual  {name, bio, image_url, wikipedia_url} | null
        ├── period_start    YYYY-MM-DD
        ├── period_end      YYYY-MM-DD
        ├── published_at
        ├── source_count
        └── sources[]
            ├── id, title, url, source, domain, lang, published_at, score
```

**Rétention par status :**
| Status | Rétention |
|--------|-----------|
| `synthesis` | Indéfinie |
| `selected` | 21 jours |
| `archived` | 4 jours |
| `raw` | 2 jours |
| legacy (pas de status) | 14 jours |

---

## score_articles.py — usage CLI

```bash
# Scorer les articles d'aujourd'hui (mode normal)
python scripts/score_articles.py

# Scorer une date spécifique (rattrapage)
python scripts/score_articles.py --date 2026-06-01

# Scorer toute une semaine ISO (nettoyage / retraitement)
python scripts/score_articles.py --week 2026-W22

# Voir le résultat sans modifier news.json
python scripts/score_articles.py --dry-run
python scripts/score_articles.py --date 2026-06-01 --dry-run
```

Via GitHub Actions (workflow_dispatch) : `score-news.yml` accepte les inputs `date`, `week`, `dry_run`.

---

## Sources par domaine

### dev_stack (10 sources)
Rust Blog · This Week in Rust · Go Blog · Mozilla Hacks · WebKit Blog · GitHub Engineering · Linux Foundation · LWN.net · Lobste.rs · LinuxFr.org

### ai_emerging (5 sources)
ArXiv CS.AI (max 10) · ArXiv CS.PL (max 8) · Papers With Code · IEEE Spectrum · ACM Tech News

### security (8 sources, `no_filter: True`)
CISA Advisories · NIST CSRC · ENISA · OpenSSF Blog · Krebs on Security · Schneier on Security · PortSwigger Blog · SANS ISC (max 5)

### health_science (4 sources, `no_filter: True`)
ArXiv CS.HC (max 8) · ArXiv q-bio (max 8) · eLife Sciences (max 8) · PLOS ONE (max 8)

### business_market (5 sources + HackerNews)
The Register · InfoQ · CNCF Blog · W3C Blog · IETF Blog
HackerNews : top 80 → score ≥ 150 → max 12 (domaine `business_market`)

### architecture (5 sources)
Dezeen Sustainable · Low-tech Magazine · ArchDaily (max 8) · TreeHugger (max 8) · Resilient Design Institute

---

## Filtrage du bruit

Sources sans `no_filter: True` passent par regex NOISE_PATTERNS :
`"day N of"` `"top N"` `"my first"` `"getting started"` `"for beginners"` `"step-by-step"` `"complete guide"` `"my journey"` `"#showdev"` `"how i"` `"my experience"` et autres.

HackerNews : filtre bruit + seuil score ≥ 150 (vs 100 précédemment).

---

## Développement local

```bash
# Fetch (sans IA)
python scripts/fetch_news.py

# Score du jour
export ANTHROPIC_API_KEY=sk-ant-...
python scripts/score_articles.py --dry-run   # voir sans modifier
python scripts/score_articles.py             # appliquer

# Synthèse hebdomadaire
python scripts/synthesize_news.py

# Vérifier news.json
python -c "import json; d=json.load(open('public/news.json')); \
  sel=[i for i in d['items'] if i.get('status')=='selected']; \
  print(len(sel), 'selected,', d['count'], 'total')"
```

---

## Fichiers clés

```
.github/workflows/
├── fetch-news.yml        Cron 06:00 UTC — sans IA
├── score-news.yml        Cron 06:30 UTC — Haiku (+ workflow_dispatch avec params)
├── synthesize-news.yml   Cron lundi 07:00 UTC — Sonnet
├── deploy.yml            Build & déploiement
└── quality.yml           Validation post-déploiement

scripts/
├── fetch_news.py         Agrégateur RSS + HN + classification
├── score_articles.py     Compétition inter-domaines (nouveau)
├── synthesize_news.py    Synthèse éditoriale hebdomadaire
└── requirements.txt      feedparser, requests, anthropic

public/
└── news.json             Fichier central (~20-100 KB, ~100-200 items)

src/components/
└── veille.rs             Composant Leptos /veille
```
