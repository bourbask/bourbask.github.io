use crate::components::footer::Footer;
use crate::components::navigation::Navigation;
use crate::services::I18nService;
use gloo_net::http::Request;
use js_sys::Date;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_query_map;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SynthesisSource {
    id: String,
    title: String,
    url: String,
    source: String,
    lang: String,
    published_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewsItem {
    id: String,
    #[serde(rename = "type", default = "default_item_type")]
    item_type: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    categories: Vec<String>,
    published_at: String,
    #[serde(default)]
    lang: String,
    #[serde(default)]
    synthesis_id: Option<String>,
    #[serde(default)]
    title_fr: Option<String>,
    #[serde(default)]
    title_en: Option<String>,
    #[serde(default)]
    period_start: Option<String>,
    #[serde(default)]
    period_end: Option<String>,
    #[serde(default)]
    content_fr: Option<String>,
    #[serde(default)]
    content_en: Option<String>,
    #[serde(default)]
    sources: Option<Vec<SynthesisSource>>,
    #[serde(default)]
    source_count: Option<usize>,
}

fn default_item_type() -> String {
    "article".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewsData {
    generated_at: String,
    count: usize,
    items: Vec<NewsItem>,
}

#[derive(Debug, Clone)]
enum NewsState {
    Loading,
    Loaded(NewsData),
    Error,
}

fn format_date_locale(iso: &str, lang: &str) -> String {
    let date_part = &iso[..iso.len().min(10)];
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return date_part.to_string();
    }
    match (
        parts[0].parse::<u32>(),
        parts[1].parse::<usize>(),
        parts[2].parse::<u32>(),
    ) {
        (Ok(y), Ok(m), Ok(d)) if (1..=12).contains(&m) => {
            if lang == "fr" {
                const MONTHS_FR: [&str; 12] = [
                    "janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.",
                    "oct.", "nov.", "déc.",
                ];
                format!("{} {} {}", d, MONTHS_FR[m - 1], y)
            } else {
                const MONTHS_EN: [&str; 12] = [
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                    "Dec",
                ];
                format!("{} {}, {}", MONTHS_EN[m - 1], d, y)
            }
        }
        _ => date_part.to_string(),
    }
}

fn lang_flag(lang: &str) -> &'static str {
    match lang {
        "fr" => "🇫🇷",
        "de" => "🇩🇪",
        "ja" => "🇯🇵",
        _ => "🇬🇧",
    }
}

fn category_label(cat: &str, i18n: &I18nService) -> String {
    match cat {
        "urgent" => i18n.t("veille.filterUrgent"),
        "good_news" => i18n.t("veille.filterGoodNews"),
        "future_watch" => i18n.t("veille.filterFutureWatch"),
        "stack_alt" => i18n.t("veille.filterStackAlt"),
        _ => i18n.t("veille.filterGeneral"),
    }
}

fn synthesis_week_label(id: &str) -> &str {
    id.rfind('_').map(|i| &id[i + 1..]).unwrap_or(id)
}

fn synthesis_excerpt(content: &str, max_chars: usize) -> String {
    let text: String = content
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_chars {
        text
    } else {
        let mut s: String = chars[..max_chars].iter().collect();
        s.push('…');
        s
    }
}

fn markdown_to_html(md: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let mut output = String::new();
    html::push_html(&mut output, Parser::new_ext(md, opts));
    // External links (http/https): open in new tab
    output.replace(
        "<a href=\"http",
        "<a target=\"_blank\" rel=\"noopener noreferrer\" href=\"http",
    )
}

#[component]
fn WeeklySynthesisCard(item: NewsItem) -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    let id = item.id.clone();
    let period_start = item.period_start.clone().unwrap_or_default();
    let period_end = item.period_end.clone().unwrap_or_default();
    let sources = item.sources.clone().unwrap_or_default();
    let source_count = item.source_count.unwrap_or(sources.len());
    let title_fr = item.title_fr.clone();
    let title_en = item.title_en.clone();
    let content_fr = item.content_fr.clone().unwrap_or_default();
    let content_en = item.content_en.clone().unwrap_or_default();
    let detail_url = format!("/veille?synthesis={}", id);

    let period_start2 = period_start.clone();
    let period_end2 = period_end.clone();

    view! {
        <article class="veille-synthesis-card">
            <div class="veille-synthesis-card-header">
                <span class="veille-synthesis-badge">
                    {move || i18n.t("veille.synthesis.aiGenerated")}
                </span>
                <span class="veille-synthesis-card-period">
                    {move || {
                        let lang = i18n.current_lang_code();
                        format!(
                            "{} {} {} {}",
                            i18n.t("veille.synthesis.weekOf"),
                            format_date_locale(&period_start2, &lang),
                            i18n.t("veille.synthesis.to"),
                            format_date_locale(&period_end2, &lang),
                        )
                    }}
                </span>
            </div>

            <h2 class="veille-synthesis-card-title">
                {move || {
                    let lang = i18n.current_lang_code();
                    if lang == "fr" {
                        title_fr.clone().or_else(|| title_en.clone()).unwrap_or_default()
                    } else {
                        title_en.clone().or_else(|| title_fr.clone()).unwrap_or_default()
                    }
                }}
            </h2>

            <p class="veille-synthesis-source-count">
                {move || format!(
                    "{} {} {}",
                    i18n.t("veille.synthesis.basedOn"),
                    source_count,
                    i18n.t("veille.synthesis.articles"),
                )}
            </p>

            <p class="veille-synthesis-excerpt">
                {move || {
                    let lang = i18n.current_lang_code();
                    let content = if lang == "fr" { &content_fr } else { &content_en };
                    synthesis_excerpt(content, 250)
                }}
            </p>

            <div class="veille-synthesis-cta">
                <a href={detail_url} class="veille-synthesis-read-more">
                    {move || i18n.t("veille.synthesis.readMore")}
                </a>
            </div>
        </article>
    }
}

