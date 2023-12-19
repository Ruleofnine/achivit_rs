use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use url::Url;
pub async fn create_db(db_url: &String) -> Result<bool, sqlx::Error> {
    let mut url = Url::parse(db_url).expect("Error parsing DATABASE_URL");
    let db_path = url.path().to_owned();
    let db_name = db_path.get(1..).expect("Error parsing path");
    url.set_path("postgres");
    let pool = PgPoolOptions::new().connect(url.as_str()).await?;
    let database_exists: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS (SELECT FROM pg_database WHERE datname = '{}')",
        db_name
    ))
    .fetch_one(&pool)
    .await?;
    if !database_exists {
        println!("Database: {} Does not exist.", db_name);
        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&pool)
            .await?;
        println!("Database: {} Created Successfully.", db_name);
    } else {
        println!("Database: {} Exists.", db_name);
        return Ok(true);
    }
    // Create the target database if it doesn't exist
    Ok(database_exists)
}
async fn intitialize_db(db_url: &String) -> Result<(), sqlx::Error> {
    let url = Url::parse(&db_url).expect("Error parsing DATABASE_URL");
    let username = url.username();
    let pool = PgPool::connect(&db_url).await?;
    println!("Connected to :{}", db_url);
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
        guild_id bigint NOT NULL,
        guild_name character varying(100) NOT NULL,
        roles_path character varying(117) NOT NULL
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
    println!("Build commencing");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("No Database URl create a .env file based on the '.env_example'");
    println!(".env DATABASE_URL : {}", db_url);
    let exists = create_db(&db_url).await?;
    match exists {
        true => (),
        false => intitialize_db(&db_url).await?,
    }
    // This print staementment is what tells cargo to only rerun the build script when the .env fil is altered.
    // if commented out the build script will always run when built.
    println!("cargo:rerun-if-changed=.env");
    Ok(())
}
