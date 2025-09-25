/**
 * Animation Manager
 * Handles scroll animations and intersection observers
 */

import { CONFIG } from "../config/settings.js";

class AnimationManager {
  constructor() {
    this.observerOptions = {
      threshold: CONFIG.OBSERVER_THRESHOLD,
      rootMargin: CONFIG.OBSERVER_ROOT_MARGIN,
    };
  }

  /**
   * Initialize animations
   */
  init() {
    this.setupIntersectionObserver();
  }

  /**
   * Setup intersection observer for scroll animations
   */
  setupIntersectionObserver() {
    const observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.style.opacity = "1";
          entry.target.style.transform = "translateY(0)";
        }
      });
    }, this.observerOptions);

    // Observe all sections
    const sections = document.querySelectorAll(".section");
    sections.forEach((section) => observer.observe(section));
  }
}

export default AnimationManager;
