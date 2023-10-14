use crate::helpers;
use crate::TEMPLATES;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::response::IntoResponse;

/// Handler to return a page with information about
/// me.
pub async fn about(headers: HeaderMap) -> impl IntoResponse {
    let context = tera::Context::new();

    let template = helpers::get_template(&headers, "about");

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
