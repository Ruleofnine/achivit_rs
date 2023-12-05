
# Archivist Discord Bot

WIP README

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
   If you don't have a bot token go to [here](https://discord.com/developers/docs/getting-started) and finish "Step:1 Creating an app".\
   After this you should have a discord bot, it will be in your server of choice, and a bot token to put in the `BOT_TOKEN` field.\

4. Set your desired debug guild in the `DEBUG_GUILD` field.\
   The Guild(server) you put your bot in will be your `DEBUG_GUILD`.\ 
   Right Click the server icon and `Copy Server ID`.\
   You must have Developer Mode enabled to do so.\
   Settings -> Advanced -> Developer Mode\ 

5. In the `DATABASE_URL` field, fill in your PostgreSQL username and password, and specify your PostgreSQL server's IP address and port.

6. After the `/` in the `DATABASE_URL field`, enter your desired database name. **NOTE:** that this database doesn't exist yet; it will be created during setup.

### Database Initialization (Linux/macOS)

If you are using Linux, you can automate the database initialization process using the provided `init_db.sh` script. Follow these steps:

1. Ensure that the `init_db.sh` script is in the project directory.

2. Open the terminal and navigate to the project directory.

3. Edit the init_db.sh script to set your PostgreSQL username and the desired database name.

4. Make the script executable:

```shell
    chmod +x init_db.sh
```
5. Run the script to create a blank database:
```shell
    ./init_db.sh
```

###  Database Initialization (Windows)

Running Bash scripts on Windows may require additional steps because Windows does not have Bash installed by default.\ 
    A common approach is to use [Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)\ 
    Regardless of how you approach it. Once on you have the ability to run Bash Scripts on your windows machine follow the [Linux/macOS](#database-initialization-linuxmacos) steps.\ 
    Alternatively you can swap your OS to Linux.


### Running the Bot

Finally, run the following command to build and run the bot:

```shell
    cargo run
```

This will start the bot and connect it to the configured Discord server.


