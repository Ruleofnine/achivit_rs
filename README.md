
# Archivist Discord Bot

WIP README

## Features

- [Lookup DF IDS]

## Getting Started

To get started with this bot, you'll need to follow these steps:

### Prerequisites

Before you begin, make sure you have the following software installed on your system:

- Rust: Install Rust from [the official website](https://www.rust-lang.org/tools/install).

- PostgreSQL: Install PostgreSQL from [the official website](https://www.postgresql.org/download/).

### Installation

1. Clone this repository to your local machine and navigate to the project directory.

```shell
   git clone https://github.com/ruleofnine/achivit_rs.git
   cd achivit_rs
```
2. Create a .env file using the provided .env_example as a template.

3. Put your bot token in the `BOT_TOKEN` field.

4. Set your desired debug guild in the `DEBUG_GUILD` field.

5. In the `DATABASE_URL` field, fill in your PostgreSQL username and password, and specify your PostgreSQL server's IP address and port.

6. After the `/` in the `DATABASE_URL field`, enter your desired database name. **NOTE:** that this database doesn't exist yet; it will be created during setup.

### Database Initialization (Linux)

If you are using Linux, you can automate the database initialization process using the provided init_db.sh script. Follow these steps:

1. Ensure that the `init_db.sh` script is in the project directory.

2.  Open the terminal and navigate to the project directory.

3. Edit the init_db.sh script to set your PostgreSQL username and the desired database name.

4. Make the script executable:

```shell

chmod +x init_db.sh
```
5. Run the script to create a blank database:
```shell

    ./init_db.sh
```
### Running the Bot
    Finally, run the following command to build and run the bot:
```shell
    cargo run
```
This will start the bot and connect it to the configured Discord server. You're now ready to use your Archivist Discord Bot!

Note: If you are using Windows or macOS, you can follow a similar process but may need to adapt the instructions for your specific environment.
