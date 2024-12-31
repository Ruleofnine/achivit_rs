
[![ci](https://img.shields.io/github/actions/workflow/status/Ruleofnine/achivit_rs/ci.yml?logo=rust&label=build&labelColor=black)](https://github.com/Ruleofnine/achivit_rs/actions/workflows/ci.yml)
[![dg](https://img.shields.io/discord/416861825888681995?label=&color=7389d8&labelColor=6a7ec2&logoColor=ffffff&logo=discord)](https://discord.gg/UrKUVDVCrv)


# Archivist Discord Bot

This is  *WIP* Discord Bot that uses the libraries Serenity and Poise to interact with the Discord API.
This bot is created for the [Dragonsgrasp Discord Server](https://discord.gg/UrKUVDVCrv).


## Features

* Lookup DF IDS
* Custom Roles Lookup
* Custom Ascendancies System
* [DragonFable Endgame Wiki](https://dragonfable-endgame.fandom.com/wiki/) search with autocomplete 
* Update checker

## Todos
- <b>Dfpedia</b>
- challenges
- update README
- fix update checker? (it should work?)
- add AQW lookup
- log to file

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

5. Populate the rest of the fields in the `.env` with their proper values
   **NOTE:** populate the `PG_DB_NAME` with the name you would like the database to be called, the build script will create the database and initialize it.  

### Building

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

The provided `build.rs` file runs <ins>automatically</ins> whenever you build/run/check etc with cargo and the `.env` file has been edited since last build.
The build file executes before compilation of the main code, and it checks to see if a postgres database with the name provied in the `.env` `PG_DB_NAME` section exists, if not it creates one and creates the needed tables.
**NOTE:** [Sqlx](https://docs.rs/sqlx/latest/sqlx/) [query](https://docs.rs/sqlx/latest/sqlx/macro.query.html) macros check your database at COMPILE TIME to ensure correctness. The program will **NOT RUN** unless you have a valid database. 
