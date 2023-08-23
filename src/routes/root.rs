use crate::{AppState, TEMPLATES};
use axum::{extract::State, response::Html};
use std::{fs, sync::Arc};

// basic handler that responds with a static string
pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = tera::Context::new();
    let posts = match fs::read_dir("posts") {
        Ok(files) => files
            .into_iter()
            .filter_map(|file| file.ok())
            .map(|file| {
                file.file_name()
                    .to_str()
                    .unwrap_or("Error")
                    .to_string()
                    .split('.')
                    .next()
                    .unwrap_or("Error")
                    .to_string()
            })
            .collect::<Vec<String>>(),
        Err(e) => {
            tracing::error!("Unable to read files in posts directory, {}", e);
            Vec::new()
        }
    };
    context.insert("text", &state.text);
    context.insert("posts", &posts);
    match TEMPLATES.render("index.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
