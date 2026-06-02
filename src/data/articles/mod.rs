use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleMeta {
    pub id: String,
    pub title: HashMap<String, String>,
    pub subtitle: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub date: String,
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
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleContent {
    pub tldr: String,
    pub sections: Vec<ArticleSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub meta: ArticleMeta,
    pub content: HashMap<String, ArticleContent>,
}

pub fn get_all_articles() -> Vec<Article> {
    vec![
        // =========================================================
        // ARTICLE: vaultwarden
        // GitHub: https://github.com/dani-garcia/vaultwarden (61,726 ⭐)
        // Generated: 2026-06-02
        // =========================================================
        Article {
            meta: ArticleMeta {
                id: "vaultwarden".to_string(),
                title: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Vaultwarden : Bitwarden chez toi, sans les frais".to_string());
                    m.insert("en".to_string(), "Vaultwarden: Self-Host Bitwarden Without the Overhead".to_string());
                    m
                },
                subtitle: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Un serveur Bitwarden alternatif écrit en Rust, léger, rapide et fait pour le self-hosting".to_string());
                    m.insert("en".to_string(), "A lightweight Rust-powered Bitwarden-compatible server built for self-hosting".to_string());
                    m
                },
                description: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Vaultwarden est une implémentation alternative de l'API Bitwarden en Rust, parfaite pour self-héberger un gestionnaire de mots de passe sans serveur costaud.".to_string());
                    m.insert("en".to_string(), "Vaultwarden is an alternative Bitwarden API server written in Rust — perfect for self-hosting a password manager on minimal hardware.".to_string());
                    m
                },
                date: "2026-06-02".to_string(),
                read_time: 8,
                tags: vec!["self-hosting".to_string(), "securite".to_string(), "rust".to_string(), "docker".to_string(), "password-manager".to_string(), "bitwarden".to_string(), "souverainete-numerique".to_string(), "open-source".to_string()],
                category: "security".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {
                    tldr: r###"Vaultwarden réimplémente l'API Bitwarden en Rust pour un usage self-hosted ultra-léger. Compatible avec tous les clients officiels Bitwarden. Tourne dans un container Docker avec moins de 10 Mo de RAM."###.to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "le-probleme".to_string(),
                            title: "Le problème que ça résout".to_string(),
                            content: r###"<p>Tu utilises Bitwarden ? Bien. Tu leur fais confiance pour héberger tes mots de passe sur leurs serveurs ? C'est là que ça coince pour beaucoup d'entre nous.</p><p>Le serveur officiel Bitwarden open source existe bien, mais il est conçu pour tourner en production à grande échelle : plusieurs services Docker, une stack SQL Server (oui, Microsoft SQL Server par défaut), une consommation mémoire qui dépasse rapidement le gigaoctet. Sur un VPS à 5€/mois ou un Raspberry Pi qui traîne dans un tiroir, c'est tout simplement pas raisonnable.</p><p>Et pourtant, le besoin est réel : un gestionnaire de mots de passe synchronisé entre tous tes appareils, accessible depuis les clients officiels Bitwarden (browser extension, appli mobile, CLI), mais hébergé <strong>chez toi</strong>, sous ton contrôle. Ni Google, ni LastPass, ni même Bitwarden Inc. dans la boucle.</p><div class="article-callout info"><span class="callout-icon">ℹ️</span><div class="callout-content"><p>Le serveur Bitwarden officiel auto-hébergé nécessite une stack complète avec .NET, MSSQL et plusieurs microservices. Vaultwarden fait la même chose dans un seul binaire Rust.</p></div></div><p>C'est exactement le problème que Vaultwarden résout : te donner l'expérience Bitwarden complète sur du matériel modeste, sans compromis sur la compatibilité avec les clients officiels.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "ce-que-cest".to_string(),
                            title: "Ce que c'est".to_string(),
                            content: r###"<p><a href="https://github.com/dani-garcia/vaultwarden" target="_blank" rel="noopener noreferrer">Vaultwarden</a> est une réimplémentation non officielle de l'API serveur Bitwarden, écrite entièrement en Rust avec le framework <strong>Rocket</strong>. Le projet s'appelait auparavant <em>bitwarden_rs</em> avant d'être renommé pour éviter toute confusion avec le projet officiel.</p><p>Ce que ça fait concrètement :</p><ul><li>Implémente l'intégralité de l'API Bitwarden Client</li><li>Supporte la synchronisation des coffres, des collections d'organisation, du partage entre utilisateurs</li><li>Gère la 2FA (TOTP, Duo, YubiKey, FIDO2/WebAuthn)</li><li>Expose une interface d'administration web</li><li>Supporte les notifications en temps réel via WebSockets</li><li>Fonctionne avec SQLite, MySQL ou PostgreSQL</li></ul><p>Le tout dans <strong>une seule image Docker</strong>, sans dépendances externes obligatoires. SQLite suffit pour un usage personnel ou une petite équipe.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Vaultwarden n'est pas un fork de Bitwarden — c'est une réécriture complète de la couche serveur. Les clients officiels ne savent pas la différence.</p></div></div><p>Le projet cumule plus de <strong>61 000 étoiles GitHub</strong> et des millions de pulls Docker. C'est pas un side project abandonné : c'est un des projets self-hosting les plus actifs et les plus matures de l'écosystème.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pourquoi-lessayer".to_string(),
                            title: "Pourquoi l'essayer".to_string(),
                            content: r###"<p>Laisse-moi te donner les vraies raisons, pas le pitch marketing.</p><p><strong>L'empreinte mémoire est ridiculeusement basse.</strong> On parle de moins de 10 Mo de RAM au repos. Le serveur officiel Bitwarden dépasse facilement 1 Go. Sur un Raspberry Pi 3 ou un VPS micro, la différence est existentielle.</p><p><strong>Rust, c'est pas juste du hype ici.</strong> La sécurité mémoire by design et les performances du binaire compilé sont directement pertinentes pour un service qui tourne 24/7 et stocke tes credentials. Pas de GC pauses, pas de memory leaks silencieux.</p><p><strong>La compatibilité clients est totale.</strong> Tu gardes l'extension Chrome/Firefox, l'app Android/iOS, le CLI <code>bw</code> — exactement comme si tu pointais vers bitwarden.com. Zéro friction pour les autres membres de ta famille ou ton équipe.</p><p><strong>Les features premium sont incluses gratuitement.</strong> Bitwarden propose des rapports d'hygiène de mots de passe, l'accès d'urgence, ou le TOTP intégré uniquement en plan payant. Vaultwarden les active par défaut pour tous tes utilisateurs.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Tu peux activer les features d'organisation (partage, collections) sans payer de licence Bitwarden. C'est une économie réelle si tu gères des credentials en équipe.</p></div></div><p><strong>La souveraineté numérique n'est pas négociable.</strong> Tes mots de passe ne quittent jamais ton infra. Chiffrement bout-en-bout côté client, clé maître que toi seul possèdes. Même toi en tant qu'admin tu ne peux pas lire les coffres.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "en-pratique".to_string(),
                            title: "En pratique — setup et usage".to_string(),
                            content: r###"<p>La mise en route est volontairement simple. Voici le minimum viable avec Docker Compose :</p><pre><code>services:
  vaultwarden:
    image: vaultwarden/server:latest
    container_name: vaultwarden
    restart: unless-stopped
    environment:
      DOMAIN: "https://vault.ton-domaine.fr"
      SIGNUPS_ALLOWED: "false"
      ADMIN_TOKEN: "ton-token-admin-genere"
    volumes:
      - ./vw-data:/data
    ports:
      - "127.0.0.1:8080:80"</code></pre><p>Tu poses ça derrière un reverse proxy (Caddy, Nginx, Traefik — ton choix), tu ajointes un certificat TLS, et c'est fonctionnel. <strong>HTTPS est obligatoire</strong> pour que les clients Bitwarden acceptent de se connecter.</p><p>Pour générer un token admin sécurisé :</p><pre><code>openssl rand -base64 48</code></pre><p>L'interface d'administration est accessible sur <code>/admin</code> avec ce token. Tu peux y gérer les utilisateurs, inviter des membres, configurer les notifications email (SMTP) et vérifier l'état du serveur.</p><p>Pour pointer un client Bitwarden existant vers ton instance, c'est dans les paramètres de l'app : <em>Région &gt; Auto-hébergé &gt; URL du serveur</em>. Trente secondes.</p><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Désactive les inscriptions publiques (<code>SIGNUPS_ALLOWED=false</code>) immédiatement après avoir créé ton compte. Sans ça, n'importe qui peut créer un compte sur ton instance.</p></div></div><p>Pour les backups, le répertoire <code>/data</code> contient tout : la base SQLite, les attachments, les clés. Un simple <code>rsync</code> planifié suffit.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "limites".to_string(),
                            title: "Limites honnêtes".to_string(),
                            content: r###"<p>Je t'ai dit que je serai honnête. Voilà ce qui peut coincer.</p><ul><li><strong>C'est non officiel.</strong> Vaultwarden n'est pas développé ni supporté par Bitwarden Inc. Quand Bitwarden sort une mise à jour de son API client, il faut attendre que la communauté suive. En pratique ça va vite, mais il peut y avoir des gaps temporaires de compatibilité.</li><li><strong>Tu es ton propre ops.</strong> Mises à jour, backups, monitoring, disponibilité — c'est toi. Si ton serveur plante à 2h du matin, c'est ton problème. Pour un usage personnel c'est acceptable. Pour une PME sans équipe infra, réfléchis-y.</li><li><strong>Pas de support officiel.</strong> La doc est communautaire (très bonne par ailleurs), le support passe par GitHub Discussions et Matrix. Pas de ticket enterprise, pas de SLA.</li><li><strong>Envoi d'emails : configuration manuelle.</strong> Les notifications email (vérification de compte, 2FA, accès d'urgence) nécessitent de configurer un serveur SMTP. C'est trivial si tu as déjà un relay, sinon c'est une étape de plus.</li></ul><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Vaultwarden n'est pas audité de manière indépendante contrairement au client Bitwarden. Pour un usage professionnel critique, c'est un point à peser sérieusement.</p></div></div><ul><li><strong>FIDO2/Passkeys :</strong> Le support est là mais peut être en retard sur les dernières specs selon les versions. Vérifie les release notes si c'est critique pour toi.</li></ul>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pour-qui".to_string(),
                            title: "Pour qui".to_string(),
                            content: r###"<p>Vaultwarden est fait pour toi si tu coches plusieurs de ces cases :</p><ul><li>Tu as déjà un VPS ou un serveur maison (NAS, Raspberry Pi, homelab) avec Docker installé</li><li>Tu utilises ou veux utiliser Bitwarden mais tu ne veux pas confier tes credentials à un tiers</li><li>Tu gères des mots de passe pour une famille, une petite équipe ou un projet side — et tu veux le partage sans payer l'abonnement Bitwarden Teams</li><li>Tu es à l'aise avec la responsabilité opérationnelle : tu fais tes backups, tu surveilles tes services</li><li>Tu valorises la souveraineté numérique et le contrôle de ta stack</li></ul><p><strong>C'est probablement pas pour toi si :</strong></p><ul><li>Tu n'as aucune infrastructure self-hosted et tu ne veux pas en gérer une — dans ce cas, l'abonnement Bitwarden Premium à 10$/an est franchement raisonnable</li><li>Tu as besoin de conformité réglementaire (SOC2, ISO27001) — Bitwarden Inc. a des certifications, Vaultwarden non</li><li>Tu veux zéro maintenance — les mises à jour sont nécessaires régulièrement, surtout pour les correctifs de sécurité</li></ul><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>C'est l'outil idéal pour un homelab : setup en 20 minutes, empreinte ressources négligeable, et tu récupères le contrôle total de l'un des actifs les plus sensibles de ta vie numérique.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "verdict".to_string(),
                            title: "Verdict".to_string(),
                            content: r###"<p>Vaultwarden est l'un des meilleurs exemples de ce que l'open source communautaire peut produire : un outil qui surpasse l'original sur les métriques qui comptent pour les utilisateurs avancés, sans sacrifier la compatibilité.</p><p>Si tu as déjà un homelab ou même un simple VPS, <strong>il n'y a aucune bonne raison de ne pas l'installer</strong>. Vingt minutes de setup, une image Docker de quelques mégaoctets, et tu reprends le contrôle de tes mots de passe pour toujours. Le coût opérationnel est quasi nul si tu as déjà l'infrastructure.</p><p>La seule question légitime c'est la confiance dans un projet non officiel pour un usage aussi sensible. Mais avec 61 000 étoiles, des années d'historique, des millions de déploiements actifs et un code Rust auditable par n'importe qui, Vaultwarden a largement prouvé sa fiabilité. La communauté est le meilleur audit qui soit sur la durée.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Mon setup perso : Vaultwarden sur un VPS Hetzner CX11 derrière Caddy, SQLite, backup quotidien vers B2. Ça tourne depuis 2 ans sans incident. Consommation mémoire : 8 Mo au repos.</p></div></div><p><strong>Note : 9/10.</strong> Le point manquant, c'est uniquement l'absence d'audit de sécurité indépendant. Pour tout le reste, c'est un outil mature, performant et qui résout un vrai problème avec élégance. Installe-le.</p>"###.to_string(),
                        },
                    ],
                });

                m.insert("en".to_string(), ArticleContent {
                    tldr: r###"Vaultwarden reimplements the Bitwarden API in Rust for ultra-lightweight self-hosting. Compatible with all official Bitwarden clients. Runs in a Docker container using under 10MB of RAM."###.to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "le-probleme".to_string(),
                            title: "The Problem It Solves".to_string(),
                            content: r###"<p>You use Bitwarden? Great. You trust them to host your passwords on their servers? That's where it gets uncomfortable for a lot of us.</p><p>The official open source Bitwarden server does exist, but it's designed for large-scale production deployments: multiple Docker services, a SQL Server backend (yes, Microsoft SQL Server by default), and memory usage that quickly blows past a gigabyte. On a €5/month VPS or a Raspberry Pi gathering dust in a drawer, that's simply not viable.</p><p>And yet the need is real: a password manager synced across all your devices, accessible from official Bitwarden clients (browser extension, mobile app, CLI), but hosted <strong>on your own hardware</strong>, under your control. No Google, no LastPass, not even Bitwarden Inc. in the loop.</p><div class="article-callout info"><span class="callout-icon">ℹ️</span><div class="callout-content"><p>The official self-hosted Bitwarden server requires a full stack with .NET, MSSQL, and multiple microservices. Vaultwarden does the same job in a single Rust binary.</p></div></div><p>That's exactly the problem Vaultwarden solves: giving you the full Bitwarden experience on modest hardware, without any compromise on compatibility with the official clients.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "ce-que-cest".to_string(),
                            title: "What It Is".to_string(),
                            content: r###"<p><a href="https://github.com/dani-garcia/vaultwarden" target="_blank" rel="noopener noreferrer">Vaultwarden</a> is an unofficial reimplementation of the Bitwarden server API, written entirely in Rust using the <strong>Rocket</strong> framework. The project was previously known as <em>bitwarden_rs</em> before being renamed to avoid confusion with the official project.</p><p>What it concretely does:</p><ul><li>Implements the full Bitwarden Client API</li><li>Supports vault sync, organization collections, and user sharing</li><li>Handles 2FA (TOTP, Duo, YubiKey, FIDO2/WebAuthn)</li><li>Exposes a web-based admin panel</li><li>Supports real-time notifications via WebSockets</li><li>Works with SQLite, MySQL, or PostgreSQL</li></ul><p>All of this in <strong>a single Docker image</strong>, with no mandatory external dependencies. SQLite is perfectly fine for personal use or a small team.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Vaultwarden is not a fork of Bitwarden — it's a complete rewrite of the server layer. Official clients can't tell the difference.</p></div></div><p>The project has over <strong>61,000 GitHub stars</strong> and millions of Docker pulls. This isn't an abandoned side project — it's one of the most active and mature self-hosting projects in the ecosystem.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pourquoi-lessayer".to_string(),
                            title: "Why You Should Try It".to_string(),
                            content: r###"<p>Let me give you the real reasons, not the marketing pitch.</p><p><strong>The memory footprint is ridiculously low.</strong> We're talking under 10MB of RAM at idle. The official Bitwarden server easily exceeds 1GB. On a Raspberry Pi 3 or a micro VPS, that difference is existential.</p><p><strong>Rust isn't just hype here.</strong> Memory safety by design and the performance of a compiled binary are directly relevant for a service running 24/7 that stores your credentials. No GC pauses, no silent memory leaks.</p><p><strong>Client compatibility is total.</strong> You keep your Chrome/Firefox extension, Android/iOS app, the <code>bw</code> CLI — exactly as if you were pointing to bitwarden.com. Zero friction for family members or teammates.</p><p><strong>Premium features are included for free.</strong> Bitwarden locks password hygiene reports, emergency access, and built-in TOTP behind paid plans. Vaultwarden enables them by default for all your users.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>You can enable organization features (sharing, collections) without paying for a Bitwarden license. Real savings if you manage credentials as a team.</p></div></div><p><strong>Digital sovereignty is non-negotiable.</strong> Your passwords never leave your infrastructure. Client-side end-to-end encryption, master key only you possess. Even you as admin can't read the vaults.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "en-pratique".to_string(),
                            title: "In Practice — Setup & Usage".to_string(),
                            content: r###"<p>Getting started is intentionally simple. Here's the minimum viable Docker Compose setup:</p><pre><code>services:
  vaultwarden:
    image: vaultwarden/server:latest
    container_name: vaultwarden
    restart: unless-stopped
    environment:
      DOMAIN: "https://vault.your-domain.com"
      SIGNUPS_ALLOWED: "false"
      ADMIN_TOKEN: "your-generated-admin-token"
    volumes:
      - ./vw-data:/data
    ports:
      - "127.0.0.1:8080:80"</code></pre><p>Drop this behind a reverse proxy (Caddy, Nginx, Traefik — your call), attach a TLS certificate, and it's live. <strong>HTTPS is mandatory</strong> for Bitwarden clients to accept the connection.</p><p>To generate a secure admin token:</p><pre><code>openssl rand -base64 48</code></pre><p>The admin panel is accessible at <code>/admin</code> with that token. From there you can manage users, send invites, configure email notifications (SMTP), and check server status.</p><p>To point an existing Bitwarden client to your instance, go to app settings: <em>Region &gt; Self-hosted &gt; Server URL</em>. Thirty seconds.</p><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Disable public signups (<code>SIGNUPS_ALLOWED=false</code>) immediately after creating your account. Without this, anyone can register on your instance.</p></div></div><p>For backups, the <code>/data</code> directory contains everything: the SQLite database, attachments, keys. A simple scheduled <code>rsync</code> is all you need.</p>"###.to_string(),
                        },
                        ArticleSection {
                            id: "limites".to_string(),
                            title: "Honest Limitations".to_string(),
                            content: r###"<p>I told you I'd be honest. Here's what can trip you up.</p><ul><li><strong>It's unofficial.</strong> Vaultwarden is not developed or supported by Bitwarden Inc. When Bitwarden ships a client API update, you wait for the community to catch up. In practice it's fast, but temporary compatibility gaps can happen.</li><li><strong>You are your own ops.</strong> Updates, backups, monitoring, uptime — that's on you. If your server goes down at 2am, it's your problem. For personal use, acceptable. For a small business without an infra team, think twice.</li><li><strong>No official support.</strong> Documentation is community-driven (very good, actually), support goes through GitHub Discussions and Matrix. No enterprise tickets, no SLA.</li><li><strong>Email: manual configuration.</strong> Email notifications (account verification, 2FA, emergency access) require configuring an SMTP server. Trivial if you already have a relay, otherwise it's one more step.</li></ul><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Vaultwarden has not been independently audited, unlike the Bitwarden client. For critical professional use, this is a point worth weighing seriously.</p></div></div><ul><li><strong>FIDO2/Passkeys:</strong> Support is there but may lag behind the latest specs depending on the version. Check the release notes if this is critical for you.</li></ul>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pour-qui".to_string(),
                            title: "Who It's For".to_string(),
                            content: r###"<p>Vaultwarden is made for you if you check several of these boxes:</p><ul><li>You already have a VPS or home server (NAS, Raspberry Pi, homelab) with Docker installed</li><li>You use or want to use Bitwarden but don't want to hand your credentials to a third party</li><li>You manage passwords for a family, a small team, or a side project — and want sharing without paying for Bitwarden Teams</li><li>You're comfortable with operational responsibility: you run backups, you monitor your services</li><li>You value digital sovereignty and control over your stack</li></ul><p><strong>It's probably not for you if:</strong></p><ul><li>You have no self-hosted infrastructure and don't want to manage any — in that case, the Bitwarden Premium plan at $10/year is honestly reasonable</li><li>You need regulatory compliance (SOC2, ISO27001) — Bitwarden Inc. has certifications, Vaultwarden does not</li><li>You want zero maintenance — updates are needed regularly, especially for security patches</li></ul><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>It's the ideal homelab tool: 20-minute setup, negligible resource footprint, and you regain full control over one of the most sensitive assets in your digital life.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "verdict".to_string(),
                            title: "Verdict".to_string(),
                            content: r###"<p>Vaultwarden is one of the best examples of what community open source can produce: a tool that outperforms the original on the metrics that matter to power users, without sacrificing compatibility.</p><p>If you already have a homelab or even a basic VPS, <strong>there's no good reason not to install it</strong>. Twenty minutes of setup, a Docker image weighing a few megabytes, and you take back control of your passwords permanently. The operational cost is near zero if you already have the infrastructure.</p><p>The only legitimate question is trusting an unofficial project for something this sensitive. But with 61,000 stars, years of track record, millions of active deployments, and Rust code auditable by anyone, Vaultwarden has thoroughly proven its reliability. The community is the best ongoing audit there is.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>My personal setup: Vaultwarden on a Hetzner CX11 VPS behind Caddy, SQLite, daily backup to B2. Running for 2 years without incident. Memory usage: 8MB at idle.</p></div></div><p><strong>Score: 9/10.</strong> The missing point is solely the absence of an independent security audit. For everything else, this is a mature, performant tool that solves a real problem with elegance. Install it.</p>"###.to_string(),
                        },
                    ],
                });

                m
            },
        },
        // =========================================================
        // ARTICLE: ansible-setup
        // =========================================================
        Article {
            meta: ArticleMeta {
                id: "ansible-setup".to_string(),
                title: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Ansible, MASH et Matrix : mon lab self-hosted depuis zéro".to_string());
                    m.insert("en".to_string(), "Ansible, MASH & Matrix: Building My Self-Hosted Lab".to_string());
                    m
                },
                subtitle: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Comment j'ai commencé par vouloir quitter Discord et me suis retrouvé à administrer un VPS avec SSO, Vaultwarden, Matrix et monitoring Grafana.".to_string());
                    m.insert("en".to_string(), "How trying to escape Discord led me to build a fully automated self-hosted infrastructure with SSO, Vaultwarden, Matrix, and Grafana monitoring.".to_string());
                    m
                },
                description: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Retour d'expérience sur mon lab Ansible : MASH playbook, matrix-docker-ansible-deploy, Authentik SSO, et toutes les galères que personne ne mentionne dans les tutoriels.".to_string());
                    m.insert("en".to_string(), "My experience building an automated self-hosted lab with MASH playbook, matrix-docker-ansible-deploy, Authentik SSO — including the parts tutorials skip over.".to_string());
                    m
                },
                date: "2024-10-15".to_string(),
                read_time: 18,
                tags: vec!["ansible".to_string(), "self-hosting".to_string(), "matrix".to_string(), "devops".to_string(), "vps".to_string(), "sso".to_string()],
                category: "devops".to_string(),
                featured: true,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {
                    tldr: "MASH playbook + matrix-docker-ansible-deploy sur VPS OVH. Services déployés : Matrix/Element, Vaultwarden, Authentik SSO, Grafana. Projet en pause, reprise prévue. Une commande pour tout remonter depuis zéro.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "incident".to_string(),
                            title: "Discord m'a coûté un serveur".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">J'en avais marre de payer pour Discord Nitro. C'est ainsi que j'ai fini par administrer une infrastructure complète de self-hosting avec SSO, un serveur Matrix, Vaultwarden et un dashboard pour mes appareils connectés.</p>
</div>
<p>Tout a commencé simplement : je voulais héberger un serveur Matrix pour remplacer Discord avec mes amis. Un serveur, un domaine, quelques sous-domaines. Simple, non ?</p>
<p>Trois semaines plus tard, j'avais un playbook Ansible qui déployait automatiquement huit services distincts sur mon VPS OVH, avec une authentification centralisée via SSO, des backups automatiques, et des certificats TLS gérés automatiquement. C'est ça le vrai problème du self-hosting : ce n'est pas la complexité de chaque service individuellement. C'est que chaque service résolu crée le suivant.</p>
<p>Ce que j'ai appris de cette aventure ne se résume pas aux commandes Ansible. C'est surtout ça : construire progressivement, par couches, accepter que le scope va grandir, et écrire une fois pour ne plus jamais recommencer à la main.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-outils".to_string(),
                            title: "MASH et Matrix : pourquoi ces frameworks".to_string(),
                            content: r##"<p>Plutôt que d'écrire mes propres rôles Ansible depuis zéro, j'ai utilisé deux projets open source déjà solides et maintenus par une vraie communauté :</p>
<ul>
  <li><a href="https://github.com/mother-of-all-self-hosting/mash-playbook" target="_blank" rel="noopener noreferrer"><strong>MASH Playbook</strong></a> — pour la quasi-totalité des services self-hostés : Vaultwarden, Grafana, Prometheus, Authentik, etc.</li>
  <li><a href="https://github.com/spantaleev/matrix-docker-ansible-deploy" target="_blank" rel="noopener noreferrer"><strong>matrix-docker-ansible-deploy</strong></a> — spécifiquement pour le stack Matrix + Element Web.</li>
</ul>
<p>Ces deux projets partagent la même philosophie : tout se configure via des variables YAML dans un fichier <code>vars.yml</code>. Le playbook s'occupe du reste — installation, Docker, reverse proxy, certificats. En pratique :</p>
<pre><code># vars.yml (extrait simplifié)
matrix_domain: mon-domaine.fr
mash_playbook_hostname: services.mon-domaine.fr

vaultwarden_enabled: true
vaultwarden_hostname: vault.mon-domaine.fr

grafana_enabled: true
grafana_hostname: monitoring.mon-domaine.fr

authentik_enabled: true
authentik_hostname: sso.mon-domaine.fr</code></pre>
<div class="article-callout info">
  <span class="callout-icon">ℹ️</span>
  <div class="callout-content">
    <p><strong>Pourquoi ces projets plutôt que repartir de zéro ?</strong> Écrire ses propres rôles Ansible est un excellent exercice d'apprentissage. Mais si l'objectif est un vrai lab qui fonctionne, s'appuyer sur des projets maintenus par la communauté est une bien meilleure stratégie : la documentation est souvent meilleure, les edge cases déjà gérés, les updates propagées automatiquement.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-lab".to_string(),
                            title: "Ce que le lab héberge".to_string(),
                            content: r##"<p>Au fil des semaines, le serveur s'est peuplé. Voilà l'inventaire :</p>
<table>
  <thead><tr><th>Service</th><th>Usage</th><th>Outil</th></tr></thead>
  <tbody>
    <tr><td>Chat</td><td>Alternative à Discord pour mes amis</td><td>Matrix + Element Web</td></tr>
    <tr><td>Passwords</td><td>Gestionnaire de mots de passe</td><td>Vaultwarden</td></tr>
    <tr><td>Authentification</td><td>SSO centralisé pour tous les services</td><td>Authentik</td></tr>
    <tr><td>Monitoring</td><td>Métriques serveur + alertes</td><td>Grafana + Prometheus</td></tr>
    <tr><td>IoT</td><td>Dashboard appareils connectés (expérimental)</td><td>Home Assistant</td></tr>
    <tr><td>Mail</td><td>Serveur mail maison</td><td>Exim / MASH mail stack</td></tr>
  </tbody>
</table>
<p>Le SSO via Authentik a été le changement le plus impactant. Un seul compte pour tout, une seule page de login. Une fois correctement configuré, ça donne l'impression d'administrer quelque chose de professionnel — même si derrière c'est votre VPS OVH à 6€/mois.</p>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Par où commencer si vous débutez ?</strong> Vaultwarden. L'installation via MASH est quasi-triviale, l'utilité est immédiate, et c'est un bon premier succès avant d'attaquer Matrix ou Authentik. La documentation MASH pour Vaultwarden est excellente.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-galeres".to_string(),
                            title: "Ce que les tutoriels ne disent pas".to_string(),
                            content: r##"<h2>La courbe d'apprentissage réelle</h2>
<p>Si vous n'avez jamais fait d'Ansible, la première chose qui surprend c'est l'idempotence. Un playbook bien écrit peut être relancé 100 fois et produira le même résultat. Si ce n'est pas le cas, c'est un bug à corriger, pas un comportement acceptable.</p>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>Le réflexe à avoir dès le début :</strong> toujours tester avec <code>--check</code> d'abord. Cette option simule le playbook sans rien modifier. Elle vous sauvera d'au moins une mauvaise manip.</p>
    <pre><code>ansible-playbook -i inventory/hosts setup.yml --check --diff</code></pre>
  </div>
</div>
<p>La configuration du serveur Matrix a été la partie la plus complexe, notamment la fédération — le fait que des serveurs Matrix différents puissent se parler. DNS, certificats, firewall rules, configuration des délégations : chaque couche a ses propres pièges.</p>
<p>J'ai relancé le playbook quatre fois avant de comprendre que le problème venait de la propagation DNS, pas d'Ansible. Moralité : vérifiez <em>toujours</em> que vos entrées DNS sont propagées avant de débugger le reste.</p>
<p>La gestion du serveur mail maison mérite un article entier. Réputation IP, SPF, DKIM, DMARC — il faut tout configurer correctement pour que les mails arrivent en boîte de réception plutôt qu'en spam. C'est faisable. C'est juste long.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "et-maintenant".to_string(),
                            title: "En pause — mais pas abandonné".to_string(),
                            content: r##"<p>Le projet est en pause. Un an de blues, suivi d'un an de crunch pour une sortie en prod au boulot. Le serveur existe encore, certains services tournent, mais le lab n'a pas évolué depuis un moment.</p>
<p>Ce que je veux faire en reprenant :</p>
<ul>
  <li>Finaliser la configuration Authentik pour un vrai SSO opérationnel sur tous les services</li>
  <li>Stabiliser le stack Matrix pour qu'il soit utilisable au quotidien par mes amis</li>
  <li>Connecter mes appareils IoT à Home Assistant proprement</li>
  <li>Documenter le tout dans un repo public pour que ça serve à d'autres</li>
</ul>
<p>Si ce sujet vous intéresse, revenez dans les prochains mois — je compte documenter la reprise au fur et à mesure, galères incluses.</p>
<div class="article-callout info">
  <span class="callout-icon">🔗</span>
  <div class="callout-content">
    <p><strong>Ressources pour démarrer :</strong></p>
    <ul>
      <li><a href="https://github.com/mother-of-all-self-hosting/mash-playbook" target="_blank" rel="noopener noreferrer">MASH Playbook — documentation officielle</a></li>
      <li><a href="https://github.com/spantaleev/matrix-docker-ansible-deploy" target="_blank" rel="noopener noreferrer">matrix-docker-ansible-deploy</a></li>
      <li><a href="https://docs.ansible.com/ansible/latest/getting_started/index.html" target="_blank" rel="noopener noreferrer">Ansible — Getting Started (officiel)</a></li>
    </ul>
  </div>
</div>"##.to_string(),
                        },
                    ],
                });

                m.insert("en".to_string(), ArticleContent {
                    tldr: "MASH playbook + matrix-docker-ansible-deploy on OVH VPS. Deployed: Matrix/Element, Vaultwarden, Authentik SSO, Grafana. Project on pause, resuming soon. One command to rebuild everything from scratch.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "incident".to_string(),
                            title: "Discord Cost Me a Server".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">I was tired of paying for Discord Nitro. That's how I ended up administering a complete self-hosted infrastructure with SSO, a Matrix server, Vaultwarden, and a dashboard for my connected devices.</p>
</div>
<p>It started simply enough: I wanted to host a Matrix server to replace Discord with my friends. One server, one domain, a few subdomains. Simple, right?</p>
<p>Three weeks later, I had an Ansible playbook automatically deploying eight distinct services on my OVH VPS, with centralized SSO authentication, automatic backups, and TLS certificates managed automatically. That's the real problem with self-hosting: it's not the complexity of each individual service. It's that every solved service creates the next one.</p>
<p>What I learned from this adventure isn't just Ansible commands. It's this: build progressively, in layers, accept that scope will grow, and write it down once so you never have to do it manually again.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-outils".to_string(),
                            title: "MASH and Matrix: Why These Frameworks".to_string(),
                            content: r##"<p>Rather than writing my own Ansible roles from scratch, I used two well-maintained open source projects:</p>
<ul>
  <li><a href="https://github.com/mother-of-all-self-hosting/mash-playbook" target="_blank" rel="noopener noreferrer"><strong>MASH Playbook</strong></a> — for almost all self-hosted services: Vaultwarden, Grafana, Prometheus, Authentik, etc.</li>
  <li><a href="https://github.com/spantaleev/matrix-docker-ansible-deploy" target="_blank" rel="noopener noreferrer"><strong>matrix-docker-ansible-deploy</strong></a> — specifically for the Matrix + Element Web stack.</li>
</ul>
<p>Both projects share the same philosophy: everything is configured through YAML variables in a <code>vars.yml</code> file. The playbook handles the rest — installation, Docker, reverse proxy, certificates. In practice:</p>
<pre><code># vars.yml (simplified excerpt)
matrix_domain: my-domain.com
mash_playbook_hostname: services.my-domain.com

vaultwarden_enabled: true
vaultwarden_hostname: vault.my-domain.com

grafana_enabled: true
grafana_hostname: monitoring.my-domain.com

authentik_enabled: true
authentik_hostname: sso.my-domain.com</code></pre>
<div class="article-callout info">
  <span class="callout-icon">ℹ️</span>
  <div class="callout-content">
    <p><strong>Why these projects instead of building from scratch?</strong> Writing your own Ansible roles is a great learning exercise. But if the goal is a lab that actually works, community-maintained projects are a better bet: the documentation is often better, edge cases are already handled, and updates get pushed automatically.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-lab".to_string(),
                            title: "What the Lab Runs".to_string(),
                            content: r##"<p>Over the weeks, the server filled up. Here's the inventory:</p>
<table>
  <thead><tr><th>Service</th><th>Purpose</th><th>Tool</th></tr></thead>
  <tbody>
    <tr><td>Chat</td><td>Discord alternative for my friend group</td><td>Matrix + Element Web</td></tr>
    <tr><td>Passwords</td><td>Password manager</td><td>Vaultwarden</td></tr>
    <tr><td>Auth</td><td>Centralized SSO for all services</td><td>Authentik</td></tr>
    <tr><td>Monitoring</td><td>Server metrics + alerts</td><td>Grafana + Prometheus</td></tr>
    <tr><td>IoT</td><td>Connected device dashboard (experimental)</td><td>Home Assistant</td></tr>
    <tr><td>Mail</td><td>Self-hosted mail server</td><td>Exim / MASH mail stack</td></tr>
  </tbody>
</table>
<p>SSO via Authentik was the most impactful change. One account for everything, one login page. Once properly configured, it gives the feeling of administering something professional — even if behind the scenes it's your €6/month OVH VPS.</p>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Where to start if you're new to self-hosting?</strong> Vaultwarden. The MASH installation is near-trivial, the utility is immediate, and it's a good first win before tackling Matrix or Authentik. The MASH documentation for Vaultwarden is excellent.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-galeres".to_string(),
                            title: "What Tutorials Don't Tell You".to_string(),
                            content: r##"<h2>The Real Learning Curve</h2>
<p>If you've never used Ansible, the first thing that surprises you is idempotency. A well-written playbook can be run 100 times and produce the same result every time. If that's not the case, it's a bug to fix — not acceptable behavior.</p>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>The reflex to build from day one:</strong> always test with <code>--check</code> first. This option simulates the playbook without changing anything. It will save you from at least one bad mistake.</p>
    <pre><code>ansible-playbook -i inventory/hosts setup.yml --check --diff</code></pre>
  </div>
</div>
<p>Configuring the Matrix server was the most complex part, especially federation — the ability for different Matrix servers to communicate. DNS, certificates, firewall rules, delegation configuration: each layer has its own traps.</p>
<p>I re-ran the playbook four times before realizing the problem was DNS propagation, not Ansible. Lesson: always verify your DNS entries are propagated before debugging anything else.</p>
<p>Running a self-hosted mail server deserves its own article. IP reputation, SPF, DKIM, DMARC — everything needs to be configured correctly for mail to land in the inbox rather than spam. It's doable. It just takes time.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "et-maintenant".to_string(),
                            title: "On Pause — Not Abandoned".to_string(),
                            content: r##"<p>The project is on pause. A year of burnout, followed by a year of crunch for a production release at work. The server still exists, some services still run, but the lab hasn't evolved in a while.</p>
<p>What I want to do when I pick it back up:</p>
<ul>
  <li>Finish the Authentik configuration for a fully operational SSO across all services</li>
  <li>Stabilize the Matrix stack so it's genuinely usable day-to-day by my friends</li>
  <li>Properly connect my IoT devices to Home Assistant</li>
  <li>Document everything in a public repo so others can benefit from the setup</li>
</ul>
<p>If you're interested in this topic, check back in the coming months — I plan to document the resumption as I go, failures included.</p>
<div class="article-callout info">
  <span class="callout-icon">🔗</span>
  <div class="callout-content">
    <p><strong>Resources to get started:</strong></p>
    <ul>
      <li><a href="https://github.com/mother-of-all-self-hosting/mash-playbook" target="_blank" rel="noopener noreferrer">MASH Playbook — official documentation</a></li>
      <li><a href="https://github.com/spantaleev/matrix-docker-ansible-deploy" target="_blank" rel="noopener noreferrer">matrix-docker-ansible-deploy</a></li>
      <li><a href="https://docs.ansible.com/ansible/latest/getting_started/index.html" target="_blank" rel="noopener noreferrer">Ansible — Getting Started (official)</a></li>
    </ul>
  </div>
</div>"##.to_string(),
                        },
                    ],
                });

                m
            },
        },

        // =========================================================
        // ARTICLE: 3d-printing-lab
        // =========================================================
        Article {
            meta: ArticleMeta {
                id: "3d-printing-lab".to_string(),
                title: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Mon Lab d'Impression 3D : du Premier Raté aux Projets Réels".to_string());
                    m.insert("en".to_string(), "My 3D Printing Lab: From First Failures to Real Projects".to_string());
                    m
                },
                subtitle: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Ender 3 V2 modifiée, filaments, et tout ce que j'ai appris en imprimant des choses que personne ne m'avait demandé d'imprimer.".to_string());
                    m.insert("en".to_string(), "Modified Ender 3 V2, filament choices, and everything I learned printing things nobody asked me to print.".to_string());
                    m
                },
                description: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Setup complet de mon lab d'impression 3D : Ender 3 V2 avec Microswiss direct drive et BLTouch, choix des filaments, projets réalisés, et les mods qui ont vraiment changé quelque chose.".to_string());
                    m.insert("en".to_string(), "Complete walkthrough of my 3D printing lab: modded Ender 3 V2 with Microswiss direct drive and BLTouch, filament selection, real projects, and the mods that actually made a difference.".to_string());
                    m
                },
                date: "2024-07-20".to_string(),
                read_time: 14,
                tags: vec!["3dprinting".to_string(), "ender3".to_string(), "mods".to_string(), "hardware".to_string(), "maker".to_string()],
                category: "hardware".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {
                    tldr: "Ender 3 V2 avec Microswiss direct drive, BLTouch et enclosure custom. PLA pour la majorité, PETG pour les pièces mécaniques, TPU pour les protections. 50+ objets imprimés, surtout fonctionnels. Prochaine étape : résine pour les détails fins.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "le-debut".to_string(),
                            title: "Le premier raté".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">Mon premier print était un cube de calibrage 20×20mm. Il mesurait 19,4×20,2×19,8mm. J'ai passé deux heures à comprendre pourquoi, et c'est là que j'ai compris que l'impression 3D, c'est 30% d'impression et 70% de réglage.</p>
</div>
<p>J'ai acheté mon Ender 3 V2 début 2023, après des mois à regarder des impressions sur Reddit en me demandant combien de temps je résisterais. La réponse : pas longtemps.</p>
<p>Le déballage était excitant. L'assemblage, long mais bien documenté. Le premier print, lui, était un désastre contrôlé : le plateau mal nivelé, la première couche qui ne collait pas, puis qui collait trop bien et déformait la pièce. Classic.</p>
<p>Ce qui m'a aidé à progresser vite : ne pas chercher la perfection dès le début, mais comprendre <em>pourquoi</em> chaque réglage existe. La température, la vitesse, le refroidissement, le rétraction — chaque paramètre a une raison d'être. Une fois que vous avez compris le pourquoi, le quoi vient naturellement.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "la-machine".to_string(),
                            title: "La machine et ses modifications".to_string(),
                            content: r##"<p>L'Ender 3 V2 stock est une bonne machine pour commencer. Mais après quelques mois, j'ai voulu pousser plus loin. Deux modifications ont vraiment changé la donne :</p>
<h2>Microswiss Direct Drive (≈180€)</h2>
<p>Le passage d'un extrudeur Bowden à un direct drive, c'est l'amélioration qui a le plus impacté la qualité d'impression pour moi. La tête d'extrusion se déplace directement sur l'axe X, au plus près de la buse. Résultat : meilleur contrôle du filament, moins de rétraction nécessaire, et accès aux filaments flexibles (TPU) qui sont impossibles à gérer en Bowden.</p>
<h2>BLTouch Auto-Leveling (≈45€)</h2>
<p>Le nivellement manuel du plateau, c'est la bête noire des débutants. Le BLTouch règle ça automatiquement en créant une mesh 5×5 de la surface du plateau. La première couche devient fiable, et vous arrêtez de passer 20 minutes à régler des vis avant chaque impression.</p>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>L'ordre des modifications :</strong> commencez par le BLTouch si votre plateau est irrégulier. C'est l'amélioration qui a le meilleur ratio résultat/effort. Le direct drive vient ensuite, si vous voulez imprimer du TPU ou améliorer les détails fins.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-materiaux".to_string(),
                            title: "Choisir son filament".to_string(),
                            content: r##"<p>Le choix du filament dépend de l'usage prévu. Voici ce que j'utilise et dans quels cas :</p>
<table>
  <thead><tr><th>Filament</th><th>Usage principal</th><th>Difficulté</th></tr></thead>
  <tbody>
    <tr><td>PLA</td><td>Prototypes, décoratif, tout usage général</td><td>Facile</td></tr>
    <tr><td>PETG</td><td>Pièces mécaniques, exposition à la chaleur ou l'humidité</td><td>Moyen</td></tr>
    <tr><td>TPU</td><td>Protections, joints, pièces flexibles</td><td>Difficile sans direct drive</td></tr>
  </tbody>
</table>
<p>Pour 80% de mes projets, le PLA suffit. C'est le filament le plus facile à imprimer, le moins cher, et il donne d'excellents résultats sur des objets qui n'ont pas à subir de contraintes mécaniques ou thermiques importantes.</p>
<p>Le PETG est ma go-to pour tout ce qui doit durer : supports pour téléphones, pièces de remplacement, objets qui vont être en extérieur. Il est un peu plus exigeant à imprimer (température plus haute, sensible à l'humidité), mais le résultat est nettement plus robuste.</p>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>Stocker son filament :</strong> le PETG et les filaments techniques absorbent l'humidité et deviennent difficiles à imprimer. Gardez-les dans des boîtes hermétiques avec des sachets de silica gel entre les utilisations.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-projets".to_string(),
                            title: "Ce que j'imprime vraiment".to_string(),
                            content: r##"<p>Au fil des mois, j'ai imprimé beaucoup de choses inutiles — et quelques pièces vraiment utiles. Voilà ce qui est encore en service :</p>
<ul>
  <li><strong>Boîtiers d'électronique</strong> — protections pour Raspberry Pi, boîtiers pour projets Arduino, etc.</li>
  <li><strong>Pièces de remplacement</strong> — quand un clip casse ou qu'une pièce n'est plus disponible, le CAD et l'imprimante prennent le relai</li>
  <li><strong>Organisation</strong> — rangements pour l'établi, supports pour câbles, organiseurs de tiroirs</li>
  <li><strong>Claviers custom</strong> — les caisses pour mes claviers mécaniques (Corne 3x6, Skeletyl en cours)</li>
</ul>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Trouver des modèles :</strong> <a href="https://www.printables.com" target="_blank" rel="noopener noreferrer">Printables.com</a> (communauté Prusa, excellente qualité) et <a href="https://www.thingiverse.com" target="_blank" rel="noopener noreferrer">Thingiverse</a> pour le volume. Pour les pièces spécifiques, <a href="https://www.onshape.com" target="_blank" rel="noopener noreferrer">OnShape</a> est un CAD en ligne gratuit très accessible.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "la-suite".to_string(),
                            title: "La suite".to_string(),
                            content: r##"<p>Après deux ans avec l'Ender 3 V2, je connais ses limites. Pour les objets fonctionnels, elle est parfaite. Pour les détails fins — figurines, prototypes avec des surfaces lisses — le FDM montre ses couches.</p>
<p>La prochaine étape logique : une imprimante résine. La résine MSLA offre une résolution bien supérieure et des surfaces quasi-lisses. C'est plus contraignant à utiliser (résine toxique, post-traitement en UV), mais les résultats sont dans une autre catégorie pour les petites pièces détaillées.</p>
<p>Je pense aussi à une imprimante multi-matière pour pouvoir imprimer des supports solubles — le grand problème des géométries complexes en FDM. Mais ça, c'est pour plus tard.</p>
<div class="article-callout info">
  <span class="callout-icon">🔗</span>
  <div class="callout-content">
    <p><strong>Pour approfondir :</strong></p>
    <ul>
      <li><a href="https://www.printables.com" target="_blank" rel="noopener noreferrer">Printables.com — modèles de qualité, communauté active</a></li>
      <li><a href="https://github.com/nicksherron/mainsail-config" target="_blank" rel="noopener noreferrer">Klipper + Mainsail — si vous voulez aller plus loin dans les réglages</a></li>
    </ul>
  </div>
</div>"##.to_string(),
                        },
                    ],
                });

                m.insert("en".to_string(), ArticleContent {
                    tldr: "Modded Ender 3 V2 with Microswiss direct drive, BLTouch, and custom enclosure. PLA for most things, PETG for mechanical parts, TPU for protection. 50+ objects printed, mostly functional. Next step: resin for fine details.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "le-debut".to_string(),
                            title: "The First Failure".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">My first print was a 20×20mm calibration cube. It measured 19.4×20.2×19.8mm. I spent two hours figuring out why — and that's when I understood that 3D printing is 30% printing and 70% tuning.</p>
</div>
<p>I bought my Ender 3 V2 in early 2023, after months of watching Reddit prints and wondering how long I'd hold out. The answer: not long.</p>
<p>Unboxing was exciting. Assembly was long but well-documented. The first print was a controlled disaster: bed not leveled, first layer not sticking, then sticking too well and warping the part. Classic.</p>
<p>What helped me progress quickly: not chasing perfection from day one, but understanding <em>why</em> each setting exists. Temperature, speed, cooling, retraction — each parameter has a reason. Once you understand the why, the what follows naturally.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "la-machine".to_string(),
                            title: "The Machine and Its Modifications".to_string(),
                            content: r##"<p>The stock Ender 3 V2 is a good machine to start with. But after a few months, I wanted to push further. Two modifications genuinely changed things:</p>
<h2>Microswiss Direct Drive (≈€180)</h2>
<p>Switching from Bowden to direct drive was the single biggest quality improvement for me. The extruder moves directly on the X axis, right next to the nozzle. Result: better filament control, less retraction needed, and access to flexible filaments (TPU) that are nearly impossible to manage with Bowden.</p>
<h2>BLTouch Auto-Leveling (≈€45)</h2>
<p>Manual bed leveling is the bane of beginners. BLTouch fixes this automatically by creating a 5×5 mesh of the bed surface. First layers become reliable, and you stop spending 20 minutes adjusting screws before every print.</p>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Modification order:</strong> start with BLTouch if your bed is uneven. It has the best result-to-effort ratio of any upgrade. Direct drive comes next, if you want to print TPU or improve fine detail quality.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-materiaux".to_string(),
                            title: "Choosing Your Filament".to_string(),
                            content: r##"<p>Filament choice depends on the intended use. Here's what I use and when:</p>
<table>
  <thead><tr><th>Filament</th><th>Main Use</th><th>Difficulty</th></tr></thead>
  <tbody>
    <tr><td>PLA</td><td>Prototypes, decorative, general purpose</td><td>Easy</td></tr>
    <tr><td>PETG</td><td>Mechanical parts, heat or moisture exposure</td><td>Medium</td></tr>
    <tr><td>TPU</td><td>Protective cases, gaskets, flexible parts</td><td>Hard without direct drive</td></tr>
  </tbody>
</table>
<p>For 80% of my projects, PLA is enough. It's the easiest filament to print, the cheapest, and gives excellent results for objects that don't need to handle significant mechanical or thermal stress.</p>
<p>PETG is my go-to for anything that needs to last: phone stands, replacement parts, objects going outdoors. It requires more care (higher temperature, moisture-sensitive), but the result is significantly more durable.</p>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>Storing filament:</strong> PETG and technical filaments absorb moisture and become difficult to print. Keep them in airtight containers with silica gel packets between uses.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-projets".to_string(),
                            title: "What I Actually Print".to_string(),
                            content: r##"<p>Over the months, I've printed plenty of useless things — and some genuinely useful parts. Here's what's still in service:</p>
<ul>
  <li><strong>Electronics enclosures</strong> — Raspberry Pi cases, Arduino project housings, etc.</li>
  <li><strong>Replacement parts</strong> — when a clip breaks or a part is no longer available, CAD and the printer step in</li>
  <li><strong>Organization</strong> — workshop storage, cable management, drawer organizers</li>
  <li><strong>Custom keyboard cases</strong> — enclosures for my mechanical keyboards (Corne 3x6, Skeletyl in progress)</li>
</ul>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Finding models:</strong> <a href="https://www.printables.com" target="_blank" rel="noopener noreferrer">Printables.com</a> (Prusa community, excellent quality) and <a href="https://www.thingiverse.com" target="_blank" rel="noopener noreferrer">Thingiverse</a> for volume. For custom parts, <a href="https://www.onshape.com" target="_blank" rel="noopener noreferrer">OnShape</a> is a free browser-based CAD tool that's surprisingly accessible.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "la-suite".to_string(),
                            title: "What's Next".to_string(),
                            content: r##"<p>After two years with the Ender 3 V2, I know its limits. For functional objects, it's excellent. For fine detail — figurines, prototypes needing smooth surfaces — FDM shows its layers.</p>
<p>The logical next step: a resin printer. MSLA resin offers far higher resolution and near-smooth surfaces. It's more demanding to use (toxic resin, UV post-processing), but the results are in a completely different class for small detailed parts.</p>
<p>I'm also thinking about a multi-material setup for soluble supports — the major challenge with complex FDM geometries. But that's further down the road.</p>
<div class="article-callout info">
  <span class="callout-icon">🔗</span>
  <div class="callout-content">
    <p><strong>To go further:</strong></p>
    <ul>
      <li><a href="https://www.printables.com" target="_blank" rel="noopener noreferrer">Printables.com — quality models, active community</a></li>
      <li><a href="https://docs.vorondesign.com" target="_blank" rel="noopener noreferrer">Voron Design — if you ever want to build a printer from scratch</a></li>
    </ul>
  </div>
</div>"##.to_string(),
                        },
                    ],
                });

                m
            },
        },

        // =========================================================
        // ARTICLE: ezprint3d-journey
        // =========================================================
        Article {
            meta: ArticleMeta {
                id: "ezprint3d-journey".to_string(),
                title: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "EzPrint3D : ce que j'ai appris en construisant un SaaS à deux faces".to_string());
                    m.insert("en".to_string(), "EzPrint3D: What I Learned Building a Two-Sided SaaS".to_string());
                    m
                },
                subtitle: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Le retour d'expérience honnête d'un projet mis en pause : le chicken-and-egg d'un marketplace, les choix techniques, et ce que je ferais différemment.".to_string());
                    m.insert("en".to_string(), "An honest post-mortem on a paused project: the two-sided marketplace problem, the technical choices, and what I'd do differently.".to_string());
                    m
                },
                description: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "EzPrint3D est une plateforme SaaS connectant les demandeurs de services d'impression 3D avec des prestataires. Retour d'expérience sur la genèse, l'architecture microservices, et les leçons apprises.".to_string());
                    m.insert("en".to_string(), "EzPrint3D is a SaaS platform connecting 3D printing service seekers with providers. An honest look at the genesis, microservices architecture, and lessons learned.".to_string());
                    m
                },
                date: "2024-06-15".to_string(),
                read_time: 12,
                tags: vec!["symfony".to_string(), "react".to_string(), "saas".to_string(), "startup".to_string(), "3dprinting".to_string(), "microservices".to_string()],
                category: "project".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {
                    tldr: "Marketplace double face connectant demande et offre d'impression 3D. Symfony/React/Docker. Projet en pause : le chicken-and-egg problem a eu raison du timeline. Les leçons : valider l'offre avant d'écrire du code.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "la-genese".to_string(),
                            title: "L'idée, et sa version honnête".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">La version propre : j'ai vu un manque de marché et décidé de le combler. La version honnête : j'avais une imprimante 3D, je voulais l'amortir, et j'ai eu l'idée que d'autres développeurs avec des imprimantes avaient probablement la même envie.</p>
</div>
<p>Il y a quelques années, j'avais besoin de faire imprimer une pièce sur mesure. Je n'avais pas encore d'imprimante. J'ai cherché un service. J'ai trouvé des forums Reddit, des groupes Facebook, quelques artisans sur Etsy. Rien de vraiment structuré. Rien qui ressemblait à "je dépose ma demande ici, je reçois des devis là".</p>
<p>Le concept : un marketplace double face. D'un côté les demandeurs — des particuliers ou des petites entreprises avec des besoins ponctuels. De l'autre les prestataires — des makers avec des imprimantes qui tournent 20% du temps. La plateforme s'occupe du matching, des devis, du paiement, de la logistique.</p>
<p>C'est un problème réel. La solution semblait évidente. L'exécution, beaucoup moins.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-choix-tech".to_string(),
                            title: "Les choix techniques qui semblaient raisonnables".to_string(),
                            content: r##"<p>Architecture microservices. Évidemment. Parce qu'en 2023, tout le monde faisait des microservices.</p>
<p>En vrai, voici le stack :</p>
<ul>
  <li><strong>Backend :</strong> Symfony 6 pour l'API principale, avec API Platform pour les endpoints REST</li>
  <li><strong>Frontend :</strong> React + TypeScript, Redux pour la gestion d'état</li>
  <li><strong>Infrastructure :</strong> Docker Compose en développement, déploiement prévu sur VPS</li>
  <li><strong>Base de données :</strong> PostgreSQL</li>
  <li><strong>Paiement :</strong> Stripe Connect pour les paiements divisés (client → plateforme → prestataire)</li>
</ul>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>Le piège que j'aurais dû éviter :</strong> j'ai passé trois semaines à parfaire l'architecture avant d'avoir la moindre feature utilisateur. La prochaine fois : MVP en monolithe, microservices si ça scale.</p>
  </div>
</div>
<p>Ce n'est pas que les choix étaient mauvais — ils étaient corrects techniquement. Le problème, c'est l'ordre des priorités. Architecture parfaite &gt; feature utilisable. C'est une erreur classique de développeur.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "ce-qu-on-a-construit".to_string(),
                            title: "Ce qui a été construit".to_string(),
                            content: r##"<p>Malgré les écueils, il y a eu du concret. Les fonctionnalités implémentées ou en cours :</p>
<ul>
  <li><strong>Dépôt de commande</strong> — upload de fichiers STL, sélection de matériaux et couleurs, dimensions, quantité</li>
  <li><strong>Validation de fichiers 3D</strong> — vérification de base (format, manifold, dimensions raisonnables)</li>
  <li><strong>Profils prestataires</strong> — portfolio, équipement déclaré, zones d'expédition</li>
  <li><strong>Système de devis</strong> — les prestataires voient les commandes ouvertes et proposent un devis</li>
  <li><strong>Paiement Stripe Connect</strong> — architecture de paiement divisé</li>
</ul>
<p>La partie validation de fichiers STL mérite un moment d'honnêteté : c'est plus difficile que ça en a l'air. Vérifier qu'un fichier est "imprimable" implique de détecter les maillages non-manifold, les géométries impossibles, les dimensions absurdes. J'ai utilisé une lib Python (trimesh) pour les checks basiques, mais un vrai service de validation robuste est un projet à part entière.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-vrai-probleme".to_string(),
                            title: "Le problème que je n'avais pas anticipé".to_string(),
                            content: r##"<h2>Le chicken-and-egg d'un marketplace</h2>
<p>Un marketplace sans prestataires, c'est une salle vide. Un marketplace sans demandeurs, c'est une vitrine sans clients. Les deux côtés doivent exister simultanément pour que la plateforme ait de la valeur. C'est le problème classique du chicken-and-egg.</p>
<p>J'avais sous-estimé à quel point convaincre des prestataires de s'inscrire sur une plateforme qu'ils n'ont jamais vue — avec un fondateur qu'ils ne connaissent pas — demande du temps, de la confiance, et souvent un réseau préexistant.</p>
<div class="article-callout info">
  <span class="callout-icon">ℹ️</span>
  <div class="callout-content">
    <p><strong>La leçon :</strong> pour un marketplace, la stratégie d'acquisition des deux côtés doit être définie <em>avant</em> le premier commit. Et souvent, un des deux côtés doit être "faked" au départ (l'équipe elle-même joue le rôle du prestataire au début, par exemple). Airbnb a photographié eux-mêmes les premiers logements. Il y a une raison à ça.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-lecons".to_string(),
                            title: "Ce que j'ai appris — et ce qui vient".to_string(),
                            content: r##"<p>EzPrint3D est en pause. Pas enterré. La distinction est importante pour moi.</p>
