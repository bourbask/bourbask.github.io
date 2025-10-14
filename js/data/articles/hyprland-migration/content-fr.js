/**
 * Hyprland Migration - French Content
 */

export const content = {
  tldr: "Migration réussie de Manjaro KDE vers Hyprland sur Arch Linux. 3 jours de config pour un environnement moderne, performant et entièrement maîtrisé. Multi-écrans parfait, workflow dev optimisé, productivité en hausse de 40%.",

  sections: [
    {
      id: "problem",
      title: "Le Problème : KDE me Frustrait",
      content: `# Le Problème : KDE me Frustrait

Après des mois sur Manjaro KDE, j'ai fini par craquer. Pas que KDE soit mauvais, mais j'avais l'impression de me battre contre mon environnement plutôt que de travailler avec.

## 🚨 Mes Frustrations Majeures

### Interface Surchargée
KDE, c'est puissant mais... trop. Des options partout, des menus dans tous les sens. Pour quelqu'un qui code 8h/jour, cette complexité visuelle me polluait l'esprit.

### Multi-écrans Capricieux  
Mon setup bureau/maison avec des configs écrans différentes ? Un cauchemar. KDE "oubliait" régulièrement mes préférences, et me repositionnait mes fenêtres n'importe comment.

### Performance Dégradée
Avec le temps, KDE devenait de plus en plus lourd. Au démarrage, j'avais 2GB de RAM utilisée... pour afficher un bureau vide.

### Contrôle Limité
Le vrai problème ? Je ne maîtrisais pas mon environnement. KDE fait beaucoup de choses "automagiques" qu'on ne peut pas vraiment personnaliser finement.

## 💡 La Révélation

En découvrant les setups Hyprland sur r/unixporn, j'ai eu une révélation : **un environnement minimal, moderne et entièrement sous mon contrôle**.

L'idée de partir d'une base propre et de construire exactement ce dont j'ai besoin m'a immédiatement séduit.`,
    },

    {
      id: "research",
      title: "Recherches et Décisions Techniques",
      content: `# Recherches et Décisions Techniques

Avant de plonger tête baissée, j'ai passé 2 semaines à rechercher la stack idéale pour mes besoins.

## 🎯 Mes Critères de Choix

- **Performance** : Plus léger que KDE
- **Contrôle** : Configuration en fichiers texte
- **Modernité** : Wayland natif, animations fluides  
- **Multi-écrans** : Support parfait différentes configs
- **Développement** : Workflow optimisé pour coder

## 🐧 Pourquoi Arch Linux ?

Manjaro, c'est bien pour débuter, mais ça reste une distro "curated". Je voulais comprendre mon système de A à Z.

**Avantages Arch :**
- Contrôle total des packages installés
- Rolling release = toujours à jour
- AUR = accès à tout l'écosystème Linux
- Documentation excellente (Arch Wiki)
- Base propre, pas de bloatware

## 🪟 Pourquoi Hyprland ?

Après avoir testé i3, Sway, et regardé AwesomeWM, Hyprland s'est imposé :

**✅ Points forts :**
- Wayland natif (fini X11 et ses bugs multi-écrans)
- Animations modernes et fluides
- Configuration simple en format fichier
- Tiling automatique intelligent
- Multi-écrans géré parfaitement
- Communauté active et réactive

## 🛠️ Stack Technique Finale

\`\`\`bash
# Composants principaux
- Hyprland (Wayland compositor)
- Waybar (barre de statut)
- Kitty (terminal GPU-accelerated)
- Rofi (lanceur d'applications)
- Dunst (notifications)

# Outils développeur
- Neovim (editeur principal)  
- Firefox Developer Edition
- Git + GitUI (interface Git TUI)
- Docker + Portainer
\`\`\`

Cette stack couvre tous mes besoins tout en restant légère et cohérente.`,
    },

    {
      id: "installation",
      title: "Installation et Configuration",
      content: `# Installation et Configuration

Le grand saut ! 3 jours intenses pour migrer complètement mon environnement.

## 📋 Plan de Migration

### Phase 1 : Backup et Préparation (4h)
\`\`\`bash
# Sauvegarde données critiques
mkdir ~/migration-backup
cp -r ~/.ssh ~/migration-backup/
cp -r ~/.config ~/migration-backup/
cp ~/.zshrc ~/.gitconfig ~/migration-backup/

# Export packages Manjaro
pacman -Qqe > ~/migration-backup/packages-manjaro.txt
\`\`\`

### Phase 2 : Installation Arch Base (6h)
L'installation Arch depuis zéro, c'est formateur mais chronophage. Heureusement, j'ai pris des notes détaillées :

\`\`\`bash
# 1. Boot USB Arch
# 2. Partitionnement (EFI + Root + Home)  
# 3. Installation base + kernel + bootloader
# 4. Configuration réseau + utilisateur
# 5. Premier boot réussi ! 🎉
\`\`\`

### Phase 3 : Hyprland et Apps (4h)
\`\`\`bash
# Installation Hyprland
sudo pacman -S hyprland waybar kitty rofi dunst

# Apps essentielles
sudo pacman -S firefox neovim git docker

# Dotfiles depuis mon repo GitHub
git clone https://github.com/bourbask/dotfiles ~/.config
\`\`\`

## ⚙️ Configuration Multi-écrans

La vraie magie d'Hyprland, c'est la gestion multi-écrans. Voici ma config :

\`\`\`bash
# ~/.config/hypr/hyprland.conf

# Setup bureau : laptop + 2 écrans externes
monitor=eDP-1,1920x1080@60,0x1080,1
monitor=HDMI-A-1,1920x1080@60,0x0,1  
monitor=DP-1,1920x1080@60,1920x0,1

# Setup maison : laptop + écran vertical + ultrawide
monitor=eDP-1,1920x1080@60,0x1080,1
monitor=HDMI-A-1,1920x1080@60,1920x1080,1,transform,1
monitor=HDMI-A-2,3440x1440@120,1920x0,1
\`\`\`

## 🎨 Thème et Esthétique

Un des gros plus d'Hyprland : les animations et l'esthétique moderne.

\`\`\`bash
# Animations fluides
animation=windows,1,7,default
animation=windowsOut,1,7,default,popin 80%
animation=border,1,10,default
animation=fade,1,10,default
\`\`\`

Résultat : un environnement qui respire la modernité !`,
    },

    {
      id: "workflow",
      title: "Workflow et Productivité",
      content: `# Workflow et Productivité

Maintenant, la vraie question : est-ce que ça améliore vraiment ma productivité de dev ?

## 📊 Résultats Mesurés

Après 3 mois d'utilisation quotidienne :

- **RAM au démarrage** : 800MB vs 2GB sous KDE (-60%)
- **Temps de boot** : 12s vs 35s (-65%)
- **Changement d'apps** : Instantané vs lag visible
- **Multi-écrans** : 0 bug vs plusieurs par semaine

## ⌨️ Shortcuts Optimisés

La vraie force d'un tiling manager : tout au clavier !

\`\`\`bash
# Mes shortcuts les plus utilisés
SUPER+Return     # Terminal (Kitty)
SUPER+D          # Lanceur apps (Rofi)  
SUPER+1-9        # Switch workspace
SUPER+SHIFT+1-9  # Move window to workspace
SUPER+F          # Fullscreen toggle
SUPER+V          # Split vertical
SUPER+H          # Split horizontal
\`\`\`

## 🚀 Workflow Type de Développement

Voici comment se déroule une session de dev type :

### 1. Démarrage (2 secondes)
- \`SUPER+1\` : Workspace code avec Neovim
- \`SUPER+2\` : Workspace browser pour docs/tests
- \`SUPER+3\` : Workspace terminal pour serveurs/builds

### 2. Context Switching (instantané)
- \`SUPER+1-9\` pour sauter entre contextes
- Tiling automatique = zéro gestion manuelle de fenêtres
- Multi-écrans = vision globale permanente

### 3. Focus Mode
- \`SUPER+F\` en fullscreen sur le code
- Notifications silencieuses pendant les sessions focus
- Pas de distractions visuelles

## 📈 Impact sur ma Productivité

**Gains mesurables :**
- Moins de temps perdu à organiser les fenêtres
- Context switching plus fluide
- Moins de distractions visuelles
- Configuration reproductible = setup identique partout

**Courbe d'apprentissage :**
- Semaine 1 : Inconfortable, beaucoup de doc
- Semaine 2 : Ça commence à cliquer  
- Mois 1 : Plus productif qu'avant
- Mois 3 : Ne peux plus m'en passer !

Le retour en arrière est impossible maintenant ! 😄`,
    },

    {
      id: "conclusion",
      title: "Bilan et Recommandations",
      content: `# Bilan et Recommandations

Après 6 mois sur Hyprland, il est temps de faire le bilan de cette migration.

## ✅ Ce qui Marche Parfaitement

### Performance
- **Boot en 12s** vs 35s sous KDE
- **800MB RAM** au démarrage vs 2GB  
- **Animations fluides** même avec 3 écrans
- **Zéro lag** dans les applications

### Stabilité
- **0 crash** en 6 mois d'utilisation
- **Multi-écrans impeccable** : aucun bug de positionnement
- **Configurations persistantes** : pas de réglages perdus

### Workflow
- **Productivité dev +40%** (estimation personnelle)
- **Context switching instantané** entre projets
- **Environnement reproductible** sur toutes mes machines

## ⚠️ Points d'Attention

### Courbe d'apprentissage
- **2-3 semaines** pour être à l'aise
- **Configuration manuelle** de beaucoup d'aspects
- **Debuggage** parfois nécessaire

### Compatibilité Apps  
- Quelques **apps propriétaires** font du X11 forcé
- **Screen sharing** nécessite des ajustements
- **Gaming** demande des configs spécifiques

## 🎯 Pour Qui Je Recommande ?

### ✅ Candidats Idéaux
- **Développeurs** qui passent 6h+ devant l'écran
- **Power users** qui aiment configurer leur environnement
- **Multi-écrans** avec setups complexes
- **Performance** : machines moins puissantes

### ❌ À Éviter Si...
- Vous voulez du **plug-and-play** total
- Vous utilisez beaucoup d'**apps propriétaires**  
- Vous n'aimez pas **mettre les mains dans la config**
- Vous débutez sur **Linux**

## 🚀 Conseils pour Migrer

### 1. Testez Avant !
\`\`\`bash
# VM ou machine de test d'abord
# Ne migrez pas votre machine principale en premier
\`\`\`

### 2. Planifiez Votre Migration
- **Backup complet** avant de commencer
- **Weekend libre** pour la transition
- **Documentation** de votre workflow actuel

### 3. Ressources Utiles
- [Hyprland Wiki](https://wiki.hyprland.org/) - Documentation officielle
- [r/hyprland](https://reddit.com/r/hyprland) - Communauté active
- [Dotfiles examples](https://github.com/hyprland-community/awesome-hyprland) - Inspirations config

## 💭 Réflexions Finales

Cette migration m'a rappelé pourquoi j'aime Linux : **le contrôle total sur mon environnement**.

Hyprland n'est pas juste un gestionnaire de fenêtres, c'est une philosophie : construire exactement l'environnement dont on a besoin, sans compromis.

**Question finale :** Est-ce que je recommande cette migration à tout le monde ? Non.  
**Mais** si vous êtes développeur, que vous aimez l'efficacité et que vous n'avez pas peur de mettre les mains dans le cambouis... alors oui, foncez ! 

Vous ne le regretterez pas. 🚀

---

*Configuration complète disponible sur mon [GitHub](https://github.com/bourbask/dotfiles)*`,
    },
  ],
};
