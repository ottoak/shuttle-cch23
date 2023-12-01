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
    tracing::info!("{}", format!("Day 1 inputs: {}", path));
    let calibrated = recalibrate(path);
    tracing::info!("{}", format!("Day 1 output: {}", calibrated));

    (StatusCode::OK, calibrated)
}

fn recalibrate(input: String) -> String {
    input
        .split("/")
        .map(|s| s.parse::<i32>().unwrap())
        .reduce(|a, b| a ^ b)
        .map(|x| x.pow(3).to_string())
        .unwrap()
}
