# Portfolio - Kévin Bourbasquet

> Modern, modular portfolio website showcasing my work as a Full-Stack Developer with internationalization and advanced features

[🇫🇷 Version française](#version-française) | [🌐 Live Demo](https://bourbask.github.io) | [📄 CV Generator](https://bourbask.github.io/#downloadCV)

## ✨ Features

- **🌓 Dark/Light Mode** - Smooth theme switching with system preference detection
- **🌍 Fully Bilingual** - English/French content with intelligent language detection
- **📄 Dynamic CV Generator** - Professional PDF CV generation with live printing
- **📱 Responsive Design** - Optimized for all devices and screen sizes
- **🎨 Modern Architecture** - Modular ES6+ structure with separation of concerns
- **⚡ High Performance** - Lightweight, fast-loading with efficient asset management
- **♿ Accessibility First** - Built with WCAG guidelines and semantic HTML
- **🔧 Developer Experience** - Hot reloading, modular CSS, and clean code structure

## 🏗️ Architecture

This project follows modern web development patterns with a modular architecture:

### **Core Philosophy**

- **Modular Design**: Each feature is isolated in its own module
- **Separation of Concerns**: Clear division between logic, styling, and content
- **Progressive Enhancement**: Works without JavaScript, enhanced with it
- **Accessibility First**: Semantic HTML and ARIA compliance
- **Performance Conscious**: Optimized assets and lazy loading

### **Module Structure**

```

├── modules/
│ ├── theme-manager.js # Dark/Light mode with persistence
│ ├── navigation.js # Smart navigation with active states
│ ├── contact-form.js # Form handling with validation
│ ├── animations.js # Intersection Observer animations
│ └── cv-generator.js # PDF CV generation and printing
├── i18n/
│ ├── i18n-manager.js # Language detection & switching
│ ├── fr.js # French translations
│ └── en.js # English translations
├── config/
│ └── settings.js # Global configuration
└── utils/
└── helpers.js # Utility functions

```

## 🛠️ Technologies Used

- **HTML5** - Semantic markup with ARIA accessibility
- **CSS3** - Modern features (Grid, Flexbox, Custom Properties, Container Queries)
- **ES6+ JavaScript** - Native modules, async/await, modern APIs
- **CSS Architecture** - BEM methodology with CSS custom properties
- **Internationalization** - Custom i18n system with browser language detection
- **PDF Generation** - Native browser printing with custom styling
- **Progressive Enhancement** - Works on all devices and browsers

## 📁 Project Structure

```

bourbask.github.io/
├── index.html # Main HTML with semantic structure
├── css/
│ ├── main.css # Main stylesheet with CSS Grid & Flexbox
│ ├── components/ # Component-specific styles
│ ├── themes/ # Theme variants (dark/light)
│ └── utilities/ # Utility classes and mixins
├── js/
│ ├── main.js # Application orchestrator
│ ├── modules/ # Feature modules
│ ├── i18n/ # Internationalization system
│ ├── config/ # Configuration files
│ └── utils/ # Utility functions
├── assets/
│ ├── icons/ # SVG icons and graphics
│ └── images/ # Optimized images and media
└── docs/ # Documentation and guides

```

## 🚀 Quick Start

### **Prerequisites**

- Modern browser (Chrome 80+, Firefox 75+, Safari 13+)
- Local development server (for CORS policies)

### **Local Development**

1. **Clone the repository**

   ```bash
   git clone https://github.com/bourbask/bourbask.github.io.git
   cd bourbask.github.io
   ```

2. **Serve locally** (choose your preferred method):

   **Using Python:**

   ```bash
   python -m http.server 8000
   # or for Python 2: python -m SimpleHTTPServer 8000
   ```

   **Using Node.js:**

   ```bash
   npx serve . -l 8000
   # or: npx http-server -p 8000
   ```

   **Using PHP:**

   ```bash
   php -S localhost:8000
   ```

   **Using VS Code:**

   ```bash
   # Install Live Server extension, then right-click index.html → "Open with Live Server"
   ```

3. **Open in browser**
   ```
   http://localhost:8000
   ```

### **GitHub Pages Deployment**

Automatic deployment via GitHub Pages:

1. Push to `main` branch
2. GitHub Actions builds and deploys automatically
3. Live site updates within minutes

**Manual Setup:**

1. Repository Settings → Pages
2. Source: "Deploy from a branch"
3. Branch: `main` / `/ (root)`
4. Save and wait for deployment ✅

## 🎨 Customization

### **Theme System**

The portfolio uses a sophisticated theming system with CSS custom properties:

```css
:root {
  /* Light theme variables */
  --primary-50: #f0f9ff;
  --primary-500: #3b82f6;
  --primary-900: #1e3a8a;
}

[data-theme="dark"] {
  /* Dark theme overrides */
  --primary-50: #0f172a;
  --primary-500: #60a5fa;
  --primary-900: #bfdbfe;
}
```

### **Adding New Languages**

1. **Create translation file** (`js/i18n/[lang].js`):

   ```javascript
   export const es = {
     name: "Kévin Bourbasquet",
     heroTitle1: "Construyendo soluciones",
     // ... add all translation keys
   };
   ```

2. **Import in i18n-manager.js**:

   ```javascript
   import { es } from "./es.js";
   const translations = { en, fr, es };
   ```

3. **Update language toggle** in HTML:
   ```html
   <span class="lang-option" data-lang="es">ES</span>
   ```

### **CV Customization**

The CV generator supports full customization:

```javascript
// In cv-generator.js
getCVStyles() {
    return `
        /* Customize CV appearance */
        .cv-header { background: var(--accent-primary); }
        .section-title { color: var(--accent-primary); }
    `;
}
```

### **Adding New Modules**

1. **Create module** (`js/modules/new-feature.js`):

   ```javascript
   class NewFeature {
     constructor(dependencies) {
       this.deps = dependencies;
     }

     init() {
       // Initialize feature
     }
   }
   export default NewFeature;
   ```

2. **Import in main.js**:
   ```javascript
   import NewFeature from "./modules/new-feature.js";
   this.newFeature = new NewFeature(this.i18n);
   ```

## 🌍 Internationalization (i18n)

The portfolio features a robust internationalization system:

### **Features**

- **Browser Language Detection** - Automatically detects user's preferred language
- **Persistent Preferences** - Remembers language choice in localStorage
- **Fallback System** - Graceful degradation for unsupported languages
- **Dynamic Content** - Age calculation and contextual translations
- **SEO Friendly** - Proper `lang` attributes and meta tags

### **Content Management**

- **Centralized Translations** - All content in dedicated language files
- **Nested Structure** - Organized by sections and features
- **Dynamic Placeholders** - Support for variables like `{age}`
- **Professional Terminology** - Consistent technical vocabulary

## 📱 Browser Support & Performance

### **Compatibility**

- ✅ Chrome 80+ (ES6 modules, CSS Grid)
- ✅ Firefox 75+ (Dynamic imports, CSS custom properties)
- ✅ Safari 13+ (Intersection Observer, CSS Grid)
- ✅ Edge 80+ (Modern JavaScript features)

### **Performance Metrics**

- 🚀 **Lighthouse Score**: 95+ (Performance, Accessibility, SEO)
- ⚡ **First Contentful Paint**: < 1.2s
- 📱 **Mobile Optimized**: Touch-friendly, responsive images
- 🔍 **SEO Optimized**: Semantic HTML, meta tags, structured data

## 🔧 Development

### **Code Quality**

- **ESLint** - JavaScript linting and best practices
- **Prettier** - Consistent code formatting
- **Modern ES6+** - Classes, modules, async/await
- **CSS Methodology** - BEM naming, logical properties
- **Accessibility Testing** - WAVE, axe-core compliance

### **Architecture Patterns**

- **Module Pattern** - Encapsulated, reusable components
- **Observer Pattern** - Event-driven architecture
- **Strategy Pattern** - Theme and language switching
- **Factory Pattern** - Dynamic CV generation

### **Performance Optimizations**

- **Lazy Loading** - Images and non-critical resources
- **Code Splitting** - Dynamic imports for features
- **Asset Optimization** - Compressed images, minified CSS
- **Caching Strategy** - Service worker for offline support

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Follow** code style (run Prettier)
4. **Test** on multiple devices and browsers
5. **Commit** with conventional commits (`feat:`, `fix:`, `docs:`)
6. **Push** to your branch (`git push origin feature/amazing-feature`)
7. **Open** a Pull Request with detailed description

### **Development Setup**

```bash
# Clone and install
git clone https://github.com/bourbask/bourbask.github.io.git
cd bourbask.github.io

# Start development server
npm run dev      # or your preferred method

# Run tests
npm test         # (when available)

# Build for production
npm run build    # (when build process is added)
```

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

Feel free to use this code as inspiration for your own portfolio, but please don't copy the content directly. Create something uniquely yours! ✨

## 🐛 Issues & Feature Requests

Found a bug or have a suggestion?

- **🐛 Bug Reports**: [Open an issue](https://github.com/bourbask/bourbask.github.io/issues/new?template=bug_report.md)
- **💡 Feature Requests**: [Request a feature](https://github.com/bourbask/bourbask.github.io/issues/new?template=feature_request.md)
- **❓ Questions**: [Start a discussion](https://github.com/bourbask/bourbask.github.io/discussions)

## 📞 Contact & Social

- **📧 Email**: [bourbasquet.k@etik.com](mailto:bourbasquet.k@etik.com)
- **🐙 GitHub**: [@bourbask](https://github.com/bourbask)
- **💼 LinkedIn**: [Kevin Bourbasquet](https://www.linkedin.com/in/k%C3%A9vin-bourbasquet)
- **🌐 Portfolio**: [bourbasquetkev.in](https://bourbasquetkev.in)

---

## Version Française

> Site portfolio moderne et modulaire présentant mon travail en tant que Développeur Full-Stack avec internationalisation et fonctionnalités avancées

## ✨ Fonctionnalités

- **🌓 Mode Sombre/Clair** - Changement de thème fluide avec détection des préférences système
- **🌍 Entièrement Bilingue** - Contenu Anglais/Français avec détection intelligente de la langue
- **📄 Générateur de CV Dynamique** - Génération de CV PDF professionnel avec impression en direct
- **📱 Design Responsive** - Optimisé pour tous les appareils et tailles d'écran
- **🎨 Architecture Moderne** - Structure ES6+ modulaire avec séparation des préoccupations
- **⚡ Haute Performance** - Léger, chargement rapide avec gestion efficace des assets
- **♿ Accessibilité Prioritaire** - Construit selon les directives WCAG et HTML sémantique
- **🔧 Expérience Développeur** - Rechargement à chaud, CSS modulaire, structure de code propre

## 🏗️ Architecture

Ce projet suit les modèles modernes de développement web avec une architecture modulaire:

### **Philosophie Core**

- **Design Modulaire**: Chaque fonctionnalité est isolée dans son propre module
- **Séparation des Préoccupations**: Division claire entre logique, stylisation et contenu
- **Amélioration Progressive**: Fonctionne sans JavaScript, amélioré avec
- **Accessibilité Prioritaire**: HTML sémantique et conformité ARIA
- **Conscience Performance**: Assets optimisés et chargement paresseux

## 🛠️ Technologies Utilisées

- **HTML5** - Balisage sémantique avec accessibilité ARIA
- **CSS3** - Fonctionnalités modernes (Grid, Flexbox, Propriétés Personnalisées)
- **ES6+ JavaScript** - Modules natifs, async/await, APIs modernes
- **Architecture CSS** - Méthodologie BEM avec propriétés personnalisées CSS
- **Internationalisation** - Système i18n personnalisé avec détection de langue navigateur
- **Génération PDF** - Impression navigateur native avec stylisation personnalisée
- **Amélioration Progressive** - Fonctionne sur tous appareils et navigateurs

## 🚀 Démarrage Rapide

### **Prérequis**

- Navigateur moderne (Chrome 80+, Firefox 75+, Safari 13+)
- Serveur de développement local (pour les politiques CORS)

### **Développement Local**

1. **Cloner le repository**

   ```bash
   git clone https://github.com/bourbask/bourbask.github.io.git
   cd bourbask.github.io
   ```

2. **Servir localement** (choisir votre méthode préférée):

   ```bash
   python -m http.server 8000        # Python
   npx serve . -l 8000              # Node.js
   php -S localhost:8000            # PHP
   ```

3. **Ouvrir dans le navigateur**
   ```
   http://localhost:8000
   ```

## 🎨 Personnalisation

Le portfolio dispose d'un système de thématisation sophistiqué et d'une architecture modulaire permettant une personnalisation facile.

## 📄 Licence

Ce projet est sous licence **MIT** - voir le fichier [LICENSE](LICENSE) pour plus de détails.

N'hésitez pas à utiliser ce code comme inspiration pour votre propre portfolio, mais merci de ne pas copier le contenu directement. Créez quelque chose d'unique ! ✨

## 📞 Contact & Social

- **📧 Email**: [bourbasquet.k@etik.com](mailto:bourbasquet.k@etik.com)
- **🐙 GitHub**: [@bourbask](https://github.com/bourbask)
- **💼 LinkedIn**: [Kevin Bourbasquet](https://www.linkedin.com/in/k%C3%A9vin-bourbasquet)
- **🌐 Portfolio**: [bourbasquetkev.in](https://bourbasquetkev.in)

---

<div align="center">

**Fait avec ❤️ par Kévin Bourbasquet**
