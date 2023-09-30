use crate::helpers;
use crate::TEMPLATES;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::response::IntoResponse;

/// Handler to return a page with information about
/// me.
pub async fn about(headers: HeaderMap) -> impl IntoResponse {
    let context = tera::Context::new();

    let (headers, template) = helpers::get_headers_and_template(&headers, "about", "/ab0ut");

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => (headers, Html(s)),
        Err(_) => (headers, Html("<html><body>Error</body></html>".to_string())),
    }
}
