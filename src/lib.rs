mod routes;
use axum::{
    routing::{any, get},
    Router,
};
use lazy_static::lazy_static;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

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
struct AppState {}

pub fn startup() -> Router {
    let state = AppState {};

    Router::new()
        .route("/", get(routes::root))
        .route("/robots.txt", any(routes::robots))
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
        .with_state(state)
}