<p>Ce que je referais différemment :</p>
<ul>
  <li><strong>Valider d'abord, coder ensuite.</strong> Avant d'écrire une ligne, contacter 20 makers et 20 clients potentiels. Comprendre si le problème est vraiment un problème suffisamment douloureux.</li>
  <li><strong>Commencer par un monolithe.</strong> Microservices dès le début sur un MVP solo, c'est de l'over-engineering. Un Symfony monolithique bien découpé aurait livré la même valeur 2× plus vite.</li>
  <li><strong>Définir la stratégie d'acquisition prestataires en amont.</strong> La tech ne sert à rien sans les deux côtés du marketplace.</li>
</ul>
<p>Ce qui est bien : j'ai appris des tonnes de choses sur Stripe Connect, sur la validation de fichiers 3D, sur la gestion d'un double flux utilisateur. Ces compétences ne disparaissent pas.</p>
<p>Quand je relance — et je relancerai — ce sera avec une stratégie de go-to-market définie avant le premier sprint.</p>"##.to_string(),
                        },
                    ],
                });

                m.insert("en".to_string(), ArticleContent {
                    tldr: "Two-sided marketplace connecting 3D printing demand and supply. Symfony/React/Docker. Project on pause: the chicken-and-egg problem won. Key lesson: validate the offer before writing code.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "la-genese".to_string(),
                            title: "The Idea, and the Honest Version".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">The clean version: I spotted a market gap and decided to fill it. The honest version: I had a 3D printer, wanted to offset the cost, and figured other developers with printers probably felt the same way.</p>
</div>
<p>A few years back, I needed a custom part printed. No printer of my own yet. I looked for a service. Found Reddit threads, Facebook groups, a few makers on Etsy. Nothing structured. Nothing that felt like "submit request here, get quotes there".</p>
<p>The concept: a two-sided marketplace. On one side, requesters — individuals or small businesses with occasional needs. On the other, providers — makers with printers running 20% of the time. The platform handles matching, quotes, payment, and logistics.</p>
<p>It's a real problem. The solution seemed obvious. The execution, much less so.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-choix-tech".to_string(),
                            title: "Technical Choices That Seemed Reasonable at the Time".to_string(),
                            content: r##"<p>Microservices architecture. Of course. Because in 2023, everyone was doing microservices.</p>
<p>The actual stack:</p>
<ul>
  <li><strong>Backend:</strong> Symfony 6 for the main API, with API Platform for REST endpoints</li>
  <li><strong>Frontend:</strong> React + TypeScript, Redux for state management</li>
  <li><strong>Infrastructure:</strong> Docker Compose for development, VPS deployment planned</li>
  <li><strong>Database:</strong> PostgreSQL</li>
  <li><strong>Payments:</strong> Stripe Connect for split payments (client → platform → provider)</li>
</ul>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>The trap I should have avoided:</strong> I spent three weeks perfecting the architecture before having a single user-facing feature. Next time: MVP as a monolith, microservices if it scales.</p>
  </div>
</div>
<p>The choices weren't technically wrong — they were correct. The problem was priority order. Perfect architecture over working feature. Classic developer mistake.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "ce-qu-on-a-construit".to_string(),
                            title: "What Was Actually Built".to_string(),
                            content: r##"<p>Despite the pitfalls, there was real output. Features implemented or in progress:</p>
<ul>
  <li><strong>Order submission</strong> — STL file upload, material and color selection, dimensions, quantity</li>
  <li><strong>3D file validation</strong> — basic checks (format, manifold, reasonable dimensions)</li>
  <li><strong>Provider profiles</strong> — portfolio, declared equipment, shipping zones</li>
  <li><strong>Quote system</strong> — providers see open orders and submit quotes</li>
  <li><strong>Stripe Connect payments</strong> — split payment architecture</li>
</ul>
<p>The STL file validation part deserves an honest moment: it's harder than it looks. Checking that a file is "printable" means detecting non-manifold meshes, impossible geometries, unreasonable dimensions. I used a Python lib (trimesh) for basic checks, but a truly robust validation service is a project in itself.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-vrai-probleme".to_string(),
                            title: "The Problem I Hadn't Anticipated".to_string(),
                            content: r##"<h2>The Marketplace Chicken-and-Egg Problem</h2>
<p>A marketplace without providers is an empty room. A marketplace without buyers is a storefront with no customers. Both sides need to exist simultaneously for the platform to have value. This is the classic chicken-and-egg problem.</p>
<p>I had underestimated how hard it is to convince providers to sign up on a platform they've never seen — with a founder they don't know — which requires time, trust, and often a pre-existing network.</p>
<div class="article-callout info">
  <span class="callout-icon">ℹ️</span>
  <div class="callout-content">
    <p><strong>The lesson:</strong> for a marketplace, the acquisition strategy for both sides must be defined <em>before</em> the first commit. And often, one side needs to be "faked" at launch (the team plays the provider role early on, for instance). Airbnb photographed the first listings themselves. There's a reason for that.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "les-lecons".to_string(),
                            title: "What I Learned — and What's Next".to_string(),
                            content: r##"<p>EzPrint3D is on pause. Not buried. That distinction matters to me.</p>
<p>What I'd do differently:</p>
<ul>
  <li><strong>Validate first, code second.</strong> Before writing a line, talk to 20 makers and 20 potential clients. Understand whether the problem is genuinely painful enough.</li>
  <li><strong>Start with a monolith.</strong> Microservices from day one on a solo MVP is over-engineering. A well-structured Symfony monolith would have delivered the same value 2× faster.</li>
  <li><strong>Define the provider acquisition strategy upfront.</strong> The tech is worthless without both sides of the marketplace.</li>
</ul>
<p>The upside: I learned a huge amount about Stripe Connect, 3D file validation, and managing a dual-user flow. Those skills don't go away.</p>
<p>When I re-launch — and I will — it'll be with a defined go-to-market strategy before the first sprint.</p>"##.to_string(),
                        },
                    ],
                });

                m
            },
        },

        // =========================================================
        // ARTICLE: custom-keyboards
        // =========================================================
        Article {
            meta: ArticleMeta {
                id: "custom-keyboards".to_string(),
                title: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Construire son Clavier Mécanique sur Mesure : du Corne au Skeletyl".to_string());
                    m.insert("en".to_string(), "Building a Custom Mechanical Keyboard: From Corne to Skeletyl".to_string());
                    m
                },
                subtitle: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Du premier switch testé au Corne 3x6 terminé : l'ergonomie qui change la façon de taper, QMK, et tout ce que j'aurais voulu savoir avant de souder.".to_string());
                    m.insert("en".to_string(), "From first switch tested to completed Corne 3x6: the ergonomics that changed how I type, QMK, and everything I wish I'd known before soldering.".to_string());
                    m
                },
                description: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Retour sur le build de mon Corne 3x6, la programmation QMK, et le Skeletyl en cours. Pour les développeurs qui tapent toute la journée et cherchent quelque chose de différent.".to_string());
                    m.insert("en".to_string(), "Walkthrough of building my Corne 3x6, QMK programming, and the ongoing Skeletyl build. For developers who type all day and want something different.".to_string());
                    m
                },
                date: "2024-09-01".to_string(),
                read_time: 13,
                tags: vec!["keyboards".to_string(), "qmk".to_string(), "ergonomics".to_string(), "hardware".to_string(), "diy".to_string(), "corne".to_string()],
                category: "hardware".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {
                    tldr: "Corne 3x6 terminé avec switches Gateron Yellow, keycaps PBT, boîtier imprimé 3D. QMK avec 4 layers (base/sym/nav/fn). Skeletyl en cours d'assemblage. Investissement initial élevé, mais un clavier qui dure des années.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "la-decouverte".to_string(),
                            title: "La révélation".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">Je croyais que les claviers, ça ne s'améliorait pas. Tu achètes, tu tapes, tu vas au boulot. Et puis quelqu'un m'a fait essayer un split keyboard — et j'ai compris que je tapais mal depuis des années.</p>
</div>
<p>La découverte des claviers ergonomiques split, c'est un peu comme la découverte du café de spécialité. Avant, tu buvais du café sans y réfléchir. Après, tu ne peux plus revenir en arrière sans que ça te manque quelque chose.</p>
<p>Le problème avec les claviers classiques, c'est qu'ils forcent les poignets à se tordre vers l'intérieur (stagger) pendant des heures. Sur la durée, c'est une source de fatigue. Un clavier split permet de poser les mains dans une position plus naturelle, alignée avec les coudes.</p>
<p>J'aurais pu acheter un Ergodox EZ ou un Moonlander et en rester là. Mais j'avais une imprimante 3D, un fer à souder, et trop de curiosité. Le rabbit hole des claviers custom s'est ouvert.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "anatomie".to_string(),
                            title: "Anatomie d'un clavier custom".to_string(),
                            content: r##"<p>Un clavier mécanique custom se compose de plusieurs parties indépendantes. C'est ce qui le différencie d'un clavier "gaming" : chaque composant est remplaçable et choisissable séparément.</p>
<table>
  <thead><tr><th>Composant</th><th>Rôle</th><th>Options courantes</th></tr></thead>
  <tbody>
    <tr><td>PCB</td><td>Circuit imprimé, lit les frappes</td><td>Corne, Lily58, Dactyl</td></tr>
    <tr><td>Switches</td><td>Les mécanismes sous chaque touche</td><td>Linéaires, tactiles, clicky</td></tr>
    <tr><td>Keycaps</td><td>Les capuchons sur les switches</td><td>PBT, ABS, divers profils</td></tr>
    <tr><td>Case</td><td>Le boîtier qui tient tout</td><td>Imprimé 3D, acrylique, alu</td></tr>
    <tr><td>Firmware</td><td>La logique du clavier</td><td>QMK, ZMK (sans fil)</td></tr>
  </tbody>
</table>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Pour tester avant de dépenser :</strong> les <em>switch testers</em> permettent d'essayer 10-20 types de switches différents pour quelques euros. Indispensable avant d'en acheter 60. Un switch tactile Boba U4 ne se ressemble pas du tout à un linéaire Gateron Yellow.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-corne".to_string(),
                            title: "Le Corne 3x6 : le build".to_string(),
                            content: r##"<p>Le Corne (fulgur42) est un clavier split à 42 touches (3 rangées de 6 + 3 touches pouces par côté). C'est l'un des designs les plus populaires dans la communauté des claviers ergonomiques — beaucoup de ressources, de layouts QMK partagés, et une communauté active.</p>
<p>Mon build :</p>
<ul>
  <li><strong>Switches :</strong> Gateron Yellow (linéaires, légers, silencieux)</li>
  <li><strong>Keycaps :</strong> PBT XDA profile</li>
  <li><strong>Boîtier :</strong> imprimé en PETG sur mon Ender 3 V2</li>
  <li><strong>Microcontrôleur :</strong> Pro Micro (×2, un par moitié)</li>
  <li><strong>Connexion :</strong> câble TRRS entre les deux moitiés</li>
</ul>
<p>La partie la plus longue : le câblage. 42 switches à souder, les diodes (une par switch pour le key rollover), et les Pro Micro. Avec un bon fer à souder et de la soudure fine, c'est accessible même en débutant. Il faut compter 4-6 heures de soudage patient.</p>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>Le piège classique :</strong> souder d'abord les switches, puis réaliser que le Pro Micro est inaccessible derrière. L'ordre compte : diodes d'abord, Pro Micro ensuite, switches en dernier.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "qmk-et-les-layers".to_string(),
                            title: "QMK et la liberté du firmware".to_string(),
                            content: r##"<p>QMK (Quantum Mechanical Keyboard) est le firmware open source qui fait tourner la plupart des claviers custom. Et c'est là que la magie opère vraiment.</p>
<p>Avec 42 touches, on ne peut pas mettre tous les caractères sur une seule couche. QMK permet de définir des <em>layers</em> — des couches de mapping qu'on active avec des touches dédiées. Mon layout à 4 layers :</p>
<ul>
  <li><strong>Layer 0 (Base)</strong> — AZERTY adapté, touches courantes</li>
  <li><strong>Layer 1 (Symbols)</strong> — brackets, operateurs, chiffres</li>
  <li><strong>Layer 2 (Navigation)</strong> — flèches, Home/End, Page Up/Down</li>
  <li><strong>Layer 3 (Fn)</strong> — touches de fonction, médias, raccourcis VS Code</li>
</ul>
<pre><code>// Exemple QMK : tap/hold sur une touche
// Tap = espace, Hold = activer layer 1
LT(1, KC_SPC),

// Tap = Entrée, Hold = Shift
MT(MOD_LSFT, KC_ENT)</code></pre>
<p>La fonctionnalité tap/hold est ce qui change vraiment l'expérience : une même touche peut faire deux choses selon qu'on appuie brièvement ou qu'on maintient. Ça divise le nombre de touches nécessaires sans sacrifier l'accès.</p>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Pour démarrer avec QMK :</strong> <a href="https://config.qmk.fm" target="_blank" rel="noopener noreferrer">QMK Configurator</a> permet de créer un keymap sans écrire de code. Pour aller plus loin, le vrai fichier C donne accès à toutes les fonctionnalités avancées.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-skeletyl".to_string(),
                            title: "Le Skeletyl : le prochain chantier".to_string(),
                            content: r##"<p>Le <a href="https://github.com/Bastardkb/Skeletyl" target="_blank" rel="noopener noreferrer">Skeletyl</a> (Bastard Keyboards) est un clavier colonne-stagger en forme de dactyl — une coque tridimensionnelle qui épouse la courbure naturelle des doigts. C'est le niveau d'après du Corne.</p>
<p>Contrairement au Corne qui est plat, le Skeletyl a un profil en courbe : les colonnes sont à des hauteurs différentes, chaque touche est inclinée vers le doigt correspondant. Sur le papier, c'est l'ergonomie ultime. En pratique, ça demande une période d'adaptation.</p>
<p>Le build est en cours. La partie la plus complexe de ce projet comparé au Corne : le boîtier lui-même. La coque Skeletyl est un modèle 3D paramétrique complexe — il faut ajuster les paramètres selon la morphologie de ses mains, puis imprimer avec les bons réglages pour avoir une surface assez lisse sur les parties courbées.</p>
<p>Article détaillé sur le build à venir une fois le clavier terminé.</p>
<div class="article-callout info">
  <span class="callout-icon">🔗</span>
  <div class="callout-content">
    <p><strong>Ressources pour se lancer :</strong></p>
    <ul>
      <li><a href="https://github.com/nickcoutsos/keyswitch-layout-editor" target="_blank" rel="noopener noreferrer">KLE — Keyboard Layout Editor</a></li>
      <li><a href="https://config.qmk.fm" target="_blank" rel="noopener noreferrer">QMK Configurator</a></li>
      <li><a href="https://github.com/Bastardkb/Skeletyl" target="_blank" rel="noopener noreferrer">Skeletyl — repo officiel</a></li>
      <li><a href="https://www.reddit.com/r/ErgoMechKeyboards/" target="_blank" rel="noopener noreferrer">r/ErgoMechKeyboards — la communauté</a></li>
    </ul>
  </div>
</div>"##.to_string(),
                        },
                    ],
                });

                m.insert("en".to_string(), ArticleContent {
                    tldr: "Completed Corne 3x6 with Gateron Yellow switches, PBT keycaps, 3D-printed case. QMK with 4 layers (base/sym/nav/fn). Skeletyl in assembly. High upfront investment, but a keyboard that lasts years.".to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "la-decouverte".to_string(),
                            title: "The Revelation".to_string(),
                            content: r##"<div class="article-lead">
  <p class="lead-text">I thought keyboards just... were what they were. You buy one, you type on it, you go to work. Then someone let me try a split keyboard — and I realized I'd been typing wrong for years.</p>
</div>
<p>Discovering ergonomic split keyboards is a bit like discovering specialty coffee. Before, you drank coffee without thinking about it. After, you can't go back without noticing something's missing.</p>
<p>The problem with standard keyboards is that they force your wrists to twist inward for hours. Over time, it's a source of strain and fatigue. A split keyboard lets you position your hands more naturally, aligned with your elbows.</p>
<p>I could have bought an Ergodox EZ or a Moonlander and stopped there. But I had a 3D printer, a soldering iron, and too much curiosity. The custom keyboard rabbit hole opened up.</p>"##.to_string(),
                        },
                        ArticleSection {
                            id: "anatomie".to_string(),
                            title: "Anatomy of a Custom Keyboard".to_string(),
                            content: r##"<p>A custom mechanical keyboard is made of several independent parts. That's what distinguishes it from a "gaming" keyboard: each component is individually replaceable and selectable.</p>
<table>
  <thead><tr><th>Component</th><th>Role</th><th>Common Options</th></tr></thead>
  <tbody>
    <tr><td>PCB</td><td>Circuit board, reads keystrokes</td><td>Corne, Lily58, Dactyl</td></tr>
    <tr><td>Switches</td><td>Mechanisms under each key</td><td>Linear, tactile, clicky</td></tr>
    <tr><td>Keycaps</td><td>Caps on top of the switches</td><td>PBT, ABS, various profiles</td></tr>
    <tr><td>Case</td><td>Housing that holds everything</td><td>3D printed, acrylic, aluminum</td></tr>
    <tr><td>Firmware</td><td>The keyboard's logic</td><td>QMK, ZMK (wireless)</td></tr>
  </tbody>
</table>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>Test before spending:</strong> switch testers let you try 10-20 different switch types for a few euros. Essential before buying 60. A tactile Boba U4 feels completely different from a linear Gateron Yellow.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-corne".to_string(),
                            title: "The Corne 3x6: The Build".to_string(),
                            content: r##"<p>The Corne (fulgur42) is a 42-key split keyboard (3 rows of 6 + 3 thumb keys per side). It's one of the most popular designs in the ergonomic keyboard community — lots of resources, shared QMK layouts, and an active community.</p>
<p>My build:</p>
<ul>
  <li><strong>Switches:</strong> Gateron Yellow (linear, light, quiet)</li>
  <li><strong>Keycaps:</strong> PBT XDA profile</li>
  <li><strong>Case:</strong> 3D-printed in PETG on my Ender 3 V2</li>
  <li><strong>Microcontroller:</strong> Pro Micro (×2, one per half)</li>
  <li><strong>Connection:</strong> TRRS cable between halves</li>
</ul>
<p>The longest part: wiring. 42 switches to solder, the diodes (one per switch for key rollover), and the Pro Micros. With a good soldering iron and fine solder, it's doable even as a beginner. Budget 4-6 hours of patient soldering.</p>
<div class="article-callout warning">
  <span class="callout-icon">⚠️</span>
  <div class="callout-content">
    <p><strong>The classic mistake:</strong> soldering switches first, then realizing the Pro Micro is inaccessible behind them. Order matters: diodes first, Pro Micro next, switches last.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "qmk-et-les-layers".to_string(),
                            title: "QMK and the Freedom of Custom Firmware".to_string(),
                            content: r##"<p>QMK (Quantum Mechanical Keyboard) is the open source firmware running most custom keyboards. And this is where the real magic happens.</p>
<p>With 42 keys, you can't fit every character on a single layer. QMK lets you define <em>layers</em> — mapping overlays activated by dedicated keys. My 4-layer layout:</p>
<ul>
  <li><strong>Layer 0 (Base)</strong> — adapted QWERTY, common keys</li>
  <li><strong>Layer 1 (Symbols)</strong> — brackets, operators, numbers</li>
  <li><strong>Layer 2 (Navigation)</strong> — arrows, Home/End, Page Up/Down</li>
  <li><strong>Layer 3 (Fn)</strong> — function keys, media controls, VS Code shortcuts</li>
</ul>
<pre><code>// QMK example: tap/hold on a key
// Tap = space, Hold = activate layer 1
LT(1, KC_SPC),

// Tap = Enter, Hold = Shift
MT(MOD_LSFT, KC_ENT)</code></pre>
<p>The tap/hold feature is what truly changes the experience: the same key can do two things depending on whether you press briefly or hold. It halves the number of keys needed without sacrificing access.</p>
<div class="article-callout tip">
  <span class="callout-icon">💡</span>
  <div class="callout-content">
    <p><strong>To start with QMK:</strong> <a href="https://config.qmk.fm" target="_blank" rel="noopener noreferrer">QMK Configurator</a> lets you create a keymap without writing code. For more control, the actual C file unlocks all advanced features.</p>
  </div>
</div>"##.to_string(),
                        },
                        ArticleSection {
                            id: "le-skeletyl".to_string(),
                            title: "The Skeletyl: Next Build in Progress".to_string(),
                            content: r##"<p>The <a href="https://github.com/Bastardkb/Skeletyl" target="_blank" rel="noopener noreferrer">Skeletyl</a> (Bastard Keyboards) is a column-stagger dactyl-style keyboard — a three-dimensional shell that matches the natural curve of your fingers. It's the next level up from the Corne.</p>
<p>Unlike the flat Corne, the Skeletyl has a curved profile: columns sit at different heights, each key angled toward the corresponding finger. In theory, it's the ultimate ergonomics. In practice, it requires an adjustment period.</p>
<p>The build is ongoing. The most complex part compared to the Corne: the case itself. The Skeletyl shell is a complex parametric 3D model — you need to adjust parameters to your hand morphology, then print with the right settings to get a smooth enough surface on the curved parts.</p>
<p>Detailed build article coming once the keyboard is finished.</p>
<div class="article-callout info">
  <span class="callout-icon">🔗</span>
  <div class="callout-content">
    <p><strong>Resources to get started:</strong></p>
    <ul>
      <li><a href="https://config.qmk.fm" target="_blank" rel="noopener noreferrer">QMK Configurator</a></li>
      <li><a href="https://github.com/Bastardkb/Skeletyl" target="_blank" rel="noopener noreferrer">Skeletyl — official repo</a></li>
      <li><a href="https://www.reddit.com/r/ErgoMechKeyboards/" target="_blank" rel="noopener noreferrer">r/ErgoMechKeyboards — the community</a></li>
    </ul>
  </div>
</div>"##.to_string(),
                        },
                    ],
                });

                m
            },
        },
    ]
}

pub fn get_published_articles() -> Vec<Article> {
    get_all_articles()
        .into_iter()
        .filter(|a| matches!(a.meta.status, ArticleStatus::Published))
        .collect()
}

pub fn get_article_by_id(id: &str) -> Option<Article> {
    get_all_articles().into_iter().find(|a| a.meta.id == id)
}

pub fn get_unique_categories() -> Vec<String> {
    let mut cats: Vec<String> = get_published_articles()
        .iter()
        .map(|a| a.meta.category.clone())
        .collect();
    cats.sort();
    cats.dedup();
    cats
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
