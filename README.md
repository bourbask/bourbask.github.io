# Portfolio - Kévin Bourbasquet

> Modern, responsive portfolio website showcasing my work as a Full Stack Developer

[🇫🇷 Version française](#version-française) | [🌐 Live Demo](https://bourbask.github.io)

## ✨ Features

- **🌓 Dark/Light Mode** - Smooth theme switching with system preference detection
- **🌍 Bilingual** - English/French content with elegant language toggle
- **📱 Responsive Design** - Optimized for all devices and screen sizes
- **🎨 Modern UI** - Clean, professional design with smooth animations
- **⚡ Performance** - Lightweight, fast-loading with no external dependencies
- **♿ Accessible** - Built with accessibility best practices

## 🛠️ Technologies Used

- **HTML5** - Semantic markup
- **CSS3** - Modern styling with CSS Grid & Flexbox
- **Vanilla JavaScript** - No frameworks, pure performance
- **CSS Custom Properties** - Dynamic theming
- **Inter Font** - Modern, readable typography
- **GitHub Pages** - Free hosting and deployment

## 📁 Project Structure

```
bourbask.github.io/
├── index.html              # Main HTML file
├── css/
│   └── style.css           # Styles with CSS custom properties
├── js/
│   └── script.js           # JavaScript functionality
├── assets/
│   └── icons/              # SVG icons and graphics
├── images/                 # Project images and photos
└── README.md               # Project documentation
```

## 🚀 Quick Start

### Local Development

1. **Clone the repository**

   ```bash
   git clone https://github.com/bourbask/bourbask.github.io.git
   cd bourbask.github.io
   ```

2. **Serve locally** (choose one method):

   **Using Python:**

   ```bash
   # Python 3
   python -m http.server 8000

   # Python 2
   python -m SimpleHTTPServer 8000
   ```

   **Using Node.js:**

   ```bash
   npx serve .
   ```

   **Using PHP:**

   ```bash
   php -S localhost:8000
   ```

3. **Open in browser**
   ```
   http://localhost:8000
   ```

### GitHub Pages Deployment

This site is automatically deployed via GitHub Pages. Any push to the `main` branch will update the live site.

**Setup GitHub Pages:**

1. Go to repository Settings
2. Navigate to "Pages" section
3. Select "Deploy from a branch"
4. Choose `main` branch and `/ (root)` folder
5. Save and wait for deployment

## 🎨 Customization

### Themes

The site supports both light and dark themes using CSS custom properties. Theme colors are defined in `:root` and `[data-theme="dark"]` selectors in `css/style.css`.

**Light Theme Colors:**

- Primary: `#ffffff`
- Secondary: `#f8fafc`
- Accent: `#3b82f6`

**Dark Theme Colors:**

- Primary: `#0f172a`
- Secondary: `#1e293b`
- Accent: `#60a5fa`

### Content Translation

Translations are managed in the `translations` object in `js/script.js`. To add a new language:

1. Add language object to `translations`
2. Update language toggle in HTML
3. Add corresponding logic in `loadLanguage()` function

### Adding New Sections

1. Add HTML structure in `index.html`
2. Style in `css/style.css`
3. Add translations if needed
4. Update navigation if required

## 📱 Browser Support

- ✅ Chrome 60+
- ✅ Firefox 55+
- ✅ Safari 12+
- ✅ Edge 79+

## 🤝 Contributing

While this is a personal portfolio, suggestions and improvements are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/improvement`)
3. Commit your changes (`git commit -am 'Add some improvement'`)
4. Push to the branch (`git push origin feature/improvement`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Issues & Bugs

Found a bug or have a suggestion? Please [open an issue](https://github.com/bourbask/bourbask.github.io/issues).

## 📞 Contact

- **Email:** [contact@bourbask.dev](mailto:contact@bourbask.dev)
- **GitHub:** [@bourbask](https://github.com/bourbask)
- **LinkedIn:** [Kevin Bourbasquet](https://linkedin.com/in/kevin-bourbasquet)

---

## Version Française

> Site portfolio moderne et responsive présentant mon travail en tant que Développeur Full Stack

## ✨ Fonctionnalités

- **🌓 Mode Sombre/Clair** - Changement de thème fluide avec détection des préférences système
- **🌍 Bilingue** - Contenu Anglais/Français avec basculement élégant
- **📱 Design Responsive** - Optimisé pour tous les appareils et tailles d'écran
- **🎨 Interface Moderne** - Design propre et professionnel avec animations fluides
- **⚡ Performance** - Léger, chargement rapide sans dépendances externes
- **♿ Accessible** - Construit avec les meilleures pratiques d'accessibilité

## 🛠️ Technologies Utilisées

- **HTML5** - Balisage sémantique
- **CSS3** - Stylage moderne avec CSS Grid & Flexbox
- **JavaScript Vanilla** - Aucun framework, performance pure
- **Propriétés Personnalisées CSS** - Thématisation dynamique
- **Police Inter** - Typographie moderne et lisible
- **GitHub Pages** - Hébergement et déploiement gratuits

## 🚀 Démarrage Rapide

### Développement Local

1. **Cloner le repository**

   ```bash
   git clone https://github.com/bourbask/bourbask.github.io.git
   cd bourbask.github.io
   ```

2. **Servir localement** (choisir une méthode):

   **Avec Python:**

   ```bash
   # Python 3
   python -m http.server 8000

   # Python 2
   python -m SimpleHTTPServer 8000
   ```

   **Avec Node.js:**

   ```bash
   npx serve .
   ```

   **Avec PHP:**

   ```bash
   php -S localhost:8000
   ```

3. **Ouvrir dans le navigateur**
   ```
   http://localhost:8000
   ```

### Déploiement GitHub Pages

Ce site est automatiquement déployé via GitHub Pages. Tout push sur la branche `main` mettra à jour le site en ligne.

## 🎨 Personnalisation

### Thèmes

Le site supporte les thèmes clair et sombre en utilisant les propriétés personnalisées CSS. Les couleurs sont définies dans les sélecteurs `:root` et `[data-theme="dark"]` dans `css/style.css`.

### Traduction du Contenu

Les traductions sont gérées dans l'objet `translations` dans `js/script.js`. Pour ajouter une nouvelle langue :

1. Ajouter l'objet langue à `translations`
2. Mettre à jour le toggle de langue dans le HTML
3. Ajouter la logique correspondante dans la fonction `loadLanguage()`

## 📄 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 📞 Contact

- **Email:** [contact@bourbask.dev](mailto:contact@bourbask.dev)
- **GitHub:** [@bourbask](https://github.com/bourbask)
- **LinkedIn:** [Kevin Bourbasquet](https://linkedin.com/in/kevin-bourbasquet)

---

**Made with ❤️ by Kévin Bourbasquet**
