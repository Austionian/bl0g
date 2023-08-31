use crate::{AppState, TEMPLATES};
use axum::{extract::State, response::Html};
use std::sync::Arc;

// basic handler that responds with a static string
pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = tera::Context::new();
    const NUMBER_OF_POSTS: usize = 4;

    if state.posts.len() > NUMBER_OF_POSTS {
        context.insert("posts", &state.posts[..NUMBER_OF_POSTS]);
    } else {
        context.insert("posts", &state.posts);
    }

    match TEMPLATES.render("index.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
