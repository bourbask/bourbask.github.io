use crate::services::I18nService;
use leptos::*;

#[component]
pub fn InterestsSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    view! {
        <section class="section interests">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title" data-key="interestsTitle">
                        {move || i18n.t("interestsTitle")}
                    </h2>
                    <p class="section-subtitle" data-key="interestsSubtitle">
                        {move || i18n.t("interestsSubtitle")}
                    </p>
                </div>

                <div class="interests-grid">
                    <div class="interest-card">
                        <div class="interest-icon" aria-hidden="true">"📸"</div>
                        <h3 data-key="photographyTitle">
                            {move || i18n.t("photographyTitle")}
                        </h3>
                        <p data-key="photographyDescription">
                            {move || i18n.t("photographyDescription")}
                        </p>
                    </div>

                    <div class="interest-card">
                        <div class="interest-icon" aria-hidden="true">"🎵"</div>
                        <h3 data-key="musicTitle">
                            {move || i18n.t("musicTitle")}
                        </h3>
                        <p data-key="musicDescription">
                            {move || i18n.t("musicDescription")}
                        </p>
                    </div>

                    <div class="interest-card">
                        <div class="interest-icon" aria-hidden="true">"🥾"</div>
                        <h3 data-key="trekkingTitle">
                            {move || i18n.t("trekkingTitle")}
                        </h3>
                        <p data-key="trekkingDescription">
                            {move || i18n.t("trekkingDescription")}
                        </p>
                    </div>

                    <div class="interest-card">
                        <div class="interest-icon" aria-hidden="true">"🐧"</div>
                        <h3 data-key="linuxTitle">
                            {move || i18n.t("linuxTitle")}
                        </h3>
                        <p data-key="linuxDescription">
                            {move || i18n.t("linuxDescription")}
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
