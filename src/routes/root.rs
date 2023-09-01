use crate::{AppState, TEMPLATES};
use axum::{extract::State, response::Html};
use std::sync::Arc;

#[derive(serde::Serialize)]
struct Job {
    company: &'static str,
    title: &'static str,
    start_year: &'static str,
    end_year: &'static str,
    image: &'static str,
}

/// Handler to return the website's index and display
/// a certain number of posts from the app's state.
pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = tera::Context::new();
    let jobs: [Job; 3] = [
        Job {
            company: "Northwestern Mutual",
            title: "Software Engineer",
            start_year: "2022",
            end_year: "Present",
            image: "nm.webp",
        },
        Job {
            company: "Shorewood Tech",
            title: "Software Engineer (contract)",
            start_year: "2021",
            end_year: "Present",
            image: "st.webp",
        },
        Job {
            company: "Catholic Charities of St. Paul and Minneapolis",
            title: "Software Developer/ Administrator",
            start_year: "2017",
            end_year: "2022",
            image: "ccspm.webp",
        },
    ];

    const NUMBER_OF_POSTS: usize = 4;

    if state.posts.len() > NUMBER_OF_POSTS {
        context.insert("posts", &state.posts[..NUMBER_OF_POSTS]);
    } else {
        context.insert("posts", &state.posts);
    }
    context.insert("nav_links", &state.nav_links);

    context.insert("jobs", &jobs);

    match TEMPLATES.render("index.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
