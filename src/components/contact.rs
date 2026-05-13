use crate::services::I18nService;
use gloo_net::http::Request;
use js_sys::{Function, Promise, Reflect};
use leptos::*;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::window;

const WORKER_URL: &str = "https://bourbask-contact.bourbask.workers.dev";

// Replace with: gpg --export --armor bourbasquet.k@etik.com
const PGP_PUBLIC_KEY: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZrtlDhYJKwYBBAHaRw8BAQdA93OZ42RBwznbudWHNbCc6QUNqe5X8hqCCQvw
kR2EzKS0KktldmluIEJvdXJiYXNxdWV0IDxib3VyYmFzcXVldC5rQGV0aWsuY29t
PoiTBBMWCgA7FiEEgs9gg2Eb103TFie2R+yHcHWoK7UFAma7ZQ4CGwMFCwkIBwIC
IgIGFQoJCAsCBBYCAwECHgcCF4AACgkQR+yHcHWoK7UVkgD/fWa0kwLoBvgZXz4m
AWkXBwZwfN7pB6L+dRd/LsT/fKoBAPmdSZtpDoGIwESrD8QwNGNDT/Moqbo/6zxr
WuzZ8jgLuDgEZrtlDhIKKwYBBAGXVQEFAQEHQFLgJwdW/munbwI+cxAodBX40cin
za+ds/6xF/bZciB6AwEIB4h4BBgWCgAgFiEEgs9gg2Eb103TFie2R+yHcHWoK7UF
Ama7ZQ4CGwwACgkQR+yHcHWoK7WgFwEAnLsDyrVHQSNpS4fPmk2RTNa1qSqnsFSg
TLqmhbuMJGAA/RiXdfh5ghX4i8q/gCTgTvYcZDIBd3bgVgscBaADQGwC
=hoWZ
-----END PGP PUBLIC KEY BLOCK-----
";

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

async fn encrypt_message(plaintext: &str) -> Result<String, String> {
    let win = window().ok_or("no window")?;
    let encrypt_fn = Reflect::get(&win, &JsValue::from_str("encryptWithPgp"))
        .map_err(|_| "encryptWithPgp not found")?;
    let encrypt_fn: Function = encrypt_fn
        .dyn_into()
        .map_err(|_| "encryptWithPgp is not a function")?;

    let promise = encrypt_fn
        .call2(
            &JsValue::NULL,
            &JsValue::from_str(PGP_PUBLIC_KEY),
            &JsValue::from_str(plaintext),
        )
        .map_err(|_| "encrypt call failed")?;

    let result = JsFuture::from(Promise::from(promise))
        .await
        .map_err(|_| "encryption failed")?;

    result
        .as_string()
        .ok_or_else(|| "encrypted result is not a string".into())
}

#[component]
pub fn ContactSection() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (status, set_status) = create_signal(FormStatus::Idle);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if status.get() == FormStatus::Sending {
            return;
        }

        let name_val = name.get_untracked();
        let email_val = email.get_untracked();
        let message_val = message.get_untracked();

        set_status.set(FormStatus::Sending);

        spawn_local(async move {
            // Build plaintext to encrypt: include sender info + message
            let plaintext = format!("De : {} <{}>\n\n{}", name_val, email_val, message_val);

            let encrypted_message = match encrypt_message(&plaintext).await {
                Ok(c) => c,
                Err(_) => {
                    set_status.set(FormStatus::Error);
                    return;
                }
            };

            let payload = ContactPayload {
                name: name_val,
                email: email_val,
                message: encrypted_message,
            };

            let result = Request::post(WORKER_URL)
                .header("Content-Type", "application/json")
                .json(&payload)
                .map(|req| req.send());

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
                            <div class="contact-method contact-method--email">
                                <a href="mailto:bourbasquet.k@etik.com" class="contact-method-email">
                                    <span class="contact-icon">"📧"</span>
                                    <span data-key="emailLabel">
                                        {move || i18n.t("emailLabel")}
                                    </span>
                                </a>
                                <div class="contact-method-pgp">
                                    <code class="pgp-fingerprint">
                                        "82CF 6083 611B D74D D316  27B6 47EC 8770 75A8 2BB5"
                                    </code>
                                    <div class="pgp-links">
                                        <a
                                            href="/bourbasquet-pgp.asc"
                                            download="bourbasquet-pgp.asc"
                                            class="pgp-link"
                                        >
                                            {move || i18n.t("pgpDownload")}
                                        </a>
                                        <span class="pgp-link-sep">"·"</span>
                                        <a
                                            href="https://keys.openpgp.org/search?q=bourbasquet.k%40etik.com"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="pgp-link"
                                        >
                                            {move || i18n.t("pgpKeyserver")}
                                        </a>
                                    </div>
                                </div>
                            </div>
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
                        <Show when=move || status.get() == FormStatus::Success>
                            <div class="form-feedback form-feedback--success">
                                {move || i18n.t("formSuccess")}
                            </div>
                        </Show>

                        <Show when=move || status.get() == FormStatus::Error>
                            <div class="form-feedback form-feedback--error">
                                {move || i18n.t("formError")}
                            </div>
                        </Show>

                        <form class="contact-form" on:submit=handle_submit>
                            <input
                                type="text"
                                name="name"
                                id="contactName"
                                prop:value=name
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                placeholder={move || i18n.t("nameLabel")}
                                attr:aria-label={move || i18n.t("nameLabel")}
                                required
                                prop:disabled=is_sending
                            />
                            <input
                                type="email"
                                name="email"
                                id="contactEmail"
                                prop:value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                placeholder={move || i18n.t("emailPlaceholder")}
                                attr:aria-label={move || i18n.t("emailPlaceholder")}
                                required
                                prop:disabled=is_sending
                            />
                            <textarea
                                name="message"
                                id="contactMessage"
                                prop:value=message
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                placeholder={move || i18n.t("messageLabel")}
                                attr:aria-label={move || i18n.t("messageLabel")}
                                rows="4"
                                required
                                prop:disabled=is_sending
                            ></textarea>
                            <p class="form-e2e-note">
                                "🔐 "
                                {move || i18n.t("e2eNote")}
                            </p>
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
