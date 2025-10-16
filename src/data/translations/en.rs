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
    map.insert("loader", "Opening print...");
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
    map.insert("cv.title", "Full-Stack Developer");
    map.insert("cv.phoneAvailable", "Available on request");
    map.insert("cv.locationMove", "Brest → Bordeaux");
    map.insert("cv.drivingLicense", "Driving License");
    map.insert("cv.summary", "Full-stack developer, {age} years old, with 5 years of experience in web development. Expertise in Symfony/React, agile methods, and passion for technical innovation. Seeking new challenges in Bordeaux.");

    // CV Section titles
    map.insert("cv.experienceTitle", "Professional Experience");
    map.insert("cv.projectsTitle", "Key Projects");
    map.insert("cv.skillsTitle", "Skills");
    map.insert("cv.educationTitle", "Education");
    map.insert("cv.languagesTitle", "Languages");
    map.insert("cv.interestsTitle", "Interests");

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
