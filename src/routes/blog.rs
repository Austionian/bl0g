use crate::{helpers::get_template, AppState, TEMPLATES};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{
    extract::{Query, State},
    response::Html,
};
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
    context.insert("nav_links", &state.nav_links);

    let previous;

    if page == 0 {
        previous = 0;
    } else {
        previous = page - 1;
    }

    let next = page + 1;

    let pages: Vec<_> = (0..(state.posts.len() / POSTS_PER_PAGE) + 1).collect();

    let pagination_data = BlogPagination {
        current: page,
        previous: format!("/bl0g?page={}", previous),
        next: format!("/bl0g?page={}", next),
        pages_len: pages.len() - 1,
        pages,
    };

    context.insert("pagination", &pagination_data);

    let template = get_template(&headers, "blog");

    let mut headers = HeaderMap::new();
    let path = format!("/bl0g?page={page}");
    headers.insert("HX-PUSH-Url", path.parse().unwrap());

    // Return the response.
    match TEMPLATES.render(&template, &context) {
        Ok(s) => (headers, Html(s)),
        Err(_) => (headers, Html("<html><body>Error</body></html>".to_string())),
    }
}
