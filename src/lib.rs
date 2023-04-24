use std::net::TcpListener;

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub item: String,
    pub completed: bool,
}

#[post("/api/todos")]
async fn new_todos(_item: web::Json<Todo>) -> impl Responder {
    //let db = new_db().await.expect("could not connect to database");

    HttpResponse::Ok().finish()
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096))
            .service(health_check)
            .service(new_todos)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
