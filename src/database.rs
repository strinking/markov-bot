use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use message::Message;
use diesel::insert;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


pub fn insert_message(connection: &PgConnection,
                      id: i64,
                      content: String,
                      guild_id: i64,
                      author_id: i64,
                      channel_id: i64)
                      -> Message {
    use schema::message;

    let new_message = Message {
        id: id,
        content: content,
        guild_id: guild_id,
        author_id: author_id,
        channel_id: channel_id,
    };

    insert(&new_message)
        .into(message::table)
        .get_result(connection)
        .expect("Could not insert message")
}
