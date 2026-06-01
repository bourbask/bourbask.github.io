use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Basic info
    map.insert("name", "Kévin Bourbasquet");
    map.insert("heroTitle1", "Créer des solutions");
    map.insert("heroTitle2", "numériques fiables");
    map.insert("heroDescription", "Développeur fullstack focalisé sur des applications claires et évolutives. Je travaille avec des outils éprouvés comme React & Symfony, tout en restant curieux des nouvelles technologies—j'explore actuellement Rust et l'intégration de l'IA.");
    map.insert("viewWork", "Voir mes créations");
    map.insert("getInTouch", "Discutons");

    // About section
    map.insert("aboutTitle", "À propos");
    map.insert(
        "aboutSubtitle",
        "Builder, résolveur de problèmes, et explorateur tech",
    );
    map.insert("aboutText1", "Développeur fullstack, {age} ans. Je construis des backends solides avec Symfony API Platform et des interfaces en React + TypeScript — avec l'idée fixe que ça doit être fiable avant d'être élégant.");
    map.insert("aboutText2", "J'explore Rust quand la robustesse n'est pas négociable, et j'intègre l'IA dans mes workflows via OpenRouter. En dehors de l'écran : les sentiers côtiers bretons pour décompresser, des claviers mécaniques custom à assembler sur l'établi, et une collection de plantes qui prend décidément trop de place.");

    // Stats
    map.insert("yearsExperience", "Années à shipper du code");
    map.insert("projectsCompleted", "Projets livrés");
    map.insert("technologiesMastered", "Technologies maîtrisées");

    // Skills
    map.insert("skillsTitle", "Ma boîte à outils");
    map.insert(
        "skillsSubtitle",
        "Des outils fiables pour construire de super logiciels",
    );
    map.insert("frontendTitle", "Frontend");
    map.insert("backendTitle", "Backend");
    map.insert("devopsTitle", "DevOps & Cloud");
    map.insert("toolsTitle", "Mes fidèles compagnons");

    // Projects
    map.insert("projectsTitle", "Ce que j'ai construit");
    map.insert(
        "projectsSubtitle",
        "Des vrais projets pour de vrais problèmes",
    );
    map.insert("ezprint3dDescription", "Une plateforme qui connecte les passionnés d'impression 3D avec les prestataires de services. Construite avec une architecture microservices et un focus sur les fonctionnalités communautaires.");
    map.insert("lekickerfouDescription", "Bot Discord pour la gestion fluide des canaux vocaux. Écrit en Rust parce que quand on a besoin de performance et de fiabilité, on prend Rust.");
    map.insert("keyboardDescription", "Claviers mécaniques sur mesure dont un Corne 3x6 terminé et un Skeletyl en cours. Parce que taper du code devrait être aussi agréable que le code qu'on écrit.");

    // Project Blog Integration
    map.insert("readStory", "📖 Lire l'Article");
    map.insert("storyComingSoon", "📝 Article Bientôt");
    map.insert("storyComingSoonBtn", "Article Bientôt Disponible");
    map.insert("readFullStory", "Lire l'Article Complet");

    // Interests
    map.insert("interestsTitle", "Quand je ne code pas");
    map.insert(
        "interestsSubtitle",
        "Aventures et hobbies qui nourrissent la créativité",
    );
    map.insert("photographyTitle", "Photographie");
    map.insert("photographyDescription", "Un sentier dans les Monts d'Arrée à l'aube, la brume sur l'Aulne, un coucher de soleil sur la rade de Brest. La photographie, c'est ma façon de ne pas oublier que je suis ailleurs qu'à l'écran.");
    map.insert("musicTitle", "Musique");
    map.insert("musicDescription", "Guitare et piano, surtout le soir. Quand je bloque sur un bug depuis trop longtemps, vingt minutes de musique valent souvent mieux qu'une heure supplémentaire de débogage. C'est empirique, mais ça marche.");
    map.insert("trekkingTitle", "Randonnée & Trek");
    map.insert("trekkingDescription", "Les sentiers côtiers de Bretagne, les chemins vers les pointes, les falaises. La marche, c'est l'endroit où j'ai le plus d'idées nettes. Aucune réunion n'a jamais aussi bien fonctionné qu'une rando en bord de mer.");
    map.insert("linuxTitle", "Bricoleur Linux");
    map.insert("linuxDescription", "Manjaro en daily driver. Les dotfiles dans un repo Git. Le gestionnaire de fenêtres configuré à la virgule près. Je sais que c'est une forme de procrastination déguisée en productivité. Mais c'est une très bonne forme de procrastination.");

    // Contact
    map.insert("contactTitle", "Construisons quelque chose");
    map.insert("contactDescription", "Toujours partant pour discuter de projets intéressants, partager des connaissances, ou simplement parler tech. Que vous cherchiez un co-fondateur technique ou souhaitiez collaborer sur de l'open source.");
    map.insert("emailLabel", "Email");
    map.insert("nameLabel", "Votre nom");
    map.insert("emailPlaceholder", "votre.email@domaine.com");
    map.insert("messageLabel", "Qu'est-ce qui vous trotte dans la tête ?");
    map.insert("sendMessage", "Envoyer le message");
    map.insert("formSending", "Envoi en cours…");
    map.insert("formSuccess", "Message envoyé ! Je vous répondrai bientôt.");
    map.insert("formError", "Une erreur est survenue. Réessayez ou contactez-moi directement par email.");

    // CV Button
    map.insert("action", "Télécharger CV");
    map.insert("cv.downloading", "Génération...");
    map.insert("cv.downloaded", "Généré!");

    // Footer
    map.insert("madeWith", "Conçu avec");
    map.insert("by", "par");
    map.insert("sourceCode", "Voir le code");

    // Form messages
    map.insert(
        "fillAllFields",
        "Merci de remplir tous les champs—j'ai hâte de vous lire !",
    );
    map.insert(
        "emailClientOpened",
        "Client email ouvert ! J'ai hâte de vous lire.",
    );

    // CV Translations
    map.insert("cv.personal.name", "Kévin Bourbasquet");
    map.insert("cv.personal.title", "Développeur Full-Stack");
    map.insert("cv.personal.email", "bourbasquet.k@etik.com");
    map.insert("cv.personal.phone", "Disponible sur demande");
    map.insert("cv.personal.location", "Brest → Bordeaux");
    map.insert("cv.personal.license", "Permis B");
    map.insert("cv.personal.portfolio", "bourbask.github.io");
    map.insert("cv.summary", "Développeur full-stack de {age} ans avec 7 ans d'expérience en développement web. Expertise Symfony/React, méthodes agiles, et passion pour l'innovation technique. En recherche de nouveaux défis à Bordeaux.");

    // CV Section titles
    map.insert("cv.sections.experience", "Expérience Professionnelle");
    map.insert("cv.sections.projects", "Projets Marquants");
    map.insert("cv.sections.skills", "Compétences");
    map.insert("cv.sections.education", "Formation");
    map.insert("cv.sections.languages", "Langues");
    map.insert("cv.sections.interests", "Centres d'intérêt");

    // CV experience item (europroc)
    map.insert("cv.experience.0.title", "Développeur Full-Stack");
    map.insert("cv.experience.0.company", "Europrocurement");
    map.insert("cv.experience.0.location", "Brest, Bretagne");
    map.insert("cv.experience.0.period", "Nov 2022 - Présent");
    map.insert(
        "cv.experience.0.achievements.0",
        "Développement d'applications pour annonces légales et formalités juridiques",
    );
    map.insert(
        "cv.experience.0.achievements.1",
        "Stack technique : Symfony, ReactJS, MySQL/MariaDB, API REST",
    );
    map.insert(
        "cv.experience.0.achievements.2",
        "Méthodes agiles, collaboration avec GitLab, intégration Redux",
    );
    map.insert(
        "cv.experience.0.achievements.3",
        "Gestion complète du cycle de développement full-stack",
    );

    // CV experience item (kooi)
    map.insert("cv.experience.1.title", "Développeur Full-Stack");
    map.insert("cv.experience.1.company", "Kooi Security");
    map.insert("cv.experience.1.location", "Lanester, Bretagne");
    map.insert("cv.experience.1.period", "Juil 2019 - Sept 2022");
    map.insert(
        "cv.experience.1.achievements.0",
        "Développement d'applications de sécurité en environnement international",
    );
    map.insert(
        "cv.experience.1.achievements.1",
        "Stack : Symfony, Vue.js, Node.js, composants web sur mesure",
    );
    map.insert(
        "cv.experience.1.achievements.2",
        "Collaboration avec équipes néerlandaises (anglais professionnel)",
    );
    map.insert(
        "cv.experience.1.achievements.3",
        "Méthodes agiles, design avec Adobe Creative Suite",
    );

    // CV experience item (sag)
    map.insert("cv.experience.2.title", "Développeur Web");
    map.insert("cv.experience.2.company", "Société Alimentaire de Guidel");
    map.insert("cv.experience.2.location", "Guidel, Bretagne");
    map.insert("cv.experience.2.period", "Déc 2018 - Fév 2019");
    map.insert(
        "cv.experience.2.achievements.0",
        "Conception et développement d'affichage web d'informations internes",
    );
    map.insert(
        "cv.experience.2.achievements.1",
        "Mise en production complète de la solution",
    );
    map.insert(
        "cv.experience.2.achievements.2",
        "Interface destinée aux équipes opérationnelles",
    );

    // CV project item (ezp)
    map.insert("cv.projects.ezprint3d.name", "EzPrint3D");
    map.insert(
        "cv.projects.ezprint3d.description",
        "Plateforme SaaS connectant services d'impression 3D et clients",
    );
    map.insert("cv.projects.ezprint3d.status", "En développement");

    // CV project item (lkf bot)
    map.insert("cv.projects.lekickerfou.name", "LeKickerFou Bot");
    map.insert(
        "cv.projects.lekickerfou.description",
        "Bot Discord haute performance pour gestion de canaux vocaux",
    );
    map.insert("cv.projects.lekickerfou.status", "Déployé");

    // CV project item (Excelsior)
    map.insert("cv.projects.excelsior.name", "Excelsior");
    map.insert(
        "cv.projects.excelsior.description",
        "Jeu d'horreur narratif basé sur un voyage dans les Pyrénées",
    );
    map.insert("cv.projects.excelsior.status", "En développement");

    // CV skill categories
    map.insert("cv.skills.categories.backend", "Backend");
    map.insert("cv.skills.categories.frontend", "Frontend");
    map.insert("cv.skills.categories.database", "Database");
    map.insert("cv.skills.categories.devops", "DevOps");
    map.insert("cv.skills.categories.learning", "En cours");

    // CV education

    // Master
    map.insert(
        "cv.education.master.degree",
        "Master Manager de Solutions Digitales et Data",
    );
    map.insert("cv.education.master.school", "CS2I Lorient");
    map.insert("cv.education.master.period", "2020 - 2022");
    map.insert(
        "cv.education.master.details",
        "Formation en alternance, spécialisation développement",
    );

    // License
    map.insert(
        "cv.education.bachelor.degree",
        "Licence Concepteur de Systèmes d'Information",
    );
    map.insert("cv.education.bachelor.school", "CS2I Lorient");
    map.insert("cv.education.bachelor.period", "2019 - 2020");
    map.insert(
        "cv.education.bachelor.details",
        "Spécialisation Développement",
    );

    // BTS
    map.insert(
        "cv.education.bts.degree",
        "BTS Systèmes Informatiques aux Organisations",
    );
    map.insert("cv.education.bts.school", "Saint-Louis Châteaulin");
    map.insert("cv.education.bts.period", "2017 - 2019");
    map.insert(
        "cv.education.bts.details",
        "Option Solutions d'infrastructure",
    );

    // CV languages
    map.insert("cv.languages.french.name", "Français");
    map.insert("cv.languages.french.level", "Natif");

    map.insert("cv.languages.english.name", "Anglais");
    map.insert("cv.languages.english.level", "Professionnel");

    // CV interests
    map.insert("cv.interests.hiking", "🥾 Randonnée");
    map.insert("cv.interests.gardening", "🌿 Jardinage");
    map.insert("cv.interests.diy", "🔧 Bricolage");
    map.insert("cv.interests.linux", "🐧 Linux");

    // CV footer
    map.insert("cv.footer", "Disponible pour un poste à Bordeaux • Préavis flexible • Portfolio complet sur bourbask.github.io");

    // CV Section titles
    map.insert("cv.experienceTitle", "Expérience Professionnelle");
    map.insert("cv.projectsTitle", "Projets Marquants");
    map.insert("cv.skillsTitle", "Compétences");
    map.insert("cv.educationTitle", "Formation");
    map.insert("cv.languagesTitle", "Langues");
    map.insert("cv.interestsTitle", "Centres d'intérêt");

    // Blog
    map.insert("blogTitle", "Blog Technique");
    map.insert(
        "blogSubtitle",
        "Réflexions, tutoriels et insights de mon parcours de développeur",
    );
    map.insert("readMore", "Lire la suite");
    map.insert("blogNavigation", "Blog");
    map.insert("categories", "Catégories");
    map.insert("tags", "Tags");

    // Article navigation
    map.insert("previousArticle", "Article précédent");
    map.insert("nextArticle", "Article suivant");
    map.insert("shareArticle", "Partager cet article");

    // Categories
    map.insert("categorySystem", "Système");
    map.insert("categoryHardware", "Hardware");
    map.insert("categoryDevops", "DevOps");
    map.insert("categoryTools", "Outils");
    map.insert("categoryProject", "Projets");

    // Blog Hero
    map.insert("blogBadge", "Blog Technique");
    map.insert("blogHeroTitle1", "Partager des connaissances");
    map.insert("blogHeroTitle2", "et expériences");
    map.insert("blogHeroDescription", "Réflexions, tutoriels et insights de mon parcours de développeur. Du système à la pratique de développement, découvrez mes apprentissages et projets.");
    map.insert("articlesPublished", "Articles Publiés");
    map.insert("minutesReading", "Minutes de Lecture");

    // Latest Article
    map.insert("latestArticle", "Dernier Article");
    map.insert("readArticle", "Lire l'Article");

    // Articles List
    map.insert("allArticlesTitle", "Tous les Articles");
    map.insert("allCategories", "Toutes les Catégories");
    map.insert("sortNewest", "Plus Récents");
    map.insert("sortOldest", "Plus Anciens");
    map.insert("sortReadingTime", "Temps de Lecture");

    // Error 404 Article
    map.insert("error404Title", "404 - Article introuvable");
    map.insert(
        "error404Message",
        "Désolé, cet article n'existe pas ou a été supprimé.",
    );
    map.insert("backToBlog", "Retour au blog");

    // FAB Navigation
    map.insert("navigation.home", "Accueil");
    map.insert("navigation.blog", "Blog");
    map.insert("navigation.cv", "CV");
    map.insert("navigation.language", "Langue");
    map.insert("navigation.theme", "Thème");
    map.insert("navigation.veille", "Veille");

    // Veille page
    map.insert("veille.title", "Veille Technologique");
    map.insert("veille.subtitle", "Actualités tech du jour");
    map.insert("veille.updatedAt", "Mis à jour le");
    map.insert("veille.loading", "Chargement des actualités…");
    map.insert("veille.error", "Impossible de charger les actualités. Réessayez plus tard.");
    map.insert("veille.filterAll", "Tout");
    map.insert("veille.filterUrgent", "Urgent");
    map.insert("veille.filterGoodNews", "Bonne nouvelle");
    map.insert("veille.filterFutureWatch", "À surveiller");
    map.insert("veille.filterStackAlt", "Alternatives stack");
    map.insert("veille.filterGeneral", "Général");
    map.insert("veille.noItems", "Aucun élément dans cette catégorie.");
    map.insert("veille.synthesis.aiGenerated", "Généré par Claude IA");
    map.insert("veille.synthesis.weekOf", "Semaine du");
    map.insert("veille.synthesis.to", "au");
    map.insert("veille.synthesis.showSources", "Sources");
    map.insert("veille.synthesis.hideSources", "Masquer les sources");
    map.insert("veille.synthesis.readMore", "Lire la synthèse →");
    map.insert("veille.synthesis.basedOn", "Basé sur");
    map.insert("veille.synthesis.articles", "articles");
    map.insert("veille.synthesis.backToFeed", "← Retour au fil");
    map.insert("veille.synthesis.filterLabel", "Synthèses");
    map.insert("veille.synthesis.similar", "Articles similaires");

    // PGP contact
    map.insert("pgpDownload", "Télécharger la clé");
    map.insert("pgpKeyserver", "Serveur de clés");
    map.insert("e2eNote", "Chiffré de bout en bout par PGP");

    // Article page
    map.insert("minRead", "min de lecture");
    map.insert("tldr", "En bref : ");
    map.insert("tableOfContents", "Table des matières");
    map.insert("contentNotAvailable", "Contenu non disponible dans cette langue.");

    // Project tags
    map.insert("tag.startup", "Startup");
    map.insert("tag.fullStack", "Full-Stack");
    map.insert("tag.bot", "Bot");
    map.insert("tag.hardware", "Hardware");
    map.insert("tag.diy", "DIY");

    // Navigation aria-labels & tooltips
    map.insert("switchToFrench", "Passer en français");
    map.insert("switchToEnglish", "Switch to English");
    map.insert("toggleTheme", "Changer le thème");
    map.insert("navigation.menu", "Menu de navigation");

    // 404 generic page
    map.insert("pageNotFound", "Page introuvable");
    map.insert("error404Generic", "Oups ! La page que vous cherchez semble avoir disparu dans le vide numérique.");
    map.insert("goHome", "Accueil");
    map.insert("exploreBlog", "Explorer le Blog");

    // Blog filters
    map.insert("blog.filter.category", "Filtrer par catégorie");
    map.insert("blog.filter.sort", "Trier les articles");

    // Footer
    map.insert("mitLicense", "Licence MIT");

    map
}
