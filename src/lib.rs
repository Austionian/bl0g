mod configuration;
mod frontmatter;
mod helpers;
mod project;
mod routes;

use axum::body::BoxBody;
use axum::{routing::get, Router};
use hyper::{Body, Request, Response};
use lazy_static::lazy_static;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::field::display;
use tracing::Span;

pub use configuration::get_configuration;
pub use frontmatter::FrontMatter;
pub use project::Project;

lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let mut tera = match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

#[derive(Clone)]
pub struct AppState {
    posts: Vec<FrontMatter>,
    projects: Vec<Project>,
}

pub fn startup() -> Router {
    // Get the posts at startup since they'll never change for the life
    // of the program.
    let posts = match fs::read_dir("data/posts") {
        Ok(files) => {
            let mut posts = files
                .into_iter()
                .filter_map(|file| file.ok())
                .filter_map(|file| fs::read_to_string(file.path()).ok())
                .filter_map(|file| FrontMatter::from_file(file).ok())
                .collect::<Vec<_>>();
            posts.sort_by(|a, b| b.date.cmp(&a.date));
            posts
        }
        Err(e) => {
            println!("Unable to read files in posts directory, {}", e);
            Vec::new()
        }
    };

    // Get the projects details at startup since they'll never change for the life
    // of the program.
    let projects = match fs::read_dir("data/projects") {
        Ok(files) => files
            .into_iter()
            .filter_map(|file| file.ok())
            .filter_map(|file| fs::read_to_string(file.path()).ok())
            .filter_map(|file| Project::from_file(file).ok())
            .collect::<Vec<_>>(),
        Err(e) => {
            println!("Unable to read files in projects directory, {}", e);
            Vec::new()
        }
    };

    let state = AppState { posts, projects };

    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/robots.txt", ServeFile::new("assets/robots.txt"))
        .route("/", get(routes::root))
        .route("/bl0g", get(routes::blog))
        .route("/bl0g/:post_name", get(routes::get_blog_post))
        .route("/ab0ut", get(routes::about))
        .route("/pr0jects", get(routes::projects))
        .fallback(routes::handle_404)
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
                    tracing::info!("http request");
                    tracing::info_span!("http-request")
                })
                .on_response(
                    |response: &Response<BoxBody>, _latency: Duration, span: &Span| {
                        span.record("http.status_code", &display(response.status()));
                        tracing::info!("http response");
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("http error, {}", error);
                    },
                ),
        )
        .with_state(Arc::new(state))
        .route("/health_check", get(routes::health_check))
}
