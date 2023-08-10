use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn robots() -> Response {
    return match std::fs::read_to_string("./public/robots.txt") {
        Ok(txt) => txt.into_response(),
        Err(_) => (StatusCode::GONE).into_response(),
    };
}
