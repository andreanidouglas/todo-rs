use std::net::TcpListener;

use sqlx::PgPool;
use todo_rust::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("failed to connecto to postgres");

    

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
