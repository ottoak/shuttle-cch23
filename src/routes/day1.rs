use axum::{
    extract::{Path},
    http::StatusCode
};
use axum::response::IntoResponse;


#[tracing::instrument]
pub async fn day1(Path(path): Path<String>) -> impl IntoResponse {
    tracing::info!("{}", format!("Day 1 inputs: {}", path));
    let calibrated = recalibrate(path);
    tracing::info!("{}", format!("Day 1 output: {}", calibrated));

    (StatusCode::OK, calibrated.to_string())
}

fn recalibrate(input: String) -> isize {
    let pre_calibrated = input
        .split("/")
        .map(|s| s.parse::<isize>().unwrap())
        .reduce(|a, b| a ^ b)
        .unwrap();

    pre_calibrated.pow(3)
}

// #[tracing::instrument]
// pub async fn day1b(Path(path): Path<String>) -> impl IntoResponse {
//
//     tracing::info!("{}", format!("Day 1 inputs: {}", path));
//
//     let pre_calibrated = path
//         .split("/")
//         .map(|s| s.parse::<isize>().unwrap())
//         .reduce(|a, b| a ^ b)
//         .unwrap();
//
//     let calibrated = pre_calibrated.pow(3);
//
//     (StatusCode::OK, calibrated.to_string())
// }
