use crate::services::{I18nService, ThemeService};
use leptos::*;

#[component]
pub fn AboutSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    view! {
        <section class="section about" id="about">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title" data-key="aboutTitle">
                        {move || i18n.t("aboutTitle")}
                    </h2>
                    <p class="section-subtitle" data-key="aboutSubtitle">
                        {move || i18n.t("aboutSubtitle")}
                    </p>
                </div>

                <div class="about-content">
                    <div class="about-info">
                        // Profile Image
                        <div class="profile-image-container">
                            <img
                                src="/public/icons/profil.png"
                                alt="Kévin Bourbasquet"
                                class="profile-image"
                            />
                            <div class="profile-decoration"></div>
                        </div>

                        // About Text
                        <div class="about-text">
                            <p data-key="aboutText1">
                                {move || i18n.t("aboutText1")}
                            </p>
                            <p data-key="aboutText2">
                                {move || i18n.t("aboutText2")}
                            </p>
                        </div>
                    </div>

                    // Statistics
                    <div class="about-stats">
                        <div class="stat">
                            <div class="stat-number">"3+"</div>
                            <div class="stat-label" data-key="yearsExperience">
                                {move || i18n.t("yearsExperience")}
                            </div>
                        </div>
                        <div class="stat">
                            <div class="stat-number">"20+"</div>
                            <div class="stat-label" data-key="projectsCompleted">
                                {move || i18n.t("projectsCompleted")}
                            </div>
                        </div>
                        <div class="stat">
                            <div class="stat-number">"5+"</div>
                            <div class="stat-label" data-key="technologiesMastered">
                                {move || i18n.t("technologiesMastered")}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
