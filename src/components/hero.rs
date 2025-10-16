use crate::services::I18nService;
use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    let scroll_to_projects = move |_| {
        if let Some(element) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("projects")
        {
            element.scroll_into_view();
        }
    };

    let scroll_to_contact = move |_| {
        if let Some(element) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("contact")
        {
            element.scroll_into_view();
        }
    };

    view! {
        <section class="hero">
            <div class="hero-container">
                // Hero Content
                <div class="hero-content">
                    <div class="hero-badge">
                        <span data-key="badge">
                            {move || i18n.t("badge")}
                        </span>
                    </div>

                    <h1 class="hero-title">
                        <span data-key="heroTitle1">
                            {move || i18n.t("heroTitle1")}
                        </span>
                        <br />
                        <span class="gradient-text" data-key="heroTitle2">
                            {move || i18n.t("heroTitle2")}
                        </span>
                    </h1>

                    <p class="hero-description" data-key="heroDescription">
                        {move || i18n.t("heroDescription")}
                    </p>

                    <div class="hero-actions">
                        <button
                            class="btn btn-primary"
                            on:click=scroll_to_projects
                            data-key="viewWork"
                        >
                            {move || i18n.t("viewWork")}
                        </button>
                        <button
                            class="btn btn-secondary"
                            on:click=scroll_to_contact
                            data-key="getInTouch"
                        >
                            {move || i18n.t("getInTouch")}
                        </button>
                    </div>
                </div>

                // Hero Visual
                <div class="hero-visual">
                    <div class="floating-card card-code">
                        <div class="code-snippet">
                            <div class="code-header">
                                <div class="code-dots">
                                    <span></span><span></span><span></span>
                                </div>
                            </div>
                            <div class="code-content">
                                <span class="code-line">
                                    "const " <span class="code-var">"developer"</span> " = {"
                                </span>
                                <span class="code-line">
                                    "name: " <span class="code-string">"\"Kévin\""</span> ","
                                </span>
                                <span class="code-line">
                                    "skills: [" <span class="code-string">"\"Vue\""</span> ", "
                                    <span class="code-string">"\"React\""</span> ", "
                                    <span class="code-string">"\"Symfony\""</span> "]"
                                </span>
                                <span class="code-line">"};"</span>
                            </div>
                        </div>
                    </div>

                    <div class="floating-card card-tech">
                        <div class="tech-icons">
                            <div class="tech-icon">"🥾"</div>
                            <div class="tech-icon">"🌿"</div>
                            <div class="tech-icon">"🔧"</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
