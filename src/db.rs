use color_eyre::Result;
use sqlx::{PgPool, query};
use std::env;


pub async fn query_with_id(pool: &PgPool, id: u64) -> Option<i32> {
    let result = query!(
        "SELECT df_id FROM df_characters WHERE discord_id = $1 ORDER BY created ASC LIMIT 1"
    ,id as i64)
    .fetch_one(pool)
    .await;

    match result {
        Ok(res) => Some(res.df_id),
        Err(_) => None,
    }
}
pub async fn establish_connection() -> Result<PgPool> {
    let connect_string = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    Ok(PgPool::connect(&connect_string).await?)
}
