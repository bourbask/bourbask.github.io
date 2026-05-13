use crate::services::I18nService;
use gloo_net::http::Request;
use leptos::*;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

// Create a free form at https://formspree.io and paste the ID here.
const FORMSPREE_ID: &str = "REPLACE_WITH_YOUR_FORMSPREE_ID";

#[derive(Serialize, Clone)]
struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

#[derive(Clone, PartialEq)]
enum FormStatus {
    Idle,
    Sending,
    Success,
    Error,
}

#[component]
pub fn ContactSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    let (name, set_name)       = create_signal(String::new());
    let (email, set_email)     = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (status, set_status)   = create_signal(FormStatus::Idle);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if status.get() == FormStatus::Sending {
            return;
        }

        let payload = ContactPayload {
            name: name.get_untracked(),
            email: email.get_untracked(),
            message: message.get_untracked(),
        };

        let endpoint = format!("https://formspree.io/f/{}", FORMSPREE_ID);
        set_status.set(FormStatus::Sending);

        spawn_local(async move {
            let result = Request::post(&endpoint)
                .header("Accept", "application/json")
                .json(&payload)
                .and_then(|req| Ok(req.send()))
                .map_err(|e| e.to_string());

            match result {
                Ok(fut) => match fut.await {
                    Ok(resp) if resp.ok() => {
                        set_status.set(FormStatus::Success);
                        set_name.set(String::new());
                        set_email.set(String::new());
                        set_message.set(String::new());
                    }
                    _ => set_status.set(FormStatus::Error),
                },
                Err(_) => set_status.set(FormStatus::Error),
            }
        });
    };

    let is_sending = move || status.get() == FormStatus::Sending;

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
                            <a href="mailto:k.bourbasquet@legal2digital.fr" class="contact-method">
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
                        // Success banner
                        <Show when=move || status.get() == FormStatus::Success>
                            <div class="form-feedback form-feedback--success">
                                {move || i18n.t("formSuccess")}
                            </div>
                        </Show>

                        // Error banner
                        <Show when=move || status.get() == FormStatus::Error>
                            <div class="form-feedback form-feedback--error">
                                {move || i18n.t("formError")}
                            </div>
                        </Show>

                        <form
                            class="contact-form"
                            on:submit=handle_submit
                        >
                            <input
                                type="text"
                                name="name"
                                prop:value=name
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                placeholder={move || i18n.t("nameLabel")}
                                required
                                prop:disabled=is_sending
                            />
                            <input
                                type="email"
                                name="email"
                                prop:value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                placeholder={move || i18n.t("emailPlaceholder")}
                                required
                                prop:disabled=is_sending
                            />
                            <textarea
                                name="message"
                                prop:value=message
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                placeholder={move || i18n.t("messageLabel")}
                                rows="4"
                                required
                                prop:disabled=is_sending
                            ></textarea>
                            <button
                                type="submit"
                                class="btn btn-primary"
                                prop:disabled=is_sending
                            >
                                {move || if is_sending() {
                                    i18n.t("formSending")
                                } else {
                                    i18n.t("sendMessage")
                                }}
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}
