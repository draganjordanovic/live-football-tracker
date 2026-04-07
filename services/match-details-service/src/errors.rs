use axum::http::StatusCode;

pub fn internal_error<E: std::fmt::Display>(error: E) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", error),
    )
}