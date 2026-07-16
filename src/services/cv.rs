use crate::data::cv::*;
use crate::services::I18nService;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Clone)]
pub struct CVService {
    i18n: I18nService,
}

impl CVService {
    pub fn new(i18n: I18nService) -> Self {
        Self { i18n }
    }

    /// Obtenir les données CV avec traductions
    pub fn get_cv_data(&self) -> CVData {
        CVData {
            personal: PersonalInfo {
                name: self.i18n.t("cv.personal.name"),
                title: self.i18n.t("cv.personal.title"),
                email: self.i18n.t("cv.personal.email"),
                phone: self.i18n.t("cv.personal.phone"),
                location: self.i18n.t("cv.personal.location"),
                portfolio: self.i18n.t("cv.personal.portfolio"),
                license: self.i18n.t("cv.personal.license"),
            },
            summary: self.i18n.t("cv.summary"),
            section_titles: SectionTitles {
                experience: self.i18n.t("cv.sections.experience"),
                projects: self.i18n.t("cv.sections.projects"),
                skills: self.i18n.t("cv.sections.skills"),
                education: self.i18n.t("cv.sections.education"),
                languages: self.i18n.t("cv.sections.languages"),
                interests: self.i18n.t("cv.sections.interests"),
            },
            experience: self.get_experience_data(),
            skill_categories: self.get_skill_categories(),
            skills: self.get_skills_data(),
            projects: self.get_projects_data(),
            education: self.get_education_data(),
            languages: self.get_languages_data(),
            interests: self.get_interests_data(),
            footer: self.i18n.t("cv.footer"),
        }
    }

    /// Générer le HTML du CV
    pub fn generate_cv_html(&self) -> String {
        let data = self.get_cv_data();

        format!(
            r#"
        <div class="cv-container">
            <header class="cv-header">
                <div class="cv-header-content">
                    <div class="cv-profile">
                        <h1 class="cv-name">{}</h1>
                        <h2 class="cv-title">{}</h2>
                        <p class="cv-summary">{}</p>
                    </div>
                    <div class="cv-contact">
                        <div class="contact-item">
                            <span class="contact-icon">📧</span>
                            <span>{}</span>
                        </div>
                        <div class="contact-item">
                            <span class="contact-icon">📱</span>
                            <span>{}</span>
                        </div>
                        <div class="contact-item">
                            <span class="contact-icon">📍</span>
                            <span>{}</span>
                        </div>
                        <div class="contact-item">
                            <span class="contact-icon">🚗</span>
                            <span>{}</span>
                        </div>
                        <div class="contact-item">
                            <span class="contact-icon">🌐</span>
                            <span>{}</span>
                        </div>
                    </div>
                </div>
            </header>

            <div class="cv-body">
                <div class="cv-main">
                    <section class="cv-section">
                        <h3 class="section-title">
                            <span class="section-icon">🚀</span>
                            <span>{}</span>
                        </h3>
                        {}
                    </section>

                    <section class="cv-section">
                        <h3 class="section-title">
                            <span class="section-icon">💡</span>
                            <span>{}</span>
                        </h3>
                        <div class="projects-grid">
                            {}
                        </div>
                    </section>
                </div>

                <div class="cv-sidebar">
                    {}
                    {}
                    {}
                    {}
                </div>
            </div>

            <footer class="cv-footer">
                <div class="footer-text">{}</div>
            </footer>
        </div>
        "#,
            data.personal.name,
            data.personal.title,
            data.summary,
            data.personal.email,
            data.personal.phone,
            data.personal.location,
            data.personal.license,
            data.personal.portfolio,
            data.section_titles.experience,
            self.render_experience(&data.experience),
            data.section_titles.projects,
            self.render_projects(&data.projects),
            self.render_skills(&data),
            self.render_education(&data.education, &data.section_titles.education),
            self.render_languages(&data.languages, &data.section_titles.languages),
            self.render_interests(&data.interests, &data.section_titles.interests),
            data.footer
        )
    }

    /// CSS complet pour le CV (repris de ton JavaScript)
    pub fn get_cv_styles(&self) -> &'static str {
        r#"
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');
        
        @page {
            size: A4;
            margin: 0;
        }
        
        @media print {
            html, body {
                width: 210mm;
                min-height: 297mm;
                margin: 0;
                padding: 0;
            }
            * {
                -webkit-print-color-adjust: exact !important;
                color-adjust: exact !important;
                print-color-adjust: exact !important;
            }
        }
        
