use crate::services::StorageService;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeService {
    pub current_theme: RwSignal<Theme>,
    storage: StorageService,
}

impl ThemeService {
    pub fn new() -> Self {
        let storage = StorageService::new();

        // Détecter le thème initial
        let initial_theme = storage
            .get_theme()
            .and_then(|theme_str| Theme::from_str(&theme_str))
            .unwrap_or_else(Self::detect_system_theme);

        // Sauvegarder le thème détecté
        storage.set_theme(initial_theme.as_str());

        let current_theme = RwSignal::new(initial_theme);

        // Appliquer le thème au DOM
        Self::apply_theme_to_dom(initial_theme);

        Self {
            current_theme,
            storage,
        }
    }

    /// Détecte le thème système
    fn detect_system_theme() -> Theme {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
                if media_query.matches() {
                    return Theme::Dark;
                }
            }
        }

        Theme::Light // Fallback
    }

    /// Applique le thème au DOM
    fn apply_theme_to_dom(theme: Theme) {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let class_list = html.class_list();

                // Supprimer les anciens thèmes
                let _ = class_list.remove_1("light-theme");
                let _ = class_list.remove_1("dark-theme");

                // Ajouter le nouveau thème
                let _ = class_list.add_1(&format!("{}-theme", theme.as_str()));

                // Ajouter l'attribut data-theme pour CSS
                let _ = html.set_attribute("data-theme", theme.as_str());
            }
        }
    }

    /// Toggle entre light/dark
    pub fn toggle_theme(&self) {
        let new_theme = self.current_theme.get().opposite();
        self.set_theme(new_theme);
    }

    /// Définit un thème spécifique
    pub fn set_theme(&self, theme: Theme) {
        self.current_theme.set(theme);
        self.storage.set_theme(theme.as_str());
        Self::apply_theme_to_dom(theme);

        log::info!("🎨 Theme switched to: {:?}", theme);
    }

}
