use crate::{AppState, TEMPLATES};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::response::IntoResponse;
use std::sync::Arc;

/// Handler to return a page with information about
/// me.
pub async fn projects(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    let mut context = tera::Context::new();

    context.insert("projects", &state.projects);

    // Return the response.
    match TEMPLATES.render("projects.html", &context) {
        Ok(s) => (headers, Html(s)),
        Err(_) => (headers, Html("<html><body>Error</body></html>".to_string())),
    }
}
