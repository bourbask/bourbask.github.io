use crate::services::{CVService, I18nService};
use leptos::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn CVDownloadButton() -> impl IntoView {
    let i18n = expect_context::<I18nService>();
    let cv_service = CVService::new(i18n.clone());

    let (is_downloading, set_downloading) = create_signal(false);
    let (is_success, set_success) = create_signal(false);

    let handle_download = move |_| {
        if is_downloading.get() {
            return;
        }

        set_downloading.set(true);
        cv_service.generate_pdf();

        set_downloading.set(false);
        set_success.set(true);

        // Réinitialiser après 2 secondes
        let window = web_sys::window().unwrap();
        let set_success_reset = set_success.clone();
        let closure = Closure::wrap(Box::new(move || {
            set_success_reset.set(false);
        }) as Box<dyn FnMut()>);

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(), // ✅ Maintenant ça marche
                2000,
            )
            .ok();

        closure.forget();
    };

    view! {
        // ✅ Reprendre le design original avec l'icône SVG
        <div class="cv-download-wrapper">
            <button
                id="downloadCV"
                class={move || {
                    let mut class = "cv-download-btn".to_string();
                    if is_downloading.get() { class.push_str(" downloading"); }
                    if is_success.get() { class.push_str(" success"); }
                    class
                }}
                aria-label="Download CV"
                disabled=move || is_downloading.get()
                on:click=handle_download
            >
                <svg
                    class="download-icon"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                >
                    <path
                        class="download-arrow"
                        d="M12 15L8 11H11V3H13V11H16L12 15Z"
                        fill="currentColor"
                    />
                    <path
                        class="download-base"
                        d="M20 16V20C20 21.1 19.1 22 18 22H6C4.9 22 4 21.1 4 20V16H6V20H18V16H20Z"
                        fill="currentColor"
                    />
                    <div class="download-fill"></div>
                </svg>
            </button>
            <div class="cv-tooltip">
                <span data-key="action">
                    {move || if is_downloading.get() {
                        i18n.t("cv.downloading")
                    } else if is_success.get() {
                        i18n.t("cv.downloaded")
                    } else {
                        i18n.t("action")
                    }}
                </span>
            </div>
        </div>
    }
}
