/**
 * Desktop Navigation Handler
 */

import { CONFIG } from "../../config/settings.js";
import { scrollToSection, debounce } from "../../utils/helpers.js";

export class DesktopNavigation {
  constructor() {
    this.nav = document.querySelector(".nav");
  }

  init() {
    this.setupScrollEffects();
    this.setupClickHandlers();
    this.bindScrollEvents();
    this.bindNavigationClicks();
  }

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

  setupClickHandlers() {
    document.addEventListener("click", (e) => {
      if (e.target.hasAttribute("onclick")) {
        e.preventDefault();
        const targetId = e.target.getAttribute("onclick").match(/'([^']+)'/)[1];
        scrollToSection(targetId);
      }
    });
  }

  bindScrollEvents() {
    window.addEventListener("scroll", () => {
      if (window.scrollY > 50) {
        document.querySelector(".nav").classList.add("scrolled");
      } else {
        document.querySelector(".nav").classList.remove("scrolled");
      }
    });
  }

  bindNavigationClicks() {
    document.addEventListener("click", (e) => {
      const link = e.target.closest("a[href^='#']");
      if (!link) return;

      e.preventDefault();
      const targetId = link.getAttribute("href").substring(1);
      const targetElement = document.getElementById(targetId);

      if (targetElement) {
        this.scrollToSection(targetElement);
      }
    });
  }

  scrollToSection(element) {
    const navbarHeight = 70;
    const elementPosition = element.getBoundingClientRect().top;
    const offsetPosition = elementPosition + window.pageYOffset - navbarHeight;

    window.scrollTo({
      top: offsetPosition,
      behavior: "smooth",
    });
  }

  /**
   * Update language when changed
   */
  updateLanguage() {}
}
