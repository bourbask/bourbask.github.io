use crate::services::I18nService;
use leptos::prelude::*;

#[component]
pub fn NotFound404() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    // Détecter si on est sur une route blog
    let is_blog_path = move || {
        web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .map(|path| {
                path.starts_with("/blog/") && path != "/blog/" && path != "/blog/index.html"
            })
            .unwrap_or(false)
    };

    view! {
        <div class="error-page">
            // Floating elements
            <div class="floating-element floating-404-1" aria-hidden="true">"🔍"</div>
            <div class="floating-element floating-404-2" aria-hidden="true">"🤖"</div>

            <div class="error-container">
                <div class="error-code">"404"</div>

                <h1 class="error-title" id="errorTitle">
                    {move || {
                        let i18n = i18n.clone();
                        if is_blog_path() {
                            i18n.t("error404Title")
                        } else {
                            i18n.t("pageNotFound")
                        }
                    }}
                </h1>

                <p class="error-message" id="errorMessage">
                    {move || {
                        let i18n = i18n.clone();
                        if is_blog_path() {
                            i18n.t("error404Message")
                        } else {
                            i18n.t("error404Generic")
                        }
                    }}
                </p>

                <div class="error-actions">
                    <a href="/" class="error-btn error-btn-primary" id="homeBtn">
                        <span>"🏠"</span>
                        <span id="homeBtnText">
                            {move || {
                                let i18n = i18n.clone();
                                i18n.t("goHome")
                            }}
                        </span>
                    </a>
                    <a href="/blog/" class="error-btn error-btn-secondary" id="blogBtn">
                        <span>"📝"</span>
                        <span id="blogBtnText">
                            {move || {
                                let i18n = i18n.clone();
                                if is_blog_path() {
                                    i18n.t("backToBlog")
                                } else {
                                    i18n.t("exploreBlog")
                                }
                            }}
                        </span>
                    </a>
                </div>
            </div>
        </div>
    }
}
