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
    tech: Vec<&'static str>,
    description: &'static str,
    link: &'static str,
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
            tech: vec!["React", "Redux", "TypeScript", "Graphql"],
            description: "I work on the client website building new products, \
                supporting legacy apps, and contributing to interal libraries \
                to support easier development. Helped transition the org to \
                Graphql. Lead the team in testing and performance strategies \
                for our React code and subgraph.<br /><br />Was recognized as \
                the individual contributor of Q2 2023 of Northwestern Mutual’s \
                web and mobile engineers for having decreased the entire client \
                website’s LCP by ~500ms. Also decreased the SSI header's size \
                by 57% (2 MB).",
            link: "https://northwesternmutual.com",
        },
        Job {
            company: "Shorewood Tech",
            title: "Software Engineer (contract)",
            start_year: "2021",
            end_year: "Present",
            image: "st.webp",
            tech: vec!["Rust", "Actix", "TypeScript", "Qwik", "Postgres"],
            description: "I was contracted to convert a headless JS app into a \
                Qwik/ Qwik-City frontend and an Actix backend for the web and \
                native mobile clients to consume.<br /><br />The frontend scores perfectly on \
                Google's Lighthouse metrics for web and mobile. The backend test \
                coverage is over 65%.",
            link: "https://shorewood.tech",
        },
        Job {
            company: "Catholic Charities of St. Paul and Minneapolis",
            title: "Software Developer/ Administrator",
            start_year: "2017",
            end_year: "2022",
            image: "ccspm.webp",
            tech: vec!["Python", "Django", "Flask", "Postgres"],
            description: "I built and maintained internal web apps (Flask and \
                Django) for critical agency wide functions, e.g. critical \
                incidents, employee management. Also administrated and built out \
                instances of SharePoint and EHR (Credible).",
            link: "https://cctwincities.org",
        },
    ];

    const NUMBER_OF_POSTS: usize = 3;

    if state.posts.len() > NUMBER_OF_POSTS {
        context.insert("posts", &state.posts[..NUMBER_OF_POSTS]);
    } else {
        context.insert("posts", &state.posts);
    }

    context.insert("jobs", &jobs);

    match TEMPLATES.render("index.html", &context) {
        Ok(s) => Html(s),
        Err(_) => Html("<html><body>Error</body></html>".to_string()),
    }
}
