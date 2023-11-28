use axum::http::StatusCode;

#[tracing::instrument]
pub async fn day0() -> StatusCode {
	tracing::info!("Day 0 error");
	StatusCode::INTERNAL_SERVER_ERROR
}