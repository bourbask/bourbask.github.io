use crate::services::ThemeService;
use leptos::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_context::<ThemeService>().expect("Theme service not found");

    view! {
        <button
            on:click={
                let theme = theme.clone();
                move |_| theme.toggle_theme()
            }
            class="theme-toggle"
            title={
                let theme = theme.clone();
                move || if theme.is_dark() { "Switch to light mode" } else { "Switch to dark mode" }
            }
        >
            <span class="theme-toggle-icon">
                {
                    let theme = theme.clone();
                    move || if theme.is_dark() { "☀️" } else { "🌙" }
                }
            </span>
        </button>
    }
}
