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

mod database;
mod schema;
mod message;
mod commands;
mod markov;
mod usermap;
mod status;

use std::env;
use usermap::UserMap;
use markov::Markov;
use serenity::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let markov = Markov::new();
    let user_map: HashMap<u64, Markov> = HashMap::new();
    let token = env::var("TOKEN").expect("You must pass TOKEN into the bot's environment");
    let connection = Arc::new(Mutex::new(database::connect()));
    let mut client = Client::new(&token);
    {
        {
            let mut data = client.data.lock().unwrap();
            data.insert::<Markov>(markov);
            data.insert::<UserMap>(user_map);
        }

        let locked_data1 = client.data.clone();
        let locked_data2 = client.data.clone();
        let locked_connection1 = connection.clone();
        let locked_connection2 = connection.clone();
        
        thread::spawn(move || {
            let mut data = locked_data1.lock().unwrap();
            let mut markov = data.get_mut::<Markov>().unwrap();
            markov::parse_messages(&locked_connection1.lock().unwrap(), markov);
            println!("Finished parsing global messages.");
        });

        thread::spawn(move || {
            let mut data = locked_data2.lock().unwrap();
            let mut markov = data.get_mut::<UserMap>().unwrap();
            markov::parse_user_messages(&locked_connection2.lock().unwrap(), markov);
            println!("Finished parsing user messages.");
        });
        
        println!("Bot is running.");
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
            .command("genword", |c| c.exec(commands::markov::generate_from_word))
            .command("genuserword",
                     |c| c.exec(commands::markov::generate_word_from_user))
            .command("help", |c| c.exec(commands::main::help))
            .command("status", |c| c.exec(commands::main::status))
            .command("game", |c| c.exec(commands::main::game))
            .command("name", |c| c.exec(commands::main::name))
            .command("nick", |c| c.exec(commands::main::nick))
    });

    client.on_message(move |ctx, msg| {
        let stripped = &msg.content_safe();
        let author = &msg.author;

        if author.bot {
            return;
        }

        let message_id = msg.id.0;
        let message_content: String = String::from(msg.content_safe());
        let guild_id = msg.guild_id().unwrap().0;
        let author_id = author.id.0;
        let channel_id = msg.channel_id.0;

        database::insert_message(&connection.lock().unwrap(),
                                 message_id,
                                 message_content,
                                 guild_id,
                                 author_id,
                                 channel_id);

        let mut data = ctx.data.lock().unwrap(); 
        {
            let mut markov = data.get_mut::<Markov>().expect("Markov does not exist");
            markov.parse(&stripped);
        }

        {
            let mut usermap = data.get_mut::<UserMap>().expect("UserMap does not exist"); 
            usermap 
                .entry(author_id) 
                .or_insert(Markov::new()) 
                .parse(&stripped); 
        }
    });

    if let Err(why) = client.start() {
        println!("Error: {:?}", why);
    }
}
