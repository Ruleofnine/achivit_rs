
# Archivist Discord Bot

This is  *WIP* Discord Bot that uses the libraries Serenity and Poise to interact with the Discord API.
This bot is created for the [Dragonsgrasp Discord Server](https://discord.gg/UrKUVDVCrv).


## Features

- [Lookup DF IDS]

## Getting Started

To get started with this bot, you'll need to follow these steps:

### Prerequisites

Before you begin, make sure you have the following software installed on your system:

- Rust: Install Rust from [the official website](https://www.rust-lang.org/tools/install).

- PostgreSQL: Install PostgreSQL from [the official website](https://www.postgresql.org/download/) and [set it up](https://www.prisma.io/dataguide/postgresql/setting-up-a-local-postgresql-database).

### Installation

1. Clone this repository to your local machine and navigate to the project directory.

```shell
   git clone https://github.com/ruleofnine/achivit_rs.git
   cd achivit_rs
```
2. Create a `.env` file using the provided `.env_example` as a template and open it with your editor of choice.

3. Put your bot token in the `BOT_TOKEN` field. 
   If you don't have a bot token go to [here](https://discord.com/developers/docs/getting-started) and finish "Step:1 Creating an app".
   After this you should have a discord bot, it will be in your server of choice, and a bot token to put in the `BOT_TOKEN` field.

4. Set your desired debug guild in the `DEBUG_GUILD` field.  
   The Guild(server) you put your bot in should be your `DEBUG_GUILD`.   
   Right Click the server icon and `Copy Server ID`.  
   You must have Developer Mode enabled to do so.  
   Settings -> Advanced -> Developer Mode  
   (there currently isn't any point to having a debug guild but might as well)

5. In the `DATABASE_URL` field, replace your PostgreSQL username and password, and specify your PostgreSQL server's IP address and port in their respective spots.  
   for example `DATABASE_URL="postgres://ruleofnine:p4ssw0rd@localhost:5432/archivitdb"`

6. After the `/` in the `DATABASE_URL field`, enter your desired database name. **NOTE:** that this database doesn't exist yet; it will be created during setup.

### Database Initialization 

Make sure you are in the project directory and have finshed all steps in [Installation](#installation).

To run a debug instance of the bot:  
```shell
    cargo run
```
to build a binary:
```shell
    cargo build --release
```
The generated binary file will be located at the `achivit_rs/target/release/archivit_rs`. 

The provided `build.rs` file runs <ins>automatically</ins> whenver you edit the `.env` file then build/run/check etc with cargo.
The build file executes before compilation of the main code, and it checks to see if a postgres database with the name provied in the `.env` `DATABASE_URL` path section exists, if not it creates one and creates the needed tables.
**NOTE:** [Sqlx](https://docs.rs/sqlx/latest/sqlx/) [query](https://docs.rs/sqlx/latest/sqlx/macro.query.html) macros check your database at COMPILE TIME to ensure correctness. The program will **NOT RUN** unless you have a valid database. 
