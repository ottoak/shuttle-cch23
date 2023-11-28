use axum::http::StatusCode;

#[tracing::instrument]
pub async fn day1() -> StatusCode {
    tracing::info!("Day 1 OK");
    StatusCode::OK
}
