use crate::{AppState, TEMPLATES};
use axum::{extract::State, response::Html};
use std::sync::Arc;

/// Handler to return a page with blog list of blog
/// posts.
pub async fn blog(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("posts", &state.posts);
    context.insert("nav_links", &state.nav_links);
    match TEMPLATES.render("blog.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
