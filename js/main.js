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
import BlogManager from "./modules/blog-manager.js";
("./modules/blog-manager.js");

class PortfolioApp {
  constructor() {
    this.i18n = new I18nManager();
    this.theme = new ThemeManager();
    this.contactForm = new ContactFormManager(this.i18n);
    this.navigation = new NavigationManager();
    this.animations = new AnimationManager();
    this.cvGenerator = new CVGenerator(this.i18n);
    this.blogManager = new BlogManager(this.i18n);
  }

  /**
   * Initialize the application
   */
  init() {
    // Initialize all modules
    this.theme.init();
    this.i18n.init();
    this.navigation.init();
    this.cvGenerator.init();

    // Initialize contact form only on main site (not needed in blog)
    if (!window.location.pathname.startsWith("/blog/")) {
      this.contactForm.init();
      this.animations.init();
    }

    // Initialize blog if on blog pages
    if (window.location.pathname.includes("/blog")) {
      this.blogManager.init();
    }

    this.setupCVDownload();
    this.setupLanguageChangeListener();
    this.setupProjectBlogNavigation();
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
      // Update Blog pages language
      if (this.blogManager) {
        this.blogManager.updateLanguage();
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

  /**
   * Setup navigation from projects to blog
   */
  setupProjectBlogNavigation() {
    document.addEventListener("click", (e) => {
      const blogLink = e.target.closest('a[href^="/blog/"]');
      if (blogLink && !window.location.pathname.includes("/blog")) {
        // Navigation from main site to blog
        window.location.href = blogLink.href;
      }
    });
  }
}

// Initialize the application when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  const app = new PortfolioApp();
  app.init();
});

// Export for global access if needed
window.PortfolioApp = PortfolioApp;
