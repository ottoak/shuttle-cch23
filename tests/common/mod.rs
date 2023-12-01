use std::net::TcpListener;
use cch23_alex_o::startup::run;

pub struct TestApp {
	pub address: String,
}
pub async fn spawn_app() -> hyper::Result<TestApp> {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
	let port = listener.local_addr().unwrap().port();
	let address = format!("http://127.0.0.1:{}", port);

	let router = run();
	let server = axum::Server::from_tcp(listener)?.serve(router.into_make_service());

	let _ = tokio::spawn(server);

	Ok(TestApp {address})
}