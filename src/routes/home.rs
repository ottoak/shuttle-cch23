use axum::http::StatusCode;

#[tracing::instrument]
pub async fn home() -> StatusCode {
	tracing::info!("Home route is A-OK");
	StatusCode::OK
}