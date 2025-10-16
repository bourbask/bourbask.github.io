use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Basic info
    map.insert("name", "Kévin Bourbasquet");
    map.insert("badge", "Ouvert aux opportunités");
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
    map.insert("aboutText1", "Développeur fullstack de {age} ans qui croit en la construction de choses qui fonctionnent vraiment. Mon terrain de jeu favori : des backends robustes avec Symfony API Platform et des interfaces intuitives avec React et TypeScript.");
    map.insert("aboutText2", "Au-delà de la stack habituelle, j'explore Rust pour les projets critiques et j'intègre des workflows IA via OpenRouter. Quand je ne code pas, vous me trouverez sur les sentiers côtiers bretons, en train de construire des claviers sur mesure ou de m'occuper de ma collection de plantes toujours grandissante.");

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
    map.insert("photographyDescription", "Capturer des moments pendant les treks et au quotidien—il y a quelque chose dans l'art de figer le temps qui parle à l'esprit développeur.");
    map.insert("musicTitle", "Musique");
    map.insert("musicDescription", "Sessions guitare et piano pour les pauses mentales. Musique et code suivent des patterns—comprendre l'un aide avec l'autre.");
    map.insert("trekkingTitle", "Randonnée & Trek");
    map.insert("trekkingDescription", "Évasions régulières sur les sentiers côtiers et chemins de campagne. Rien ne vide l'esprit comme une bonne rando en bord de mer—et c'est là que mes meilleures idées de debug arrivent.");
    map.insert("linuxTitle", "Bricoleur Linux");
    map.insert("linuxDescription", "Utilisateur quotidien de Manjaro qui aime vraiment configurer ses systèmes—si ça tourne sous Linux et que ça peut être optimisé, ça m'intéresse probablement.");

    // Contact
    map.insert("contactTitle", "Construisons quelque chose");
    map.insert("contactDescription", "Toujours partant pour discuter de projets intéressants, partager des connaissances, ou simplement parler tech. Que vous cherchiez un co-fondateur technique ou souhaitiez collaborer sur de l'open source.");
    map.insert("emailLabel", "Email");
    map.insert("nameLabel", "Votre nom");
    map.insert("emailPlaceholder", "votre.email@domaine.com");
    map.insert("messageLabel", "Qu'est-ce qui vous trotte dans la tête ?");
    map.insert("sendMessage", "Envoyer le message");

    // CV Button
    map.insert("loader", "Ouverture impression...");
    map.insert("action", "Télécharger CV");

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
    map.insert("cv.title", "Développeur Full-Stack");
    map.insert("cv.phoneAvailable", "Disponible sur demande");
    map.insert("cv.locationMove", "Brest → Bordeaux");
    map.insert("cv.drivingLicense", "Permis B");
    map.insert("cv.summary", "Développeur full-stack de {age} ans avec 5 ans d'expérience en développement web. Expertise Symfony/React, méthodes agiles, et passion pour l'innovation technique. Recherche de nouveaux défis à Bordeaux.");

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

    map
}
