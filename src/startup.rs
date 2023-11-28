use axum::{
	routing::get,
	Router
};

use crate::routes::{home, day0};

pub fn run() -> Router {
	// build our application
	let app = Router::new()
		.route("/", get(home))
		.route("/-1/error", get(day0));

	app
}