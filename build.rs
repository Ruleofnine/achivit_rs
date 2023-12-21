use anyhow::Result;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
pub fn get_db_url() -> Result<String> {
    let (pg_user, pg_pass, pg_ip, pg_port, db_name) = get_env_info()?;
    Ok(format!(
        "postgres://{pg_user}:{pg_pass}@{pg_ip}:{pg_port}/{db_name}"
    ))
}
pub fn get_env_info() -> Result<(String, String, String, String, String)> {
    let pg_user = env::var("PG_USER").expect("expected 'PG_USER'");
    let pg_pass = env::var("PG_PASS").expect("expected 'PG_PASS'");
    let pg_ip = env::var("PG_IP").expect("expected 'PG_IP'");
    let pg_port = env::var("PG_PORT").expect("expected 'PG_PORT'");
    let db_name = env::var("PG_DB_NAME").expect("expected 'PG_DB_NAME'");
    Ok((pg_user, pg_pass, pg_ip, pg_port, db_name))
}
pub async fn db_needs_to_be_created() -> Result<bool> {
    let (username, pg_pass, pg_ip, pg_port, db_name) = get_env_info()?;
    let url = format!("postgres://{username}:{pg_pass}@{pg_ip}:{pg_port}/postgres");
    let pool = PgPoolOptions::new().connect(&url).await?;
    let database_exists: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS (SELECT FROM pg_database WHERE datname = '{}')",
        db_name
    ))
    .fetch_one(&pool)
    .await?;
    Ok(!database_exists)
}
pub async fn create_db() -> Result<()> {
    let (username, pg_pass, pg_ip, pg_port, db_name) = get_env_info()?;
    let url = format!("postgres://{username}:{pg_pass}@{pg_ip}:{pg_port}/postgres");
    let pool = PgPoolOptions::new().connect(&url).await?;
    println!("Database: {} Does not exist.", db_name);
    sqlx::query(&format!("CREATE DATABASE {}", db_name))
        .execute(&pool)
        .await?;
    println!("Database: {} Created Successfully.", db_name);
    Ok(())
}
pub async fn initialize_db() -> Result<()> {
    create_db().await?;
    let db_url = get_db_url()?;
    let pool = PgPool::connect(&db_url).await?;
    let username = env::var("PG_USER")?;
    println!("Connected to {}", db_url);
    println!("Initializing Database to User: {}", username);
    let sql_commands = format!(
        r#"
        CREATE TABLE public.users (
            discord_name character varying(50) NOT NULL,
            discord_id bigint NOT NULL,
            created timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
            registered_by character varying(50) NOT NULL
        );
        ALTER TABLE public.users OWNER TO {0};
        CREATE UNIQUE INDEX discord_id_unique ON public.users USING btree (discord_id);
        CREATE UNIQUE INDEX discord_name_unique ON public.users USING btree (discord_name);

        CREATE TABLE public.df_characters (
            discord_id bigint NOT NULL,
            df_id integer NOT NULL,
            created timestamp with time zone NOT NULL DEFAULT now(),
            character_name character varying(32) NOT NULL,
            registered_by character varying(50) NOT NULL
        );
        ALTER TABLE public.df_characters OWNER TO {0};
        CREATE UNIQUE INDEX df_characters_df_id_key ON public.df_characters USING btree (df_id);
        ALTER TABLE public.df_characters ADD CONSTRAINT df_characters_discord_id_fkey FOREIGN KEY (discord_id) REFERENCES public.users(discord_id);
        ALTER TABLE public.df_characters ADD CONSTRAINT fk_war_lb_df_id FOREIGN KEY (df_id) REFERENCES public.df_characters(df_id) ON DELETE CASCADE;

        CREATE TABLE public.guild_settings(
        guild_id bigint NOT NULL UNIQUE,
        guild_name character varying(100) NOT NULL,
        roles_path character varying(111) NOT NULL
        );
        ALTER TABLE public.guild_settings OWNER TO {0};
"#,
        username
    );
    let sql_statements: Vec<&str> = sql_commands.split(';').map(|s| s.trim()).collect();
    for sql_statement in sql_statements {
        if !sql_statement.is_empty() {
            sqlx::query(sql_statement).execute(&pool).await?;
        }
    }
    println!("All Tables Created Successfully");
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    if db_needs_to_be_created().await? {
        initialize_db().await.expect("failed to create db");
    }
    // This print staementment is what tells cargo to only rerun the build script when the .env fil is altered.
    // if commented out the build script will always run when built.
    println!("cargo:rerun-if-changed=.env");
    Ok(())
}
