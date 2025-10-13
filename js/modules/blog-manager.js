/**
 * Blog Manager - Handles blog functionality
 */

import {
  articles,
  getPublishedArticles,
  getArticleById,
  getFeaturedArticles,
} from "../data/articles/index.js";

class BlogManager {
  constructor(i18nManager) {
    this.i18nManager = i18nManager;
    this.currentLang = i18nManager.currentLang;
    this.currentArticle = null;

    // Category to emoji mapping
    this.categoryEmojis = {
      system: "🐧",
      hardware: "⚙️",
      devops: "🔧",
      tools: "🛠️",
      project: "🚀",
      default: "📝",
    };
  }

  /**
   * Initialize blog functionality
   */
  init() {
    this.bindEvents();
    this.handleRouting();
    console.log("✅ Blog Manager initialized");
  }

  /**
   * Handle URL routing for blog
   */
  handleRouting() {
    const path = window.location.pathname;

    if (path.startsWith("/blog/")) {
      const articleId = path.split("/")[2];
      if (articleId && articleId !== "index.html") {
        this.loadArticle(articleId);
      } else {
        this.loadBlogIndex();
      }
    }
  }

  /**
   * Load blog index page
   */
  loadBlogIndex() {
    const articles = getPublishedArticles();
    this.renderBlogIndex(articles);
  }

  /**
   * Load specific article
   */
  loadArticle(articleId) {
    const article = getArticleById(articleId);
    if (!article) {
      this.show404();
      return;
    }

    this.currentArticle = article;
    this.renderArticle(article);
    this.updatePageMeta(article);
  }

  /**
   * Get emoji for category
   */
  getCategoryEmoji(category) {
    return this.categoryEmojis[category] || this.categoryEmojis.default;
  }

  /**
   * Check if image exists and return appropriate image HTML
   */
  async getImageHtml(imageSrc, altText, className = "") {
    try {
      // Try to load the image
      await this.checkImageExists(imageSrc);
      return `<img src="${imageSrc}" alt="${altText}" class="${className}" loading="lazy" />`;
    } catch {
      // Image doesn't exist, return placeholder
      const category = this.currentArticle?.meta?.category || "default";
      return `<div class="article-placeholder ${className}">
        ${this.getCategoryEmoji(category)}
      </div>`;
    }
  }

