use crate::services::I18nService;
use leptos::*;

#[component]
pub fn ContactSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // Logique de soumission du formulaire ici
    };

    view! {
        <section class="section contact" id="contact">
            <div class="container">
                <div class="contact-content">
                    <div class="contact-info">
                        <h2 class="section-title" data-key="contactTitle">
                            {move || i18n.t("contactTitle")}
                        </h2>
                        <p data-key="contactDescription">
                            {move || i18n.t("contactDescription")}
                        </p>

                        <div class="contact-methods">
                            <a href="mailto:contact@bourbask.dev" class="contact-method">
                                <span class="contact-icon">"📧"</span>
                                <span data-key="emailLabel">
                                    {move || i18n.t("emailLabel")}
                                </span>
                            </a>
                            <a
                                href="https://github.com/bourbask"
                                class="contact-method"
                                target="_blank"
                            >
                                <span class="contact-icon">"🐙"</span>
                                <span>"GitHub"</span>
                            </a>
                            <a
                                href="https://www.linkedin.com/in/k%C3%A9vin-bourbasquet"
                                class="contact-method"
                                target="_blank"
                            >
                                <span class="contact-icon">"💼"</span>
                                <span>"LinkedIn"</span>
                            </a>
                        </div>
                    </div>

                    <div class="contact-form-container">
                        <form class="contact-form" on:submit=handle_submit>
                            <input
                                type="text"
                                placeholder={move || i18n.t("nameLabel")}
                                data-placeholder="nameLabel"
                                required
                            />
                            <input
                                type="email"
                                placeholder={move || i18n.t("emailPlaceholder")}
                                data-placeholder="emailPlaceholder"
                                required
                            />
                            <textarea
                                placeholder={move || i18n.t("messageLabel")}
                                data-placeholder="messageLabel"
                                rows="4"
                                required
                            ></textarea>
                            <button
                                type="submit"
                                class="btn btn-primary"
                                data-key="sendMessage"
                            >
                                {move || i18n.t("sendMessage")}
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}
