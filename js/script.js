// Translations object (embedded to avoid CORS issues)
const translations = {
  en: {
    name: "Kévin Bourbasquet",
    badge: "Available for work",
    heroTitle1: "Building digital",
    heroTitle2: "experiences",
    heroDescription:
      "Full-stack developer passionate about creating scalable web applications, exploring DevOps practices, and crafting unique digital experiences.",
    viewWork: "View My Work",
    getInTouch: "Get In Touch",
    aboutTitle: "About Me",
    aboutSubtitle: "Developer, creator, and technology enthusiast",
    aboutText1:
      "I'm a 26-year-old full-stack developer with a passion for building scalable web applications using modern technologies. My expertise lies in creating microservice architectures with Symfony API Platform and crafting intuitive user interfaces with Vue.js and React.js.",
    aboutText2:
      "Beyond coding, I'm deeply interested in DevOps practices, from CI/CD pipelines to infrastructure automation with tools like OpenTofu, Ansible, and Docker. I believe in the power of technology to solve real-world problems.",
    yearsExperience: "Years Experience",
    projectsCompleted: "Projects Completed",
    technologiesMastered: "Technologies Mastered",
    skillsTitle: "Skills & Technologies",
    skillsSubtitle: "Tools and technologies I work with",
    frontendTitle: "Frontend",
    backendTitle: "Backend",
    devopsTitle: "DevOps",
    toolsTitle: "Tools",
    projectsTitle: "Featured Projects",
    projectsSubtitle: "Some of my recent work and side projects",
    ezprint3dDescription:
      "A platform connecting 3D printing service providers with customers. Built with modern microservices architecture and focuses on user experience.",
    lekickerfouDescription:
      "Discord bot for automated voice channel management. Built with Rust for performance and reliability.",
    keyboardDescription:
      "Hand-built mechanical keyboards including a Corne 3x6. Exploring ergonomic designs and custom firmware.",
    interestsTitle: "Beyond Code",
    interestsSubtitle: "Creative pursuits and hobbies that inspire my work",
    photographyTitle: "Photography",
    photographyDescription:
      "Capturing moments and perspectives through the lens, exploring composition and storytelling.",
    musicTitle: "Music",
    musicDescription:
      "Playing guitar and piano, creating melodies and exploring different musical styles and techniques.",
    woodworkingTitle: "Woodworking",
    woodworkingDescription:
      "Crafting functional and artistic pieces from wood, combining traditional techniques with modern tools.",
    linuxTitle: "Linux Enthusiast",
    linuxDescription:
      "Daily Manjaro user, exploring system optimization and contributing to open-source projects.",
    contactTitle: "Let's Work Together",
    contactDescription:
      "I'm always interested in new opportunities and collaborations. Whether you have a project in mind or just want to chat about technology, feel free to reach out.",
    emailLabel: "Email",
    nameLabel: "Your Name",
    emailPlaceholder: "Your Email",
    messageLabel: "Your Message",
    sendMessage: "Send Message",
    madeWith: "Made with",
    by: "by",
    sourceCode: "Source Code",
  },
  fr: {
    name: "Kévin Bourbasquet",
    badge: "Disponible pour travailler",
    heroTitle1: "Créer des expériences",
    heroTitle2: "numériques",
    heroDescription:
      "Développeur full-stack passionné par la création d'applications web évolutives, l'exploration des pratiques DevOps et la création d'expériences numériques uniques.",
    viewWork: "Voir mon travail",
    getInTouch: "Me contacter",
    aboutTitle: "À propos de moi",
    aboutSubtitle: "Développeur, créateur et passionné de technologie",
    aboutText1:
      "Je suis un développeur full-stack de 26 ans passionné par la création d'applications web évolutives utilisant des technologies modernes. Mon expertise réside dans la création d'architectures de microservices avec Symfony API Platform et la création d'interfaces utilisateur intuitives avec Vue.js et React.js.",
    aboutText2:
      "Au-delà du codage, je m'intéresse profondément aux pratiques DevOps, des pipelines CI/CD à l'automatisation d'infrastructure avec des outils comme OpenTofu, Ansible et Docker. Je crois au pouvoir de la technologie pour résoudre des problèmes du monde réel.",
    yearsExperience: "Années d'expérience",
    projectsCompleted: "Projets terminés",
    technologiesMastered: "Technologies maîtrisées",
    skillsTitle: "Compétences et technologies",
    skillsSubtitle: "Outils et technologies avec lesquels je travaille",
    frontendTitle: "Frontend",
    backendTitle: "Backend",
    devopsTitle: "DevOps",
    toolsTitle: "Outils",
    projectsTitle: "Projets phares",
    projectsSubtitle:
      "Quelques-uns de mes travaux récents et projets personnels",
    ezprint3dDescription:
      "Une plateforme connectant les fournisseurs de services d'impression 3D avec les clients. Construite avec une architecture de microservices moderne et se concentre sur l'expérience utilisateur.",
    lekickerfouDescription:
      "Bot Discord pour la gestion automatisée des canaux vocaux. Construit avec Rust pour les performances et la fiabilité.",
    keyboardDescription:
      "Claviers mécaniques construits à la main, y compris un Corne 3x6. Exploration de designs ergonomiques et de firmware personnalisé.",
    interestsTitle: "Au-delà du code",
    interestsSubtitle:
      "Activités créatives et hobbies qui inspirent mon travail",
    photographyTitle: "Photographie",
    photographyDescription:
      "Capturer des moments et des perspectives à travers l'objectif, explorer la composition et la narration.",
    musicTitle: "Musique",
    musicDescription:
      "Jouer de la guitare et du piano, créer des mélodies et explorer différents styles et techniques musicales.",
    woodworkingTitle: "Menuiserie",
    woodworkingDescription:
      "Créer des pièces fonctionnelles et artistiques en bois, combinant techniques traditionnelles et outils modernes.",
    linuxTitle: "Passionné Linux",
    linuxDescription:
      "Utilisateur quotidien de Manjaro, exploration de l'optimisation système et contribution à des projets open-source.",
    contactTitle: "Travaillons ensemble",
    contactDescription:
      "Je suis toujours intéressé par de nouvelles opportunités et collaborations. Que vous ayez un projet en tête ou que vous vouliez simplement discuter de technologie, n'hésitez pas à me contacter.",
    emailLabel: "Email",
    nameLabel: "Votre nom",
    emailPlaceholder: "Votre email",
    messageLabel: "Votre message",
    sendMessage: "Envoyer le message",
    madeWith: "Fait avec",
    by: "par",
    sourceCode: "Code source",
  },
};

