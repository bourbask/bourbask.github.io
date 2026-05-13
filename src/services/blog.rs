use crate::data::articles::{
    get_article_by_id, get_published_articles, get_unique_categories, Article,
};
use leptos::*;

#[derive(Debug, Clone)]
pub struct BlogService {
    articles: RwSignal<Vec<Article>>,
    filtered_articles: RwSignal<Vec<Article>>,
    current_category_filter: RwSignal<String>,
    current_sort: RwSignal<String>,
}

impl BlogService {
    pub fn new() -> Self {
        let articles = get_published_articles();
        let articles_signal = create_rw_signal(articles.clone());

        Self {
            articles: articles_signal,
            filtered_articles: create_rw_signal(articles),
            current_category_filter: create_rw_signal(String::new()),
            current_sort: create_rw_signal("newest".to_string()),
        }
    }

    pub fn get_articles(&self) -> Vec<Article> {
        self.filtered_articles.get()
    }

    pub fn get_article_by_id(&self, id: &str) -> Option<Article> {
        get_article_by_id(id)
    }

    pub fn get_latest_article(&self) -> Option<Article> {
        self.articles.with(|articles| articles.first().cloned())
    }

    pub fn get_total_read_time(&self) -> u32 {
        self.articles
            .with(|articles| articles.iter().map(|a| a.meta.read_time).sum())
    }

    pub fn get_categories(&self) -> Vec<String> {
        get_unique_categories()
    }

    pub fn apply_filters(&self, category: String, sort: String) {
        self.current_category_filter.set(category.clone());
        self.current_sort.set(sort.clone());

        let mut articles = self.articles.get();

        // Filter by category
        if !category.is_empty() {
            articles.retain(|article| article.meta.category == category);
        }

        // Sort articles
        match sort.as_str() {
            "oldest" => {
                articles.sort_by(|a, b| a.meta.date.cmp(&b.meta.date));
            }
            "reading-time" => {
                articles.sort_by_key(|a| a.meta.read_time);
            }
            _ => {
                articles.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
            }
        }

        self.filtered_articles.set(articles);
    }

}
