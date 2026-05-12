use crate::data::translations::Language;
use chrono::{Datelike, NaiveDate, Utc};
use leptos::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct I18nService {
    pub current_language: RwSignal<Language>,
    translations: RwSignal<HashMap<&'static str, &'static str>>,
    birth_date: NaiveDate,
}

impl I18nService {
    pub fn new() -> Self {
        // Date de naissance pour calculer l'âge
        let birth_date = NaiveDate::from_ymd_opt(1999, 1, 1).expect("Invalid birth date");

        let storage = crate::services::StorageService::new();
        // Priority: 1) persisted user choice, 2) browser language, 3) EN
        let initial_lang = storage
            .get_language()
            .and_then(|s| match s.as_str() {
                "fr" => Some(Language::Fr),
                "en" => Some(Language::En),
                _ => None,
            })
            .unwrap_or_else(|| {
                // Read navigator.language (e.g. "fr", "fr-FR", "fr-BE")
                let browser_lang = web_sys::window()
                    .and_then(|w| w.navigator().language())
                    .unwrap_or_default();
                if browser_lang.starts_with("fr") {
                    Language::Fr
                } else {
                    Language::En
                }
            });

        let current_language = create_rw_signal(initial_lang);
        let translations = create_rw_signal(current_language.get().get_translations());

        Self {
            current_language,
            translations,
            birth_date,
        }
    }

    /// Calculer l'âge actuel
    fn calculate_age(&self) -> u32 {
        let today = Utc::now().date_naive();
        let mut age = today.year() - self.birth_date.year();

        if today.month() < self.birth_date.month()
            || (today.month() == self.birth_date.month() && today.day() < self.birth_date.day())
        {
            age -= 1;
        }

        age as u32
    }

    /// Remplacer les placeholders dans le texte
    fn process_placeholders(&self, text: &str) -> String {
        let age = self.calculate_age();
        text.replace("{age}", &age.to_string())
    }

    /// Obtenir une traduction par clé (avec support dot notation)
    pub fn t(&self, key: &str) -> String {
        self.translations.with(|translations| {
            let raw_text = translations
                .get(key)
                .unwrap_or(&key) // Fallback vers la clé si pas trouvé
                .to_string();

            // Traiter les placeholders
            self.process_placeholders(&raw_text)
        })
    }

    /// Changer de langue
    pub fn set_language(&self, lang: Language) {
        crate::services::StorageService::new().set_language(lang.as_str());
        self.current_language.set(lang.clone());
        self.translations.set(lang.get_translations());
    }

    /// Toggle entre FR/EN (et autres langues à l'avenir)
    pub fn toggle_language(&self) {
        let new_lang = match self.current_language.get() {
            Language::En => Language::Fr,
            Language::Fr => Language::En,
            // Ajouter d'autres langues ici quand nécessaire
        };
        self.set_language(new_lang);
    }

    /// Obtenir le code langue actuel
    pub fn current_lang_code(&self) -> String {
        self.current_language.get().as_str().to_string()
    }

    /// Vérifier si une langue est supportée
    pub fn is_language_supported(&self, lang_code: &str) -> bool {
        matches!(lang_code, "en" | "fr")
    }

    /// Vérifier si la langue courante est l'anglais
    pub fn is_english(&self) -> bool {
        matches!(self.current_language.get(), Language::En)
    }

    /// Vérifier si la langue courante est le français
    pub fn is_french(&self) -> bool {
        matches!(self.current_language.get(), Language::Fr)
    }

    /// Définir la langue depuis un string (utile pour URL params)
    pub fn set_language_from_str(&self, lang_str: &str) {
        let lang = Language::from_str(lang_str);
        self.set_language(lang);
    }
}
