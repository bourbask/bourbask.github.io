/**
 * Internationalization Manager
 * Handles language switching with browser language detection
 */

import { CONFIG } from "../config/settings.js";
import { calculateAge } from "../utils/helpers.js";
import { en } from "./en.js";
import { fr } from "./fr.js";

class I18nManager {
  constructor() {
    this.translations = { en, fr };
    this.supportedLanguages = Object.keys(this.translations);
    this.langToggle = document.getElementById("langToggle");

    // Detect initial language from browser/localStorage
    this.currentLang = this.detectInitialLanguage();
  }

  /**
   * Detect initial language preference
   */
  detectInitialLanguage() {
    // 1. Check localStorage first (user explicitly chose)
    const savedLang = localStorage.getItem("language");
    if (savedLang && this.supportedLanguages.includes(savedLang)) {
      return savedLang;
    }

    // 2. Check browser language preferences
    const browserLanguages = this.getBrowserLanguages();

    for (const browserLang of browserLanguages) {
      // Check exact match first (e.g., 'fr')
      if (this.supportedLanguages.includes(browserLang)) {
        return browserLang;
      }

      // Check language prefix (e.g., 'fr-FR' -> 'fr')
      const langPrefix = browserLang.split("-")[0];
      if (this.supportedLanguages.includes(langPrefix)) {
        return langPrefix;
      }
    }

    // 3. Fallback to project default
    return CONFIG.DEFAULT_LANG;
  }

  /**
   * Get browser languages in order of preference
   */
  getBrowserLanguages() {
    const languages = [];

    // Modern browsers - multiple languages
    if (navigator.languages && navigator.languages.length > 0) {
      languages.push(...navigator.languages);
    }

    // Fallback - single language
    if (navigator.language) {
      languages.push(navigator.language);
    }

    // Legacy fallback
    if (navigator.userLanguage) {
      languages.push(navigator.userLanguage);
    }

    return [...new Set(languages)]; // Remove duplicates
  }

  /**
   * Initialize i18n system
   */
  init() {
    this.loadLanguage(this.currentLang, false); // Don't save initial detection
    this.setupEventListeners();
  }

  /**
   * Process translations with dynamic values
   */
  processTranslations(translations) {
    const age = calculateAge(CONFIG.BIRTH_DATE);
    const processed = { ...translations };

    // Replace {age} placeholder in aboutText1
    if (processed.aboutText1) {
      processed.aboutText1 = processed.aboutText1.replace("{age}", age);
    }

    // Replace {age} placeholder in summary
    if (processed.summary) {
      processed.summary = processed.summary.replace("{age}", age);
    }

    return processed;
  }

  /**
   * Load and apply language
   * @param {string} lang - Language code to load
   * @param {boolean} saveToStorage - Whether to save the preference
   */
  loadLanguage(lang, saveToStorage = true) {
    if (!this.translations[lang]) {
      console.warn(
        `Language ${lang} not found, falling back to ${CONFIG.DEFAULT_LANG}`
      );
      lang = CONFIG.DEFAULT_LANG;
    }

    const processedTranslations = this.processTranslations(
      this.translations[lang]
    );

    // Update text content
    this.updateTextContent(processedTranslations);

    // Update placeholders
    this.updatePlaceholders(processedTranslations);

    // Update UI state
    this.updateLangToggleUI(lang);

    this.currentLang = lang;

    if (saveToStorage) {
      localStorage.setItem("language", lang);
    }
  }

  /**
   * Update text content for elements with data-key
   */
  updateTextContent(translations) {
    const elements = document.querySelectorAll("[data-key]");
    elements.forEach((element) => {
      const key = element.getAttribute("data-key");
      if (translations[key]) {
        element.textContent = translations[key];
      }
    });
  }

  /**
   * Update placeholders for form elements
   */
  updatePlaceholders(translations) {
    const placeholderElements = document.querySelectorAll("[data-placeholder]");
    placeholderElements.forEach((element) => {
      const key = element.getAttribute("data-placeholder");
      if (translations[key]) {
        element.placeholder = translations[key];
      }
    });
  }

  /**
   * Update language toggle UI
   */
  updateLangToggleUI(lang) {
    if (!this.langToggle) return;

    const langOptions = this.langToggle.querySelectorAll(".lang-option");
    langOptions.forEach((option) => {
      option.classList.remove("active");
      if (option.getAttribute("data-lang") === lang) {
        option.classList.add("active");
      }
    });

    this.langToggle.setAttribute("data-lang", lang);
  }

  /**
   * Toggle between languages
   */
  toggleLanguage() {
    const availableLangs = this.supportedLanguages;
    const currentIndex = availableLangs.indexOf(this.currentLang);
    const nextIndex = (currentIndex + 1) % availableLangs.length;
    const newLang = availableLangs[nextIndex];

    this.loadLanguage(newLang, true); // Always save manual choices
  }

  /**
   * Reset to browser preference
   */
  resetToBrowserPreference() {
    localStorage.removeItem("language");
    const browserLang = this.detectInitialLanguage();
    this.loadLanguage(browserLang, false);
  }

  /**
   * Get current translation
   */
  t(key) {
    const processedTranslations = this.processTranslations(
      this.translations[this.currentLang]
    );
    return processedTranslations[key] || key;
  }

  /**
   * Setup event listeners
   */
  setupEventListeners() {
    if (this.langToggle) {
      this.langToggle.addEventListener("click", () => this.toggleLanguage());
    }
  }

  /**
   * Check if current language is from browser preference
   */
  isUsingBrowserPreference() {
    return !localStorage.getItem("language");
  }

  /**
   * Get supported languages
   */
  getSupportedLanguages() {
    return this.supportedLanguages;
  }
}

export default I18nManager;