#[component]
fn SynthesisDetailPage(item: NewsItem, other_syntheses: Vec<NewsItem>) -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let is_sources_open = RwSignal::new(false);

    let period_start = item.period_start.clone().unwrap_or_default();
    let period_end = item.period_end.clone().unwrap_or_default();
    let sources = item.sources.clone().unwrap_or_default();
    let source_count = item.source_count.unwrap_or(sources.len());
    let title_fr = item.title_fr.clone();
    let title_en = item.title_en.clone();
    let content_fr = item.content_fr.clone().unwrap_or_default();
    let content_en = item.content_en.clone().unwrap_or_default();

    let period_start2 = period_start.clone();
    let period_end2 = period_end.clone();

    view! {
        <div class="synthesis-detail">
            <div class="synthesis-detail-nav">
                <a href="/veille" class="synthesis-back-btn">
                    {move || i18n.t("veille.synthesis.backToFeed")}
                </a>
            </div>

            <header class="synthesis-detail-header">
                <div class="synthesis-detail-meta">
                    <span class="veille-synthesis-badge">
                        {move || i18n.t("veille.synthesis.aiGenerated")}
                    </span>
                    <span class="veille-synthesis-card-period">
                        {move || {
                            let lang = i18n.current_lang_code();
                            format!(
                                "{} {} {} {}",
                                i18n.t("veille.synthesis.weekOf"),
                                format_date_locale(&period_start2, &lang),
                                i18n.t("veille.synthesis.to"),
                                format_date_locale(&period_end2, &lang),
                            )
                        }}
                    </span>
                    <span class="synthesis-detail-count">
                        {move || format!(
                            "{} {} {}",
                            i18n.t("veille.synthesis.basedOn"),
                            source_count,
                            i18n.t("veille.synthesis.articles"),
                        )}
                    </span>
                </div>

                <h1 class="synthesis-detail-title">
                    {move || {
                        let lang = i18n.current_lang_code();
                        if lang == "fr" {
                            title_fr.clone().or_else(|| title_en.clone()).unwrap_or_default()
                        } else {
                            title_en.clone().or_else(|| title_fr.clone()).unwrap_or_default()
                        }
                    }}
                </h1>
            </header>

            <div class="synthesis-detail-body">
                {move || {
                    let lang = i18n.current_lang_code();
                    let content = if lang == "fr" { content_fr.clone() } else { content_en.clone() };
                    let html = markdown_to_html(&content);
                    view! { <div class="synthesis-detail-content" inner_html={html}></div> }
                }}
            </div>

            {if !other_syntheses.is_empty() {
                let i18n2 = i18n.clone();
                view! {
                    <div class="synthesis-similar">
                        <p class="synthesis-similar-title">
                            {move || i18n2.t("veille.synthesis.similar")}
                        </p>
                        <div class="synthesis-similar-grid">
                            {other_syntheses.iter().map(|s| {
                                let week = synthesis_week_label(&s.id).to_string();
                                let t_fr = s.title_fr.clone().unwrap_or_default();
                                let t_en = s.title_en.clone().unwrap_or_default();
                                let sid = s.id.clone();
                                let i18n3 = i18n.clone();
                                view! {
                                    <a href={format!("/veille?synthesis={}", sid)}
                                       class="synthesis-similar-card">
                                        <p class="synthesis-similar-week">{week}</p>
                                        <p class="synthesis-similar-card-title">
                                            {move || {
                                                let lang = i18n3.current_lang_code();
                                                if lang == "fr" { t_fr.clone() } else { t_en.clone() }
                                            }}
                                        </p>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            {if !sources.is_empty() {
                view! {
                    <div class="synthesis-detail-sources">
                        <button
                            class="veille-synthesis-sources-toggle"
                            on:click=move |_| is_sources_open.update(|v| *v = !*v)
                        >
                            {move || if is_sources_open.get() {
                                i18n.t("veille.synthesis.hideSources")
                            } else {
                                format!("{} ({})", i18n.t("veille.synthesis.showSources"), source_count)
                            }}
                        </button>
                        {move || if is_sources_open.get() {
                            let src_clone = sources.clone();
                            view! {
                                <ul class="veille-synthesis-sources-list">
                                    {src_clone.into_iter().map(|src| {
                                        let flag = lang_flag(&src.lang);
                                        view! {
                                            <li>
                                                <span class="veille-source">{src.source.clone()}</span>
                                                <span class="veille-lang">{flag}</span>
                                                <a href={src.url.clone()} target="_blank" rel="noopener noreferrer">
                                                    {src.title.clone()}
                                                </a>
                                                <span class="veille-date">
                                                    {format_date_locale(&src.published_at, "en")}
                                                </span>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}

const STATIC_FILTERS: &[(&str, &str)] = &[
    ("all", "veille.filterAll"),
    ("urgent", "veille.filterUrgent"),
    ("good_news", "veille.filterGoodNews"),
    ("future_watch", "veille.filterFutureWatch"),
    ("stack_alt", "veille.filterStackAlt"),
];

#[component]
pub fn VeillePage() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let (news_state, set_news_state) = signal(NewsState::Loading);
    let (active_filter, set_active_filter) = signal("all".to_string());
    let query = use_query_map();
    let synthesis_param = move || query.with(|q| q.get("synthesis").unwrap_or_default());

    Effect::new(move |_| {
        if !synthesis_param().is_empty() {
            if let Some(window) = web_sys::window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }
    });

    spawn_local(async move {
        let url = format!("/public/news.json?_={}", Date::now() as u64);
        match Request::get(&url).send().await {
            Ok(resp) => match resp.json::<NewsData>().await {
                Ok(data) => set_news_state.set(NewsState::Loaded(data)),
                Err(_) => set_news_state.set(NewsState::Error),
            },
            Err(_) => set_news_state.set(NewsState::Error),
        }
    });

    view! {
        <div class="veille-page">
            <Navigation />

            <section class="veille-hero">
                <div class="veille-hero-container">
                    <div class="blog-badge">
                        <span>{move || i18n.t("navigation.veille")}</span>
                    </div>
                    <h1 class="blog-hero-title">
                        <span>{move || i18n.t("veille.title")}</span>
                        <br />
                        <span class="gradient-text">{move || i18n.t("veille.subtitle")}</span>
                    </h1>
                    {move || match news_state.get() {
                        NewsState::Loaded(ref data) => {
                            let lang = i18n.current_lang_code();
                            let date = format_date_locale(&data.generated_at, &lang);
                            view! {
                                <p class="veille-updated">
                                    {move || i18n.t("veille.updatedAt")}
                                    {" "}{date.clone()}
                                </p>
                            }.into_any()
                        }
                        _ => view! { <p></p> }.into_any(),
                    }}
                </div>
            </section>

            <section class="veille-content">
                <div class="veille-container">
                    {move || {
                        let synth_id = synthesis_param();
                        if !synth_id.is_empty() {
                            match news_state.get() {
                                NewsState::Loading => view! {
                                    <div class="veille-status">
                                        <p>{move || i18n.t("veille.loading")}</p>
                                    </div>
                                }.into_any(),
                                NewsState::Error => view! {
                                    <div class="veille-status veille-error">
                                        <p>{move || i18n.t("veille.error")}</p>
                                    </div>
                                }.into_any(),
                                NewsState::Loaded(data) => {
                                    let others: Vec<NewsItem> = data.items.iter()
                                        .filter(|i| i.item_type == "synthesis" && i.id != synth_id)
                                        .take(3)
                                        .cloned()
                                        .collect();
                                    match data.items.into_iter().find(|i| i.id == synth_id) {
                                        Some(s) => view! {
                                            <SynthesisDetailPage item=s other_syntheses=others />
                                        }.into_any(),
                                        None => view! {
                                            <div class="veille-status">
                                                <a href="/veille" class="synthesis-back-btn">
                                                    {move || i18n.t("veille.synthesis.backToFeed")}
                                                </a>
                                            </div>
                                        }.into_any(),
                                    }
                                }
                            }
                        } else {
                            view! {
                                <div class="veille-filters">
                                    {STATIC_FILTERS.iter().map(|(key, label_key)| {
                                        let key_str = key.to_string();
                                        let key_str2 = key_str.clone();
                                        view! {
                                            <button
                                                class=move || {
                                                    if active_filter.get() == key_str {
                                                        "veille-filter-btn veille-filter-btn-active"
                                                    } else {
                                                        "veille-filter-btn"
                                                    }
                                                }
                                                on:click=move |_| set_active_filter.set(key_str2.clone())
                                            >
                                                {move || i18n.t(label_key)}
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>

                                {move || match news_state.get() {
                                    NewsState::Loading => view! {
                                        <div class="veille-status">
                                            <p>{move || i18n.t("veille.loading")}</p>
                                        </div>
                                    }.into_any(),

                                    NewsState::Error => view! {
                                        <div class="veille-status veille-error">
                                            <p>{move || i18n.t("veille.error")}</p>
                                        </div>
                                    }.into_any(),

                                    NewsState::Loaded(data) => {
                                        let synth_filters: Vec<(String, String)> = data.items.iter()
                                            .filter(|i| i.item_type == "synthesis")
                                            .map(|s| (s.id.clone(), synthesis_week_label(&s.id).to_string()))
                                            .collect();

                                        let filter = active_filter.get();
                                        let filtered: Vec<NewsItem> = data.items.into_iter()
                                            .filter(|item| {
                                                if filter.starts_with("synthesis_") {
                                                    return item.id == filter
                                                        || item.synthesis_id.as_deref() == Some(filter.as_str());
                                                }
                                                item.item_type == "synthesis"
                                                    || filter == "all"
                                                    || item.categories.iter().any(|c| c == &filter)
                                            })
                                            .collect();

                                        view! {
                                            {if !synth_filters.is_empty() {
                                                view! {
                                                    <div class="veille-filters veille-filters-synthesis">
                                                        <span class="veille-filter-label">
                                                            {move || i18n.t("veille.synthesis.filterLabel")}
                                                        </span>
                                                        {synth_filters.into_iter().map(|(sid, week_label)| {
                                                            let sid2 = sid.clone();
                                                            view! {
                                                                <button
                                                                    class=move || {
                                                                        if active_filter.get() == sid {
                                                                            "veille-filter-btn veille-filter-btn-active"
                                                                        } else {
                                                                            "veille-filter-btn"
                                                                        }
                                                                    }
                                                                    on:click=move |_| set_active_filter.set(sid2.clone())
                                                                >
                                                                    {week_label}
                                                                </button>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                ().into_any()
                                            }}

                                            {if filtered.is_empty() {
                                                view! {
                                                    <div class="veille-status">
                                                        <p>{move || i18n.t("veille.noItems")}</p>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="veille-list">
                                                        {filtered.into_iter().map(|item| {
                                                            if item.item_type == "synthesis" {
                                                                view! {
                                                                    <WeeklySynthesisCard item=item />
                                                                }.into_any()
                                                            } else {
                                                                let cats = item.categories.clone();
                                                                let flag = lang_flag(&item.lang);
                                                                let synth_id = item.synthesis_id.clone();
                                                                let pub_at = item.published_at.clone();
                                                                view! {
                                                                    <article class="veille-card">
                                                                        <div class="veille-card-header">
                                                                            <span class="veille-source">{item.source.clone()}</span>
                                                                            <span class="veille-date">
                                                                                {move || {
                                                                                    let lang = i18n.current_lang_code();
                                                                                    format_date_locale(&pub_at, &lang)
                                                                                }}
                                                                            </span>
                                                                            <span class="veille-lang">{flag}</span>
                                                                            {synth_id.map(|sid| {
                                                                                let week = synthesis_week_label(&sid).to_string();
                                                                                let sid2 = sid.clone();
                                                                                view! {
                                                                                    <button
                                                                                        class="veille-synth-tag"
                                                                                        on:click=move |_| set_active_filter.set(sid2.clone())
                                                                                    >
                                                                                        {week}
                                                                                    </button>
                                                                                }
                                                                            })}
                                                                        </div>
                                                                        <h3 class="veille-card-title">
                                                                            <a href={item.url.clone()} target="_blank" rel="noopener noreferrer">
                                                                                {item.title.clone()}
                                                                            </a>
                                                                        </h3>
                                                                        <div class="veille-card-cats">
                                                                            {cats.into_iter().map(|cat| {
                                                                                let label = category_label(&cat, &i18n);
                                                                                view! {
                                                                                    <span class={format!("veille-cat veille-cat-{}", cat)}>
                                                                                        {label}
                                                                                    </span>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </div>
                                                                    </article>
                                                                }.into_any()
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                }.into_any()
                                            }}
                                        }.into_any()
                                    }
                                }}
                            }.into_any()
                        }
                    }}
                </div>
            </section>

            <Footer />
        </div>
    }
}
