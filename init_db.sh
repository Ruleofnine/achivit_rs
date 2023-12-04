
#!/bin/bash

# Set your PostgreSQL connection parameters
DB_HOST="localhost"
DB_PORT="5432"
DB_USER="username"
DB_NAME="password"  # Change to your desired name

# Create a new database
createdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME

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
echo "$SQL_COMMANDS" | psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME
