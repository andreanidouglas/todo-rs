use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub email: String,
    pub completed: bool,
}

#[tracing::instrument(
    name = "Adding new todo",
    skip(item, pool),
    fields(
        request_id = %Uuid::new_v4(),
        todo_name = %item.name,
        todo_email = %item.email,
        todo_completed = %item.completed
    )
)]
#[post("/api/todos")]
pub async fn new_todos(item: web::Json<Todo>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_todo(&pool, &item).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new todo details in the database",
    skip(item, pool),
    fields(
        request_id = %Uuid::new_v4(),
        todo_name = %item.name,
        todo_email, %item.email,
        todo_completed = %item.completed
    )
)]
pub async fn insert_todo(pool: &PgPool, item: &web::Json<Todo>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO todos (id, name, email, completed, created_at)
        VALUES ($1, $2, $3, $4, $5)
    "#,
        Uuid::new_v4(),
        item.name,
        item.email,
        item.completed,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
