use sqlx::postgres::Postgres;
use sqlx::Pool;

pub mod todo;

pub type Db = Pool<Postgres>;

pub async fn new_db() -> Result<Db, sqlx::Error> {
    let pool =
        Pool::<Postgres>::connect("postgres://postgres:insecurepassword@pg:5432/postgres").await?;
    Ok(pool)
}
