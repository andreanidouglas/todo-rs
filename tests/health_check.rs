use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use todo_rust::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber)
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

#[allow(clippy::let_underscore_future)]
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    if std::env::var("DOCKER_BUILD").is_ok() {
        configuration.database.host = std::env::var("POSTGRES_HOST")
            .expect("docker build must have $POSTGRES_HOST explicit variable");
    }

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate database");

    connection_pool
}

#[tokio::test]
async fn health_check_should_return_ok() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request to /health_check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn post_todo_returns_400_if_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("\"completed\": true}", "missing item"),
        ("{\"name\": \"error test case\"}", "missing completed"),
        (
            "{\"email\": \"error missing email\"}",
            "missing name and completed",
        ),
        (
            "{\"name\": \"error missing email\", \"completed\": true}",
            "missing email",
        ),
        ("{}", "missing item and completed"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/api/todos", &app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "{\"name\": \"ok test case\", \"completed\": true, \"email\": \"email@email.com\"}";

    let response = client
        .post(&format!("{}/api/todos", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email, completed from todos",)
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to fetch saved todo");

    assert_eq!("ok test case", saved.name);
    assert!(saved.completed);
}
