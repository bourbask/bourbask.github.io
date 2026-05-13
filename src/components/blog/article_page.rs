use crate::components::NotFound404;
use crate::data::articles::Article;
use crate::services::{BlogService, I18nService};
use leptos::prelude::*;

#[component]
pub fn ArticlePage(article_id: String) -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let blog_service = use_context::<BlogService>().expect("Blog service not found");

    let article = blog_service.get_article_by_id(&article_id);

    view! {
        {move || {
            if let Some(article) = &article {
                let lang = i18n.current_lang_code();
                view! { <ArticleView article=article.clone() lang=lang /> }.into_any()
            } else {
                view! { <NotFound404 /> }.into_any()
            }
        }}
    }
}

#[component]
pub fn ArticleView(article: Article, lang: String) -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    let title = RwSignal::new(article.meta.title.get(&lang).cloned().unwrap_or_default());
    let subtitle = RwSignal::new(
        article
            .meta
            .subtitle
            .get(&lang)
            .cloned()
            .unwrap_or_default(),
    );
    let content = RwSignal::new(article.content.get(&lang).cloned());
    let _meta = RwSignal::new(article.meta.clone());

    // Mettre à jour quand la langue change
    Effect::new(move |_| {
        let current_lang = i18n.current_lang_code();
        title.set(
            article
                .meta
                .title
                .get(&current_lang)
                .cloned()
                .unwrap_or_default(),
        );
        subtitle.set(
            article
                .meta
                .subtitle
                .get(&current_lang)
                .cloned()
                .unwrap_or_default(),
        );
        content.set(article.content.get(&current_lang).cloned());
    });

    view! {
        <article class="blog-article">
            <header class="article-header">
                <nav class="article-breadcrumb">
                    <a href="/blog/" data-key="blogTitle">
                        {move || {
                            let i18n = i18n.clone();
                            i18n.t("blogTitle")
                        }}
                    </a>
                    <span class="breadcrumb-separator">"/"</span>
                    <span class="breadcrumb-current">{move || title.get()}</span>
                </nav>

                <div class="article-meta">
                    <time class="article-date">{article.meta.date.clone()}</time>
                    <span class="article-read-time">{article.meta.read_time}" min read"</span>
                    <div class="article-tags">
                        {article.meta.tags.iter().cloned()
                            .map(|tag| view! {
                                <span class="tag">{tag}</span>
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>

                <h1 class="article-title">{move || title.get()}</h1>
                <p class="article-subtitle">{move || subtitle.get()}</p>

                {move || content.get().map(|c| view! {
                    <div class="article-tldr">
                        <strong>"TL;DR: "</strong> {c.tldr.clone()}
                    </div>
                })}


                <div class="article-hero">
                    <div class="article-placeholder article-hero-image">
                        {crate::data::articles::get_category_emoji(&article.meta.category)}
                    </div>
                </div>
            </header>

            <div class="article-content">
                {move || {
                    match content.get() {
                        Some(content) => {
                            view! {
                                <>
                                    <div class="article-toc">
                                        <h3>"Table of Contents"</h3>
                                        <ul>
                                            {content.sections.iter()
                                                .map(|section| {
                                                    let href = format!("#{}", section.id);
                                                    let title = section.title.clone();
                                                    view! {
                                                        <li>
                                                            <a href={href}>{title}</a>
                                                        </li>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </ul>
                                    </div>

                                    <div class="article-body">
                                        {content.sections.iter()
                                            .map(|section| {
                                                let id = section.id.clone();
                                                let html = parse_markdown(&section.content);
                                                view! {
                                                    <section id={id} class="article-section">
                                                        <div inner_html={html}></div>
                                                    </section>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </>
                            }.into_any()
                        }
                        None => {
                            view! {
                                <div class="article-error">
                                    <p>"Content not available in this language."</p>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </div>

            <footer class="article-footer">
                <div class="article-navigation">
                    <a href="/blog/" class="btn-back">
                        "← "
                        {move || {
                            let i18n = i18n.clone();
                            i18n.t("backToBlog")
                        }}
                    </a>
                </div>
            </footer>
        </article>
    }
}

fn parse_markdown(content: &str) -> String {
    content.to_string()
}
