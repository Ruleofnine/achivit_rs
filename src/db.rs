use color_eyre::Result;
use sqlx::{query, PgPool};
use std::env;
use log::info;

/// query! macros check the query against the database at COMPILE TIME
/// So without a valid database the program will not run
pub async fn query_with_id(pool: &PgPool, id: u64) -> Result<Option<i32>> {
    let result = query!(
        "SELECT df_id FROM df_characters WHERE discord_id = $1 ORDER BY created ASC LIMIT 1",
        id as i64
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|res| res.df_id))
}
pub async fn establish_connection() -> Result<PgPool> {
    let pg_user = env::var("PG_USER").expect("expected 'PG_USER' in '.env'");
    let pg_pass = env::var("PG_PASS").expect("expeted 'PG_PASS' in '.env'");
    let pg_ip = env::var("PG_IP").expect("expected 'PG_IP' in '.env'");
    let pg_port = env::var("PG_PORT").expect("expected 'PG_PORT' in '.env'");
    let pg_database = env::var("PG_DB_NAME").expect("expected 'PG_DB_NAME' in '.env'");
    let connect_string = format!("postgres://{pg_user}:{pg_pass}@{pg_ip}:{pg_port}/{pg_database}");
    let pool = PgPool::connect(&connect_string).await.expect("Failed to connect to DB, make sure set up '.env'");
    info!("Connected to :{connect_string}");
    Ok(pool)
}

