use crate::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use hyper::header;
use std::sync::Arc;

/// Handler to return the rss feed.
pub async fn feed(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rss_entries = state
        .posts
        .iter()
        .take(10)
        .map(|post| post.to_string())
        .collect::<String>();

    let feed = format!(
        r#"<feed xml:lang="en">
<generator uri="https://r00ks.io/" version="0.1.0">r00ks</generator>
<link href="https://r00ks.io/feed.xml" rel="self" type="application/xml"/>
<link href="https://r00ks.io/" rel="alternate" type="text/html"/>
<id>https://r00ks.io/</id>
<title>r00ks Blog</title>
<subtitle>
Learning and building in public.
</subtitle>
<author>
<name>Austin Rooks</name>
<uri>https://github.com/Austionian</uri>
</author>
{rss_entries}
</feed>
"#,
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());

    (headers, feed)
}
