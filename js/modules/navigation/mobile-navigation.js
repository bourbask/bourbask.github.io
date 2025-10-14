/**
 * Mobile FAB Navigation Handler
 */

export class MobileNavigation {
  constructor(dependencies = {}) {
    this.cvGenerator = dependencies.cvGenerator;
    this.i18nManager = dependencies.i18nManager;
    this.themeManager = dependencies.themeManager;

    this.isOpen = false;
    this.isDragging = false;
    this.dragTarget = null;
  }

  init() {
    this.bindFAB();
    this.bindNavItems();
    this.updateBlogNavigation();
    this.updateMobileLabels();
  }

  bindFAB() {
    const fab = document.getElementById("mobileFab");
    const navItems = document.getElementById("mobileNavItems");
    const overlay = document.getElementById("mobileNavOverlay");

    if (!fab || !navItems || !overlay) return;

    // Disable context menu
    [fab, ...navItems.querySelectorAll(".mobile-nav-item")].forEach(
      (element) => {
        element.addEventListener("contextmenu", (e) => e.preventDefault());
        element.addEventListener("selectstart", (e) => e.preventDefault());
      }
    );

    // Simple toggle for tap
    fab.addEventListener("click", (e) => {
      if (e.detail !== 0) {
        // Not programmatic
        this.toggleMenu();
      }
    });

    // Touch handling for drag
    this.setupTouchHandling(fab, navItems);

    // Close on overlay
    overlay.addEventListener("click", () => this.closeMenu());

    this.fab = fab;
    this.navItems = navItems;
    this.overlay = overlay;
  }

  setupTouchHandling(fab, navItems) {
    let startX,
      startY,
      startTime,
      longPressTimer,
      hasMoved = false;

    fab.addEventListener(
      "touchstart",
      (e) => {
        e.preventDefault();

        startX = e.touches[0].clientX;
        startY = e.touches[0].clientY;
        startTime = Date.now();
        hasMoved = false;

        fab.classList.add("touched");
        setTimeout(() => fab.classList.remove("touched"), 200);

        longPressTimer = setTimeout(() => {
          if (!hasMoved && !this.isOpen) {
            this.openMenu();
            this.isDragging = true;
            if (navigator.vibrate) navigator.vibrate([50, 50, 100]);
          }
        }, 400);
      },
      { passive: false }
    );

    fab.addEventListener(
      "touchmove",
      (e) => {
        e.preventDefault();

        const currentX = e.touches[0].clientX;
        const currentY = e.touches[0].clientY;
        const deltaX = Math.abs(startX - currentX);
        const deltaY = Math.abs(startY - currentY);

        if (deltaX > 10 || deltaY > 10) {
          hasMoved = true;
          clearTimeout(longPressTimer);
        }

        if (this.isOpen && this.isDragging) {
          this.updateMagneticEffects(currentX, currentY);
        }
      },
      { passive: false }
    );

    fab.addEventListener(
      "touchend",
      (e) => {
        e.preventDefault();
        clearTimeout(longPressTimer);

        if (this.isDragging && this.dragTarget) {
          this.triggerAction(this.dragTarget);
        } else if (!hasMoved && !this.isDragging) {
          this.toggleMenu();
        }

        this.resetDragState();
      },
      { passive: false }
    );
  }

  updateMagneticEffects(touchX, touchY) {
    let closestItem = null;
    let minDistance = Infinity;

    // Reset all items
    this.navItems.querySelectorAll(".mobile-nav-item").forEach((item) => {
      item.classList.remove("near-target", "ready-trigger");
    });
    this.fab.classList.remove("near-target", "ready-trigger");

    // Find closest item
    this.navItems.querySelectorAll(".mobile-nav-item").forEach((item) => {
      const rect = item.getBoundingClientRect();
      const itemX = rect.left + rect.width / 2;
      const itemY = rect.top + rect.height / 2;

      const distance = Math.sqrt(
        Math.pow(touchX - itemX, 2) + Math.pow(touchY - itemY, 2)
      );

      if (distance < 80 && distance < minDistance) {
        minDistance = distance;
        closestItem = item;
      }
    });

    // Apply effects
    if (closestItem) {
      if (minDistance < 50) {
        // Ready to trigger
        closestItem.classList.add("ready-trigger");
        this.fab.classList.add("ready-trigger");
        this.dragTarget = closestItem;
        if (navigator.vibrate) navigator.vibrate(80);
      } else {
        // Near target
        closestItem.classList.add("near-target");
        this.fab.classList.add("near-target");
        this.dragTarget = null;
        if (navigator.vibrate) navigator.vibrate(30);
      }
    } else {
      this.dragTarget = null;
    }
  }

