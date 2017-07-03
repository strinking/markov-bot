extern crate rand;
extern crate regex;
extern crate dotenv;
extern crate typemap;
#[macro_use]
extern crate serenity;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;


mod schema;
mod message;
mod commands;
mod markov;
mod usermap;
mod database;

use std::env;
use usermap::UserMap;
use markov::Markov;
use serenity::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    let markov = Markov::new();
    let user_map: HashMap<u64, Markov> = HashMap::new();
    let token = env::var("TOKEN").expect("You must pass TOKEN into the bot's environment");
    let connection = Arc::new(Mutex::new(database::connect()));                       
    let mut client = Client::new(&token);
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<Markov>(markov);
        data.insert::<UserMap>(user_map);       
        markov::parse_messages(&connection.lock().unwrap(), data.get_mut::<Markov>().unwrap());
        markov::parse_user_messages(&connection.lock().unwrap(), data.get_mut::<UserMap>().unwrap()); 
    }

    client.with_framework(|f| {
        f.configure(|c| {
                           c.prefix("%")
                               .allow_whitespace(true)
                               .on_mention(true)
                               .ignore_bots(true)
                               .ignore_webhooks(true)
                       })
            .command("genuser", |c| c.exec(commands::markov::generate_user))
            .command("gen", |c| c.exec(commands::markov::generate))
            .command("help", |c| c.exec(commands::main::help))
            .command("status", |c| c.exec(commands::main::status))
            .command("game", |c| c.exec(commands::main::game))
            .command("name", |c| c.exec(commands::main::name))
            .command("nick", |c| c.exec(commands::main::nick))
    });

    client.on_message(move |ctx, msg| {
        let stripped = &msg.content_safe();
        let author = &msg.author;

        let message_id= msg.id.0 as i64;
        let message_content: String = String::from(msg.content_safe());
        let guild_id = msg.guild_id().unwrap().0 as i64;
        let author_id = author.id.0 as i64;
        let channel_id = msg.channel_id.0 as i64;
        
        database::insert_message(&connection.lock().unwrap(), message_id, message_content, guild_id, author_id, channel_id);

        if author.bot {
            return;
        }

        let mut data = ctx.data.lock().unwrap();
        match data.get_mut::<Markov>() {
            Some(markov) => {
                markov.parse(&stripped);
            }
            None => {
                panic!("Markov does not exist.");
            }
        }

        match data.get_mut::<UserMap>() {
            Some(user_map) => {
                let mut markov = user_map.entry(author.id.0).or_insert(Markov::new());
                markov.parse(&stripped);
            }
            None => {
                println!("UserMap does not exist.");
            }
        }
    });
 
    if let Err(why) = client.start() {
        println!("Error: {:?}", why);
    }
}
