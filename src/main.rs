#[macro_use]
extern crate serenity;
extern crate typemap;

mod commands;
mod markov;

use serenity::Client;
use markov::Markov;
use typemap::Key;
use std::env;

impl Key for Markov {
    type Value = Markov;
}

fn main() {
    let markov = Markov::new();

    let mut client = Client::new(&env::var("TOKEN").unwrap()); {
        let mut data = client.data.lock().unwrap();
        data.insert::<Markov>(markov);
    }

    client.with_framework(|f| f
        .configure(|c| c.prefix("!"))
        .command("markov", |c| c.exec(commands::markov::generate))
        .command("help", |c| c.exec(commands::main::help)));

    client.on_message(move |_ctx, msg| {
        if !msg.author.bot {
            let mut data = _ctx.data.lock().unwrap();
            let markov = data.get_mut::<Markov>().unwrap();

            markov.parse(&msg.content);
        }
    });

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
