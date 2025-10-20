use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Basic info
    map.insert("name", "Kévin Bourbasquet");
    map.insert("badge", "Open to opportunities");
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
    map.insert("aboutText1", "I'm a {age}-year-old fullstack developer who believes in building things that actually work. My sweet spot is crafting robust backends with Symfony API Platform while creating intuitive frontends with React and TypeScript.");
    map.insert("aboutText2", "Beyond the usual stack, I'm exploring Rust for performance-critical projects and integrating AI workflows via OpenRouter. When I'm not coding, you'll find me on Brittany's coastal trails, building custom keyboards, or tending to my ever-growing plant collection.");

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
    map.insert("photographyDescription", "Capturing moments during treks and daily life. There's something about freezing time that appeals to the developer mindset.");
    map.insert("musicTitle", "Music");
    map.insert("musicDescription", "Guitar and piano sessions for mental breaks. Music and code both follow patterns—understanding one helps with the other.");
    map.insert("trekkingTitle", "Trekking & Hiking");
    map.insert("trekkingDescription", "Regular escapes to coastal paths and countryside trails. Nothing clears the mind like a good hike by the sea—plus it's where my best debugging ideas happen.");
    map.insert("linuxTitle", "Linux Tinkerer");
    map.insert("linuxDescription", "Daily Manjaro user who actually enjoys configuring systems. If it runs on Linux and can be optimized, I'm probably interested.");

    // Contact
    map.insert("contactTitle", "Let's Build Something");
    map.insert("contactDescription", "Always up for discussing interesting projects, sharing knowledge, or just talking tech. Whether you need a technical co-founder or want to collaborate on open source work.");
    map.insert("emailLabel", "Email");
    map.insert("nameLabel", "Your Name");
    map.insert("emailPlaceholder", "your.email@domain.com");
    map.insert("messageLabel", "What's on your mind?");
    map.insert("sendMessage", "Send Message");

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
    map.insert("cv.personal.title", "Full-Stack Developer PLIP");
    map.insert("cv.personal.email", "bourbasquet.k@etik.com");
    map.insert("cv.personal.phone", "Available on request");
    map.insert("cv.personal.location", "Brest → Bordeaux");
    map.insert("cv.personal.license", "Driving License");
    map.insert("cv.personal.portfolio", "bourbask.github.io");
    map.insert("cv.summary", "Full-stack developer, {age} years old, with 5 years of experience in web development. Expertise in Symfony/React, agile methods, and passion for technical innovation. Seeking new challenges in Bordeaux.");

    // CV Section titles
    map.insert("cv.sections.experience", "Professional Experience");
    map.insert("cv.sections.projects", "Key Projects");
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
        "Development of applications for legal announcements and legal formalities",
    );
    map.insert(
        "cv.experience.0.achievements.1",
        "Technical stack: Symfony, ReactJS, MySQL/MariaDB, REST API",
    );
    map.insert(
        "cv.experience.0.achievements.2",
        "Agile methods, collaboration with GitLab, Redux integration",
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
        "Stack: Symfony, Vue.js, Node.js, custom web components",
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
        "Saas platform connecting 3D printing services and clients",
    );
    map.insert("cv.projects.ezprint3d.status", "In development");

    // CV project item (lkf bot)
    map.insert("cv.projects.lekickerfou.name", "LeKickerFou Bot");
    map.insert(
        "cv.projects.lekickerfou.description",
        "High-performance Discord bot for voice channel management",
    );
    map.insert("cv.projects.lekickerfou.status", "Deployed");

    // CV project item (Excelsior)
    map.insert("cv.projects.excelsior.name", "Excelsior");
    map.insert(
        "cv.projects.excelsior.description",
        "Narrative horror game based on journey in the Pyrenees",
    );
    map.insert("cv.projects.excelsior.status", "In development");

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
    map.insert("cv.languages.english.level", "Professional");

    // CV interests
    map.insert("cv.interests.hiking", "🥾 Hiking");
    map.insert("cv.interests.gardening", "🌿 Gardening");
    map.insert("cv.interests.diy", "🔧 DIY Projects");
    map.insert("cv.interests.linux", "🐧 Linux");

    // CV footer
    map.insert("cv.footer", "Available for a position in Bordeaux • Flexible notice period • Complete portfolio on bourbask.github.io");

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

    map
}
