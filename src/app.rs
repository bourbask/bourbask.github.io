use crate::components::blog::BlogPage;
use crate::components::*;
use crate::services::{BlogService, I18nService, ThemeService};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

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
                    <Routes fallback=|| view! { <NotFound404/> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("/blog") view=BlogPage/>
                        <Route path=StaticSegment("/veille") view=VeillePage/>
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
