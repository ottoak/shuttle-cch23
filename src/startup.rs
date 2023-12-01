use axum::Router;

use crate::routes;

pub fn run() -> Router {
    // build our application
    Router::new().nest("/", routes::router())
}
