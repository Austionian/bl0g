use crate::{helpers::get_template, AppState, TEMPLATES};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::State, response::Html};
use std::sync::Arc;

/// Handler to return a page with information about
/// me.
pub async fn photos(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    let mut context = tera::Context::new();

    context.insert("nav_links", &state.nav_links);

    // Return the response.
    match TEMPLATES.render("photos.html", &context) {
        Ok(s) => (headers, Html(s)),
        Err(_) => (headers, Html("<html><body>Error</body></html>".to_string())),
    }
}