// State management
let currentLang = "en";
let currentTheme = localStorage.getItem("theme") || "light";

// DOM elements
const langToggle = document.getElementById("langToggle");
const themeToggle = document.getElementById("themeToggle");

function calculateAge(birthDate) {
  const today = new Date();
  const birth = new Date(birthDate);
  let age = today.getFullYear() - birth.getFullYear();
  const monthDiff = today.getMonth() - birth.getMonth();

  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birth.getDate())) {
    age--;
  }

  return age;
}

function updateAgeInTranslations() {
  const age = calculateAge("1989-06-18");

  // Mettre à jour les traductions
  translations.en.aboutText1 = `I'm a ${age}-year-old full-stack developer with a passion for building scalable web applications using modern technologies. My expertise lies in creating microservice architectures with Symfony API Platform and crafting intuitive user interfaces with Vue.js and React.js.`;

  translations.fr.aboutText1 = `Je suis un développeur full-stack de ${age} ans passionné par la création d'applications web évolutives utilisant des technologies modernes. Mon expertise réside dans la création d'architectures de microservices avec Symfony API Platform et la création d'interfaces utilisateur intuitives avec Vue.js et React.js.`;
}

// Initialize on page load
document.addEventListener("DOMContentLoaded", function () {
  updateAgeInTranslations();

  // Set initial theme
  document.documentElement.setAttribute("data-theme", currentTheme);

  // Load initial language
  loadLanguage(currentLang);

  // Set up event listeners
  setupEventListeners();

  // Initialize form
  setupContactForm();
});