  /**
   * Check if image exists
   */
  checkImageExists(src) {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve();
      img.onerror = () => reject();
      img.src = src;
    });
  }

  /**
   * Render image or placeholder
   */
  renderImageOrPlaceholder(imageSrc, altText, category, className = "") {
    // For now, we'll assume placeholder since we don't have real images
    // In real scenario, you'd check if the image exists
    const isPlaceholder =
      imageSrc.includes("placeholder.jpg") ||
      imageSrc.includes("/images/placeholder");

    if (isPlaceholder) {
      return `<div class="article-placeholder ${className}">
        ${this.getCategoryEmoji(category)}
      </div>`;
    }

    return `<img src="${imageSrc}" alt="${altText}" class="${className}" 
            onerror="this.style.display='none'; this.nextElementSibling.style.display='flex';" loading="lazy" />
            <div class="article-placeholder ${className}" style="display: none;">
              ${this.getCategoryEmoji(category)}
            </div>`;
  }

  /**
   * Render blog index page with hero section
   */
  renderBlogIndex(articles) {
    const container = document.getElementById("blog-container");
    if (!container) return;

    const latestArticle = this.getLatestArticle();

    container.innerHTML = `
    <!-- Blog Hero Section -->
    <section class="blog-hero">
      <div class="blog-hero-container">
        <!-- Hero Content -->
        <div class="blog-hero-content">
          <div class="blog-badge">
            <span data-key="blogBadge">Technical Blog</span>
          </div>

          <h1 class="blog-hero-title">
            <span data-key="blogHeroTitle1">Sharing knowledge</span><br />
            <span class="gradient-text" data-key="blogHeroTitle2">and experiences</span>
          </h1>

          <p class="blog-hero-description" data-key="blogHeroDescription">
            Thoughts, tutorials, and insights from my development journey. 
            From system administration to development practices, discover my learnings and projects.
          </p>

          <div class="blog-hero-stats">
            <div class="blog-stat">
              <div class="blog-stat-number">${articles.length}</div>
              <div class="blog-stat-label" data-key="articlesPublished">Articles Published</div>
            </div>
            <div class="blog-stat">
              <div class="blog-stat-number">${this.getTotalReadTime(
                articles
              )}</div>
              <div class="blog-stat-label" data-key="minutesReading">Minutes of Reading</div>
            </div>
            <div class="blog-stat">
              <div class="blog-stat-number">${
                this.getUniqueCategories(articles).length
              }</div>
              <div class="blog-stat-label" data-key="categories">Categories</div>
            </div>
          </div>
        </div>

        <!-- Latest Article Card -->
        <div class="blog-hero-visual">
          ${latestArticle ? this.renderLatestArticleCard(latestArticle) : ""}
        </div>
      </div>
    </section>

    <!-- Articles List Section -->
    <section class="blog-articles-section">
      <div class="blog-articles-container">
        <div class="blog-articles-header">
          <h2 class="blog-articles-title" data-key="allArticlesTitle">All Articles</h2>
          <div class="blog-filters">
            <select class="category-filter" id="categoryFilter">
              <option value="" data-key="allCategories">All Categories</option>
              ${this.getUniqueCategories(articles)
                .map((cat) => `<option value="${cat}">${cat}</option>`)
                .join("")}
            </select>
            <select class="sort-filter" id="sortFilter">
              <option value="newest" data-key="sortNewest">Newest First</option>
              <option value="oldest" data-key="sortOldest">Oldest First</option>
              <option value="reading-time" data-key="sortReadingTime">Reading Time</option>
            </select>
          </div>
        </div>

        <div class="blog-articles-list" id="articlesList">
          ${articles
            .map((article) => this.renderArticleListItem(article))
            .join("")}
        </div>
      </div>
    </section>
    `;

    this.initializeFilters();
  }

  /**
   * Get latest published article
   */
  getLatestArticle() {
    const published = getPublishedArticles();
    return published.length > 0 ? published[0] : null;
  }

  /**
   * Get total reading time for all articles
   */
  getTotalReadTime(articles) {
    return articles.reduce(
      (total, article) => total + article.meta.readTime,
      0
    );
  }

  /**
   * Get unique categories
   */
  getUniqueCategories(articles) {
    return [...new Set(articles.map((article) => article.meta.category))];
  }

  /**
   * Render latest article card for hero
   */
  renderLatestArticleCard(article) {
    return `
    <div class="latest-article-card">
      <div class="latest-article-badge">
        <span data-key="latestArticle">Latest Article</span>
      </div>
      
      <div class="latest-article-image">
        ${this.renderImageOrPlaceholder(
          article.meta.image,
          article.meta.title[this.currentLang],
          article.meta.category
        )}
        <div class="latest-article-overlay">
          <span class="latest-article-category">${article.meta.category}</span>
        </div>
      </div>
      
      <div class="latest-article-content">
        <h3 class="latest-article-title">${
          article.meta.title[this.currentLang]
        }</h3>
        <p class="latest-article-description">${
          article.meta.description[this.currentLang]
        }</p>
        
        <div class="latest-article-meta">
          <time class="latest-article-date">${this.formatDate(
            article.meta.date
          )}</time>
          <span class="latest-article-read-time">${
            article.meta.readTime
          } min</span>
        </div>
        
        <a href="/blog/${article.meta.id}/" class="latest-article-cta">
          <span data-key="readArticle">Read Article</span>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        </a>
      </div>
    </div>
    `;
  }

  /**
   * Render article list item
   */
  renderArticleListItem(article) {
    return `
    <article class="article-list-item" data-category="${
      article.meta.category
    }" data-date="${article.meta.date}">
      <div class="article-list-image">
        ${this.renderImageOrPlaceholder(
          article.meta.image,
          article.meta.title[this.currentLang],
          article.meta.category
        )}
      </div>
      
      <div class="article-list-content">
        <div class="article-list-meta">
          <time class="article-list-date">${this.formatDate(
            article.meta.date
          )}</time>
          <span class="article-list-category">${article.meta.category}</span>
          <span class="article-list-read-time">${
            article.meta.readTime
          } min</span>
        </div>
        
        <h3 class="article-list-title">
          <a href="/blog/${article.meta.id}/">${
      article.meta.title[this.currentLang]
    }</a>
        </h3>
        
        <p class="article-list-description">${
          article.meta.description[this.currentLang]
        }</p>
        
        <div class="article-list-tags">
          ${article.meta.tags
            .slice(0, 3)
            .map((tag) => `<span class="article-list-tag">${tag}</span>`)
            .join("")}
        </div>
      </div>
    </article>
    `;
  }

  /**
   * Initialize filters functionality
   */
  initializeFilters() {
    const categoryFilter = document.getElementById("categoryFilter");
    const sortFilter = document.getElementById("sortFilter");

    if (categoryFilter) {
      categoryFilter.addEventListener("change", () => this.applyFilters());
    }

    if (sortFilter) {
      sortFilter.addEventListener("change", () => this.applyFilters());
    }
  }

  /**
   * Apply filters to articles list
   */
  applyFilters() {
    const categoryFilter = document.getElementById("categoryFilter");
    const sortFilter = document.getElementById("sortFilter");
    const articlesList = document.getElementById("articlesList");

    if (!categoryFilter || !sortFilter || !articlesList) return;

    const selectedCategory = categoryFilter.value;
    const sortBy = sortFilter.value;

    let articles = getPublishedArticles();

    // Filter by category
    if (selectedCategory) {
      articles = articles.filter(
        (article) => article.meta.category === selectedCategory
      );
    }

    // Sort articles
    switch (sortBy) {
      case "oldest":
        articles.sort((a, b) => new Date(a.meta.date) - new Date(b.meta.date));
        break;
      case "reading-time":
        articles.sort((a, b) => a.meta.readTime - b.meta.readTime);
        break;
      case "newest":
      default:
        articles.sort((a, b) => new Date(b.meta.date) - new Date(a.meta.date));
        break;
    }

    articlesList.innerHTML = articles
      .map((article) => this.renderArticleListItem(article))
      .join("");
  }

  /**
   * Render individual article
   */
  renderArticle(article) {
    const container = document.getElementById("blog-container");
    if (!container) return;

    const content = article.content[this.currentLang];

    container.innerHTML = `
    <article class="blog-article">
      <header class="article-header">
        <nav class="article-breadcrumb">
          <a href="/blog/" data-key="blogTitle">Blog</a>
          <span class="breadcrumb-separator">/</span>
          <span class="breadcrumb-current">${
            article.meta.title[this.currentLang]
          }</span>
        </nav>
        
        <div class="article-meta">
          <time class="article-date">${this.formatDate(
            article.meta.date
          )}</time>
          <span class="article-read-time">${
            article.meta.readTime
          } min read</span>
          <div class="article-tags">
            ${article.meta.tags
              .map((tag) => `<span class="tag">${tag}</span>`)
              .join("")}
          </div>
        </div>

        <h1 class="article-title">${article.meta.title[this.currentLang]}</h1>
        <p class="article-subtitle">${
          article.meta.subtitle[this.currentLang]
        }</p>
        
        <div class="article-tldr">
          <strong>TL;DR:</strong> ${content.tldr}
        </div>
        
        <div class="article-hero">
          ${this.renderImageOrPlaceholder(
            article.meta.image,
            article.meta.title[this.currentLang],
            article.meta.category,
            "article-hero-image"
          )}
        </div>
      </header>

      <div class="article-content">
        <div class="article-toc">
          <h3>Table of Contents</h3>
          <ul>
            ${content.sections
              .map(
                (section) =>
                  `<li><a href="#${section.id}">${section.title}</a></li>`
              )
              .join("")}
          </ul>
        </div>

        <div class="article-body">
          ${content.sections
            .map(
              (section) => `
            <section id="${section.id}" class="article-section">
              ${this.parseMarkdown(section.content)}
            </section>
          `
            )
            .join("")}
        </div>
      </div>

      <footer class="article-footer">
        ${this.renderArticleNavigation()}
        ${this.renderShareButtons(article)}
      </footer>
    </article>
    `;

    this.highlightCode();
    this.setupScrollSpy();
  }

  /**
   * Render article navigation
   */
  renderArticleNavigation() {
    return `
      <div class="article-navigation">
        <a href="/blog/" class="btn-back">← Retour au blog</a>
      </div>
    `;
  }

  /**
   * Render share buttons
   */
  renderShareButtons(article) {
    const url = encodeURIComponent(window.location.href);
    const title = encodeURIComponent(article.meta.title[this.currentLang]);

    return `
      <div class="share-buttons">
        <h4>Partager cet article</h4>
        <div class="share-links">
          <a href="https://twitter.com/intent/tweet?url=${url}&text=${title}" target="_blank" rel="noopener">
            Twitter
          </a>
          <a href="https://www.linkedin.com/sharing/share-offsite/?url=${url}" target="_blank" rel="noopener">
            LinkedIn
          </a>
        </div>
      </div>
    `;
  }

  /**
   * Show 404 page
   */
  show404() {
    const container = document.getElementById("blog-container");
    if (!container) return;

    container.innerHTML = `
      <div class="error-404">
        <h1>404 - Article non trouvé</h1>
        <p>Désolé, cet article n'existe pas ou a été supprimé.</p>
        <a href="/blog/" class="btn-primary">Retour au blog</a>
      </div>
    `;
  }

  /**
   * Update page meta for article
   */
  updatePageMeta(article) {
    document.title = `${
      article.meta.title[this.currentLang]
    } - Kevin Bourbasquet`;
    const metaDesc = document.querySelector('meta[name="description"]');
    if (metaDesc) {
      metaDesc.setAttribute(
        "content",
        article.meta.description[this.currentLang]
      );
    }
  }

  /**
   * Parse markdown-like content to HTML
   */
  parseMarkdown(content) {
    return content
      .replace(/^### (.*$)/gm, "<h3>$1</h3>")
      .replace(/^## (.*$)/gm, "<h2>$1</h2>")
      .replace(/^# (.*$)/gm, "<h1>$1</h1>")
      .replace(
        /```(\w+)?\n([\s\S]*?)```/g,
        '<pre><code class="language-$1">$2</code></pre>'
      )
      .replace(/`([^`]+)`/g, "<code>$1</code>")
      .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
      .replace(/^\- (.*$)/gm, "<li>$1</li>")
      .replace(/(<li>.*<\/li>)/s, "<ul>$1</ul>")
      .replace(
        /\[([^\]]+)\]\(([^)]+)\)/g,
        '<a href="$2" target="_blank">$1</a>'
      )
      .replace(/\n\n/g, "</p><p>")
      .replace(/^(.+)$/gm, "<p>$1</p>")
      .replace(/<p><h([1-6])>/g, "<h$1>")
      .replace(/<\/h([1-6])><\/p>/g, "</h$1>")
      .replace(/<p><ul>/g, "<ul>")
      .replace(/<\/ul><\/p>/g, "</ul>");
  }

  /**
   * Format date for display
   */
  formatDate(dateString) {
    const date = new Date(dateString);
    const options = { year: "numeric", month: "long", day: "numeric" };
    return date.toLocaleDateString(this.currentLang, options);
  }

  /**
   * Update language when changed
   */
  updateLanguage() {
    this.currentLang = this.i18nManager.currentLang;

    if (this.currentArticle) {
      this.renderArticle(this.currentArticle);
    } else {
      this.loadBlogIndex();
    }
  }

  /**
   * Bind events
   */
  bindEvents() {
    document.addEventListener("click", (e) => {
      const link = e.target.closest('a[href^="/blog/"]');
      if (link) {
        e.preventDefault();
        const path = new URL(link.href).pathname;
        this.navigateTo(path);
      }
    });
  }

  /**
   * Navigate to path
   */
  navigateTo(path) {
    history.pushState(null, "", path);
    this.handleRouting();
  }

  /**
   * Highlight code blocks - placeholder
   */
  highlightCode() {
    console.log("Code highlighting would go here");
  }

  /**
   * Setup scroll spy for table of contents
   */
  setupScrollSpy() {
    const sections = document.querySelectorAll(".article-section");
    const tocLinks = document.querySelectorAll(".article-toc a");

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const id = entry.target.id;
            tocLinks.forEach((link) => {
              link.classList.toggle(
                "active",
                link.getAttribute("href") === `#${id}`
              );
            });
          }
        });
      },
      { rootMargin: "-20% 0px -35% 0px" }
    );

    sections.forEach((section) => observer.observe(section));
  }
}

export default BlogManager;
