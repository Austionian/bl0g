use axum::extract::Path;
use axum::response::{Html, IntoResponse};

#[derive(serde::Deserialize)]
struct D1Post {
    id: uuid::Uuid,
    read_count: u32,
}

/// Handler to the number of times a blog post has been read
pub async fn read_count(Path(post_id): Path<String>) -> impl IntoResponse {
    let response = reqwest::get(format!(
        "https://worker-rust.austin-e33.workers.dev/{}",
        post_id
    ))
    .await
    .unwrap()
    .json::<D1Post>()
    .await
    .unwrap()
    .read_count;

    Html(response.to_string())
}
