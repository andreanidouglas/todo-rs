use std::time::SystemTime;

use super::Db;
use sqlx::FromRow;

pub struct TodoMac;

#[derive(Default, Debug, FromRow)]
pub struct Todo {
    id: i64,
    title: String,
    description: String,
    status: String,
    created_by: i64,
    created: i64,
}

pub struct TodoNew {
    pub title: String,
    pub description: String,
}

pub struct TodoUpdate {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String 
}

const TABLE: &str = "todos";

impl TodoMac {

    pub async fn list(db: &Db) -> Result<Vec<Todo>, sqlx::Error> {

        let query = format!("SELECT id, title, description, status, created_by, created FROM {} ORDER BY id DESC;", TABLE);

        let records: Vec<Todo> = sqlx::query_as(&query)
            .fetch_all(db)
            .await?;

        Ok(records)

    }

    pub async fn create(db: &Db, data: TodoNew) -> Result<u64, sqlx::Error> {
        let query = format!("INSERT INTO {} (title, description, status, created_by, created) VALUES ($1, $2, $3, $4, $5)", TABLE);
        let timenow: i64 = i64::try_from(
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("could not get unix datetime u64").as_millis())
            .unwrap_or(0);
        let x = sqlx::query(&query)
            .bind(data.title.to_string())
            .bind(data.description)
            .bind("OPEN")
            .bind(1)
            .bind(timenow);

        Ok(x.execute(db).await?.rows_affected())
    }

    pub async fn update(db: &Db, data: TodoUpdate) -> Result<u64, sqlx::Error> {
        let query = format!("UPDATE {} SET title = $1, description = $2, status=$3 WHERE id = $4", TABLE);
        let x = sqlx::query(&query)
            .bind(data.title.to_string())
            .bind(data.description)
            .bind(data.status)
            .bind(data.id);

        Ok(x.execute(db).await?.rows_affected())
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::new_db;


    #[tokio::test]
    async fn data_mod_todo_create() {
        let c = TodoNew {title: "Hello".to_string(), description: "This is a todo description".to_string()};

        let db = new_db().await.expect("could not create new db");

        let rec = TodoMac::create(&db, c).await;
        match rec {
            Ok(x) => assert_eq!(1, x),
            Err(e) => {
                eprintln!("could not execute todo query: {:?}", e);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn data_mod_todo_update() {
        let c = TodoUpdate { title: "Updated".to_string(), description: "This is an updated description".to_string(),
            status: "CLOSED".to_string(), id: 1000 };
        let db = new_db().await.expect("could not create new db");

        let rec = TodoMac::update(&db, c).await;
        match rec {
            Ok(x) => {
                assert_eq!(x, 1);
                let sql = "SELECT * FROM todos WHERE id = 1000";
                let record: Result<Todo, sqlx::Error> = sqlx::query_as(sql).fetch_one(&db).await;
                if let Ok(x) = record {
                    assert!(x.status == "CLOSED".to_string() && x.title == "Updated".to_string())
                }
            }
            Err(e) => {
                eprintln!("could not execute todo query: {:?}", e);
                assert!(false);
            }
        }

    }
}
