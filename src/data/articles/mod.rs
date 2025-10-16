use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleMeta {
    pub id: String,
    pub title: HashMap<String, String>, // "en" -> "Title", "fr" -> "Titre"
    pub subtitle: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub date: String, // "2024-10-15"
    pub read_time: u32,
    pub tags: Vec<String>,
    pub category: String,
    pub featured: bool,
    pub image: String,
    pub status: ArticleStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticleStatus {
    Published,
    Draft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSection {
    pub id: String,
    pub title: String,
    pub content: String, // Markdown content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleContent {
    pub tldr: String,
    pub sections: Vec<ArticleSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub meta: ArticleMeta,
    pub content: HashMap<String, ArticleContent>, // "en" -> content, "fr" -> content
}

// Articles database
pub fn get_all_articles() -> Vec<Article> {
    vec![
        // Article 3D Printing Lab
        Article {
            meta: ArticleMeta {
                id: "3d-printing-lab".to_string(),
                title: {
                    let mut map = HashMap::new();
                    map.insert(
                        "fr".to_string(),
                        "Mon Lab d'Impression 3D : Setup et Projets".to_string(),
                    );
                    map.insert(
                        "en".to_string(),
                        "My 3D Printing Lab: Setup and Projects".to_string(),
                    );
                    map
                },
                subtitle: {
                    let mut map = HashMap::new();
                    map.insert(
                        "fr".to_string(),
                        "De l'Ender 3 V2 aux projets custom : mon évolution dans l'impression 3D"
                            .to_string(),
                    );
                    map.insert(
                        "en".to_string(),
                        "From Ender 3 V2 to custom projects: my evolution in 3D printing"
                            .to_string(),
                    );
                    map
                },
                description: {
                    let mut map = HashMap::new();
                    map.insert("fr".to_string(), "Tour de mon setup d'impression 3D, mods apportés à l'Ender 3 V2, projets réalisés, et prochaines évolutions matérielles.".to_string());
                    map.insert("en".to_string(), "Tour of my 3D printing setup, mods made to the Ender 3 V2, completed projects, and next hardware evolutions.".to_string());
                    map
                },
                date: "2024-07-20".to_string(),
                read_time: 16,
                tags: vec![
                    "3dprinting".to_string(),
                    "ender3".to_string(),
                    "mods".to_string(),
                    "projects".to_string(),
                    "hardware".to_string(),
                ],
                category: "hardware".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Draft,
            },
            content: {
                let mut content_map = HashMap::new();

                // Contenu français
                content_map.insert("fr".to_string(), ArticleContent {
                    tldr: "Setup Ender 3 V2 moddée : Microswiss hotend, BLTouch, enclosure custom. 50+ projets imprimés, de l'utilitaire au décoratif. Prochaine étape : imprimante résine pour détails fins.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "setup-initial".to_string(),
                            title: "Mon Setup d'Impression 3D".to_string(),
                            content: r#"# Mon Setup d'Impression 3D

Présentation de mon lab personnel d'impression 3D, optimisé pour des projets variés.

## 🖨️ L'Imprimante : Ender 3 V2 Modifiée

**Base :** Creality Ender 3 V2 (achetée début 2023)
- **Volume** : 220×220×250mm
- **Couche mini** : 0.1mm
- **Vitesse max** : 180mm/s (pratique: 60mm/s)

## 🔧 Modifications Apportées

**Microswiss Direct Drive (180€) :**
- **Hotend** all-metal pour filaments techniques
- **Extruder** direct drive = meilleur contrôle filament
- **Température** jusqu'à 300°C vs 260°C stock

**BLTouch Auto-Leveling (45€) :**
- **Compensation** défauts plateau automatique
- **Mesh** 5×5 points pour précision optimale
- **Fiabilité** première couche grandement améliorée"#.to_string(),
                        }
                    ],
                });

                // Contenu anglais
                content_map.insert("en".to_string(), ArticleContent {
                    tldr: "Modded Ender 3 V2 setup: Microswiss hotend, BLTouch, custom enclosure. 50+ printed projects, from utility to decorative. Next step: resin printer for fine details.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "setup-initial".to_string(),
                            title: "My 3D Printing Setup".to_string(),
                            content: r#"# My 3D Printing Setup

Presentation of my personal 3D printing lab, optimized for various projects.

## 🖨️ The Printer: Modified Ender 3 V2

**Base:** Creality Ender 3 V2 (bought early 2023)
- **Volume**: 220×220×250mm
- **Min layer**: 0.1mm
- **Max speed**: 180mm/s (practical: 60mm/s)

## 🔧 Applied Modifications

**Microswiss Direct Drive (180€):**
- **All-metal hotend** for technical filaments
- **Direct drive extruder** = better filament control
- **Temperature** up to 300°C vs 260°C stock

**BLTouch Auto-Leveling (45€):**
- **Automatic** bed defect compensation
- **5×5 mesh** points for optimal precision
- **First layer** reliability greatly improved"#.to_string(),
                        }
                    ],
                });

                content_map
            },
        },
        // Article Ansible Setup
        Article {
            meta: ArticleMeta {
                id: "ansible-setup".to_string(),
                title: {
                    let mut map = HashMap::new();
                    map.insert(
                        "fr".to_string(),
                        "Automatisation VPS avec Ansible : Mon Setup OVH en Code".to_string(),
                    );
                    map.insert(
                        "en".to_string(),
                        "VPS Automation with Ansible: My OVH Setup as Code".to_string(),
                    );
                    map
                },
                subtitle: {
                    let mut map = HashMap::new();
                    map.insert("fr".to_string(), "Comment j'ai automatisé le déploiement et la maintenance de mon VPS OVH avec Ansible".to_string());
                    map.insert(
                        "en".to_string(),
                        "How I automated the deployment and maintenance of my OVH VPS with Ansible"
                            .to_string(),
                    );
                    map
                },
                description: {
                    let mut map = HashMap::new();
                    map.insert("fr".to_string(), "Guide complet de mon setup Ansible pour gérer mon VPS OVH : de l'installation initiale aux déploiements applicatifs.".to_string());
                    map.insert("en".to_string(), "Complete guide to my Ansible setup for managing my OVH VPS: from initial installation to application deployments.".to_string());
                    map
                },
                date: "2024-10-15".to_string(),
                read_time: 18,
                tags: vec![
                    "ansible".to_string(),
                    "devops".to_string(),
                    "vps".to_string(),
                    "automation".to_string(),
                    "infrastructure".to_string(),
                    "security".to_string(),
                ],
                category: "devops".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut content_map = HashMap::new();

                content_map.insert("fr".to_string(), ArticleContent {
                    tldr: "Playbook Ansible complet pour VPS OVH : hardening sécurité, stack Docker, monitoring Grafana, déploiements automatisés. 1 commande = serveur production-ready.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "introduction".to_string(),
                            title: "Pourquoi Ansible pour mon VPS ?".to_string(),
                            content: r#"# Pourquoi Ansible pour mon VPS ?

Gérer un serveur à la main, c'est fastidieux et source d'erreurs. Après avoir refait 3 fois la config de mon VPS OVH suite à des "oops", j'ai décidé d'automatiser.

## 🚨 Les Problèmes Sans Automation

**Configuration manuelle :**
- **Incohérence** entre environnements
- **Oublis** de config critique
- **Temps perdu** à refaire les mêmes actions
- **Erreurs humaines** (typos, mauvais chemins...)

## 💡 L'Approche Infrastructure as Code

Avec Ansible, tout devient :
- **Reproductible** : même résultat à chaque fois
- **Versionnable** : Git track tous les changements
- **Testable** : validation avant prod
- **Documenté** : le code est la documentation"#.to_string(),
                        }
                    ],
                });

                content_map.insert("en".to_string(), ArticleContent {
                    tldr: "Complete Ansible playbook for OVH VPS: security hardening, Docker stack, Grafana monitoring, automated deployments. 1 command = production-ready server.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "introduction".to_string(),
                            title: "Why Ansible for my VPS?".to_string(),
                            content: r#"# Why Ansible for my VPS?

Managing a server manually is tedious and error-prone. After redoing my OVH VPS config 3 times following "oops" moments, I decided to automate.

## 🚨 Problems Without Automation

**Manual configuration:**
- **Inconsistency** between environments
- **Forgetting** critical config
- **Time wasted** redoing the same actions
- **Human errors** (typos, wrong paths...)

## 💡 Infrastructure as Code Approach

With Ansible, everything becomes:
- **Reproducible**: same result every time
- **Versionable**: Git tracks all changes
- **Testable**: validation before prod
- **Documented**: code is the documentation"#.to_string(),
                        }
                    ],
                });

                content_map
            },
        },
        // Tu peux ajouter les autres articles ici...
    ]
}

// Utilitaires
pub fn get_published_articles() -> Vec<Article> {
    get_all_articles()
        .into_iter()
        .filter(|article| matches!(article.meta.status, ArticleStatus::Published))
        .collect()
}

pub fn get_article_by_id(id: &str) -> Option<Article> {
    get_all_articles()
        .into_iter()
        .find(|article| article.meta.id == id)
}

pub fn get_featured_articles() -> Vec<Article> {
    get_published_articles()
        .into_iter()
        .filter(|article| article.meta.featured)
        .collect()
}

pub fn get_articles_by_category(category: &str) -> Vec<Article> {
    get_published_articles()
        .into_iter()
        .filter(|article| article.meta.category == category)
        .collect()
}

pub fn get_unique_categories() -> Vec<String> {
    let mut categories: Vec<String> = get_published_articles()
        .iter()
        .map(|article| article.meta.category.clone())
        .collect();
    categories.sort();
    categories.dedup();
    categories
}

pub fn get_category_emoji(category: &str) -> &'static str {
    match category {
        "system" => "🐧",
        "hardware" => "⚙️",
        "devops" => "🔧",
        "tools" => "🛠️",
        "project" => "🚀",
        _ => "📝",
    }
}
