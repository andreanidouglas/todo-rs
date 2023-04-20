use std::time::SystemTime;

use super::Db;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub struct TodoMac;

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Todo {
    id: i64,
    title: String,
    description: String,
    status: String,
    created_by: i64,
    created: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TodoNew {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoUpdate {
    pub title: String,
    pub description: String,
    pub status: String,
}

const TABLE: &str = "todos";

impl TodoMac {
    pub async fn list(db: &Db) -> Result<Vec<Todo>, sqlx::Error> {
        let query = format!(
            "SELECT id, title, description, status, created_by, created FROM {} ORDER BY id DESC;",
            TABLE
        );

        let records: Vec<Todo> = sqlx::query_as(&query).fetch_all(db).await?;

        Ok(records)
    }

    pub async fn create(db: &Db, data: TodoNew) -> Result<u64, sqlx::Error> {
        let query = format!("INSERT INTO {} (title, description, status, created_by, created) VALUES ($1, $2, $3, $4, $5)", TABLE);
        let timenow: i64 = i64::try_from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("could not get unix datetime u64")
                .as_millis(),
        )
        .unwrap_or(0);
        let x = sqlx::query(&query)
            .bind(data.title.to_string())
            .bind(data.description)
            .bind("OPEN")
            .bind(1)
            .bind(timenow);

        Ok(x.execute(db).await?.rows_affected())
    }

    pub async fn update(db: &Db, id: i64, data: TodoUpdate) -> Result<u64, sqlx::Error> {
        let query = format!(
            "UPDATE {} SET title = $1, description = $2, status=$3 WHERE id = $4",
            TABLE
        );
        let x = sqlx::query(&query)
            .bind(data.title.to_string())
            .bind(data.description)
            .bind(data.status)
            .bind(id);

        Ok(x.execute(db).await?.rows_affected())
    }
}
