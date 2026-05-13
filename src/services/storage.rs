use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, Debug)]
pub struct StorageService;

impl StorageService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_language(&self) -> Option<String> {
        LocalStorage::get("portfolio_language").ok()
    }

    pub fn set_language(&self, lang: &str) {
        let _ = LocalStorage::set("portfolio_language", lang);
    }

    pub fn get_theme(&self) -> Option<String> {
        LocalStorage::get("portfolio_theme").ok()
    }

    pub fn set_theme(&self, theme: &str) {
        let _ = LocalStorage::set("portfolio_theme", theme);
    }

}
