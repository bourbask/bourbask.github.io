/**
 * Navigation Manager
 * Handles navigation interactions and scroll effects
 */

import { CONFIG } from "../config/settings.js";
import { scrollToSection, debounce } from "../utils/helpers.js";

class NavigationManager {
  constructor() {
    this.nav = document.querySelector(".nav");
  }

  /**
   * Initialize navigation system
   */
  init() {
    this.setupScrollEffects();
    this.setupClickHandlers();
  }

  /**
   * Setup navigation scroll effects
   */
  setupScrollEffects() {
    const handleScroll = debounce(() => {
      if (window.scrollY > CONFIG.NAV_SCROLL_THRESHOLD) {
        this.nav.style.background = "rgba(var(--bg-primary-rgb), 0.95)";
      } else {
        this.nav.style.background = "var(--bg-primary)";
      }
    }, 10);

    window.addEventListener("scroll", handleScroll);
  }

  /**
   * Setup click handlers for smooth scrolling
   */
  setupClickHandlers() {
    document.addEventListener("click", (e) => {
      if (e.target.hasAttribute("onclick")) {
        e.preventDefault();
        const targetId = e.target.getAttribute("onclick").match(/'([^']+)'/)[1];
        scrollToSection(targetId);
      }
    });
  }
}

export default NavigationManager;
