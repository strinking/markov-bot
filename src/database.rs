use diesel::{LoadDsl, Connection};
use diesel::pg::PgConnection;
use message::Message;
use dotenv::dotenv;
use diesel::insert;
use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


pub fn insert_message(connection: &PgConnection,
                      id: u64,
                      content: String,
                      guild_id: u64,
                      author_id: u64,
                      channel_id: u64)
                      -> Message {
    use schema::message;

    let new_message = Message {
        id: id as i64,
        content: content,
        guild_id: guild_id as i64,
        author_id: author_id as i64,
        channel_id: channel_id as i64,
    };

    insert(&new_message)
        .into(message::table)
        .get_result(connection)
        .expect("Could not insert message")
}
