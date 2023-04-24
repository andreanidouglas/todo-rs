use std::net::TcpListener;

#[allow(clippy::let_underscore_future)]
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = todo_rust::run(listener).expect("failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_should_return_ok() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request to /health_check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn post_todo_returns_400_if_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("\"completed\": true}", "missing item"),
        ("{\"item\": \"error test case\"}", "missing completed"),
        ("{}", "missing item and completed"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/api/todos", &address))
            .header("content-type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

#[tokio::test]
async fn post_todo_returns_200_for_valid_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "{\"item\": \"error test case\", \"completed\": true}";

    let response = client
        .post(&format!("{}/api/todos", &address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16())
}
