use color_eyre::{Result,eyre::eyre};
use sqlx::{PgPool, Row};
use std::env;


pub async fn query_with_id(pool: &PgPool, id: u64) -> Option<i32> {
    let result = sqlx::query(
        "SELECT df_id FROM df_characters WHERE discord_id = $1 ORDER BY created ASC LIMIT 1",
    )
    .bind(id as i64)
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(row)) => {
            let num: i32 = row.get("df_id");
            Some(num)
        }
        Ok(None) => None,
        Err(_) => None,
    }
}
pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let connect_string = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    Ok(PgPool::connect(&connect_string).await?)
}
