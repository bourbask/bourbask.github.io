use crate::services::I18nService;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    let i18n = use_context::<I18nService>().expect("I18n service not found");

    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-content">
                    <p>
                        <span data-key="madeWith">
                            {move || i18n.t("madeWith")}
                        </span>
                        " ❤️ "
                        <span data-key="by">
                            {move || i18n.t("by")}
                        </span>
                        " Kévin Bourbasquet"
                    </p>
                    <p class="footer-links">
                        <a
                            href="https://github.com/bourbask/bourbask.github.io"
                            target="_blank"
                            data-key="sourceCode"
                        >
                            {move || i18n.t("sourceCode")}
                        </a>
                        " • "
                        <a
                            href="https://github.com/bourbask/bourbask.github.io/blob/main/LICENSE"
                            target="_blank"
                        >
                            "MIT License"
                        </a>
                    </p>
                </div>
            </div>
        </footer>
    }
}
