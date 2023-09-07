use crate::TEMPLATES;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::response::IntoResponse;

/// Handler to return a page with information about
/// me.
pub async fn about(headers: HeaderMap) -> impl IntoResponse {
    let context = tera::Context::new();

    // Return the response.
    match TEMPLATES.render("about.html", &context) {
        Ok(s) => (headers, Html(s)),
        Err(_) => (headers, Html("<html><body>Error</body></html>".to_string())),
    }
}
