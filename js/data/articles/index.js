/**
 * Articles Database - Main Index
 * Dynamic import and assembly of all articles
 */

// Import article metadata
import { meta as hyprlandMeta } from "./hyprland-migration/meta.js";
import { meta as keyboardsMeta } from "./custom-keyboards/meta.js";
import { meta as ansibleMeta } from "./ansible-setup/meta.js";
import { meta as neovimMeta } from "./neovim-discovery/meta.js";
import { meta as ezprint3dMeta } from "./ezprint3d-journey/meta.js";
import { meta as printingLabMeta } from "./3d-printing-lab/meta.js";

// Import French content
import { content as hyprlandFr } from "./hyprland-migration/content-fr.js";
import { content as keyboardsFr } from "./custom-keyboards/content-fr.js";
import { content as ansibleFr } from "./ansible-setup/content-fr.js";
import { content as neovimFr } from "./neovim-discovery/content-fr.js";
import { content as ezprint3dFr } from "./ezprint3d-journey/content-fr.js";
import { content as printingLabFr } from "./3d-printing-lab/content-fr.js";

// Import English content
import { content as hyprlandEn } from "./hyprland-migration/content-en.js";
import { content as keyboardsEn } from "./custom-keyboards/content-en.js";
import { content as ansibleEn } from "./ansible-setup/content-en.js";
import { content as neovimEn } from "./neovim-discovery/content-en.js";
import { content as ezprint3dEn } from "./ezprint3d-journey/content-en.js";
import { content as printingLabEn } from "./3d-printing-lab/content-en.js";

// Assemble articles
export const articles = {
  "hyprland-migration": {
    meta: hyprlandMeta,
    content: {
      fr: hyprlandFr,
      en: hyprlandEn,
    },
  },
  "custom-keyboards": {
    meta: keyboardsMeta,
    content: {
      fr: keyboardsFr,
      en: keyboardsEn,
    },
  },
  "ansible-setup": {
    meta: ansibleMeta,
    content: {
      fr: ansibleFr,
      en: ansibleEn,
    },
  },
  "neovim-discovery": {
    meta: neovimMeta,
    content: {
      fr: neovimFr,
      en: neovimEn,
    },
  },
  "ezprint3d-journey": {
    meta: ezprint3dMeta,
    content: {
      fr: ezprint3dFr,
      en: ezprint3dEn,
    },
  },
  "3d-printing-lab": {
    meta: printingLabMeta,
    content: {
      fr: printingLabFr,
      en: printingLabEn,
    },
  },
};

// Export utility functions
export const getArticlesByCategory = (category) => {
  return Object.values(articles).filter(
    (article) =>
      article.meta.category === category && article.meta.status === "published"
  );
};

export const getFeaturedArticles = () => {
  return Object.values(articles).filter(
    (article) => article.meta.featured && article.meta.status === "published"
  );
};

export const getPublishedArticles = () => {
  return Object.values(articles)
    .filter((article) => article.meta.status === "published")
    .sort((a, b) => new Date(b.meta.date) - new Date(a.meta.date));
};

export const getArticleById = (id) => {
  return articles[id] || null;
};
