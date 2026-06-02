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
                    m.insert("fr".to_string(), "Vaultwarden : Bitwarden chez toi, sans les compromis".to_string());
                    m.insert("en".to_string(), "Vaultwarden: Self-Host Bitwarden Without the Bloat".to_string());
                    m
                },
                subtitle: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Un serveur Bitwarden alternatif en Rust, léger, rapide et 100% compatible avec les clients officiels".to_string());
                    m.insert("en".to_string(), "A lightweight Rust-powered Bitwarden-compatible server, perfect for self-hosted password management".to_string());
                    m
                },
                description: {
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "Vaultwarden est une implémentation alternative du serveur Bitwarden en Rust, idéale pour le self-hosting sur petit matériel.".to_string());
                    m.insert("en".to_string(), "Vaultwarden is a lightweight Rust alternative to the official Bitwarden server, perfect for self-hosting on low-resource hardware.".to_string());
                    m
                },
                date: "2026-06-02".to_string(),
                read_time: 8,
                tags: vec!["self-hosting".to_string(), "securite".to_string(), "rust".to_string(), "docker".to_string(), "password-manager".to_string(), "bitwarden".to_string(), "souverainete-numerique".to_string(), "homelab".to_string()],
                category: "devtools".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            },
            content: {
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {
                    tldr: r###"Vaultwarden réimplémente l'API Bitwarden en Rust avec une empreinte mémoire infime. Compatible avec tous les clients Bitwarden officiels, il se déploie en une commande Docker sur n'importe quel VPS ou Raspberry Pi. Souveraineté totale sur tes mots de passe, zéro abonnement."###.to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "le-probleme".to_string(),
                            title: "Le problème que ça résout".to_string(),
                            content: r###"<p>Les gestionnaires de mots de passe cloud, c'est pratique. Jusqu'au jour où tu te demandes <strong>qui a réellement accès à tes secrets</strong>. Bitwarden est une excellente option — open source, audité, clients disponibles partout. Mais l'instance officielle est hébergée chez eux, et auto-héberger le serveur officiel est une autre histoire.</p><p>Le serveur Bitwarden officiel est une stack .NET avec des dépendances lourdes : SQL Server (ou SQLite dans les versions récentes), plusieurs services qui tournent en parallèle, une consommation mémoire qui commence autour de <strong>1 à 2 Go de RAM</strong> au repos. Pour un homelab sur Raspberry Pi 4 ou un petit VPS à 5€/mois, c'est rédhibitoire.</p><p>Le vrai problème : tu veux la <strong>souveraineté numérique totale</strong> sur tes mots de passe — tes clés SSH, tokens d'API, accès serveurs, secrets de projets — sans sacrifier le confort des clients officiels (extensions navigateur, apps mobiles iOS/Android, CLI). Tu veux synchronisation en temps réel, 2FA, partage de coffres, et tout ça sur ton propre matériel.</p><div class="article-callout info"><span class="callout-icon">ℹ️</span><div class="callout-content"><p>Bitwarden a considérablement amélioré son serveur officiel ces dernières années, notamment avec le support SQLite. Vaultwarden reste néanmoins l'option de référence pour les contraintes de ressources et la simplicité de déploiement.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "ce-que-cest".to_string(),
                            title: "Ce que c'est".to_string(),
                            content: r###"<p><strong>Vaultwarden</strong> (anciennement <em>bitwarden_rs</em>) est une réimplémentation complète de l'API serveur Bitwarden, écrite en <strong>Rust</strong> avec le framework <a href="https://rocket.rs" target="_blank" rel="noopener noreferrer">Rocket</a>. Le projet est maintenu par <a href="https://github.com/dani-garcia" target="_blank" rel="noopener noreferrer">dani-garcia</a> et la communauté, et cumule plus de <strong>61 000 étoiles GitHub</strong> — c'est l'un des projets self-hosting les plus populaires de l'écosystème.</p><p>Ce n'est <strong>pas un fork</strong> de Bitwarden : c'est une réécriture from scratch qui implémente les mêmes endpoints d'API. Les clients officiels Bitwarden (extension Chrome/Firefox, apps Android/iOS, client desktop, CLI <code>bw</code>) ne font aucune différence avec le serveur officiel. Tu pointes ton client vers ton instance, et tout fonctionne.</p><ul><li><strong>Langage :</strong> Rust — performances natives, empreinte mémoire ultra-faible (~10 Mo au démarrage)</li><li><strong>Base de données :</strong> SQLite par défaut, MySQL et PostgreSQL supportés</li><li><strong>Déploiement :</strong> image Docker officielle, binaire statique disponible</li><li><strong>Licence :</strong> AGPL-3.0 — libre et copyleft</li><li><strong>Fonctionnalités :</strong> coffres individuels et organisations, 2FA (TOTP, WebAuthn, Duo), sends, attachements, API complète</li></ul><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Vaultwarden n'est pas affilié à Bitwarden Inc. C'est un projet communautaire indépendant. Le README contient un disclaimer explicite à ce sujet.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pourquoi-lessayer".to_string(),
                            title: "Pourquoi l'essayer".to_string(),
                            content: r###"<p>La réponse courte : <strong>10 Mo de RAM vs 1 Go</strong>. Sur un homelab ou un petit VPS, ça change tout. Mais ce n'est pas que ça.</p><p>Ce qui distingue vraiment Vaultwarden, c'est la <strong>complétude fonctionnelle</strong>. Contrairement à d'autres alternatives légères qui sacrifient des features, Vaultwarden implémente quasiment tout ce que propose Bitwarden Premium, y compris des fonctionnalités qui nécessitent un abonnement payant sur l'instance officielle :</p><ul><li><strong>TOTP intégré</strong> dans le gestionnaire (code 2FA stocké et généré côté vault)</li><li><strong>Bitwarden Send</strong> — partage sécurisé de secrets avec expiration</li><li><strong>Organisations et collections</strong> — partage de coffres entre membres d'une équipe</li><li><strong>WebAuthn / passkeys</strong> — support des clés hardware (YubiKey, etc.)</li><li><strong>Rapports de sécurité</strong> — mots de passe exposés, réutilisés, faibles</li><li><strong>API REST complète</strong> — automatisation possible avec le CLI <code>bw</code></li></ul><p>Le projet est actif depuis 2018, la version 1.36.0 est récente, et la communauté est solide avec un channel Matrix et un forum Discourse. Pour un outil qui gère tes secrets, la longévité du projet compte.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Sur l'instance officielle Bitwarden, les fonctionnalités TOTP et rapports de sécurité nécessitent un abonnement Premium à 10$/an. Sur Vaultwarden auto-hébergé, tout est gratuit.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "en-pratique".to_string(),
                            title: "En pratique — setup et usage".to_string(),
                            content: r###"<p>Le déploiement est remarquablement simple. La façon la plus propre est via <strong>Docker Compose</strong> avec un reverse proxy HTTPS devant (Traefik, Caddy, ou nginx). Vaultwarden <strong>exige HTTPS</strong> pour fonctionner — les clients Bitwarden refusent les connexions non sécurisées.</p><p>Un <code>docker-compose.yml</code> minimal :</p><pre><code>services:
  vaultwarden:
    image: vaultwarden/server:latest
    container_name: vaultwarden
    restart: unless-stopped
    environment:
      DOMAIN: "https://vault.ton-domaine.com"
      SIGNUPS_ALLOWED: "false"
      ADMIN_TOKEN: "un-token-tres-secret"
    volumes:
      - ./vw-data:/data
    ports:
      - "127.0.0.1:8080:80"</code></pre><p>Quelques variables d'environnement clés :</p><ul><li><code>SIGNUPS_ALLOWED=false</code> — désactive les inscriptions publiques après avoir créé ton compte</li><li><code>ADMIN_TOKEN</code> — active le panneau d'administration sur <code>/admin</code></li><li><code>SMTP_*</code> — configuration email pour les invitations et 2FA</li><li><code>DATABASE_URL</code> — pour pointer vers PostgreSQL ou MySQL si besoin</li></ul><p>Une fois déployé, tu pointes l'extension navigateur Bitwarden vers <code>https://vault.ton-domaine.com</code> dans les paramètres du serveur. C'est tout. Import depuis LastPass, 1Password ou l'instance Bitwarden officielle disponible nativement.</p><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Pense à mettre en place des backups réguliers du volume <code>/data</code> (contient la base SQLite et les attachements). C'est tes mots de passe — une sauvegarde quotidienne vers un stockage distant est indispensable.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "limites".to_string(),
                            title: "Limites honnêtes".to_string(),
                            content: r###"<p>Vaultwarden est excellent, mais il faut être honnête sur ce qu'il implique.</p><ul><li><strong>Tu es ton propre ops.</strong> Mises à jour, backups, certificats TLS, monitoring de disponibilité — tout ça c'est toi. Si ton serveur tombe et que tu es en déplacement, tu n'as plus accès à tes mots de passe depuis les appareils non synchronisés.</li><li><strong>Projet non officiel.</strong> Vaultwarden n'est pas maintenu par Bitwarden Inc. Si l'API Bitwarden change de façon incompatible, il faut attendre que la communauté suive. Ça s'est globalement bien passé jusqu'ici, mais c'est un risque réel.</li><li><strong>Pas d'audit de sécurité indépendant.</strong> Le serveur Bitwarden officiel est audité régulièrement par des tiers. Vaultwarden ne l'a pas été de façon formelle. Pour un outil qui stocke tes secrets, c'est un point à peser.</li><li><strong>HTTPS obligatoire.</strong> Pas de test en local sans setup TLS (ou contournement via <code>DOMAIN=http://...</code> en dev uniquement). Caddy simplifie beaucoup la chose avec le TLS automatique.</li><li><strong>Fonctionnalités enterprise absentes.</strong> SSO/SAML, Directory Connector, et certaines fonctionnalités Bitwarden pour les grandes organisations ne sont pas implémentées.</li></ul><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Ne jamais exposer le panneau <code>/admin</code> sans authentification forte, et désactiver les inscriptions publiques (<code>SIGNUPS_ALLOWED=false</code>) immédiatement après la création de ton compte.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pour-qui".to_string(),
                            title: "Pour qui".to_string(),
                            content: r###"<p>Vaultwarden est taillé pour un profil précis. Tu corresponds probablement si :</p><ul><li>Tu as déjà <strong>un homelab ou un VPS</strong> qui tourne, avec Docker et un reverse proxy en place</li><li>Tu veux <strong>zéro dépendance cloud</strong> pour tes secrets — clés SSH, tokens GitHub, accès serveurs, credentials de projets clients</li><li>Tu utilises Bitwarden (ou tu veux y migrer depuis LastPass/1Password) et tu veux garder les clients officiels</li><li>Tu es à l'aise avec <strong>la responsabilité des backups</strong> et la maintenance d'une infra minimaliste</li><li>Tu veux offrir une instance partagée à ta famille ou une petite équipe sans payer d'abonnement</li></ul><p>En revanche, Vaultwarden n'est probablement pas fait pour toi si :</p><ul><li>Tu n'as pas d'infrastructure self-hosted et tu ne veux pas en gérer une</li><li>Tu as besoin d'un gestionnaire de mots de passe pour une équipe de 50+ personnes avec SSO d'entreprise</li><li>La question de l'audit de sécurité formel est non-négociable pour toi ou ton entreprise</li></ul><div class="article-callout info"><span class="callout-icon">ℹ️</span><div class="callout-content"><p>Pour une famille de 3-5 personnes sur un Raspberry Pi 4 avec 4 Go de RAM, Vaultwarden consommera moins de 30 Mo. Le reste de la RAM est pour tes autres services.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "verdict".to_string(),
                            title: "Verdict".to_string(),
                            content: r###"<p><strong>Vaultwarden est l'un des meilleurs projets self-hosting qui existent.</strong> Pas de compromis sur les fonctionnalités, une empreinte ridiculeusement faible, un déploiement Docker en 10 minutes, et une compatibilité parfaite avec l'écosystème Bitwarden. C'est le genre de projet qui fait la réputation du self-hosting : ça marche, ça reste en place, et ça ne te demande rien une fois configuré.</p><p>Personnellement, c'est le premier service que je déploie sur toute nouvelle infra. Avant le monitoring, avant le CI/CD, avant tout. Parce que les mots de passe sont le nerf de la guerre, et avoir sa propre instance Vaultwarden sur un VPS à 5€/mois avec backups quotidiens vers un bucket S3 — c'est probablement <strong>la meilleure décision de sécurité personnelle</strong> que tu puisses prendre en une après-midi.</p><p>Les limites existent — pas d'audit formel, tu es ton propre ops — mais elles sont connues et acceptables pour le profil cible. La communauté est active, le projet est mature, et 61 000 étoiles GitHub ne mentent pas.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p><strong>Recommandation :</strong> déploie-le ce week-end. Installe l'extension Bitwarden dans ton navigateur, pointe-la vers ton instance, migre depuis ton gestionnaire actuel. Dans 2 heures tu ne reviens pas en arrière.</p></div></div>"###.to_string(),
                        },
                    ],
                });

                m.insert("en".to_string(), ArticleContent {
                    tldr: r###"Vaultwarden reimplements the Bitwarden API in Rust with a tiny memory footprint. Compatible with all official Bitwarden clients, it deploys in a single Docker command on any VPS or Raspberry Pi. Full password sovereignty, zero subscription."###.to_string(),
                    sections: vec![
                        ArticleSection {
                            id: "le-probleme".to_string(),
                            title: "The Problem It Solves".to_string(),
                            content: r###"<p>Cloud password managers are convenient — until you start wondering <strong>who actually has access to your secrets</strong>. Bitwarden is a great option: open source, audited, clients available everywhere. But the official instance is hosted on their infrastructure, and self-hosting the official server is a different story.</p><p>The official Bitwarden server is a .NET stack with heavy dependencies: SQL Server (or SQLite in recent versions), multiple services running in parallel, and memory usage starting around <strong>1–2 GB of RAM</strong> at idle. On a Raspberry Pi 4 or a cheap 5€/month VPS, that's a dealbreaker.</p><p>The real problem: you want <strong>full digital sovereignty</strong> over your passwords — SSH keys, API tokens, server credentials, project secrets — without sacrificing the convenience of official clients (browser extensions, iOS/Android apps, CLI). You want real-time sync, 2FA, vault sharing, all running on your own hardware.</p><div class="article-callout info"><span class="callout-icon">ℹ️</span><div class="callout-content"><p>Bitwarden has significantly improved its official server in recent years, including SQLite support. Vaultwarden nonetheless remains the go-to option for resource-constrained environments and deployment simplicity.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "ce-que-cest".to_string(),
                            title: "What It Is".to_string(),
                            content: r###"<p><strong>Vaultwarden</strong> (formerly <em>bitwarden_rs</em>) is a complete reimplementation of the Bitwarden server API, written in <strong>Rust</strong> using the <a href="https://rocket.rs" target="_blank" rel="noopener noreferrer">Rocket</a> framework. The project is maintained by <a href="https://github.com/dani-garcia" target="_blank" rel="noopener noreferrer">dani-garcia</a> and the community, with over <strong>61,000 GitHub stars</strong> — one of the most popular self-hosting projects in the ecosystem.</p><p>This is <strong>not a fork</strong> of Bitwarden: it's a ground-up rewrite implementing the same API endpoints. Official Bitwarden clients (Chrome/Firefox extension, Android/iOS apps, desktop client, <code>bw</code> CLI) can't tell the difference from the official server. Point your client at your instance and everything just works.</p><ul><li><strong>Language:</strong> Rust — native performance, ultra-low memory footprint (~10 MB at startup)</li><li><strong>Database:</strong> SQLite by default, MySQL and PostgreSQL supported</li><li><strong>Deployment:</strong> official Docker image, static binary available</li><li><strong>License:</strong> AGPL-3.0 — free and copyleft</li><li><strong>Features:</strong> individual and organization vaults, 2FA (TOTP, WebAuthn, Duo), sends, attachments, full API</li></ul><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>Vaultwarden is not affiliated with Bitwarden Inc. It's an independent community project. The README includes an explicit disclaimer about this.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pourquoi-lessayer".to_string(),
                            title: "Why You Should Try It".to_string(),
                            content: r###"<p>Short answer: <strong>10 MB of RAM vs 1 GB</strong>. On a homelab or small VPS, that changes everything. But that's not the whole story.</p><p>What truly sets Vaultwarden apart is its <strong>functional completeness</strong>. Unlike other lightweight alternatives that sacrifice features, Vaultwarden implements nearly everything Bitwarden Premium offers — including features that require a paid subscription on the official instance:</p><ul><li><strong>Built-in TOTP</strong> inside the vault manager (2FA codes stored and generated vault-side)</li><li><strong>Bitwarden Send</strong> — secure secret sharing with expiration</li><li><strong>Organizations and collections</strong> — vault sharing across team members</li><li><strong>WebAuthn / passkeys</strong> — hardware key support (YubiKey, etc.)</li><li><strong>Security reports</strong> — exposed, reused, and weak passwords</li><li><strong>Full REST API</strong> — automation possible with the <code>bw</code> CLI</li></ul><p>The project has been active since 2018, version 1.36.0 is recent, and the community is solid with a Matrix channel and Discourse forum. For a tool managing your secrets, project longevity matters.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p>On the official Bitwarden instance, TOTP and security report features require a Premium subscription at $10/year. On a self-hosted Vaultwarden, everything is free.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "en-pratique".to_string(),
                            title: "In Practice — Setup & Usage".to_string(),
                            content: r###"<p>Deployment is remarkably straightforward. The cleanest approach is via <strong>Docker Compose</strong> with an HTTPS reverse proxy in front (Traefik, Caddy, or nginx). Vaultwarden <strong>requires HTTPS</strong> to function — Bitwarden clients refuse insecure connections.</p><p>A minimal <code>docker-compose.yml</code>:</p><pre><code>services:
  vaultwarden:
    image: vaultwarden/server:latest
    container_name: vaultwarden
    restart: unless-stopped
    environment:
      DOMAIN: "https://vault.your-domain.com"
      SIGNUPS_ALLOWED: "false"
      ADMIN_TOKEN: "a-very-secret-token"
    volumes:
      - ./vw-data:/data
    ports:
      - "127.0.0.1:8080:80"</code></pre><p>Key environment variables:</p><ul><li><code>SIGNUPS_ALLOWED=false</code> — disable public signups after creating your account</li><li><code>ADMIN_TOKEN</code> — enables the admin panel at <code>/admin</code></li><li><code>SMTP_*</code> — email configuration for invitations and 2FA</li><li><code>DATABASE_URL</code> — to point to PostgreSQL or MySQL if needed</li></ul><p>Once deployed, point the Bitwarden browser extension to <code>https://vault.your-domain.com</code> in the server settings. That's it. Native import from LastPass, 1Password, or the official Bitwarden instance is available out of the box.</p><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Set up regular backups of the <code>/data</code> volume (contains the SQLite database and attachments). These are your passwords — a daily backup to remote storage is non-negotiable.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "limites".to_string(),
                            title: "Honest Limitations".to_string(),
                            content: r###"<p>Vaultwarden is excellent, but let's be honest about what it entails.</p><ul><li><strong>You are your own ops team.</strong> Updates, backups, TLS certificates, uptime monitoring — all on you. If your server goes down while you're traveling, you lose access to passwords on unsynced devices.</li><li><strong>Unofficial project.</strong> Vaultwarden is not maintained by Bitwarden Inc. If the Bitwarden API changes in a breaking way, you wait for the community to catch up. It's gone well so far, but it's a real risk.</li><li><strong>No independent security audit.</strong> The official Bitwarden server undergoes regular third-party audits. Vaultwarden has not been formally audited. For a tool storing your secrets, this is a point worth weighing.</li><li><strong>HTTPS is mandatory.</strong> No local testing without TLS setup (or workaround via <code>DOMAIN=http://...</code> for dev only). Caddy makes this much easier with automatic TLS.</li><li><strong>Enterprise features missing.</strong> SSO/SAML, Directory Connector, and some Bitwarden features for large organizations are not implemented.</li></ul><div class="article-callout warning"><span class="callout-icon">⚠️</span><div class="callout-content"><p>Never expose the <code>/admin</code> panel without strong authentication, and disable public signups (<code>SIGNUPS_ALLOWED=false</code>) immediately after creating your account.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "pour-qui".to_string(),
                            title: "Who It's For".to_string(),
                            content: r###"<p>Vaultwarden is built for a specific profile. You're likely a match if:</p><ul><li>You already have <strong>a homelab or VPS</strong> running, with Docker and a reverse proxy in place</li><li>You want <strong>zero cloud dependency</strong> for your secrets — SSH keys, GitHub tokens, server access, client project credentials</li><li>You use Bitwarden (or want to migrate from LastPass/1Password) and want to keep using official clients</li><li>You're comfortable with <strong>backup responsibility</strong> and maintaining a minimal infrastructure</li><li>You want to offer a shared instance to your family or a small team without paying a subscription</li></ul><p>On the other hand, Vaultwarden is probably not for you if:</p><ul><li>You don't have self-hosted infrastructure and don't want to manage any</li><li>You need a password manager for a 50+ person team with enterprise SSO</li><li>Formal security auditing is non-negotiable for you or your organization</li></ul><div class="article-callout info"><span class="callout-icon">ℹ️</span><div class="callout-content"><p>For a family of 3–5 people on a Raspberry Pi 4 with 4 GB of RAM, Vaultwarden will consume less than 30 MB. The rest of the RAM is for your other services.</p></div></div>"###.to_string(),
                        },
                        ArticleSection {
                            id: "verdict".to_string(),
                            title: "Verdict".to_string(),
                            content: r###"<p><strong>Vaultwarden is one of the best self-hosting projects out there.</strong> No feature compromises, a ridiculously small footprint, Docker deployment in 10 minutes, and perfect compatibility with the Bitwarden ecosystem. It's the kind of project that defines what self-hosting can be: it works, it stays up, and it asks nothing of you once configured.</p><p>Personally, it's the first service I deploy on any new infrastructure. Before monitoring, before CI/CD, before anything else. Because passwords are the lifeblood of everything, and running your own Vaultwarden instance on a 5€/month VPS with daily backups to an S3 bucket is probably <strong>the best personal security decision</strong> you can make in an afternoon.</p><p>The limitations exist — no formal audit, you're your own ops — but they're known and acceptable for the target profile. The community is active, the project is mature, and 61,000 GitHub stars don't lie.</p><div class="article-callout tip"><span class="callout-icon">💡</span><div class="callout-content"><p><strong>Recommendation:</strong> deploy it this weekend. Install the Bitwarden extension in your browser, point it at your instance, migrate from your current manager. Two hours from now you won't be going back.</p></div></div>"###.to_string(),
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
