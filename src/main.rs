#[macro_use]
extern crate serenity;

mod markov;

use markov::Markov;
use serenity::Client;

use std::str::FromStr;
use std::sync::{Arc, Mutex};

use std::env;

fn main() {
    let mut discord_client = Client::new(&env::var("TOKEN").unwrap());
    let mut markov = Arc::new(Mutex::new(Markov::new()));

    discord_client.on_message(move |_ctx, msg| {
        let mut new_markov = markov.lock().unwrap();
        
        new_markov.parse(msg.content.as_str());
        
        if msg.content.to_lowercase().starts_with("!generate") {
            let words = msg.content.split(" ").collect::<Vec<&str>>();

            if words.len() > 1 {
                if let Err(why) = msg.channel_id.say(&new_markov.generate(FromStr::from_str(words[1]).unwrap())) {
                    println!("Error: {:?}", why);
                }
            } else {
                if let Err(why) = msg.channel_id.say(&new_markov.generate(10)) {
                    println!("Error: {:?}", why);
                }
            }
        }
    });

    if let Err(why) = discord_client.start() {
        println!("Error: {:?}", why);
    }
}
