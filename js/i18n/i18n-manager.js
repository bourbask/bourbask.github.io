/**
 * Internationalization Manager
 * Handles language switching and text replacement
 */

import { CONFIG } from "../config/settings.js";
import { calculateAge } from "../utils/helpers.js";
import { en } from "./en.js";
import { fr } from "./fr.js";

class I18nManager {
  constructor() {
    this.translations = { en, fr };
    this.currentLang = CONFIG.DEFAULT_LANG;
    this.langToggle = document.getElementById("langToggle");
  }

  /**
   * Initialize i18n system
   */
  init() {
    this.loadLanguage(this.currentLang);
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

    return processed;
  }

  /**
   * Load and apply language
   */
  loadLanguage(lang) {
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
    const newLang = this.currentLang === "en" ? "fr" : "en";
    this.loadLanguage(newLang);
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
}

export default I18nManager;
