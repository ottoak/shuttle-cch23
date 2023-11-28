use cch23_alex_o::startup::run;
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {

    let router = run();

    Ok(router.into())
}
