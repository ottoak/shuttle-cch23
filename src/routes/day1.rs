use axum::{
    extract::Path,
    http::StatusCode
};
use axum::response::IntoResponse;


#[tracing::instrument]
pub async fn day1(Path((num1, num2)): Path<(isize, isize)>) -> impl IntoResponse {
    tracing::info!("{}", format!("Day 1 inputs: {}, {}", num1, num2));
    let calibrated = recalibrate(num1, num2);
    tracing::info!("{}", format!("Day 1 output: {}", calibrated));

    (StatusCode::OK, calibrated.to_string())
}

fn recalibrate(num1: isize, num2: isize) -> isize {
    let xor = num1 ^ num2;

    xor.pow(3)
}
