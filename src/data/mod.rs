
use sqlx::postgres::Postgres;
use sqlx::Pool;


pub mod todo;


pub type Db = Pool<Postgres>;

pub async fn new_db() -> Result<Db, sqlx::Error> {
   let pool = Pool::<Postgres>::connect("postgres://postgres:insecure_password@pg:5432/postgres").await?;
    Ok(pool)
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
}
