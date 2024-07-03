use crate::{
    frontmatter::{deserialize_frontmatter, FrontMatter},
    helpers, TEMPLATES,
};
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use comrak::{markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, Options, Plugins};
use hyper::header;
use reqwest::Client;

/// A handler function that will load a post, convert it to HTML, and
/// either return just the post, or an entire page containing the post depending
/// from where the post was requested.
pub async fn get_blog_post(headers: HeaderMap, Path(post_name): Path<String>) -> impl IntoResponse {
    // Create the context that will be passed to the template.
    let mut context = tera::Context::new();

    // Load the markdown file to a string.
    let md = helpers::read_post_to_string(&post_name).unwrap_or("Unable to load post.".to_string());

    // Parse the frontmatter and post body from the markdown string.
    let (frontmatter, body) = match deserialize_frontmatter::<FrontMatter>(&md) {
        Ok((frontmatter, body)) => (frontmatter, body),
        Err(e) => {
            tracing::error!(
                "Failed to deseriale the requested post -> {}: {:?}",
                &post_name,
                e
            );
            (
                FrontMatter::default(),
                "Unable to load the post.".to_string(),
            )
        }
    };

    // Update the read count of the post in a different thread so the response
    // from the worker isn't blocking request to get the post.
    if std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into()) != "local" {
        tokio::spawn(async move {
            let api_token = std::env::var("API_TOKEN").unwrap();
            let client = Client::new();
            let _ = client
                .post(format!(
                    "https://worker-rust.austin-e33.workers.dev/{}",
                    frontmatter.id
                ))
                .header("API_TOKEN", api_token.trim())
                .send()
                .await;
        });
    }

    let s = SyntectAdapter::new(Some("base16-ocean.dark"));
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&s);

    // Parse the post's markdown into an html string.
    let post_html = markdown_to_html_with_plugins(&body, &Options::default(), &plugins);

    // Add data to the template's context.
    context.insert("frontmatter", &frontmatter);
    context.insert("post_html", &post_html);

    let template = helpers::get_template(&headers, "post");

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => ([(header::VARY, "HX-Request")], Html(s)),
        Err(e) => {
            tracing::error!("Failed rendering the template: {}", e);
            (
                [(header::VARY, "HX-Request")],
                Html("<html><body>Error</body></html>".to_string()),
            )
        }
    }
}
