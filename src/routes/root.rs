use crate::TEMPLATES;
use axum::response::Html;

// basic handler that responds with a static string
pub async fn root() -> Html<String> {
    let context = tera::Context::new();
    match TEMPLATES.render("index.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
