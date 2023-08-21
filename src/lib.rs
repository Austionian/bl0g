mod configuration;
mod frontmatter;
mod helpers;
mod routes;

use axum::body::BoxBody;
use axum::{routing::get, Router};
use hyper::{Body, Request, Response};
use lazy_static::lazy_static;
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
pub use frontmatter::deserialize_frontmatter;

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
    text: String,
}

pub fn startup() -> Router {
    let state = AppState {
        text: "Austin!".to_string(),
    };

    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/robots.txt", ServeFile::new("assets/robots.txt"))
        .route("/", get(routes::root))
        .route("/post/:post_name", get(routes::post))
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
