


use actix_web::{HttpServer, App, web, Responder, get, patch, HttpResponse, middleware, post};
use data::new_db;
use data::todo::TodoMac;

use crate::data::todo::{TodoUpdate, TodoNew};

mod data;

#[get("/api/todos")]
async fn todos() -> impl Responder {
    let db = new_db().await.expect("could not connect to database");
    
    let todos = TodoMac::list(&db).await.expect("could not query database for todos");
    format!("{}", serde_json::to_string(&todos).expect("could not deserialize result"))
}

#[post("/api/todos/")]
async fn new_todos(item: web::Json<TodoNew>) -> HttpResponse {
    let db = new_db().await.expect("could not connect to database");

    let todo_new = TodoMac::create(&db, item.0).await.expect("could not insert into the database");

    return HttpResponse::Ok().json(todo_new);

    
}

#[patch("/api/todos/{id}")]
async fn update_todos(item: web::Json<TodoUpdate>, id: web::Path<i64>) -> HttpResponse {
    let db = new_db().await.expect("could not connect to database");

    let todo_update = TodoMac::update(&db, *id, item.0).await.expect("could not update item");

    HttpResponse::Ok().json(todo_update)

}

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(hello)
            .service(new_todos)
            .service(todos)
            .service(update_todos)
    
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