        .cv-container {
            font-family: 'Inter', sans-serif;
            width: 210mm;
            min-height: 297mm;
            background: white;
            color: #1a1a1a;
            font-size: 12px;
            line-height: 1.35;
            padding: 0;
            margin: 0;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;
        }

        .cv-header {
            background: #2d6a4f !important;
            color: white;
            padding: 16px 30px;
            position: relative;
            overflow: hidden;
            flex-shrink: 0;
            -webkit-print-color-adjust: exact !important;
            print-color-adjust: exact !important;
        }

        .cv-header::before {
            content: "🌿";
            position: absolute;
            top: -15px;
            right: -5px;
            font-size: 90px;
            color: #52b788;
            opacity: 0.25;
            transform: rotate(-15deg);
            z-index: 1;
            filter: brightness(1.3);
        }

        .cv-header::after {
            content: "🍃";
            position: absolute;
            bottom: -25px;
            left: -15px;
            font-size: 70px;
            color: #52b788;
            opacity: 0.2;
            transform: rotate(25deg);
            z-index: 1;
            filter: brightness(1.2);
        }

        .cv-header-content {
            position: relative;
            z-index: 2;
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 30px;
        }

        .cv-profile { flex: 2; }
        .cv-contact { flex: 1; }

        .cv-name {
            font-size: 30px;
            font-weight: 700;
            margin: 0 0 5px 0;
            color: white !important;
            text-shadow: 0 1px 2px rgba(0,0,0,0.1);
        }

        .cv-title {
            font-size: 15px;
            font-weight: 400;
            margin: 0 0 10px 0;
            color: white !important;
            opacity: 0.95;
        }

        .cv-summary {
            font-size: 12px;
            line-height: 1.45;
            margin: 0;
            color: white !important;
            opacity: 0.92;
        }

        .contact-item {
            display: flex;
            align-items: center;
            gap: 16px;
            font-size: 11.5px;
            color: white !important;
            margin-top: 6px;
            margin-bottom: 6px;
            opacity: 0.9;
        }

        .contact-icon { 
            width: 14px; 
            text-align: center; 
        }

        .cv-body {
            display: flex;
            gap: 20px;
            padding: 14px 30px;
            flex: 1;
            min-height: 0;
        }

        .cv-main {
            flex: 2.2;
            display: flex;
            flex-direction: column;
            gap: 10px;
        }

        .cv-sidebar {
            flex: 0.8;
            display: flex;
            flex-direction: column;
            gap: 4px;
        }

        .cv-section {
            margin-bottom: 0;
            page-break-inside: avoid;
            break-inside: avoid;
        }

        .section-title {
            font-size: 14px;
            font-weight: 600;
            margin: 0 0 8px 0;
            color: #2d6a4f !important;
            display: flex;
            align-items: center;
            gap: 8px;
            border-bottom: 2px solid #e2e8f0;
            padding-bottom: 4px;
        }

        .section-icon {
            font-size: 13px;
        }

        .experience-item {
            background: #fafbfc;
            border: 1px solid #e2e8f0;
            border-radius: 6px;
            padding: 7px;
            margin-bottom: 8px;
            transition: box-shadow 0.2s;
            page-break-inside: avoid;
            break-inside: avoid;
        }

        .experience-item:hover {
            box-shadow: 0 2px 8px rgba(0,0,0,0.08);
        }

