mod common;
#[tokio::test]
async fn day1_test_core() {
	let test_app = common::spawn_app().await.expect("Failed to spawn test app");

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/1/4/8", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!("1728", response.text().await.unwrap());
}

#[tokio::test]
async fn day1_test_bonus_1() {
	let test_app = common::spawn_app().await.expect("Failed to spawn test app");

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/1/10", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!("1000", response.text().await.unwrap());
}

#[tokio::test]
async fn day1_test_bonus_2() {
	let test_app = common::spawn_app().await.expect("Failed to spawn test app");

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/1/4/5/8/10", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!("27", response.text().await.unwrap());
}