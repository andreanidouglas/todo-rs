use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::health_check;
use crate::routes::new_todos;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096))
            .service(health_check)
            .service(new_todos)
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
