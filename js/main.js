/**
 * Main Application Entry Point
 * Orchestrates all application modules
 */

import I18nManager from "./i18n/i18n-manager.js";
import ThemeManager from "./modules/theme-manager.js";
import ContactFormManager from "./modules/contact-form.js";
import NavigationManager from "./modules/navigation.js";
import AnimationManager from "./modules/animations.js";
import CVGenerator from "./modules/cv-generator.js";

class PortfolioApp {
  constructor() {
    this.i18n = new I18nManager();
    this.theme = new ThemeManager();
    this.contactForm = new ContactFormManager(this.i18n);
    this.navigation = new NavigationManager();
    this.animations = new AnimationManager();
    this.cvGenerator = new CVGenerator(this.i18n);
  }

  /**
   * Initialize the application
   */
  init() {
    // Initialize all modules
    this.theme.init();
    this.i18n.init();
    this.contactForm.init();
    this.navigation.init();
    this.animations.init();
    this.cvGenerator.init();
    this.setupCVDownload();
    this.setupLanguageChangeListener();

    console.log("✅ Portfolio application initialized");
  }

  /**
   * Setup CV download functionality
   */
  setupCVDownload() {
    const downloadBtn = document.getElementById("downloadCV");
    if (downloadBtn) {
      downloadBtn.addEventListener("click", async () => {
        await this.cvGenerator.generatePDF();
      });
    }
  }

  /**
   * Setup language change listener to update CV
   */
  setupLanguageChangeListener() {
    // Store original toggle method
    const originalToggle = this.i18n.toggleLanguage.bind(this.i18n);

    // Override toggle method to update CV language
    this.i18n.toggleLanguage = () => {
      originalToggle();

      // Update CV generator language
      if (this.cvGenerator) {
        this.cvGenerator.updateLanguage();
      }
    };

    // Also listen for direct language changes
    const originalLoadLanguage = this.i18n.loadLanguage.bind(this.i18n);
    this.i18n.loadLanguage = (lang, saveToStorage = true) => {
      originalLoadLanguage(lang, saveToStorage);

      // Update CV generator language
      if (this.cvGenerator) {
        this.cvGenerator.updateLanguage();
      }
    };
  }
}

// Initialize the application when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  const app = new PortfolioApp();
  app.init();
});

// Export for global access if needed
window.PortfolioApp = PortfolioApp;
