mod common;

#[tokio::test]
async fn day0_test_root() {
	let test_app = common::spawn_app().await.expect("Failed to spawn test app");

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
	let test_app = common::spawn_app().await.expect("Failed to spawn test app");

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/-1/error", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_server_error());
	assert_eq!(Some(0), response.content_length());
}