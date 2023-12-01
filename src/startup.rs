use axum::{routing::get, Router};

use crate::routes::*;

pub fn run() -> Router {
    // build our application
    Router::new()
        .route("/", get(home))
        .route("/-1/error", get(day0))
        //.route("/1/:num1/:num2", get(day1))
        .route("/1/*nums", get(day1))
}
