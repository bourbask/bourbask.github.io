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
                            "Page Not Found".to_string()
                        }
                    }}
                </h1>

                <p class="error-message" id="errorMessage">
                    {move || {
                        let i18n = i18n.clone();
                        if is_blog_path() {
                            i18n.t("error404Message")
                        } else {
                            match i18n.current_language.get().as_str() {
                                "fr" => "Oups ! La page que vous cherchez semble avoir disparu dans le vide numérique.",
                                _ => "Oops! The page you're looking for seems to have vanished into the digital void."
                            }.to_string()
                        }.to_string()
                    }}
                </p>

                <div class="error-actions">
                    <a href="/" class="error-btn error-btn-primary" id="homeBtn">
                        <span>"🏠"</span>
                        <span id="homeBtnText">
                            {move || {
                                let i18n = i18n.clone();
                                match i18n.current_language.get().as_str() {
                                    "fr" => "Accueil",
                                    _ => "Go Home"
                                }
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
                                    match i18n.current_language.get().as_str() {
                                        "fr" => "Explorer le Blog",
                                        _ => "Explore Blog"
                                    }.to_string()
                                }
                            }}
                        </span>
                    </a>
                </div>
            </div>
        </div>
    }
}
