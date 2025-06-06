mod configuration;
mod frontmatter;
mod helpers;
mod job;
mod project;
mod routes;

use axum::{Router, routing::get};
use std::fs;
use std::sync::Arc;
use std::sync::LazyLock;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub use configuration::get_configuration;
pub use frontmatter::FrontMatter;
use job::{JOBS, Job};
pub use project::Project;

static TEMPLATES: LazyLock<tera::Tera> = LazyLock::new(|| {
    let mut tera = match tera::Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {e}");
            ::std::process::exit(1);
        }
    };
    // Do not escape files ending in .content (the generated html from md posts)
    tera.autoescape_on(vec![".content"]);
    tera
});

#[derive(Clone)]
pub struct AppState {
    posts: Vec<FrontMatter>,
    projects: Vec<Project>,
    jobs: &'static [Job; 3],
}

pub fn startup() -> Result<Router, String> {
    // Get the posts at startup since they'll never change for the life
    // of the program.
    let posts = match fs::read_dir("content/posts") {
        Ok(files) => {
            let mut posts = files
                .into_iter()
                .filter_map(|file| file.ok())
                .filter_map(|file| fs::read_to_string(file.path()).ok())
                .filter_map(|file| FrontMatter::from_file(file).ok())
                .filter(|frontmatter| {
                    // If developing locally, always show draft posts
                    std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "production".into())
                        == "local"
                        || !frontmatter.draft.unwrap_or(false)
                })
                .collect::<Vec<_>>();
            posts.sort_by(|a, b| b.date.cmp(&a.date));
            Ok(posts)
        }
        Err(e) => Err(format!("Unable to read files in posts directory, {e}")),
    }?;

    // Get the projects details at startup since they'll never change for the life
    // of the program.
    let projects = match fs::read_dir("content/projects") {
        Ok(files) => Ok(files
            .into_iter()
            .filter_map(|file| file.ok())
            .filter_map(|file| fs::read_to_string(file.path()).ok())
            .filter_map(|file| Project::from_file(file).ok())
            .collect::<Vec<_>>()),
        Err(e) => Err(format!("Unable to read files in projects directory, {e}")),
    }?;

    let state = AppState {
        posts,
        projects,
        jobs: &JOBS,
    };

    let blog_routes = Router::new()
        .route("/", get(routes::blog))
        .route("/{post_name}", get(routes::get_blog_post));

    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/robots.txt", ServeFile::new("assets/robots.txt"))
        .route("/", get(routes::root))
        .nest("/bl0g", blog_routes)
        .route("/pr0jects", get(routes::projects))
        .route("/game", get(routes::game))
        .route("/read_c0unt/{post_id}", get(routes::read_count))
        .route("/feed.xml", get(routes::feed))
        .fallback(routes::handle_404)
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state))
        .route("/health_check", get(routes::health_check)))
}
