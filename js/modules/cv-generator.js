/**
 * CV Generator
 * Optimized for full A4 page layout
 */
class CVGenerator {
  constructor(i18nManager) {
    this.i18nManager = i18nManager;
    this.currentLang = i18nManager.currentLang;
  }

  /**
   * Get processed CV data with translations and dynamic age
   */
  getCVData() {
    const cvTranslations = this.i18nManager.processTranslations(
      this.i18nManager.translations[this.currentLang].cv
    );

    return {
      personal: {
        name: this.i18nManager.translations[this.currentLang].name,
        title: cvTranslations.title,
        email: "bourbasquet.k@etik.com",
        phone: cvTranslations.phoneAvailable,
        location: cvTranslations.locationMove,
        portfolio: "bourbask.github.io",
        license: cvTranslations.drivingLicense,
      },
      summary: cvTranslations.summary,
      sectionTitles: {
        experience: cvTranslations.experienceTitle,
        projects: cvTranslations.projectsTitle,
        skills: cvTranslations.skillsTitle,
        education: cvTranslations.educationTitle,
        languages: cvTranslations.languagesTitle,
        interests: cvTranslations.interestsTitle,
      },
      experience: cvTranslations.experiences,
      skillCategories: cvTranslations.skillCategories,
      skills: cvTranslations.skills,
      projects: cvTranslations.projects,
      education: cvTranslations.education,
      languages: cvTranslations.languagesList,
      interests: cvTranslations.interestsList,
      footer: cvTranslations.footer,
    };
  }

