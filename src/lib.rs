mod routes;
use axum::{routing::get, Router};
use lazy_static::lazy_static;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let mut tera = match tera::Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

#[derive(Clone)]
pub struct AppState {
    text: String,
}

pub fn startup() -> Router {
    let state = AppState {
        text: "Austin!".to_string(),
    };

    Router::new()
        .route("/", get(routes::root))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/robots.txt", ServeFile::new("assets/robots.txt"))
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
        .with_state(Arc::new(state))
}
