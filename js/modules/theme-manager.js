/**
 * Theme Manager
 * Handles light/dark theme switching and persistence
 */

import { CONFIG } from "../config/settings.js";

class ThemeManager {
  constructor() {
    this.currentTheme = localStorage.getItem("theme") || CONFIG.DEFAULT_THEME;
    this.themeToggle = document.getElementById("themeToggle");
  }

  /**
   * Initialize theme system
   */
  init() {
    this.applyTheme(this.currentTheme);
    this.setupEventListeners();
  }

  /**
   * Apply theme to document
   */
  applyTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme);
    this.currentTheme = theme;
    localStorage.setItem("theme", theme);
  }

  /**
   * Toggle between light and dark themes
   */
  toggleTheme() {
    const newTheme = this.currentTheme === "light" ? "dark" : "light";
    this.applyTheme(newTheme);
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
}

export default ThemeManager;
