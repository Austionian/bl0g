use crate::helpers;
use crate::{AppState, TEMPLATES};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{
    extract::{Query, State},
    response::Html,
};
use hyper::header;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct Pagination {
    page: Option<usize>,
}

#[derive(serde::Serialize)]
struct BlogPagination {
    current: usize,
    previous: String,
    next: String,
    pages: Vec<usize>,
    pages_len: usize,
}

/// Handler to return a page with blog list of blog
/// posts.
pub async fn blog(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    const POSTS_PER_PAGE: usize = 5;
    let page = pagination.0.page.unwrap_or(0);

    let start = page * POSTS_PER_PAGE;
    let end = start + POSTS_PER_PAGE;

    let mut context = tera::Context::new();

    if state.posts.len() < end {
        context.insert("posts", &state.posts[start..]);
    } else {
        context.insert("posts", &state.posts[start..end]);
    }

    let previous = if page == 0 { 0 } else { page - 1 };

    let next = page + 1;

    let pages: Vec<_> = (0..(state.posts.len() / POSTS_PER_PAGE) + 1).collect();

    let pagination_data = BlogPagination {
        current: page,
        previous: format!("/bl0g?page={previous}"),
        next: format!("/bl0g?page={next}"),
        pages_len: pages.len() - 1,
        pages,
    };

    context.insert("pagination", &pagination_data);

    let template = helpers::get_template(&headers, "blog");

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => ([(header::VARY, "HX-Request")], Html(s)),
        Err(e) => {
            tracing::error!("Unable to load blog: {}", e);
            (
                [(header::VARY, "HX-Request")],
                Html("<html><body>Error</body></html>".to_string()),
            )
        }
    }
}
