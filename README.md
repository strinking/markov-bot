## markov-bot
A Discord bot that takes post from all of the servers its in and stores them in a database to generate markov chains based on the markov principle. This bot is a combination of both a database of messages and a markov chain generator.

### Requirements
- Cargo
- Postgres/MySQL/SQLite
- Diesel-cli
### Running the bot
- Create a database and put the hostname/address in `.env` in `markov-bot/src`
- Install diesel-cli by running `cargo install diesel-cli`
- Run `diesel migration run` to create the table
- Check to make sure that the database exists
- Run the bot by specifying `TOKEN` as an environment variable example: `TOKEN=1234567890 cargo run`
