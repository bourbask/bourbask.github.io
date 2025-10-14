/**
 * Hyprland Migration - English Content
 */

export const content = {
  tldr: "Successful migration from Manjaro KDE to Hyprland on Arch Linux. 3 days of configuration for a modern, performant and fully mastered environment. Perfect multi-screens, optimized dev workflow, 40% productivity increase.",

  sections: [
    {
      id: "problem",
      title: "The Problem: KDE Frustrated Me",
      content: `# The Problem: KDE Frustrated Me

After months on Manjaro KDE, I finally snapped. Not that KDE is bad, but I felt like I was fighting against my environment rather than working with it.

## 🚨 My Major Frustrations

### Cluttered Interface
KDE is powerful but... too much. Options everywhere, menus in all directions. For someone who codes 8h/day, this visual complexity was polluting my mind.

### Capricious Multi-screens
My office/home setup with different screen configs? A nightmare. KDE regularly "forgot" my preferences and repositioned my windows randomly.

### Degraded Performance
Over time, KDE became heavier and heavier. At startup, I had 2GB RAM used... to display an empty desktop.

### Limited Control
The real problem? I didn't master my environment. KDE does a lot of "automagical" things that you can't really customize finely.

## 💡 The Revelation

Discovering Hyprland setups on r/unixporn, I had a revelation: **a minimal, modern environment entirely under my control**.

The idea of starting from a clean base and building exactly what I need immediately appealed to me.`,
    },

    {
      id: "research",
      title: "Research and Technical Decisions",
      content: `# Research and Technical Decisions

Before diving headfirst, I spent 2 weeks researching the ideal stack for my needs.

## 🎯 My Selection Criteria

- **Performance**: Lighter than KDE
- **Control**: Configuration in text files
- **Modernity**: Native Wayland, fluid animations
- **Multi-screens**: Perfect support for different configs
- **Development**: Optimized workflow for coding

## 🐧 Why Arch Linux?

Manjaro is good for beginners, but it remains a "curated" distro. I wanted to understand my system from A to Z.

**Arch Advantages:**
- Total control over installed packages
- Rolling release = always up to date
- AUR = access to entire Linux ecosystem
- Excellent documentation (Arch Wiki)
- Clean base, no bloatware

## 🪟 Why Hyprland?

After testing i3, Sway, and looking at AwesomeWM, Hyprland stood out:

**✅ Strengths:**
- Native Wayland (goodbye X11 and its multi-screen bugs)
- Modern and fluid animations
- Simple configuration in file format
- Intelligent automatic tiling
- Perfect multi-screen management
- Active and responsive community

## 🛠️ Final Technical Stack

\`\`\`bash
# Main components
- Hyprland (Wayland compositor)
- Waybar (status bar)
- Kitty (GPU-accelerated terminal)
- Rofi (application launcher)
- Dunst (notifications)

# Developer tools
- Neovim (main editor)
- Firefox Developer Edition
- Git + GitUI (Git TUI interface)
- Docker + Portainer
\`\`\`

This stack covers all my needs while remaining lightweight and consistent.`,
    },

    {
      id: "installation",
      title: "Installation and Configuration",
      content: `# Installation and Configuration

The big leap! 3 intense days to completely migrate my environment.

## 📋 Migration Plan

### Phase 1: Backup and Preparation (4h)
\`\`\`bash
# Critical data backup
mkdir ~/migration-backup
cp -r ~/.ssh ~/migration-backup/
cp -r ~/.config ~/migration-backup/
cp ~/.zshrc ~/.gitconfig ~/migration-backup/

# Export Manjaro packages
pacman -Qqe > ~/migration-backup/packages-manjaro.txt
\`\`\`

### Phase 2: Arch Base Installation (6h)
Installing Arch from scratch is educational but time-consuming. Fortunately, I took detailed notes:

\`\`\`bash
# 1. Boot Arch USB
# 2. Partitioning (EFI + Root + Home)
# 3. Base installation + kernel + bootloader
# 4. Network + user configuration
# 5. First successful boot! 🎉
\`\`\`

### Phase 3: Hyprland and Apps (4h)
\`\`\`bash
# Hyprland installation
sudo pacman -S hyprland waybar kitty rofi dunst

# Essential apps
sudo pacman -S firefox neovim git docker

# Dotfiles from my GitHub repo
git clone https://github.com/bourbask/dotfiles ~/.config
\`\`\`

## ⚙️ Multi-screen Configuration

Hyprland's real magic is multi-screen management. Here's my config:

\`\`\`bash
# ~/.config/hypr/hyprland.conf

# Office setup: laptop + 2 external screens
monitor=eDP-1,1920x1080@60,0x1080,1
monitor=HDMI-A-1,1920x1080@60,0x0,1
monitor=DP-1,1920x1080@60,1920x0,1

# Home setup: laptop + vertical screen + ultrawide
monitor=eDP-1,1920x1080@60,0x1080,1
monitor=HDMI-A-1,1920x1080@60,1920x1080,1,transform,1
monitor=HDMI-A-2,3440x1440@120,1920x0,1
\`\`\`

## 🎨 Theme and Aesthetics

One of Hyprland's big advantages: animations and modern aesthetics.

\`\`\`bash
# Fluid animations
animation=windows,1,7,default
animation=windowsOut,1,7,default,popin 80%
animation=border,1,10,default
animation=fade,1,10,default
\`\`\`

Result: an environment that breathes modernity!`,
    },

    // ... autres sections en anglais
  ],
};
