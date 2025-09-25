/**
 * Main Application Entry Point
 * Orchestrates all application modules
 */

import I18nManager from "./i18n/i18n-manager.js";
import ThemeManager from "./modules/theme-manager.js";
import ContactFormManager from "./modules/contact-form.js";
import NavigationManager from "./modules/navigation.js";
import AnimationManager from "./modules/animations.js";

class PortfolioApp {
  constructor() {
    this.i18n = new I18nManager();
    this.theme = new ThemeManager();
    this.contactForm = new ContactFormManager(this.i18n);
    this.navigation = new NavigationManager();
    this.animations = new AnimationManager();
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

    console.log("✅ Portfolio application initialized");
  }
}

// Initialize the application when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  const app = new PortfolioApp();
  app.init();
});

// Export for global access if needed
window.PortfolioApp = PortfolioApp;
