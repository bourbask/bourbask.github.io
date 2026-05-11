pub mod en;
pub mod fr;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    En,
    Fr,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s {
            "fr" => Language::Fr,
            _ => Language::En, // English par défaut
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Fr => "fr",
        }
    }

    pub fn get_translations(&self) -> HashMap<&'static str, &'static str> {
        match self {
            Language::En => en::get_translations(),
            Language::Fr => fr::get_translations(),
        }
    }
}
