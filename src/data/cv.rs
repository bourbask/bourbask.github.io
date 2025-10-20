use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CVData {
    pub personal: PersonalInfo,
    pub summary: String,
    pub section_titles: SectionTitles,
    pub experience: Vec<Experience>,
    pub skill_categories: SkillCategories,
    pub skills: Skills,
    pub projects: Vec<Project>,
    pub education: Vec<Education>,
    pub languages: Vec<LanguageSkill>,
    pub interests: Vec<String>,
    pub footer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub portfolio: String,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionTitles {
    pub experience: String,
    pub projects: String,
    pub skills: String,
    pub education: String,
    pub languages: String,
    pub interests: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub location: String,
    pub period: String,
    pub achievements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCategories {
    pub technical: String,
    pub frontend: String,
    pub backend: String,
    pub tools: String,
    pub learning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub backend: Vec<String>,
    pub frontend: Vec<String>,
    pub database: Vec<String>,
    pub devops: Vec<String>,
    pub learning: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub status: String,
    pub description: String,
    pub tech: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub degree: String,
    pub school: String,
    pub period: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSkill {
    pub name: String,
    pub level: String,
}
