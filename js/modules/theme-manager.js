/**
 * Theme Manager
 * Handles light/dark theme switching with browser preference detection
 */

import { CONFIG } from "../config/settings.js";

class ThemeManager {
  constructor() {
    this.themeToggle = document.getElementById("themeToggle");
    this.mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    // Detect initial theme from browser/localStorage
    this.currentTheme = this.detectInitialTheme();
  }

  /**
   * Detect initial theme preference
   */
  detectInitialTheme() {
    // 1. Check localStorage first (user explicitly chose)
    const savedTheme = localStorage.getItem("theme");
    if (savedTheme && (savedTheme === "light" || savedTheme === "dark")) {
      return savedTheme;
    }

    // 2. Check browser preference
    if (
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches
    ) {
      return "dark";
    } else if (
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: light)").matches
    ) {
      return "light";
    }

    // 3. Fallback to project default
    return CONFIG.DEFAULT_THEME;
  }

  /**
   * Initialize theme system
   */
  init() {
    this.applyTheme(this.currentTheme);
    this.setupEventListeners();
    this.watchBrowserPreferences();
  }

  /**
   * Watch for browser preference changes
   */
  watchBrowserPreferences() {
    if (this.mediaQuery) {
      this.mediaQuery.addEventListener("change", (e) => {
        // Only auto-switch if user hasn't manually set a preference
        const savedTheme = localStorage.getItem("theme");
        if (!savedTheme) {
          const newTheme = e.matches ? "dark" : "light";
          this.applyTheme(newTheme, false); // false = don't save to localStorage
        }
      });
    }
  }

  /**
   * Apply theme to document
   * @param {string} theme - Theme to apply
   * @param {boolean} saveToStorage - Whether to save the preference
   */
  applyTheme(theme, saveToStorage = true) {
    document.documentElement.setAttribute("data-theme", theme);
    this.currentTheme = theme;

    if (saveToStorage) {
      localStorage.setItem("theme", theme);
    }
  }

  /**
   * Toggle between light and dark themes
   */
  toggleTheme() {
    const newTheme = this.currentTheme === "light" ? "dark" : "light";
    this.applyTheme(newTheme, true); // Always save manual choices
  }

  /**
   * Reset to browser preference
   */
  resetToBrowserPreference() {
    localStorage.removeItem("theme");
    const browserTheme = this.mediaQuery?.matches ? "dark" : "light";
    this.applyTheme(browserTheme, false);
  }

  /**
   * Setup event listeners
   */
  setupEventListeners() {
    if (this.themeToggle) {
      this.themeToggle.addEventListener("click", () => this.toggleTheme());
    }
  }

  /**
   * Get current theme
   */
  getCurrentTheme() {
    return this.currentTheme;
  }

  /**
   * Check if current theme is from browser preference
   */
  isUsingBrowserPreference() {
    return !localStorage.getItem("theme");
  }
}

export default ThemeManager;