        .exp-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 4px;
        }

        .exp-title-group {
            flex: 1;
        }

        .exp-title {
            font-size: 13.5px;
            font-weight: 600;
            color: #1e293b;
            margin: 0 0 3px 0;
        }

        .exp-company {
            font-size: 11px;
            color: #64748b;
            font-weight: 500;
        }

        .exp-period {
            font-size: 10.5px;
            color: #2d6a4f !important;
            font-weight: 600;
            background: #dcfce7 !important;
            padding: 3px 8px;
            border-radius: 12px;
            white-space: nowrap;
            -webkit-print-color-adjust: exact !important;
        }

        .exp-achievements {
            margin: 0;
            padding-left: 16px;
        }

        .exp-achievements li {
            margin-bottom: 1px;
            color: #475569;
            font-size: 11px;
            line-height: 1.35;
        }

        .projects-grid {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .project-item {
            background: white;
            border: 1px solid #d1d5db;
            border-radius: 6px;
            padding: 6px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
            page-break-inside: avoid;
            break-inside: avoid;
        }

        .project-header {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 4px;
        }

        .project-name {
            font-size: 13px;
            font-weight: 600;
            margin: 0;
            color: #1e293b;
        }

        .project-status {
            font-size: 9.5px;
            color: #2d6a4f !important;
            font-weight: 600;
            background: #dcfce7 !important;
            padding: 2px 6px;
            border-radius: 8px;
            -webkit-print-color-adjust: exact !important;
        }

        .project-desc {
            font-size: 10.5px;
            color: #64748b;
            margin: 0 0 4px 0;
            line-height: 1.3;
        }

        .project-tech {
            font-size: 9.5px;
            color: #2d6a4f !important;
            font-weight: 500;
        }

        .skill-category {
            margin-bottom: 3px;
            page-break-inside: avoid;
            break-inside: avoid;
        }

        .skill-category h4 {
            font-size: 10.5px;
            font-weight: 600;
            margin: 0 0 2px 0;
            color: #374151;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .skill-list {
            font-size: 10px;
            color: #64748b;
            line-height: 1.3;
            margin-bottom: 2px;
        }

        .skill-list.learning {
            color: #d97706;
            font-style: italic;
        }

        .education-item {
            background: #f8fafc;
            border-radius: 6px;
            padding: 5px;
            margin-bottom: 3px;
            page-break-inside: avoid;
            break-inside: avoid;
        }

        .edu-degree {
            font-size: 10.5px;
            font-weight: 600;
            margin: 0 0 3px 0;
            color: #1e293b;
            line-height: 1.25;
        }

        .edu-school {
            font-size: 10.5px;
            color: #2d6a4f !important;
            font-weight: 600;
            margin-bottom: 2px;
        }

        .edu-period {
            font-size: 9.5px;
            color: #64748b;
            margin-bottom: 3px;
        }

        .edu-details {
            font-size: 9.5px;
            color: #64748b;
            margin: 0;
            font-style: italic;
        }

        .languages-grid {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }

        .language-item {
            display: flex;
            flex-direction: column;
            gap: 0;
            padding: 2px 4px;
            background: #f1f5f9;
            border-radius: 4px;
            font-size: 10.5px;
        }

        .lang-name { 
            font-weight: 600; 
            color: #374151;
        }
        
        .lang-level { 
            color: #64748b; 
            font-weight: 500;
        }

        .interests-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 3px;
        }

        .interest-item {
            font-size: 9.5px;
            color: #64748b;
            padding: 1px;
        }

        .cv-footer {
            background: #f8fafc;
            padding: 6px 30px;
            text-align: center;
            border-top: 1px solid #e2e8f0;
            flex-shrink: 0;
        }

        .footer-text {
            font-size: 9.5px;
            color: #64748b;
            font-style: italic;
            font-weight: 500;
        }
        "#
    }
    pub fn generate_pdf(&self) {
        let html_content = self.generate_cv_html();
        let styles = self.get_cv_styles();

        if let Some(window) = window() {
            match window.open_with_url_and_target_and_features("", "_blank", "width=800,height=600")
            {
                Ok(Some(print_window)) => {
                    if let Some(document) = print_window.document() {
                        // ✅ Créer les éléments directement
                        if let (Ok(head), Ok(body)) = (
                            document.head().ok_or("No head"),
                            document.body().ok_or("No body"),
                        ) {
                            // Ajouter les styles
                            if let Ok(style_element) = document.create_element("style") {
                                style_element.set_text_content(Some(styles));
                                let _ = head.append_child(&style_element);
                            }

                            // Ajouter le contenu HTML
                            body.set_inner_html(&html_content);

                            // Lancer l'impression
                            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                                let _ = print_window.focus();
                                let _ = print_window.print();
                            })
                                as Box<dyn FnMut()>);

                            window
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    closure.as_ref().unchecked_ref(),
                                    1000,
                                )
                                .ok();

                            closure.forget();

                            web_sys::console::log_1(&"CV window created successfully".into());
                        }
                    }
                }
                _ => {
                    web_sys::console::error_1(&"Failed to open window".into());
                }
            }
        }
    }

    // ========== MÉTHODES DE RENDU HTML ==========

    fn render_experience(&self, experiences: &[Experience]) -> String {
        experiences
            .iter()
            .map(|exp| {
                format!(
                    r#"
                <div class="experience-item">
                    <div class="exp-header">
                        <div class="exp-title-group">
                            <h4 class="exp-title">{}</h4>
                            <div class="exp-company">{} • {}</div>
                        </div>
                        <span class="exp-period">{}</span>
                    </div>
                    <ul class="exp-achievements">
                        {}
                    </ul>
                </div>
            "#,
                    exp.title,
                    exp.company,
                    exp.location,
                    exp.period,
                    exp.achievements
                        .iter()
                        .map(|a| format!("<li>{}</li>", a))
                        .collect::<Vec<_>>()
                        .join("")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn render_projects(&self, projects: &[Project]) -> String {
        projects
            .iter()
            .map(|proj| {
                format!(
                    r#"
                <div class="project-item">
                    <div class="project-header">
                        <h4 class="project-name">{}</h4>
                        <span class="project-status">{}</span>
                    </div>
                    <p class="project-desc">{}</p>
                    <div class="project-tech">{}</div>
                </div>
            "#,
                    proj.name,
                    proj.status,
                    proj.description,
                    proj.tech.join(" • ")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn render_skills(&self, data: &CVData) -> String {
        let skill_sections = [
            ("backend", &data.skills.backend),
            ("frontend", &data.skills.frontend),
            ("database", &data.skills.database),
            ("devops", &data.skills.devops),
            ("learning", &data.skills.learning),
        ];

        let skills_html = skill_sections
            .iter()
            .map(|(key, skills)| {
                let title = match *key {
                    "backend" => &data.skill_categories.backend,
                    "frontend" => &data.skill_categories.frontend,
                    "database" => &data.skill_categories.database,
                    "devops" => &data.skill_categories.devops,
                    "learning" => &data.skill_categories.learning,
                    _ => "",
                };

                let class = if *key == "learning" { " learning" } else { "" };

                format!(
                    r#"
                <div class="skill-category">
                    <h4>{}</h4>
                    <div class="skill-list{}">{}</div>
                </div>
            "#,
                    title,
                    class,
                    skills.join(" • ")
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"
            <section class="cv-section">
                <h3 class="section-title">
                    <span class="section-icon">🛠️</span>
                    <span>{}</span>
                </h3>
                {}
            </section>
        "#,
            data.section_titles.skills, skills_html
        )
    }

    fn render_education(&self, education: &[Education], title: &str) -> String {
        let education_html = education
            .iter()
            .map(|edu| {
                format!(
                    r#"
                <div class="education-item">
                    <h4 class="edu-degree">{}</h4>
                    <div class="edu-school">{}</div>
                    <div class="edu-period">{}</div>
                    <p class="edu-details">{}</p>
                </div>
            "#,
                    edu.degree, edu.school, edu.period, edu.details
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"
            <section class="cv-section">
                <h3 class="section-title">
                    <span class="section-icon">🎓</span>
                    <span>{}</span>
                </h3>
                {}
            </section>
        "#,
            title, education_html
        )
    }

    fn render_languages(&self, languages: &[LanguageSkill], title: &str) -> String {
        let languages_html = languages
            .iter()
            .map(|lang| {
                format!(
                    r#"
                <div class="language-item">
                    <span class="lang-name">{}</span>
                    <span class="lang-level">{}</span>
                </div>
            "#,
                    lang.name, lang.level
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"
            <section class="cv-section">
                <h3 class="section-title">
                    <span class="section-icon">🌍</span>
                    <span>{}</span>
                </h3>
                <div class="languages-grid">
                    {}
                </div>
            </section>
        "#,
            title, languages_html
        )
    }

    fn render_interests(&self, interests: &[String], title: &str) -> String {
        let interests_html = interests
            .iter()
            .map(|interest| format!("<div class=\"interest-item\">{}</div>", interest))
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"
            <section class="cv-section">
                <h3 class="section-title">
                    <span class="section-icon">🎯</span>
                    <span>{}</span>
                </h3>
                <div class="interests-grid">
                    {}
                </div>
            </section>
        "#,
            title, interests_html
        )
    }

    fn get_experience_data(&self) -> Vec<Experience> {
        vec![
            Experience {
                title: self.i18n.t("cv.experience.0.title"),
                company: self.i18n.t("cv.experience.0.company"),
                location: self.i18n.t("cv.experience.0.location"),
                period: self.i18n.t("cv.experience.0.period"),
                achievements: vec![
                    self.i18n.t("cv.experience.0.achievements.0"),
                    self.i18n.t("cv.experience.0.achievements.1"),
                    self.i18n.t("cv.experience.0.achievements.2"),
                ],
            },
            Experience {
                title: self.i18n.t("cv.experience.1.title"),
                company: self.i18n.t("cv.experience.1.company"),
                location: self.i18n.t("cv.experience.1.location"),
                period: self.i18n.t("cv.experience.1.period"),
                achievements: vec![
                    self.i18n.t("cv.experience.1.achievements.0"),
                    self.i18n.t("cv.experience.1.achievements.1"),
                    self.i18n.t("cv.experience.1.achievements.2"),
                ],
            },
            Experience {
                title: self.i18n.t("cv.experience.2.title"),
                company: self.i18n.t("cv.experience.2.company"),
                location: self.i18n.t("cv.experience.2.location"),
                period: self.i18n.t("cv.experience.2.period"),
                achievements: vec![
                    self.i18n.t("cv.experience.2.achievements.0"),
                    self.i18n.t("cv.experience.2.achievements.1"),
                    self.i18n.t("cv.experience.2.achievements.2"),
                ],
            },
        ]
    }

    fn get_skill_categories(&self) -> SkillCategories {
        SkillCategories {
            backend: self.i18n.t("cv.skills.categories.backend"),
            frontend: self.i18n.t("cv.skills.categories.frontend"),
            database: self.i18n.t("cv.skills.categories.database"),
            devops: self.i18n.t("cv.skills.categories.devops"),
            learning: self.i18n.t("cv.skills.categories.learning"),
        }
    }

    fn get_skills_data(&self) -> Skills {
        Skills {
            backend: vec![
                "Node.js".to_string(),
                "Express".to_string(),
                "Rust".to_string(),
                "Laravel".to_string(),
                "FastAPI".to_string(),
            ],
            frontend: vec![
                "React".to_string(),
                "Vue.js".to_string(),
                "Leptos".to_string(),
                "HTML5".to_string(),
                "CSS3".to_string(),
                "SASS".to_string(),
                "Tailwind".to_string(),
                "Responsive Design".to_string(),
            ],
            database: vec![
                "MySQL".to_string(),
                "PostgreSQL".to_string(),
                "MongoDB".to_string(),
                "Redis".to_string(),
            ],
            devops: vec!["Docker".to_string(), "CI/CD".to_string(), "AWS".to_string()],
            learning: vec![
                "Rust".to_string(),
                "Kubernetes".to_string(),
                "Microservices".to_string(),
                "Claude Code".to_string(),
                "Prompt Engineering".to_string(),
            ],
        }
    }

    fn get_projects_data(&self) -> Vec<Project> {
        vec![
            Project {
                name: self.i18n.t("cv.projects.ezprint3d.name"),
                status: self.i18n.t("cv.projects.ezprint3d.status"),
                description: self.i18n.t("cv.projects.ezprint3d.description"),
                tech: vec![
                    "Symfony".to_string(),
                    "React".to_string(),
                    "Docker".to_string(),
                    "PostgreSQL".to_string(),
                ],
            },
            Project {
                name: self.i18n.t("cv.projects.kairos.name"),
                status: self.i18n.t("cv.projects.kairos.status"),
                description: self.i18n.t("cv.projects.kairos.description"),
                tech: vec![
                    "Rust".to_string(),
                    "LLM API".to_string(),
                    "Architecture plugin".to_string(),
                ],
            },
            Project {
                name: self.i18n.t("cv.projects.ledgerburner.name"),
                status: self.i18n.t("cv.projects.ledgerburner.status"),
                description: self.i18n.t("cv.projects.ledgerburner.description"),
                tech: vec![
                    "Python".to_string(),
                    "LLM local".to_string(),
                    "SQLite".to_string(),
                ],
            },
        ]
    }

    fn get_education_data(&self) -> Vec<Education> {
        vec![
            Education {
                degree: self.i18n.t("cv.education.master.degree"),
                school: self.i18n.t("cv.education.master.school"),
                period: self.i18n.t("cv.education.master.period"),
                details: self.i18n.t("cv.education.master.details"),
            },
            Education {
                degree: self.i18n.t("cv.education.bachelor.degree"),
                school: self.i18n.t("cv.education.bachelor.school"),
                period: self.i18n.t("cv.education.bachelor.period"),
                details: self.i18n.t("cv.education.bachelor.details"),
            },
            Education {
                degree: self.i18n.t("cv.education.bts.degree"),
                school: self.i18n.t("cv.education.bts.school"),
                period: self.i18n.t("cv.education.bts.period"),
                details: self.i18n.t("cv.education.bts.details"),
            },
        ]
    }

    fn get_languages_data(&self) -> Vec<LanguageSkill> {
        vec![
            LanguageSkill {
                name: self.i18n.t("cv.languages.french.name"),
                level: self.i18n.t("cv.languages.french.level"),
            },
            LanguageSkill {
                name: self.i18n.t("cv.languages.english.name"),
                level: self.i18n.t("cv.languages.english.level"),
            },
        ]
    }

    fn get_interests_data(&self) -> Vec<String> {
        vec![
            self.i18n.t("cv.interests.hiking"),
            self.i18n.t("cv.interests.gardening"),
            self.i18n.t("cv.interests.diy"),
            self.i18n.t("cv.interests.linux"),
        ]
    }
}
