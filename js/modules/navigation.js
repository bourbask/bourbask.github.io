/**
 * Navigation Manager
 * Handles navigation interactions and scroll effects
 */

import { DesktopNavigation } from "./navigation/desktop-navigation.js";
import { MobileNavigation } from "./navigation/mobile-navigation.js";

class NavigationManager {
  constructor(dependencies = {}) {
    this.cvGenerator = dependencies.cvGenerator;
    this.i18nManager = dependencies.i18nManager;
    this.themeManager = dependencies.themeManager;
  }

  init() {
    this.desktop = new DesktopNavigation();
    this.mobile = new MobileNavigation({
      cvGenerator: this.cvGenerator,
      i18nManager: this.i18nManager,
      themeManager: this.themeManager,
    });

    this.desktop.init();
    this.mobile.init();
  }

  /**
   * Update language for both desktop and mobile navigation
   */
  updateLanguage() {
    if (this.mobile) {
      this.mobile.updateLanguage();
    }
    if (this.desktop) {
      this.desktop.updateLanguage();
    }
  }
}

export default NavigationManager;
