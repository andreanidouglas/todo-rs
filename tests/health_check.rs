#[allow(clippy::let_underscore_future)]
fn spawn_app() {
    let server = todo_rust::run().expect("failed to bind address");

    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_should_return_ok() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("failed to execute request to /health_check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
