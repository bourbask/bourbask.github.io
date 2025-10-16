use crate::services::I18nService;
use leptos::*;

#[component]
pub fn LangToggle() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    view! {
        <button
            on:click={
                let i18n = i18n.clone();
                move |_| i18n.toggle_language()
            }
            class="lang-toggle"
            title={
                let i18n = i18n.clone();
                move || if i18n.is_english() { "Passer en français" } else { "Switch to English" }
            }
        >
            <span class="lang-toggle-flag">
                {
                    let i18n = i18n.clone();
                    move || if i18n.is_english() { "🇺🇸" } else { "🇫🇷" }
                }
            </span>
            <span class="lang-toggle-text">
                {
                    let i18n = i18n.clone();
                    move || if i18n.is_english() { "EN" } else { "FR" }
                }
            </span>
        </button>
    }
}
