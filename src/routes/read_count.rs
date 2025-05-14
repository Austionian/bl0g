use axum::extract::Path;
use axum::response::{Html, IntoResponse};

#[derive(serde::Deserialize)]
struct D1Post {
    read_count: u32,
}

/// Handler to the number of times a blog post has been read. Could maybe call
/// this from the client, but leaving here to potentially use as a api key
/// protected proxy in the future. Would need to update the worker to only return the number.
pub async fn read_count(Path(post_id): Path<String>) -> impl IntoResponse {
    let response = reqwest::get(format!(
        "https://worker-rust.austin-e33.workers.dev/{post_id}",
    ))
    .await
    .unwrap()
    .json::<D1Post>()
    .await
    .unwrap()
    .read_count;

    Html(response.to_string())
}
