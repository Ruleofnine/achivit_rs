use color_eyre::Result;
use sqlx::{query, PgPool};
use std::env;

/// query! macros check the query against the database at COMPILE TIME
/// So without a valid database the program will not run
/// run 'Cargo run --bin init_db' to create an inital database
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
    let connect_string = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    Ok(PgPool::connect(&connect_string).await?)
}
