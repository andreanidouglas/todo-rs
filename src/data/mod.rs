use std::fs;

use sqlx::postgres::Postgres;
use sqlx::Pool;


pub mod todo;


pub type Db = Pool<Postgres>;

pub async fn new_db() -> Result<Db, sqlx::Error> {
   let pool = Pool::<Postgres>::connect("postgres://postgres:pgsql@localhost:5432/postgres").await?;
    Ok(pool)
}

pub async fn init_db(db: &Db) -> Result<(), sqlx::Error> {
    let path = "sql/00-initdb.sql";
    let sqls = fs::read_to_string(path)?; //.unwrap_or_else(|e| {
//        eprintln!("could not read sql file {}: {:?}", path, e);

    for sql in sqls.split(';') {
        sqlx::query(sql).execute(db).await?;
    }


    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn data_mod_get_connection() {

        let db = new_db().await;
        println!("error is: {:?}", &db);
        assert_eq!(true, db.is_ok());
    }

    #[tokio::test]
    async fn data_mod_init_db() {
        let db = new_db().await.expect("could not create new db");
        let initialized = init_db(&db).await;
        println!("error is: {:?}", initialized);
        assert_eq!(true, initialized.is_ok());


    }
}
