use crate::{helpers::get_template, TEMPLATES};
use axum::{extract::Path, http::HeaderMap, response::Html};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, io};

fn extract_md(post_name: String) -> Result<String, io::Error> {
    fs::read_to_string(format!("./posts/{post_name}.md"))
}

pub async fn post(headers: HeaderMap, Path(post_name): Path<String>) -> Html<String> {
    let mut context = tera::Context::new();

    let md = extract_md(post_name).unwrap_or("Unable to load post.".to_string());
    let md = markdown_to_html(&md, &ComrakOptions::default());

    context.insert("post_html", &md);

    let template = get_template(headers, "post");

    match TEMPLATES.render(&template, &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
