use crate::services::I18nService;
use leptos::*;

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    view! {
        <section class="section projects" id="projects">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title" data-key="projectsTitle">
                        {move || i18n.t("projectsTitle")}
                    </h2>
                    <p class="section-subtitle" data-key="projectsSubtitle">
                        {move || i18n.t("projectsSubtitle")}
                    </p>
                </div>

                <div class="projects-grid">
                    // Featured Project
                    <div class="project-card featured">
                        <div class="project-image">
                            <div class="project-placeholder" aria-hidden="true">"🖨️"</div>
                        </div>
                        <div class="project-content">
                            <div class="project-tags">
                                <span class="project-tag">"Startup"</span>
                                <span class="project-tag">"Full-Stack"</span>
                            </div>
                            <div class="project-header">
                                <h3 class="project-title">
                                    <a
                                        href="/blog/?article=ezprint3d-journey"
                                        class="project-link"
                                    >
                                        "EzPrint3D"
                                    </a>
                                </h3>
                                <div class="project-badge">
                                    <span data-key="readStory">
                                        {move || i18n.t("readStory")}
                                    </span>
                                </div>
                            </div>
                            <p class="project-description" data-key="ezprint3dDescription">
                                {move || i18n.t("ezprint3dDescription")}
                            </p>
                            <div class="project-tech">
                                <span>"Symfony"</span>
                                <span>"Vue.js"</span>
                                <span>"Docker"</span>
                            </div>
                            <div class="project-actions">
                                <a
                                    href="/blog/?article=ezprint3d-journey"
                                    class="project-cta"
                                    data-key="readFullStory"
                                >
                                    <span>{move || i18n.t("readFullStory")}</span>
                                    <svg
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                    >
                                        <path d="M5 12h14M12 5l7 7-7 7" />
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>

                    // Project 2
                    <div class="project-card">
                        <div class="project-image">
                            <div class="project-placeholder" aria-hidden="true">"🤖"</div>
                        </div>
                        <div class="project-content">
                            <div class="project-tags">
                                <span class="project-tag">"Bot"</span>
                                <span class="project-tag">"Rust"</span>
                            </div>
                            <div class="project-header">
                                <h3 class="project-title">"LeKickerFou"</h3>
                                <div class="project-badge coming-soon">
                                    <span data-key="storyComingSoon">
                                        {move || i18n.t("storyComingSoon")}
                                    </span>
                                </div>
                            </div>
                            <p class="project-description" data-key="lekickerfouDescription">
                                {move || i18n.t("lekickerfouDescription")}
                            </p>
                            <div class="project-tech">
                                <span>"Rust"</span>
                                <span>"Discord API"</span>
                                <span>"Tokio"</span>
                            </div>
                            <div class="project-actions">
                                <button class="project-cta disabled" disabled>
                                    <span>"🔒"</span>
                                    <span data-key="storyComingSoonBtn">
                                        {move || i18n.t("storyComingSoonBtn")}
                                    </span>
                                    <svg
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                    >
                                        <path d="M5 12h14M12 5l7 7-7 7" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>

                    // Project 3
                    <div class="project-card">
                        <div class="project-image">
                            <div class="project-placeholder" aria-hidden="true">"⌨️"</div>
                        </div>
                        <div class="project-content">
                            <div class="project-tags">
                                <span class="project-tag">"Hardware"</span>
                                <span class="project-tag">"DIY"</span>
                            </div>
                            <div class="project-header">
                                <h3 class="project-title">
                                    <a href="/blog/?article=custom-keyboards" class="project-link">
                                        "Custom Keyboards"
                                    </a>
                                </h3>
                                <div class="project-badge">
                                    <span data-key="readStory">
                                        {move || i18n.t("readStory")}
                                    </span>
                                </div>
                            </div>
                            <p class="project-description" data-key="keyboardDescription">
                                {move || i18n.t("keyboardDescription")}
                            </p>
                            <div class="project-tech">
                                <span>"QMK"</span>
                                <span>"3D Printing"</span>
                                <span>"Electronics"</span>
                            </div>
                            <div class="project-actions">
                                <a
                                    href="/blog/?article=custom-keyboards"
                                    class="project-cta"
                                    data-key="readFullStory"
                                >
                                    <span>{move || i18n.t("readFullStory")}</span>
                                    <svg
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                    >
                                        <path d="M5 12h14M12 5l7 7-7 7" />
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
