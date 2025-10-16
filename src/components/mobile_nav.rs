use crate::services::{I18nService, ThemeService};
use leptos::*;

#[component]
pub fn MobileFloatingNav() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let theme = use_context::<ThemeService>().expect("Theme service not found");

    let (is_open, set_is_open) = create_signal(false);

    let toggle_nav = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    let close_nav = move |_| {
        set_is_open.set(false);
    };

    view! {
        <div class="mobile-floating-nav" id="mobileFloatingNav">
            // Overlay pour fermer
            <div
                class="mobile-nav-overlay"
                id="mobileNavOverlay"
                class:active=move || is_open.get()
                on:click=close_nav
            ></div>

            // Navigation Items
            <div
                class="mobile-nav-items"
                id="mobileNavItems"
                class:active=move || is_open.get()
            >
                <button class="mobile-nav-item" id="mobileBlogBtn" data-label="Blog">
                    "📝"
                </button>

                <button
                    class="mobile-nav-item"
                    id="mobileCVBtn"
                    data-label="Download CV"
                >
                    "📄"
                </button>

                <button
                    class="mobile-nav-item"
                    id="mobileLangBtn"
                    data-label="Language"
                    on:click=move |_| i18n.toggle_language()
                >
                    <span class="mobile-lang-flag">
                        {move || match i18n.current_language.get().as_str() {
                            "fr" => "🇫🇷",
                            _ => "🇺🇸"
                        }}
                    </span>
                </button>

                <button
                    class="mobile-nav-item"
                    id="mobileThemeBtn"
                    data-label="Theme"
                    on:click=move |_| theme.toggle_theme()
                >
                    <svg
                        class="theme-icon sun-icon"
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
                        class="theme-icon moon-icon"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                    >
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </button>
            </div>

            // Main FAB Button
            <button class="mobile-fab" id="mobileFab" on:click=toggle_nav>
                <div class="fab-icon">
                    <span class="fab-icon-line"></span>
                    <span class="fab-icon-line"></span>
                    <span class="fab-icon-line"></span>
                </div>
            </button>
        </div>
    }
}
