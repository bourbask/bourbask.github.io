use crate::components::{ArticlePage, Footer, MobileFloatingNav, Navigation};
use crate::data::articles::{get_category_emoji, Article};
use crate::services::{BlogService, I18nService, ThemeService};
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let blog_service = BlogService::new();
    provide_context(blog_service.clone());

    // Query params pour les articles
    let query = use_query_map();
    let article_id = move || query.with(|q| q.get("article").cloned().unwrap_or_default());

    view! {
        <div class="blog-container">
            <Navigation />

            <main id="blog-container">
                {move || {
                    let current_article = article_id();
                    if current_article.is_empty() {
                        // Afficher l'index du blog
                        view! { <BlogIndex /> }.into_view()
                    } else {
                        // Afficher l'article spécifique
                        view! { <ArticlePage article_id=current_article /> }.into_view()
                    }
                }}
            </main>

            <Footer />
            <MobileFloatingNav />
        </div>
    }
}

#[component]
pub fn BlogIndex() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let blog_service = use_context::<BlogService>().expect("Blog service not found");

    // ✅ Créer des signaux pour toutes les données réactives
    let articles = create_rw_signal(blog_service.get_articles());
    let latest_article = create_rw_signal(blog_service.get_latest_article());
    let categories = create_rw_signal(blog_service.get_categories());
    let total_read_time = create_rw_signal(blog_service.get_total_read_time());

    let (category_filter, set_category_filter) = create_signal(String::new());
    let (sort_filter, set_sort_filter) = create_signal("newest".to_string());

    // ✅ Cloner le service pour l'utiliser dans l'effect
    let blog_service_cloned = blog_service.clone();

    // Apply filters when they change
    create_effect(move |_| {
        blog_service_cloned.apply_filters(category_filter.get(), sort_filter.get());
        articles.set(blog_service_cloned.get_articles());
        // Mettre à jour aussi le temps de lecture total si nécessaire
        total_read_time.set(blog_service_cloned.get_total_read_time());
    });

    view! {
        // Blog Hero Section
        <section class="blog-hero">
            <div class="blog-hero-container">
                // Hero Content
                <div class="blog-hero-content">
                    <div class="blog-badge">
                        <span data-key="blogBadge">
                            {move || {
                                let i18n = i18n.clone();
                                i18n.t("blogBadge")
                            }}
                        </span>
                    </div>

                    <h1 class="blog-hero-title">
                        <span data-key="blogHeroTitle1">
                            {move || {
                                let i18n = i18n.clone();
                                i18n.t("blogHeroTitle1")
                            }}
                        </span>
                        <br />
                        <span class="gradient-text" data-key="blogHeroTitle2">
                            {move || {
                                let i18n = i18n.clone();
                                i18n.t("blogHeroTitle2")
                            }}
                        </span>
                    </h1>

                    <p class="blog-hero-description" data-key="blogHeroDescription">
                        {move || {
                            let i18n = i18n.clone();
                            i18n.t("blogHeroDescription")
                        }}
                    </p>

                    <div class="blog-hero-stats">
                        <div class="blog-stat">
                            // ✅ Utiliser le signal
                            <div class="blog-stat-number">{move || articles.get().len()}</div>
                            <div class="blog-stat-label" data-key="articlesPublished">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("articlesPublished")
                                }}
                            </div>
                        </div>
                        <div class="blog-stat">
                            // ✅ Utiliser le signal au lieu de blog_service directement
                            <div class="blog-stat-number">{move || total_read_time.get()}</div>
                            <div class="blog-stat-label" data-key="minutesReading">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("minutesReading")
                                }}
                            </div>
                        </div>
                        <div class="blog-stat">
                            // ✅ Utiliser le signal
                            <div class="blog-stat-number">{move || categories.get().len()}</div>
                            <div class="blog-stat-label" data-key="categories">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("categories")
                                }}
                            </div>
                        </div>
                    </div>
                </div>

                // Latest Article Card
                <div class="blog-hero-visual">
                    {move || {
                        // ✅ Utiliser le signal
                        match latest_article.get() {
                            Some(article) => {
                                let lang = i18n.current_lang_code();
                                view! { <LatestArticleCard article=article lang=lang /> }.into_view()
                            }
                            None => {
                                view! { <div></div> }.into_view()
                            }
                        }
                    }}
                </div>
            </div>
        </section>

        // Articles List Section
        <section class="blog-articles-section">
            <div class="blog-articles-container">
                <div class="blog-articles-header">
                    <h2 class="blog-articles-title" data-key="allArticlesTitle">
                        {move || {
                            let i18n = i18n.clone();
                            i18n.t("allArticlesTitle")
                        }}
                    </h2>
                    <div class="blog-filters">
                        <select
                            class="category-filter"
                            id="categoryFilter"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_category_filter.set(value);
                            }
                        >
                            <option value="" data-key="allCategories">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("allCategories")
                                }}
                            </option>
                            // ✅ Utiliser le signal pour les catégories
                            {move || {
                                categories.get().into_iter()
                                    .map(|cat| view! {
                                        <option value={cat.clone()}>{cat}</option>
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </select>
                        <select
                            class="sort-filter"
                            id="sortFilter"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_sort_filter.set(value);
                            }
                        >
                            <option value="newest" data-key="sortNewest">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("sortNewest")
                                }}
                            </option>
                            <option value="oldest" data-key="sortOldest">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("sortOldest")
                                }}
                            </option>
                            <option value="reading-time" data-key="sortReadingTime">
                                {move || {
                                    let i18n = i18n.clone();
                                    i18n.t("sortReadingTime")
                                }}
                            </option>
                        </select>
                    </div>
                </div>

                <div class="blog-articles-list" id="articlesList">
                    {move || {
                        let lang = i18n.current_lang_code();
                        articles.get().into_iter()
                            .map(|article| view! {
                                <ArticleListItem article=article lang=lang.clone() />
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn LatestArticleCard(article: Article, lang: String) -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let title = article.meta.title.get(&lang).cloned().unwrap_or_default();
    let description = article
        .meta
        .description
        .get(&lang)
        .cloned()
        .unwrap_or_default();

    view! {
        <div class="latest-article-card">
            <div class="latest-article-badge">
                <span data-key="latestArticle">
                    {move || {
                        let i18n = i18n.clone();
                        i18n.t("latestArticle")
                    }}
                </span>
            </div>

            <div class="latest-article-image">
                <div class="article-placeholder">
                    {get_category_emoji(&article.meta.category)}
                </div>
                <div class="latest-article-overlay">
                    <span class="latest-article-category">{&article.meta.category}</span>
                </div>
            </div>

            <div class="latest-article-content">
                <h3 class="latest-article-title">
                    <a href={format!("/blog?article={}", article.meta.id)} class="latest-article-link">
                        {title}
                    </a>
                </h3>
                <p class="latest-article-description">{description}</p>

                <div class="latest-article-meta">
                    <time class="latest-article-date">{&article.meta.date}</time>
                    <span class="latest-article-read-time">{article.meta.read_time}" min"</span>
                </div>

                <a href={format!("/blog?article={}", article.meta.id)} class="latest-article-cta">
                    <span data-key="readArticle">
                        {move || {
                            let i18n = i18n.clone();
                            i18n.t("readArticle")
                        }}
                    </span>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <path d="M5 12h14M12 5l7 7-7 7"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn ArticleListItem(article: Article, lang: String) -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let title = article.meta.title.get(&lang).cloned().unwrap_or_default();
    let description = article
        .meta
        .description
        .get(&lang)
        .cloned()
        .unwrap_or_default();

    view! {
        <article
            class="article-list-item"
            data-category={&article.meta.category}
            data-date={&article.meta.date}
        >
            <div class="article-list-image">
                <div class="article-placeholder">
                    {get_category_emoji(&article.meta.category)}
                </div>
            </div>

            <div class="article-list-content">
                <div class="article-list-meta">
                    <time class="article-list-date">{&article.meta.date}</time>
                    <span class="article-list-category">{&article.meta.category}</span>
                    <span class="article-list-read-time">{article.meta.read_time}" min"</span>
                </div>

                <h3 class="article-list-title">
                    <a href={format!("/blog?article={}", article.meta.id)}>{title}</a>
                </h3>

                <p class="article-list-description">{description}</p>

                <div class="article-list-tags">
                    {article.meta.tags.iter().take(3)
                        .map(|tag| view! {
                            <span class="article-list-tag">{tag}</span>
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </article>
    }
}
