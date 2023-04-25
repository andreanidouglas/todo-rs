use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub completed: bool,
}

#[post("/api/todos")]
pub async fn new_todos(item: web::Json<Todo>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO todos (id, name, completed, created_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        item.name,
        item.completed,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
