use crate::components::footer::Footer;
use crate::components::navigation::Navigation;
use crate::services::I18nService;
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewsItem {
    id: String,
    title: String,
    url: String,
    source: String,
    categories: Vec<String>,
    published_at: String,
    lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SynthesisSection {
    category: String,
    summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DailySynthesis {
    headline: String,
    tldr: String,
    #[serde(default)]
    sections: Vec<SynthesisSection>,
    #[serde(default)]
    key_takeaways: Vec<String>,
    signal_vs_noise: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewsData {
    generated_at: String,
    count: usize,
    #[serde(default)]
    synthesis: Option<HashMap<String, DailySynthesis>>,
    items: Vec<NewsItem>,
}

#[derive(Debug, Clone)]
enum NewsState {
    Loading,
    Loaded(NewsData),
    Error,
}

fn format_date(iso: &str) -> String {
    iso.chars().take(10).collect()
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

#[component]
pub fn VeillePage() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let (news_state, set_news_state) = signal(NewsState::Loading);
    let (active_filter, set_active_filter) = signal("all".to_string());

    // Synthesis signals — set once on load, read reactively with language
    let (synthesis_en, set_synthesis_en) = signal::<Option<DailySynthesis>>(None);
    let (synthesis_fr, set_synthesis_fr) = signal::<Option<DailySynthesis>>(None);
    let is_synthesis_open = RwSignal::new(true);

    spawn_local(async move {
        match Request::get("/public/news.json").send().await {
            Ok(resp) => match resp.json::<NewsData>().await {
                Ok(data) => {
                    if let Some(ref synth_map) = data.synthesis {
                        set_synthesis_en.set(synth_map.get("en").cloned());
                        set_synthesis_fr.set(synth_map.get("fr").cloned());
                    }
                    set_news_state.set(NewsState::Loaded(data));
                }
                Err(_) => set_news_state.set(NewsState::Error),
            },
            Err(_) => set_news_state.set(NewsState::Error),
        }
    });

    let filters = vec![
        ("all", "veille.filterAll"),
        ("urgent", "veille.filterUrgent"),
        ("good_news", "veille.filterGoodNews"),
        ("future_watch", "veille.filterFutureWatch"),
        ("stack_alt", "veille.filterStackAlt"),
    ];

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
                            let date = format_date(&data.generated_at).to_string();
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

                    // Synthesis block — reactive to language and open/close toggle
                    {move || {
                        let lang_code = i18n.current_lang_code();
                        let is_open = is_synthesis_open.get();
                        let synth = if lang_code == "fr" {
                            synthesis_fr.get().or_else(|| synthesis_en.get())
                        } else {
                            synthesis_en.get().or_else(|| synthesis_fr.get())
                        };

                        match synth {
                            None => ().into_any(),
                            Some(s) => {
                                let headline = s.headline.clone();
                                let tldr = s.tldr.clone();
                                let sections = s.sections.clone();
                                let key_takeaways = s.key_takeaways.clone();
                                let signal_vs_noise = s.signal_vs_noise.clone();

                                view! {
                                    <div class="veille-synthesis">
                                        <div class="veille-synthesis-header">
                                            <h2 class="veille-synthesis-title">
                                                {i18n.t("veille.synthesis.title")}
                                            </h2>
                                            <button
                                                class="veille-synthesis-toggle"
                                                on:click=move |_| is_synthesis_open.update(|v| *v = !*v)
                                            >
                                                {if is_open {
                                                    i18n.t("veille.synthesis.hide")
                                                } else {
                                                    i18n.t("veille.synthesis.show")
                                                }}
                                            </button>
                                            <span class="veille-synthesis-badge">
                                                {i18n.t("veille.synthesis.aiGenerated")}
                                            </span>
                                        </div>
                                        {if is_open {
                                            view! {
                                                <div class="veille-synthesis-body">
                                                    <p class="veille-synthesis-headline">{headline}</p>
                                                    <div class="veille-synthesis-tldr">
                                                        <strong class="veille-synthesis-label">
                                                            {i18n.t("veille.synthesis.tldr")}
                                                        </strong>
                                                        <p>{tldr}</p>
                                                    </div>
                                                    {if !sections.is_empty() {
                                                        view! {
                                                            <div class="veille-synthesis-sections">
                                                                {sections.into_iter().map(|section| {
                                                                    let label = category_label(&section.category, &i18n);
                                                                    view! {
                                                                        <div class="veille-synthesis-section">
                                                                            <span class={format!("veille-cat veille-cat-{}", section.category)}>
                                                                                {label}
                                                                            </span>
                                                                            <p>{section.summary}</p>
                                                                        </div>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        ().into_any()
                                                    }}
                                                    {if !key_takeaways.is_empty() {
                                                        view! {
                                                            <div class="veille-synthesis-takeaways">
                                                                <strong class="veille-synthesis-label">
                                                                    {i18n.t("veille.synthesis.keyTakeaways")}
                                                                </strong>
                                                                <ul>
                                                                    {key_takeaways.into_iter().map(|t| {
                                                                        view! { <li>{t}</li> }
                                                                    }).collect::<Vec<_>>()}
                                                                </ul>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        ().into_any()
                                                    }}
                                                    <div class="veille-synthesis-noise">
                                                        <strong class="veille-synthesis-label">
                                                            {i18n.t("veille.synthesis.signalVsNoise")}
                                                        </strong>
                                                        <p>{signal_vs_noise}</p>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            ().into_any()
                                        }}
                                    </div>
                                }.into_any()
                            }
                        }
                    }}

                    // Category filter tabs
                    <div class="veille-filters">
                        {filters.into_iter().map(|(key, label_key)| {
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

                    // News list
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
                            let filter = active_filter.get();
                            let filtered: Vec<NewsItem> = data.items.into_iter()
                                .filter(|item| {
                                    filter == "all" || item.categories.iter().any(|c| c == &filter)
                                })
                                .collect();

                            if filtered.is_empty() {
                                view! {
                                    <div class="veille-status">
                                        <p>{move || i18n.t("veille.noItems")}</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="veille-list">
                                        {filtered.into_iter().map(|item| {
                                            let cats = item.categories.clone();
                                            let flag = lang_flag(&item.lang);
                                            view! {
                                                <article class="veille-card">
                                                    <div class="veille-card-header">
                                                        <span class="veille-source">{item.source.clone()}</span>
                                                        <span class="veille-date">{format_date(&item.published_at)}</span>
                                                        <span class="veille-lang">{flag}</span>
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
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </div>
            </section>
            <Footer />
        </div>
    }
}
