use crate::{
    components::ui::CVDownloadButton,
    data::translations::Language,
    services::{I18nService, ThemeService},
};
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");
    let theme = use_context::<ThemeService>().expect("Theme service not found");
    let location = use_location();
    let pathname = move || location.pathname.get();

    let is_scrolled = create_rw_signal(false);

    create_effect(move |_| {
        let closure = Closure::wrap(Box::new(move || {
            let scrolled = web_sys::window()
                .and_then(|w| w.scroll_y().ok())
                .unwrap_or(0.0)
                > 50.0;
            is_scrolled.set(scrolled);
        }) as Box<dyn Fn()>);

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }
        closure.forget();
    });

    view! {
        <nav class="nav" class:scrolled=move || is_scrolled.get()>
            <div class="nav-container">
                // Logo → projects section
                <a href="/#projects" class="nav-logo-link">
                    <span class="logo-text logo-full" data-key="name">
                        {move || i18n.t("name")}
                    </span>
                    <span class="logo-text logo-short">"KB"</span>
                </a>

                // Page links
                <div class="nav-links">
                    <A
                        href="/"
                        class=move || if pathname() == "/" {
                            "nav-link nav-link-active"
                        } else {
                            "nav-link"
                        }
                    >
                        {move || i18n.t("navigation.home")}
                    </A>
                    <A
                        href="/blog"
                        class=move || if pathname() == "/blog" {
                            "nav-link nav-link-active"
                        } else {
                            "nav-link"
                        }
                    >
                        {move || i18n.t("navigation.blog")}
                    </A>
                    <A
                        href="/veille"
                        class=move || if pathname() == "/veille" {
                            "nav-link nav-link-active"
                        } else {
                            "nav-link"
                        }
                    >
                        {move || i18n.t("navigation.veille")}
                    </A>
                </div>

                // Controls
                <div class="nav-controls">
                    <CVDownloadButton/>

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
