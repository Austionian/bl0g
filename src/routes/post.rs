use crate::{frontmatter::deserialize_frontmatter, helpers::get_template, TEMPLATES};
use axum::{extract::Path, http::HeaderMap, response::Html};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, io};

fn extract_md(post_name: String) -> Result<String, io::Error> {
    fs::read_to_string(format!("./posts/{post_name}.md"))
}

#[derive(serde::Deserialize)]
pub struct FrontMatter {
    title: String,
    date: String,
    description: String,
}

pub async fn get_post(headers: HeaderMap, Path(post_name): Path<String>) -> Html<String> {
    // Create the context that will be passed to the template.
    let mut context = tera::Context::new();

    // Load the markdown file to a string.
    let md = extract_md(post_name).unwrap_or("Unable to load post.".to_string());

    // Parse the frontmatter and post body from the markdown string.
    let (frontmatter, body) = deserialize_frontmatter::<FrontMatter>(&md).unwrap_or((
        FrontMatter {
            title: "Error".to_string(),
            date: String::default(),
            description: String::default(),
        },
        "Unable to load the post.".to_string(),
    ));

    // Parse the post's markdown into an html string.
    let post_html = markdown_to_html(&body, &ComrakOptions::default());

    // Add data to the template's context.
    context.insert("title", &frontmatter.title);
    context.insert("date", &frontmatter.date);
    context.insert("post_html", &post_html);

    // Determine which template to use.
    let template = get_template(headers, "post");

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}

impl FrontMatter {
    pub fn new(title: String) -> Self {
        Self {
            title,
            date: chrono::Local::now().to_string(),
            description: String::default(),
        }
    }
}

impl ToString for FrontMatter {
    fn to_string(&self) -> String {
        format!(
            r#"---
title: {}
date: {}
description: {}
---"#,
            self.title, self.date, self.description
        )
    }
}
