use crate::{helpers::get_template, TEMPLATES};
use axum::{http::HeaderMap, response::Html};

pub async fn blog(headers: HeaderMap) -> Html<String> {
    let context = tera::Context::new();
    let template = get_template(headers, "blog");
    match TEMPLATES.render(&template, &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<h1>Error</h1>".to_string()),
    }
}
