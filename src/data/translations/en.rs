use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Basic info
    map.insert("name", "Kévin Bourbasquet");
    map.insert("heroTitle1", "Building reliable");
    map.insert("heroTitle2", "digital solutions");
    map.insert("heroDescription", "Fullstack developer focused on creating clear, scalable applications. I work with proven tools like React & Symfony, while staying curious about emerging tech—currently exploring Rust and AI integration.");
    map.insert("viewWork", "See What I Build");
    map.insert("getInTouch", "Let's Connect");

    // About section
    map.insert("aboutTitle", "About Me");
    map.insert(
        "aboutSubtitle",
        "Builder, problem-solver, and tech explorer",
    );
    map.insert("aboutText1", "Fullstack developer, {age}. I build solid backends with Symfony API Platform and React + TypeScript interfaces — with the fixed idea that reliability comes before elegance.");
    map.insert("aboutText2", "I explore Rust when robustness is non-negotiable, and I integrate AI into my workflows via OpenRouter. Away from the screen: hiking Brittany's coastal trails, assembling custom mechanical keyboards on the workbench, and a plant collection that keeps claiming more space.");

    // Stats
    map.insert("yearsExperience", "Years Shipping Code");
    map.insert("projectsCompleted", "Projects Delivered");
    map.insert("technologiesMastered", "Core Technologies");

    // Skills
    map.insert("skillsTitle", "My Toolbox");
    map.insert(
        "skillsSubtitle",
        "Reliable tools for building great software",
    );
    map.insert("frontendTitle", "Frontend");
    map.insert("backendTitle", "Backend");
    map.insert("devopsTitle", "DevOps & Cloud");
    map.insert("toolsTitle", "Daily Drivers");

    // Projects
    map.insert("projectsTitle", "Things I've Built");
    map.insert("projectsSubtitle", "Real projects solving real problems");
    map.insert("ezprint3dDescription", "A platform connecting 3D printing enthusiasts with service providers. Built with microservices architecture and a focus on community-driven features.");
    map.insert("lekickerfouDescription", "Discord bot for seamless voice channel management. Written in Rust because when you need performance and reliability, you need Rust.");
    map.insert("keyboardDescription", "Custom mechanical keyboards including a completed Corne 3x6 and a Skeletyl in progress. Because typing should feel as good as the code you write.");

    // Project Blog Integration
    map.insert("readStory", "📖 Read Story");
    map.insert("storyComingSoon", "📝 Story Coming Soon");
    map.insert("storyComingSoonBtn", "Story Coming Soon");
    map.insert("readFullStory", "Read Full Story");

    // Interests
    map.insert("interestsTitle", "When I'm Not Coding");
    map.insert(
        "interestsSubtitle",
        "Adventures and hobbies that fuel creativity",
    );
    map.insert("photographyTitle", "Photography");
    map.insert("photographyDescription", "Coastal trails at dawn, sea fog over the cliffs, the estuary at golden hour. Photography is how I keep proof that I do occasionally step away from a screen.");
    map.insert("musicTitle", "Music");
    map.insert("musicDescription", "Guitar and piano, mostly evenings. When a bug has had me for two hours straight, twenty minutes of playing fixes more than another hour of debugging. Can't explain it. Stopped trying.");
    map.insert("trekkingTitle", "Trekking & Hiking");
    map.insert("trekkingDescription", "Brittany's coastal paths are where I do my best thinking. Not multitasking, not podcasts — just walking, sea wind, and whatever problem I've been turning over all week. It usually sorts itself out.");
    map.insert("linuxTitle", "Linux Tinkerer");
    map.insert("linuxDescription", "Daily Manjaro user with an embarrassingly detailed dotfiles repo. I've configured my window manager more times than I've shipped features. I'm aware this is a problem. I'm comfortable with it.");

    // Contact
    map.insert("contactTitle", "Let's Build Something");
    map.insert("contactDescription", "Always up for discussing interesting projects, sharing knowledge, or just talking tech. Whether you need a technical co-founder or want to collaborate on open source work.");
    map.insert("emailLabel", "Email");
    map.insert("nameLabel", "Your Name");
    map.insert("emailPlaceholder", "your.email@domain.com");
    map.insert("messageLabel", "What's on your mind?");
    map.insert("sendMessage", "Send Message");
    map.insert("formSending", "Sending…");
    map.insert("formSuccess", "Message sent! I'll get back to you soon.");
    map.insert("formError", "Something went wrong. Please try again or email me directly.");

    // CV Button
    map.insert("cv.downloading", "Opening print...");
    map.insert("cv.downloaded", "Generated!");
    map.insert("action", "Download CV");

    // Footer
    map.insert("madeWith", "Crafted with");
    map.insert("by", "by");
    map.insert("sourceCode", "View Source");

    // Form messages
    map.insert(
        "fillAllFields",
        "Please fill in all fields—I'd love to hear from you!",
    );
    map.insert(
        "emailClientOpened",
        "Email client opened! Looking forward to hearing from you.",
    );

    // CV Translations
    map.insert("cv.personal.name", "Kévin Bourbasquet");
    map.insert("cv.personal.title", "Full-Stack Developer");
    map.insert("cv.personal.email", "contact@kevin-bourbasquet.fr");
    map.insert("cv.personal.phone", "Available on request");
    map.insert("cv.personal.location", "100% Remote");
    map.insert("cv.personal.portfolio", "www.bourbasquetkev.in");
    map.insert("cv.summary", "A nature lover at {age} and immersed in development for 7 years, I've built solid expertise in Symfony, React and micro-services architectures, working in agile teams. That same curiosity now drives me to bring AI into my daily practice, shipping faster without sacrificing quality. I'm currently looking for new challenges, aiming for a role that supports remote work.");

    // CV Section titles
    map.insert("cv.sections.experience", "Professional Experience");
    map.insert("cv.sections.projects", "Current Projects");
    map.insert("cv.sections.skills", "Skills");
    map.insert("cv.sections.education", "Education");
    map.insert("cv.sections.languages", "Languages");
    map.insert("cv.sections.interests", "Interests");

    // CV experience item (europroc)
    map.insert("cv.experience.0.title", "Full-Stack Developer");
    map.insert("cv.experience.0.company", "Europrocurement");
    map.insert("cv.experience.0.location", "Brest, Brittany");
    map.insert("cv.experience.0.period", "Nov 2022 - Present");
    map.insert(
        "cv.experience.0.achievements.0",
        "Business applications with high regulatory complexity: legal announcement publishing and legal formality management",
    );
    map.insert(
        "cv.experience.0.achievements.1",
        "Technical stack: Symfony, React, MySQL/MariaDB, ApiPlatform, micro-services architecture",
    );
    map.insert(
        "cv.experience.0.achievements.2",
        "Claude Code used daily to speed up refactoring, testing and code review on a complex business codebase",
    );
    map.insert(
        "cv.experience.0.achievements.3",
        "Complete management of full-stack development cycle",
    );

    // CV experience item (kooi)
    map.insert("cv.experience.1.title", "Full-Stack Developer");
    map.insert("cv.experience.1.company", "Kooi Security");
    map.insert("cv.experience.1.location", "Lanester, Brittany");
    map.insert("cv.experience.1.period", "Jul 2019 - Sep 2022");
    map.insert(
        "cv.experience.1.achievements.0",
        "Development of security applications in international environment",
    );
    map.insert(
        "cv.experience.1.achievements.1",
        "Stack: Symfony, Vue.js, Node.js, micro-services architecture, custom web components",
    );
    map.insert(
        "cv.experience.1.achievements.2",
        "Collaboration with Dutch teams (professional English)",
    );
    map.insert(
        "cv.experience.1.achievements.3",
        "Agile methods, design with Adobe Creative Suite",
    );

    // CV experience item (sag)
    map.insert("cv.experience.2.title", "Web Developer");
    map.insert("cv.experience.2.company", "Société Alimentaire de Guidel");
    map.insert("cv.experience.2.location", "Guidel, Brittany");
    map.insert("cv.experience.2.period", "Dec 2018 - Feb 2019");
    map.insert(
        "cv.experience.2.achievements.0",
        "Design and development of web display for internal information",
    );
    map.insert(
        "cv.experience.2.achievements.1",
        "Complete production deployment of the solution",
    );
    map.insert(
        "cv.experience.2.achievements.2",
        "Interface for operational teams",
    );

    // CV project item (ezp)
    map.insert("cv.projects.ezprint3d.name", "EzPrint3D");
    map.insert(
        "cv.projects.ezprint3d.description",
        "SaaS platform connecting 3D printing services and clients",
    );
    map.insert("cv.projects.ezprint3d.status", "Rebuild in progress");

    // CV project item (lkf bot)
    map.insert("cv.projects.kairos.name", "Kairos");
    map.insert(
        "cv.projects.kairos.description",
        "Modular AI-powered personal assistant: orchestrating agents and tools to automate everyday tasks",
    );
    map.insert("cv.projects.kairos.status", "Functional MVP");

    // CV project item (LedgerBurner)
    map.insert("cv.projects.nora.name", "Nora");
    map.insert(
        "cv.projects.nora.description",
        "Self-hosted financial dashboard built on Firefly III: multi-account aggregation, transparent financial health scores, fully self-hosted architecture",
    );
    map.insert("cv.projects.nora.status", "V1 in production");

    // CV skill categories
    map.insert("cv.skills.categories.backend", "Backend");
    map.insert("cv.skills.categories.frontend", "Frontend");
    map.insert("cv.skills.categories.database", "Database");
    map.insert("cv.skills.categories.devops", "DevOps");
    map.insert("cv.skills.categories.learning", "Learning");

    // CV education

    // Master
    map.insert(
        "cv.education.master.degree",
        "Master Manager of Digital Solutions and Data",
    );
    map.insert("cv.education.master.school", "CS2I Lorient");
    map.insert("cv.education.master.period", "2020 - 2022");
    map.insert(
        "cv.education.master.details",
        "Work-study program, development specialization",
    );

    // License
    map.insert(
        "cv.education.bachelor.degree",
        "Bachelor Information Systems Designer",
    );
    map.insert("cv.education.bachelor.school", "CS2I Lorient");
    map.insert("cv.education.bachelor.period", "2019 - 2020");
    map.insert(
        "cv.education.bachelor.details",
        "Work-study program, development specialization",
    );

    // BTS
    map.insert(
        "cv.education.bts.degree",
        "BTS Computer Systems for Organizations",
    );
    map.insert("cv.education.bts.school", "Saint-Louis Châteaulin");
    map.insert("cv.education.bts.period", "2017 - 2019");
    map.insert(
        "cv.education.bts.details",
        "Infrastructure Solutions option",
    );

    // CV languages
    map.insert("cv.languages.french.name", "French");
    map.insert("cv.languages.french.level", "Native");

    map.insert("cv.languages.english.name", "English");
    map.insert("cv.languages.english.level", "TOEIC 885/990");

    // CV interests
    map.insert("cv.interests.hiking", "🥾 Hiking");
    map.insert("cv.interests.gardening", "🌿 Gardening");
    map.insert("cv.interests.diy", "🔧 DIY Projects");
    map.insert("cv.interests.linux", "🐧 Linux");

    // CV footer
    map.insert("cv.footer", "Available for a 100% remote position • Flexible notice period • Complete portfolio on www.bourbasquetkev.in");

    // Blog
    map.insert("blogTitle", "Technical Blog");
    map.insert(
        "blogSubtitle",
        "Thoughts, tutorials, and insights from my development journey",
    );
    map.insert("readMore", "Read more");
    map.insert("blogNavigation", "Blog");
    map.insert("categories", "Categories");
    map.insert("tags", "Tags");

    // Article navigation
    map.insert("previousArticle", "Previous article");
    map.insert("nextArticle", "Next article");
    map.insert("shareArticle", "Share this article");

    // Categories
    map.insert("categorySystem", "System");
    map.insert("categoryHardware", "Hardware");
    map.insert("categoryDevops", "DevOps");
    map.insert("categoryTools", "Tools");
    map.insert("categoryProject", "Projects");

    // Blog Hero
    map.insert("blogBadge", "Technical Blog");
    map.insert("blogHeroTitle1", "Sharing knowledge");
    map.insert("blogHeroTitle2", "and experiences");
    map.insert("blogHeroDescription", "Thoughts, tutorials, and insights from my development journey. From system administration to development practices, discover my learnings and projects.");
    map.insert("articlesPublished", "Articles Published");
    map.insert("minutesReading", "Minutes of Reading");

    // Latest Article
    map.insert("latestArticle", "Latest Article");
    map.insert("readArticle", "Read Article");

    // Articles List
    map.insert("allArticlesTitle", "All Articles");
    map.insert("allCategories", "All Categories");
    map.insert("sortNewest", "Newest First");
    map.insert("sortOldest", "Oldest First");
    map.insert("sortReadingTime", "Reading Time");

    // Error 404 Article
    map.insert("error404Title", "404 - Article Not Found");
    map.insert(
        "error404Message",
        "Sorry, this article doesn't exist or has been removed.",
    );
    map.insert("backToBlog", "Back to Blog");

    // FAB Navigation
    map.insert("navigation.home", "Home");
    map.insert("navigation.blog", "Blog");
    map.insert("navigation.cv", "CV");
    map.insert("navigation.language", "Language");
    map.insert("navigation.theme", "Theme");
    map.insert("navigation.veille", "Tech Watch");

    // Veille page
    map.insert("veille.title", "Tech Watch");
    map.insert("veille.subtitle", "Daily curated tech news");
    map.insert("veille.updatedAt", "Updated");
    map.insert("veille.loading", "Fetching news…");
    map.insert("veille.error", "Unable to load news. Try again later.");
    map.insert("veille.filterAll", "All");
    map.insert("veille.filterAI", "AI");
    map.insert("veille.filterUrgent", "Urgent");
    map.insert("veille.filterGoodNews", "Good News");
    map.insert("veille.filterFutureWatch", "Future Watch");
    map.insert("veille.filterStackAlt", "Stack Alternatives");
    map.insert("veille.filterGeneral", "General");
    map.insert("veille.noItems", "No items in this category.");
    map.insert("veille.synthesis.aiGenerated", "Generated by Claude AI");
    map.insert("veille.synthesis.aiBadge", "AI Brief");
    map.insert("veille.synthesis.forecastBadge", "Forecast");
    map.insert("veille.synthesis.vectorsTitle", "Trend vectors");
    map.insert("veille.synthesis.historicalAnalogue", "Historical analogue");
    map.insert("veille.synthesis.soWhat", "What to do");
    map.insert("veille.synthesis.confidence", "Confidence");
    map.insert("veille.synthesis.leadingIndicator", "Leading indicator");
    map.insert("veille.synthesis.falsifier", "What would prove this wrong");
    map.insert("veille.synthesis.weekOf", "Week of");
    map.insert("veille.synthesis.to", "to");
    map.insert("veille.synthesis.showSources", "Sources");
    map.insert("veille.synthesis.hideSources", "Hide sources");
    map.insert("veille.synthesis.readMore", "Read synthesis →");
    map.insert("veille.synthesis.basedOn", "Based on");
    map.insert("veille.synthesis.articles", "articles");
    map.insert("veille.synthesis.backToFeed", "← Back to feed");
    map.insert("veille.synthesis.filterLabel", "Syntheses");
    map.insert("veille.synthesis.similar", "Similar articles");

    // PGP contact
    map.insert("pgpDownload", "Download key");
    map.insert("pgpKeyserver", "Keyserver");
    map.insert("e2eNote", "End-to-end encrypted with PGP");

    // Article page
    map.insert("minRead", "min read");
    map.insert("tldr", "TL;DR: ");
    map.insert("tableOfContents", "Table of Contents");
    map.insert("contentNotAvailable", "Content not available in this language.");

    // Project tags
    map.insert("tag.startup", "Startup");
    map.insert("tag.fullStack", "Full-Stack");
    map.insert("tag.bot", "Bot");
    map.insert("tag.hardware", "Hardware");
    map.insert("tag.diy", "DIY");

    // Navigation aria-labels & tooltips
    map.insert("switchToFrench", "Switch to French");
    map.insert("switchToEnglish", "Switch to English");
    map.insert("toggleTheme", "Toggle theme");
    map.insert("navigation.menu", "Navigation menu");

    // 404 generic page
    map.insert("pageNotFound", "Page Not Found");
    map.insert("error404Generic", "Oops! The page you're looking for seems to have vanished into the digital void.");
    map.insert("goHome", "Go Home");
    map.insert("exploreBlog", "Explore Blog");

    // Blog filters
    map.insert("blog.filter.category", "Filter by category");
    map.insert("blog.filter.sort", "Sort articles");

    // Footer
    map.insert("mitLicense", "MIT License");

    map
}
