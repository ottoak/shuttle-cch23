use axum::http::StatusCode;

#[tracing::instrument]
pub async fn home() -> StatusCode {
	tracing::info!("Home OK");
	StatusCode::OK
}