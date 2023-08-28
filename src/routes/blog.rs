use crate::{AppState, TEMPLATES};
use axum::{extract::State, response::Html};
use std::sync::Arc;

// basic handler that responds with a static string
pub async fn blog(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("posts", &state.posts);
    match TEMPLATES.render("blog.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
