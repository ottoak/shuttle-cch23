use axum::{http::StatusCode, routing::get, Router};

pub fn router() -> Router {
	Router::new()
		.route("/", get(home))
		.route("/-1/error", get(day0))
}
#[tracing::instrument]
pub async fn home() -> StatusCode {
	tracing::info!("Home route is A-OK");
	StatusCode::OK
}
#[tracing::instrument]
pub async fn day0() -> StatusCode {
	tracing::error!("This is an error!");
	StatusCode::INTERNAL_SERVER_ERROR
}