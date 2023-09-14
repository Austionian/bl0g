use crate::{
    frontmatter::{deserialize_frontmatter, FrontMatter},
    helpers::get_headers_and_template,
    TEMPLATES,
};
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, io};

pub fn read_post_to_string(post_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("data/posts/{post_name}.md"))
}

/// A handler function that will load a post, convert it to HTML, and
/// either return just the post, or an entire page containing the post depending
/// from where the post was requested.
pub async fn get_blog_post(headers: HeaderMap, Path(post_name): Path<String>) -> impl IntoResponse {
    // Create the context that will be passed to the template.
    let mut context = tera::Context::new();

    // Load the markdown file to a string.
    let md = read_post_to_string(&post_name).unwrap_or("Unable to load post.".to_string());

    // Parse the frontmatter and post body from the markdown string.
    let (frontmatter, body) = deserialize_frontmatter::<FrontMatter>(&md).unwrap_or((
        FrontMatter::default(),
        "Unable to load the post.".to_string(),
    ));

    // Parse the post's markdown into an html string.
    let post_html = markdown_to_html(&body, &ComrakOptions::default());

    // Add data to the template's context.
    context.insert("frontmatter", &frontmatter);
    context.insert("post_html", &post_html);

    // Determine which template to use.
    let (headers, template) = get_headers_and_template(&headers, "post");

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => (headers, Html(s)),
        Err(_) => (headers, Html("<html><body>Error</body></html>".to_string())),
    }
}
