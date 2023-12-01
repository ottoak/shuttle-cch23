use std::num::ParseIntError;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn router() -> Router {
    Router::new().route("/1/*path", get(day1))
}

#[tracing::instrument]
pub async fn day1(Path(path): Path<String>) -> impl IntoResponse {
    match recalibrate(path) {
        Ok(c) => {
            tracing::info!("{}", format!("Successfully calibrated: {}", c));
            (StatusCode::OK, c)
        }
        Err(e) => {
            tracing::error!("{}", format!("Could not parse path: {}", e));
            (StatusCode::BAD_REQUEST, e.to_string())
        }
    }
}

fn recalibrate(input: String) -> Result<String, ParseIntError> {
    let numbers: Result<Vec<_>, _>= input
        .split("/")
        .map(|s| s.parse::<i32>())
        .collect();

    match numbers {
        Ok(n) => {
            let output = n.into_iter()
                .reduce(|a, b| a ^ b)
                .map(|x| x.pow(3).to_string())
                .unwrap();

            Ok(output)
        }
        Err(e) => {
            Err(e)
        }
    }
}
