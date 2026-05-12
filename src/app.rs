use crate::components::blog::BlogPage;
use crate::components::*;
use crate::services::{BlogService, I18nService, ThemeService};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let i18n = I18nService::new();
    let theme = ThemeService::new();
    let blog = BlogService::new();

    provide_context(i18n.clone());
    provide_context(theme.clone());
    provide_context(blog.clone());

    view! {
        <Router>
            <div class="app">
                <main class="main-content">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/blog" view=BlogPage/>
                        <Route path="/veille" view=VeillePage/>
                        <Route path="/*any" view=NotFound404/>
                    </Routes>
                </main>
                <MobileFloatingNav />
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <>
            <Navigation />
            <HeroSection />
            <AboutSection />
            <SkillsSection />
            <ProjectsSection />
            <InterestsSection />
            <ContactSection />
            <Footer />
        </>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <>
            <Navigation />
            <NotFound404 />
        </>
    }
}
