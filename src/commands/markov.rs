use markov::Markov;
use serenity::model::{Message, UserId};
use serenity::utils::MessageBuilder;
use std::collections::HashMap;
use ::usermap::UserMap;

fn output_markov(markov: &Markov, message: &Message, length: u32) {
    const ERROR_MESSAGE: &str = "(Haven't collected enough data yet)";

    let generated = match markov.generate(length) {
        Some(x) => x.as_str(),
        None => ERROR_MESSAGE,
    };
    let _ = message.channel_id.say(generated);
}

command!(generate(ctx, message, args) {
    let mut data = ctx.data.lock().unwrap();
    let markov = data.get_mut::<Markov>().unwrap();

    match args.get(0) {
        Some(arg) => {
            match arg.parse::<u32>() {
                Ok(len) => {
                    output_markov(&markov, &message, len);
                },
                Err(_) => {
                    // Ignore argument errors
                }
            }
        },
        None => {
            output_markov(&markov, &message, 20);
        }
    }
});

command!(generate_user(ctx, message, args) {
    let mut data =  ctx.data.lock().unwrap();
    let mut markov = data.get_mut::<UserMap>().unwrap();
    let author = &message.author.name;
    match args.get(0) {
        Some(arg) => {
            if arg.starts_with("<") {
                if arg.ends_with(">") {
                    if !UserId::from_str(arg).unwrap().get().unwrap().bot {
                        let mut user = UserId::from_str(arg).unwrap().get();
                        let mut name = user.unwrap().name;
                        let mut markov = markov.entry(name).or_insert_with(Markov::new);
                        let generated_string = markov.generate(100);

                        let msg = MessageBuilder::new()
                            .push(UserId::from_str(arg).unwrap().get().unwrap().name)
                            .push(": ")
                            .push(generated_string)
                            .build();

                        let _ = message.channel_id.say(&msg);
                    } else {
                        let _ = message.channel_id.say("Why, just why?");
                    }
                }
            } else {
                let _ = message.channel_id.say("User does not exist or is not in this server");
            }
        }

        None => {
            let mut markov = markov.entry(String::from(author.as_str())).or_insert(Markov::new());
            let generated_string = markov.generate(100);
            let _ = message.channel_id.say(&generated_string);
        }
    }
});
