use crate::services::I18nService;
use leptos::*;

#[component]
pub fn SkillsSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    view! {
        <section class="section skills">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title" data-key="skillsTitle">
                        {move || i18n.t("skillsTitle")}
                    </h2>
                    <p class="section-subtitle" data-key="skillsSubtitle">
                        {move || i18n.t("skillsSubtitle")}
                    </p>
                </div>

                <div class="skills-grid">
                    // Frontend Skills
                    <div class="skill-category">
                        <h3 data-key="frontendTitle">
                            {move || i18n.t("frontendTitle")}
                        </h3>
                        <div class="skill-tags">
                            <span class="skill-tag">"Vue.js"</span>
                            <span class="skill-tag">"React.js"</span>
                            <span class="skill-tag">"JavaScript"</span>
                            <span class="skill-tag">"TypeScript"</span>
                            <span class="skill-tag">"CSS/SCSS"</span>
                        </div>
                    </div>

                    // Backend Skills
                    <div class="skill-category">
                        <h3 data-key="backendTitle">
                            {move || i18n.t("backendTitle")}
                        </h3>
                        <div class="skill-tags">
                            <span class="skill-tag">"Symfony"</span>
                            <span class="skill-tag">"API Platform"</span>
                            <span class="skill-tag">"PHP"</span>
                            <span class="skill-tag">"Node.js"</span>
                            <span class="skill-tag">"PostgreSQL"</span>
                        </div>
                    </div>

                    // DevOps Skills
                    <div class="skill-category">
                        <h3 data-key="devopsTitle">
                            {move || i18n.t("devopsTitle")}
                        </h3>
                        <div class="skill-tags">
                            <span class="skill-tag">"Docker"</span>
                            <span class="skill-tag">"OpenTofu"</span>
                            <span class="skill-tag">"Ansible"</span>
                            <span class="skill-tag">"GitHub Actions"</span>
                            <span class="skill-tag">"GitLab CI"</span>
                        </div>
                    </div>

                    // Tools
                    <div class="skill-category">
                        <h3 data-key="toolsTitle">
                            {move || i18n.t("toolsTitle")}
                        </h3>
                        <div class="skill-tags">
                            <span class="skill-tag">"Manjaro Linux"</span>
                            <span class="skill-tag">"Git"</span>
                            <span class="skill-tag">"VS Code"</span>
                            <span class="skill-tag">"Figma"</span>
                            <span class="skill-tag">"Postman"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
