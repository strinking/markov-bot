extern crate rand;
#[macro_use]
extern crate serenity;
extern crate typemap;

mod commands;
mod markov;
mod pool;
mod usermap;

use markov::Markov;
use serenity::Client;
use std::collections::HashMap;
use std::env;
use std::process;
use usermap::UserMap;

fn main() {
    let markov = Markov::new();
    let user_map: HashMap<u64, Markov> = HashMap::new();
    let token = env::var("TOKEN").expect("You must pass TOKEN into the bot's environment");
    let mut client = Client::new(&token);
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<Markov>(markov);
        data.insert::<UserMap>(user_map);
    }

    client.with_framework(|f| {
        f.configure(|c| {
            c.prefix("-")
                .allow_whitespace(true)
                .on_mention(true)
                .ignore_bots(true)
                .ignore_webhooks(true)
        }).command("genuser", |c| c.exec(commands::markov::generate_user))
            .command("gen", |c| c.exec(commands::markov::generate))
            .command("help", |c| c.exec(commands::main::help))
    });

    client.on_message(move |ctx, msg| {
        let author = msg.author;

        if author.bot {
            return;
        }

        let mut data = ctx.data.lock().unwrap();
        match data.get_mut::<Markov>() {
            Some(markov) => {
                markov.parse(&msg.content);
            }
            None => {
                panic!("Markov does not exist.");
            }
        }

        match data.get_mut::<UserMap>() {
            Some(user_map) => {
                let mut markov = user_map.entry(author.id.0).or_insert(Markov::new());
                markov.parse(&msg.content);
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