  toggleMenu() {
    this.isOpen ? this.closeMenu() : this.openMenu();
  }

  openMenu() {
    this.isOpen = true;
    this.fab.classList.add("active");
    this.navItems.classList.add("active");
    this.overlay.classList.add("active");
    document.body.classList.add("mobile-nav-open");

    if (navigator.vibrate) navigator.vibrate(50);
  }

  closeMenu() {
    this.isOpen = false;
    this.fab.classList.remove("active");
    this.navItems.classList.remove("active");
    this.overlay.classList.remove("active");
    document.body.classList.remove("mobile-nav-open");

    this.resetDragState();
  }

  resetDragState() {
    this.isDragging = false;
    this.dragTarget = null;

    this.fab.classList.remove("near-target", "ready-trigger", "dragging");
    this.navItems.querySelectorAll(".mobile-nav-item").forEach((item) => {
      item.classList.remove("near-target", "ready-trigger", "activated");
    });
  }

  triggerAction(target) {
    target.classList.add("activated");
    if (navigator.vibrate) navigator.vibrate([100, 50, 150]);

    setTimeout(() => {
      target.click();
      this.closeMenu();
    }, 100);
  }

  bindNavItems() {
    // Blog
    document.getElementById("mobileBlogBtn")?.addEventListener("click", (e) => {
      e.preventDefault();
      if (window.location.pathname.includes("/blog/")) {
        window.location.href = "/";
      } else {
        window.location.href = "/blog/";
      }
    });

    // CV Download
    document
      .getElementById("mobileCVBtn")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();

        try {
          if (this.cvGenerator) {
            await this.cvGenerator.generatePDF();
          } else {
            console.error("❌ CVGenerator not available");
          }
        } catch (error) {
          console.error("❌ Mobile CV generation error:", error);
        }
      });

    // Language Toggle
    document.getElementById("mobileLangBtn")?.addEventListener("click", (e) => {
      e.preventDefault();
      document.getElementById("langToggle")?.click();
      this.updateMobileLangDisplay();
    });

    // Theme Toggle
    document
      .getElementById("mobileThemeBtn")
      ?.addEventListener("click", (e) => {
        e.preventDefault();
        document.getElementById("themeToggle")?.click();
      });
  }

  updateBlogNavigation() {
    const blogBtn = document.getElementById("mobileBlogBtn");
    if (!blogBtn) return;

    if (window.location.pathname.includes("/blog/")) {
      blogBtn.innerHTML = "🏠";
      blogBtn.setAttribute("data-label", "Home");
    } else {
      blogBtn.innerHTML = "📝";
      blogBtn.setAttribute("data-label", "Blog");
    }
  }

  updateMobileLangDisplay() {
    const mobileLangFlag = document.querySelector(".mobile-lang-flag");
    const currentLang = document
      .querySelector(".lang-option.active")
      ?.getAttribute("data-lang");

    if (mobileLangFlag && currentLang) {
      mobileLangFlag.textContent = currentLang === "en" ? "🇺🇸" : "🇫🇷";
    }
  }
  /**
   * Update all mobile navigation labels with current language
   */
  updateMobileLabels() {
    // Force récupération de la langue actuelle
    const currentLang = this.i18nManager.currentLang;

    // Blog button
    const blogBtn = document.getElementById("mobileBlogBtn");
    if (blogBtn) {
      if (window.location.pathname.includes("/blog/")) {
        const homeLabel = this.i18nManager.t("navigation.home");
        blogBtn.setAttribute("data-label", homeLabel);
      } else {
        const blogLabel = this.i18nManager.t("navigation.blog");
        blogBtn.setAttribute("data-label", blogLabel);
      }
    }

    // CV button
    const cvBtn = document.getElementById("mobileCVBtn");
    if (cvBtn) {
      const cvLabel = this.i18nManager.t("navigation.cv");
      cvBtn.setAttribute("data-label", cvLabel);
    }

    // Language button
    const langBtn = document.getElementById("mobileLangBtn");
    if (langBtn) {
      const langLabel = this.i18nManager.t("navigation.language");
      langBtn.setAttribute("data-label", langLabel);
    }

    // Theme button
    const themeBtn = document.getElementById("mobileThemeBtn");
    if (themeBtn) {
      const themeLabel = this.i18nManager.t("navigation.theme");
      themeBtn.setAttribute("data-label", themeLabel);
    }
  }

  /**
   * Update language when changed
   */
  updateLanguage() {
    if (!this.i18nManager) return;

    // Update all mobile navigation labels
    this.updateMobileLabels();
    this.updateBlogNavigation();
    this.updateMobileLangDisplay();
  }
}
