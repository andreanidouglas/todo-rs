use actix_web::{HttpServer, App, middleware, get, Responder, post, web, HttpResponse, patch, dev::Server};
use data::todo::TodoUpdate;

use crate::data::{new_db, todo::{TodoMac, TodoNew}};

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

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello World")
}


pub fn run() -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(hello)
            .service(health_check)
            .service(new_todos)
            .service(todos)
            .service(update_todos)
    
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    Ok(server)
}