  /**
   * Generate complete CV HTML structure
   */
  generateCVHTML() {
    const data = this.getCVData();

    return `
            <div class="cv-container">
                <header class="cv-header">
                    <div class="cv-header-content">
                        <div class="cv-profile">
                            <h1 class="cv-name">${data.personal.name}</h1>
                            <h2 class="cv-title">${data.personal.title}</h2>
                            <p class="cv-summary">${data.summary}</p>
                        </div>
                        <div class="cv-contact">
                            <div class="contact-item">
                                <span class="contact-icon">📧</span>
                                <span>${data.personal.email}</span>
                            </div>
                            <div class="contact-item">
                                <span class="contact-icon">📱</span>
                                <span>${data.personal.phone}</span>
                            </div>
                            <div class="contact-item">
                                <span class="contact-icon">📍</span>
                                <span>${data.personal.location}</span>
                            </div>
                            <div class="contact-item">
                                <span class="contact-icon">🚗</span>
                                <span>${data.personal.license}</span>
                            </div>
                            <div class="contact-item">
                                <span class="contact-icon">🌐</span>
                                <span>${data.personal.portfolio}</span>
                            </div>
                        </div>
                    </div>
                </header>

                <div class="cv-body">
                    <div class="cv-main">
                        <section class="cv-section">
                            <h3 class="section-title">
                                <span class="section-icon">🚀</span>
                                <span>${data.sectionTitles.experience}</span>
                            </h3>
                            ${data.experience
                              .map(
                                (exp) => `
                                <div class="experience-item">
                                    <div class="exp-header">
                                        <div class="exp-title-group">
                                            <h4 class="exp-title">${
                                              exp.title
                                            }</h4>
                                            <div class="exp-company">${
                                              exp.company
                                            } • ${exp.location}</div>
                                        </div>
                                        <span class="exp-period">${
                                          exp.period
                                        }</span>
                                    </div>
                                    <ul class="exp-achievements">
                                        ${exp.achievements
                                          .map(
                                            (achievement) =>
                                              `<li>${achievement}</li>`
                                          )
                                          .join("")}
                                    </ul>
                                </div>
                            `
                              )
                              .join("")}
                        </section>

                        <section class="cv-section">
                            <h3 class="section-title">
                                <span class="section-icon">💡</span>
                                <span>${data.sectionTitles.projects}</span>
                            </h3>
                            <div class="projects-grid">
                                ${data.projects
                                  .map(
                                    (project) => `
                                    <div class="project-item">
                                        <div class="project-header">
                                            <h4 class="project-name">${
                                              project.name
                                            }</h4>
                                            <span class="project-status">${
                                              project.status
                                            }</span>
                                        </div>
                                        <p class="project-desc">${
                                          project.description
                                        }</p>
                                        <div class="project-tech">${project.tech.join(
                                          " • "
                                        )}</div>
                                    </div>
                                `
                                  )
                                  .join("")}
                            </div>
                        </section>
                    </div>

                    <div class="cv-sidebar">
                        <section class="cv-section">
                            <h3 class="section-title">
                                <span class="section-icon">🛠️</span>
                                <span>${data.sectionTitles.skills}</span>
                            </h3>
                            ${Object.entries(data.skillCategories)
                              .map(
                                ([key, title]) => `
                                <div class="skill-category">
                                    <h4>${title}</h4>
                                    <div class="skill-list${
                                      key === "learning" ? " learning" : ""
                                    }">${data.skills[key].join(" • ")}</div>
                                </div>
                            `
                              )
                              .join("")}
                        </section>

                        <section class="cv-section">
                            <h3 class="section-title">
                                <span class="section-icon">🎓</span>
                                <span>${data.sectionTitles.education}</span>
                            </h3>
                            ${data.education
                              .map(
                                (edu) => `
                                <div class="education-item">
                                    <h4 class="edu-degree">${edu.degree}</h4>
                                    <div class="edu-school">${edu.school}</div>
                                    <div class="edu-period">${edu.period}</div>
                                    <p class="edu-details">${edu.details}</p>
                                </div>
                            `
                              )
                              .join("")}
                        </section>

                        <section class="cv-section">
                            <h3 class="section-title">
                                <span class="section-icon">🌍</span>
                                <span>${data.sectionTitles.languages}</span>
                            </h3>
                            <div class="languages-grid">
                                ${data.languages
                                  .map(
                                    (lang) => `
                                    <div class="language-item">
                                        <span class="lang-name">${lang.name}</span>
                                        <span class="lang-level">${lang.level}</span>
                                    </div>
                                `
                                  )
                                  .join("")}
                            </div>
                        </section>

                        <section class="cv-section">
                            <h3 class="section-title">
                                <span class="section-icon">🎯</span>
                                <span>${data.sectionTitles.interests}</span>
                            </h3>
                            <div class="interests-grid">
                                ${data.interests
                                  .map(
                                    (interest) =>
                                      `<div class="interest-item">${interest}</div>`
                                  )
                                  .join("")}
                            </div>
                        </section>
                    </div>
                </div>

                <footer class="cv-footer">
                    <div class="footer-text">${data.footer}</div>
                </footer>
            </div>
        `;
  }

  /**
   * Complete CSS styles optimized for full A4 page
   */
  getCVStyles() {
    return `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');
        
        @page {
            size: A4;
            margin: 0;
        }
        
        @media print {
            html, body {
                width: 210mm;
                height: 297mm;
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
            height: 297mm;
            background: white;
            color: #1a1a1a;
            font-size: 13px;
            line-height: 1.4;
            padding: 0;
            margin: 0;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;
        }

        /* HEADER avec ton vert de marque */
        .cv-header {
            background: #16a34a !important; /* Ton vert foncé de marque */
            color: white;
            padding: 25px 30px;
            position: relative;
            overflow: hidden;
            flex-shrink: 0;
            -webkit-print-color-adjust: exact !important;
            print-color-adjust: exact !important;
        }

        /* Emojis en arrière-plan VISIBLES */
        .cv-header::before {
            content: "🌿";
            position: absolute;
            top: -15px;
            right: -5px;
            font-size: 90px;
            color: #22c55e; /* Ton vert plus clair pour contraste */
            opacity: 0.25; /* Plus opaque pour être visible */
            transform: rotate(-15deg);
            z-index: 1;
            filter: brightness(1.3); /* Plus lumineux */
        }

        .cv-header::after {
            content: "🍃";
            position: absolute;
            bottom: -25px;
            left: -15px;
            font-size: 70px;
            color: #22c55e; /* Ton vert plus clair */
            opacity: 0.2; /* Légèrement plus opaque */
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
            font-size: 32px;
            font-weight: 700;
            margin: 0 0 6px 0;
            color: white !important;
            text-shadow: 0 1px 2px rgba(0,0,0,0.1);
        }

        .cv-title {
            font-size: 16px;
            font-weight: 400;
            margin: 0 0 12px 0;
            color: white !important;
            opacity: 0.95;
        }

        .cv-summary {
            font-size: 13px;
            line-height: 1.5;
            margin: 0;
            color: white !important;
            opacity: 0.92;
        }

        .contact-item {
            display: flex;
            align-items: center;
            gap: 16px;
            font-size: 12px;
            color: white !important;
            margin-top: 9px;
            margin-bottom: 9px;
            opacity: 0.9;
        }

        .contact-icon { 
            width: 14px; 
            text-align: center; 
        }

        /* BODY - Redistribué pour remplir la page */
        .cv-body {
            display: flex;
            gap: 25px;
            padding: 20px 30px;
            flex: 1;
            min-height: 0;
        }

        .cv-main { 
            flex: 2.2; 
            display: flex;
            flex-direction: column;
            gap: 16px;
        }
        
        .cv-sidebar { 
            flex: 0.8; 
            display: flex;
            flex-direction: column;
            gap: 6px;
        }

        .cv-section {
            margin-bottom: 0;
        }

        /* SECTION TITLES - Avec ton vert */
        .section-title {
            font-size: 16px;
            font-weight: 600;
            margin: 0 0 12px 0;
            color: #16a34a !important; /* Ton vert foncé */
            display: flex;
            align-items: center;
            gap: 8px;
            border-bottom: 2px solid #e2e8f0;
            padding-bottom: 6px;
        }

        .section-icon {
            font-size: 14px;
        }

        /* EXPERIENCE - Design amélioré */
        .experience-item {
            background: #fafbfc;
            border: 1px solid #e2e8f0;
            border-radius: 6px;
            padding: 8px;
            margin-bottom: 12px;
            transition: box-shadow 0.2s;
        }

        .experience-item:hover {
            box-shadow: 0 2px 8px rgba(0,0,0,0.08);
        }

        .exp-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 6px;
        }

        .exp-title-group {
            flex: 1;
        }

        .exp-title {
            font-size: 15px;
            font-weight: 600;
            color: #1e293b;
            margin: 0 0 4px 0;
        }

        .exp-company {
            font-size: 12px;
            color: #64748b;
            font-weight: 500;
        }

        .exp-period {
            font-size: 11px;
            color: #16a34a !important; /* Ton vert */
            font-weight: 600;
            background: #dcfce7 !important; /* Fond vert clair */
            padding: 4px 8px;
            border-radius: 12px;
            white-space: nowrap;
            -webkit-print-color-adjust: exact !important;
        }

        .exp-achievements {
            margin: 0;
            padding-left: 16px;
        }

        .exp-achievements li {
            margin-bottom: 2px;
            color: #475569;
            font-size: 12px;
            line-height: 1.4;
        }

        /* PROJECTS - Design cards */
        .projects-grid {
            display: flex;
            flex-direction: column;
            gap: 12px;
        }

        .project-item {
            background: white;
            border: 1px solid #d1d5db;
            border-radius: 6px;
            padding: 7px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }

        .project-header {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 6px;
        }

        .project-name {
            font-size: 14px;
            font-weight: 600;
            margin: 0;
            color: #1e293b;
        }

        .project-status {
            font-size: 10px;
            color: #16a34a !important; /* Ton vert */
            font-weight: 600;
            background: #dcfce7 !important; /* Fond vert clair */
            padding: 2px 6px;
            border-radius: 8px;
            -webkit-print-color-adjust: exact !important;
        }

        .project-desc {
            font-size: 11px;
            color: #64748b;
            margin: 0 0 6px 0;
            line-height: 1.3;
        }

        .project-tech {
            font-size: 10px;
            color: #16a34a !important; /* Ton vert */
            font-weight: 500;
        }

        /* SKILLS - Texte simple mais lisible */
        .skill-category {
            margin-bottom: 5px;
        }

        .skill-category h4 {
            font-size: 12px;
            font-weight: 600;
            margin: 0 0 4px 0;
            color: #374151;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .skill-list {
            font-size: 11px;
            color: #64748b;
            line-height: 1.4;
            margin-bottom: 4px;
        }

        .skill-list.learning {
            color: #d97706;
            font-style: italic;
        }

        /* EDUCATION - Plus d'espace */
        .education-item {
            background: #f8fafc;
            border-radius: 6px;
            padding: 6px;
            margin-bottom: 5px;
        }

        .edu-degree {
            font-size: 12px;
            font-weight: 600;
            margin: 0 0 4px 0;
            color: #1e293b;
            line-height: 1.3;
        }

        .edu-school {
            font-size: 11px;
            color: #16a34a !important; /* Ton vert */
            font-weight: 600;
            margin-bottom: 2px;
        }

        .edu-period {
            font-size: 10px;
            color: #64748b;
            margin-bottom: 4px;
        }

        .edu-details {
            font-size: 10px;
            color: #64748b;
            margin: 0;
            font-style: italic;
        }

        /* LANGUAGES */
        .languages-grid {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }

        .language-item {
            display: flex;
            justify-content: space-between;
            padding: 2px 4px;
            background: #f1f5f9;
            border-radius: 4px;
            font-size: 11px;
        }

        .lang-name { 
            font-weight: 600; 
            color: #374151;
        }
        
        .lang-level { 
            color: #64748b; 
            font-weight: 500;
        }

        /* INTERESTS */
        .interests-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 4px;
        }

        .interest-item {
            font-size: 10px;
            color: #64748b;
            padding: 2px;
        }

        /* FOOTER */
        .cv-footer {
            background: #f8fafc;
            padding: 12px 30px;
            text-align: center;
            border-top: 1px solid #e2e8f0;
            flex-shrink: 0;
        }

        .footer-text {
            font-size: 10px;
            color: #64748b;
            font-style: italic;
            font-weight: 500;
        }
    `;
  }

  /**
   * Create temporary visible CV element for PDF generation
   */
  createTemporaryCV() {
    const overlay = document.createElement("div");
    overlay.id = "cv-overlay";
    overlay.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
            background: rgba(255,255,255,0.95);
            z-index: 999999;
            overflow: auto;
            padding: 20px;
            box-sizing: border-box;
        `;

    const cvElement = document.createElement("div");
    cvElement.id = "temp-cv";
    cvElement.innerHTML = this.generateCVHTML();
    cvElement.style.cssText = `
            max-width: 210mm;
            margin: 0 auto;
            background: white;
            box-shadow: 0 0 20px rgba(0,0,0,0.1);
        `;

    overlay.appendChild(cvElement);

    const styleElement = document.createElement("style");
    styleElement.textContent = this.getCVStyles();
    document.head.appendChild(styleElement);

    document.body.appendChild(overlay);

    return { overlay, cvElement, styleElement };
  }

  /**
   * Generate and print PDF
   */
  async generatePDF() {
    try {
      const downloadBtn = document.getElementById("downloadCV");
      const tooltip = document.querySelector(".cv-tooltip span[data-key]");

      if (downloadBtn) {
        downloadBtn.classList.add("loading");
        downloadBtn.disabled = true;
        if (tooltip) {
          tooltip.textContent = this.i18nManager.t("loader");
        }
      }

      const printWindow = window.open("", "_blank", "width=800,height=600");

      printWindow.document.write(`
                <!DOCTYPE html>
                <html>
                <head>
                    <meta charset="utf-8">
                    <title>Kevin Bourbasquet - CV</title>
                    <style>${this.getCVStyles()}</style>
                </head>
                <body>
                    ${this.generateCVHTML()}
                </body>
                </html>
            `);

      printWindow.document.close();

      await new Promise((resolve) => setTimeout(resolve, 1000));

      printWindow.focus();
      printWindow.print();

      // Success state
      if (downloadBtn) {
        downloadBtn.classList.remove("loading");
        downloadBtn.classList.add("success");
      }

      setTimeout(() => {
        if (downloadBtn) {
          downloadBtn.classList.remove("success");
          downloadBtn.disabled = false;
          if (tooltip) {
            tooltip.textContent = this.i18nManager.t("action");
          }
        }
      }, 2000);
    } catch (error) {
      console.error("Error generating PDF:", error);
      alert(`Erreur: ${error.message}`);

      const downloadBtn = document.getElementById("downloadCV");
      const tooltip = document.querySelector(".cv-tooltip span[data-key]");

      if (downloadBtn) {
        downloadBtn.classList.remove("loading", "success");
        downloadBtn.disabled = false;
        if (tooltip) {
          tooltip.textContent = this.i18nManager.t("action");
        }
      }
    }
  }

  /**
   * Update CV when language changes
   */
  updateLanguage() {
    this.currentLang = this.i18nManager.currentLang;

    // Update tooltip text
    const tooltip = document.querySelector(".cv-tooltip span[data-key]");
    if (tooltip && !document.getElementById("downloadCV").disabled) {
      tooltip.textContent = this.i18nManager.t("action");
    }
  }

  /**
   * Initialize CV generator
   */
  init() {
    console.log(`CV Generator initialized in ${this.currentLang}`);

    // Force initials translation of the button
    this.updateLanguage();
  }
}

export default CVGenerator;
