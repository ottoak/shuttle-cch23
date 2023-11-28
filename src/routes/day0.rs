use axum::http::StatusCode;

#[tracing::instrument]
pub async fn day0() -> StatusCode {
	tracing::error!("This is an error!");
	StatusCode::INTERNAL_SERVER_ERROR
}