// Language switching
function loadLanguage(lang) {
  const elements = document.querySelectorAll("[data-key]");
  const placeholderElements = document.querySelectorAll("[data-placeholder]");

  elements.forEach((element) => {
    const key = element.getAttribute("data-key");
    if (translations[lang] && translations[lang][key]) {
      element.textContent = translations[lang][key];
    }
  });

  placeholderElements.forEach((element) => {
    const key = element.getAttribute("data-placeholder");
    if (translations[lang] && translations[lang][key]) {
      element.placeholder = translations[lang][key];
    }
  });

  // Update lang toggle appearance
  const langOptions = langToggle.querySelectorAll(".lang-option");
  langOptions.forEach((option) => {
    option.classList.remove("active");
    if (option.getAttribute("data-lang") === lang) {
      option.classList.add("active");
    }
  });

  langToggle.setAttribute("data-lang", lang);
  currentLang = lang;
}

// Theme switching
function toggleTheme() {
  currentTheme = currentTheme === "light" ? "dark" : "light";
  document.documentElement.setAttribute("data-theme", currentTheme);
  localStorage.setItem("theme", currentTheme);
}

// Event listeners
function setupEventListeners() {
  // Language toggle
  langToggle.addEventListener("click", function () {
    const newLang = currentLang === "en" ? "fr" : "en";
    loadLanguage(newLang);
  });

  // Theme toggle
  themeToggle.addEventListener("click", toggleTheme);

  // Smooth scrolling for navigation
  document.addEventListener("click", function (e) {
    if (e.target.hasAttribute("onclick")) {
      e.preventDefault();
      const targetId = e.target.getAttribute("onclick").match(/'([^']+)'/)[1];
      scrollToSection(targetId);
    }
  });
}

// Smooth scroll function
function scrollToSection(sectionId) {
  const element = document.getElementById(sectionId);
  if (element) {
    const navHeight = document.querySelector(".nav").offsetHeight;
    const elementPosition = element.offsetTop - navHeight - 20;

    window.scrollTo({
      top: elementPosition,
      behavior: "smooth",
    });
  }
}

// Contact form setup
function setupContactForm() {
  const form = document.querySelector(".contact-form");

  form.addEventListener("submit", function (e) {
    e.preventDefault();

    // Get form data
    const formData = new FormData(form);
    const name =
      formData.get("name") || form.querySelector('input[type="text"]').value;
    const email =
      formData.get("email") || form.querySelector('input[type="email"]').value;
    const message =
      formData.get("message") || form.querySelector("textarea").value;

    // Simple validation
    if (!name || !email || !message) {
      alert(
        currentLang === "en"
          ? "Please fill in all fields."
          : "Veuillez remplir tous les champs."
      );
      return;
    }

    // Create mailto link
    const subject = encodeURIComponent(`Portfolio Contact from ${name}`);
    const body = encodeURIComponent(
      `Name: ${name}\nEmail: ${email}\n\nMessage:\n${message}`
    );
    const mailtoLink = `mailto:contact@bourbask.dev?subject=${subject}&body=${body}`;

    // Open email client
    window.location.href = mailtoLink;

    // Show success message
    alert(
      currentLang === "en" ? "Email client opened!" : "Client email ouvert !"
    );

    // Reset form
    form.reset();
  });
}

// Intersection Observer for animations
const observerOptions = {
  threshold: 0.1,
  rootMargin: "0px 0px -50px 0px",
};

const observer = new IntersectionObserver(function (entries) {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.style.opacity = "1";
      entry.target.style.transform = "translateY(0)";
    }
  });
}, observerOptions);

// Observe all sections for animation
document.addEventListener("DOMContentLoaded", function () {
  const sections = document.querySelectorAll(".section");
  sections.forEach((section) => {
    observer.observe(section);
  });
});

// Navigation background on scroll
window.addEventListener("scroll", function () {
  const nav = document.querySelector(".nav");
  if (window.scrollY > 50) {
    nav.style.background = "rgba(var(--bg-primary-rgb), 0.95)";
  } else {
    nav.style.background = "var(--bg-primary)";
  }
});
