// public/js/animations.js - Exactement comme avant !
class AnimationManager {
  constructor() {
    this.observerOptions = {
      threshold: 0.1,
      rootMargin: "0px 0px -10% 0px",
    };
    this.init();
  }

  init() {
    this.setupIntersectionObserver();
  }

  setupIntersectionObserver() {
    const observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.style.opacity = "1";
          entry.target.style.transform = "translateY(0)";
          entry.target.classList.add("animate-in");
        }
      });
    }, this.observerOptions);

    // Observer toutes les sections quand le DOM est prêt
    const observeSections = () => {
      const sections = document.querySelectorAll(
        ".section, .animate-on-scroll"
      );
      sections.forEach((section) => observer.observe(section));
    };

    if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", observeSections);
    } else {
      setTimeout(observeSections, 100);
    }
  }
}

// Initialiser automatiquement
window.animationManager = new AnimationManager();
