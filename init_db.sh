#!/bin/bash

# Define the path to the .env file
script_dir="$(dirname "$0")"
env_file="$script_dir/.env"

# Function to load environment variables from .env file
load_env() {
    if [ -f "$env_file" ]; then
        source "$env_file" && echo "Environment variables loaded successfully."
    else
        echo "Error: .env file not found."
        exit 1
    fi
}

# Function to parse DATABASE_URL into components
parse_database_url() {
    if [[ $DATABASE_URL =~ ^([^:]+)://([^:]+):([^@]+)@([^:]+):([^/]+)/(.+)$ ]]; then
        DB_TYPE="${BASH_REMATCH[1]}"
        DB_USER="${BASH_REMATCH[2]}"
        DB_PASSWORD="${BASH_REMATCH[3]}"
        DB_HOST="${BASH_REMATCH[4]}"
        DB_PORT="${BASH_REMATCH[5]}"
        DB_NAME="${BASH_REMATCH[6]}"
    else
        echo "Error: Failed to parse DATABASE_URL."
        exit 1
    fi
}

# Function to check if the database exists
check_database_existence() {
    if psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
        echo "Database $DB_NAME exists."
    else
        echo "Database $DB_NAME does not exist."
        echo "Creating Database $DB_NAME..."

        # Create a new database
        createdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME"

        # Define SQL statements to recreate tables
        SQL_COMMANDS=$(cat <<-END
        CREATE TABLE public.users (
            discord_name character varying(50) NOT NULL,
            discord_id bigint NOT NULL,
            created timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
            registered_by character varying(50) NOT NULL
        );
        ALTER TABLE public.users OWNER TO $DB_USER;
        CREATE UNIQUE INDEX discord_id_unique ON public.users USING btree (discord_id);
        CREATE UNIQUE INDEX discord_name_unique ON public.users USING btree (discord_name);

        CREATE TABLE public.df_characters (
            discord_id bigint NOT NULL,
            df_id integer NOT NULL,
            created timestamp with time zone NOT NULL DEFAULT now(),
            character_name character varying(32) NOT NULL,
            registered_by character varying(50) NOT NULL
        );
        ALTER TABLE public.df_characters OWNER TO $DB_USER;
        CREATE UNIQUE INDEX df_characters_df_id_key ON public.df_characters USING btree (df_id);
        ALTER TABLE public.df_characters ADD CONSTRAINT df_characters_discord_id_fkey FOREIGN KEY (discord_id) REFERENCES public.users(discord_id);
        ALTER TABLE public.df_characters ADD CONSTRAINT fk_war_lb_df_id FOREIGN KEY (df_id) REFERENCES public.df_characters(df_id) ON DELETE CASCADE;
END
)

        # Run SQL statements to recreate tables
        echo "$SQL_COMMANDS" | psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME"
    fi
}

# Main execution

# Load environment variables from .env file
load_env

# Parse DATABASE_URL
parse_database_url

# Check if the database exists and initialize if needed
check_database_existence


