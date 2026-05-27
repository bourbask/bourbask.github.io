use crate::services::{CVService, I18nService, ThemeService};
use leptos::prelude::*;

#[component]
pub fn MobileFloatingNav() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let theme = use_context::<ThemeService>().expect("Theme service not found");
    let cv_service = CVService::new(i18n.clone());

    // State (open/close, drag) is managed entirely by fab-interaction.js.
    // Leptos owns only the rendered structure and the action handlers on items.
    view! {
        <>
        // Overlay — class:active toggled by JS
        <div class="mobile-nav-overlay" id="mobileNavOverlay"></div>

        <div class="mobile-floating-nav" id="mobileFloatingNav">
            // Arc items — class:active toggled by JS
            <div class="mobile-nav-items" id="mobileNavItems">
                <a href="/blog" class="mobile-nav-item"
                    attr:data-label={move || i18n.t("navigation.blog")}
                    aria-label={move || i18n.t("navigation.blog")}
                >
                    "📝"
                </a>
                <a href="/veille" class="mobile-nav-item"
                    attr:data-label={move || i18n.t("navigation.veille")}
                    aria-label={move || i18n.t("navigation.veille")}
                >
                    "📡"
                </a>
                <button
                    class="mobile-nav-item"
                    id="mobileCVBtn"
                    type="button"
                    attr:data-label={move || i18n.t("navigation.cv")}
                    aria-label={move || i18n.t("action")}
                    on:click=move |_| cv_service.generate_pdf()
                >
                    "📄"
                </button>
                <button
                    class="mobile-nav-item"
                    id="mobileLangBtn"
                    type="button"
                    attr:data-label={move || i18n.t("navigation.language")}
                    aria-label={move || match i18n.current_language.get().as_str() {
                        "fr" => i18n.t("switchToEnglish"),
                        _    => i18n.t("switchToFrench"),
                    }}
                    on:click=move |_| i18n.toggle_language()
                >
                    <span class="mobile-lang-flag">
                        {move || match i18n.current_language.get().as_str() {
                            "fr" => "🇫🇷",
                            _    => "🇬🇧",
                        }}
                    </span>
                </button>
                <button
                    class="mobile-nav-item"
                    id="mobileThemeBtn"
                    type="button"
                    attr:data-label={move || i18n.t("navigation.theme")}
                    aria-label={move || i18n.t("toggleTheme")}
                    on:click=move |_| theme.toggle_theme()
                >
                    <svg class="theme-icon sun-icon"  viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <circle cx="12" cy="12" r="5"></circle>
                        <line x1="12" y1="1"  x2="12" y2="3"></line>
                        <line x1="12" y1="21" x2="12" y2="23"></line>
                        <line x1="4.22"  y1="4.22"  x2="5.64"  y2="5.64"></line>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                        <line x1="1"  y1="12" x2="3"  y2="12"></line>
                        <line x1="21" y1="12" x2="23" y2="12"></line>
                        <line x1="4.22"  y1="19.78" x2="5.64"  y2="18.36"></line>
                        <line x1="18.36" y1="5.64"  x2="19.78" y2="4.22"></line>
                    </svg>
                    <svg class="theme-icon moon-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </button>
            </div>

            // FAB — on:click intentionally absent; JS (fab-interaction.js) handles
            // all touch interaction: long-press → drag mode, short tap → tap toggle.
            <button class="mobile-fab" id="mobileFab" type="button" aria-label={move || i18n.t("navigation.menu")} aria-expanded="false">
                <div class="fab-icon">
                    <span class="fab-icon-line"></span>
                    <span class="fab-icon-line"></span>
                    <span class="fab-icon-line"></span>
                </div>
            </button>
        </div>
        </>
    }
}
