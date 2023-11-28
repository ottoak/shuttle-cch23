use std::net::TcpListener;
use shuttle_runtime::tokio;

use cch23_alex_o::startup::run;

pub struct TestApp {
	pub address: String,
}
async fn spawn_app() -> hyper::Result<TestApp> {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
	let port = listener.local_addr().unwrap().port();
	let address = format!("http://127.0.0.1:{}", port);

	let router = run();
	let server = axum::Server::from_tcp(listener)?.serve(router.into_make_service());

	let _ = tokio::spawn(server);

	Ok(TestApp {address})
}

#[tokio::test]
async fn day0_test_root() {
	let test_app = spawn_app().await.expect("Failed to spawn test app");

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn day0_test_err() {
	let test_app = spawn_app().await.expect("Failed to spawn test app");

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/-1/error", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_server_error());
	assert_eq!(Some(0), response.content_length());
}