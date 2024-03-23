use crate::{AppState, TEMPLATES};
use axum::{extract::State, response::Html};
use std::sync::Arc;

/// Handler to return the website's index and display
/// a certain number of posts from the app's state.
pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = tera::Context::new();

    const NUMBER_OF_POSTS: usize = 3;

    if state.posts.len() > NUMBER_OF_POSTS {
        context.insert("posts", &state.posts[..NUMBER_OF_POSTS]);
    } else {
        context.insert("posts", &state.posts);
    }

    context.insert("jobs", &state.jobs);

    match TEMPLATES.render("index.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
