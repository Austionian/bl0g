use crate::TEMPLATES;
use axum::response::Html;

pub async fn game() -> Html<String> {
    let context = tera::Context::new();

    match TEMPLATES.render("game.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
