use color_eyre::Result;
use sqlx::PgPool;
use std::env;

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let db_name = env::var("DB").expect("missing DB");
    let user = env::var("DB_USER").expect("missing DB_USER");
    let password = env::var("DB_PASSWORD").expect("missing DB_PASSWORD");
    let ip = env::var("HOST").expect("missing DB_HOST");
    let port = env::var("PORT").expect("missing PORT");
    let connect_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, ip, port, db_name
    );
    Ok(PgPool::connect(&connect_string).await?)
}
