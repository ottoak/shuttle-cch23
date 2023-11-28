use axum::{routing::get, Router};

use crate::routes::*;

pub fn run() -> Router {
    // build our application
    let app = Router::new()
        .route("/", get(home))
        .route("/-1/error", get(day0))
        .route("/1", get(day1));

    app
}
