use crate::{
    components::ui::CVDownloadButton, // ✅ Ajouter l'import
    data::translations::Language,
    services::{I18nService, ThemeService},
};
use leptos::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let theme = use_context::<ThemeService>().expect("Theme service not found");

    view! {
        <nav class="nav">
            <div class="nav-container">
                // Logo
                <div class="nav-logo">
                    <span class="logo-text" data-key="name">
                        {move || i18n.t("name")}
                    </span>
                </div>

                // Navigation Controls
                <div class="nav-controls">
                    // Blog
                    <a href="/blog/" class="nav-link blog-link">
                        <span data-key="blogNavigation">
                            {move || i18n.t("blogNavigation")}
                        </span>
                    </a>

                    // ✅ Utiliser le composant CV Download
                    <CVDownloadButton/>

                    // Language Toggle
                    <button
                        class="lang-toggle"
                        id="langToggle"
                        attr:data-lang={move || i18n.current_language.get().as_str()}
                        on:click=move |_| {
                            let current = i18n.current_language.get();
                            let new_lang = match current {
                                Language::En => "fr",
                                Language::Fr => "en",
                            };
                            i18n.set_language_from_str(new_lang);
                        }
                    >
                        <span
                            class="lang-option"
                            class:active=move || i18n.current_language.get().as_str() == "en"
                            data-lang="en"
                        >
                            "EN"
                        </span>
                        <span
                            class="lang-option"
                            class:active=move || i18n.current_language.get().as_str() == "fr"
                            data-lang="fr"
                        >
                            "FR"
                        </span>
                        <div class="lang-slider"></div>
                    </button>

                    // Theme Toggle
                    <button
                        class="theme-toggle"
                        id="themeToggle"
                        on:click=move |_| theme.toggle_theme()
                    >
                        <div class="theme-icon-container">
                            <svg
                                class="sun-icon"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                            >
                                <circle cx="12" cy="12" r="5"></circle>
                                <line x1="12" y1="1" x2="12" y2="3"></line>
                                <line x1="12" y1="21" x2="12" y2="23"></line>
                                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                                <line x1="1" y1="12" x2="3" y2="12"></line>
                                <line x1="21" y1="12" x2="23" y2="12"></line>
                                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                            </svg>
                            <svg
                                class="moon-icon"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                            >
                                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                            </svg>
                        </div>
                    </button>
                </div>
            </div>
        </nav>
    }
}
