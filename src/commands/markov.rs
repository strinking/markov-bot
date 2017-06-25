use markov::Markov;
use usermap::UserMap;
use serenity::utils::MessageBuilder;
use serenity::model::{Message, UserId};

const DEFAULT_GENERATION_LENGTH: u32 = 20;
const ERROR_MESSAGE: &str = "(Haven't collected enough data yet)";

fn output_markov(markov: &Markov, message: &Message, length: u32, start: Option<&String>) {
    match markov.generate(length, start) {
        Some(x) => {
            let msg = MessageBuilder::new()
                .push(x)
                .build();
            let _ = message.channel_id.say(msg.as_str());
        }
        None => {
            let _ = message.channel_id.say(ERROR_MESSAGE);
        }
    }
}

command!(generate(ctx, message, args) {
    let mut data = ctx.data.lock().unwrap();
    let markov = data.get_mut::<Markov>().unwrap();
    let mut length: u32 = 0;
    
    match args.get(0) {
        Some(a) => {
            if let Ok(len) = a.parse::<u32>() {
                length += len;
            }
        },
        None => {
            length += DEFAULT_GENERATION_LENGTH;
        }
    }

    match args.get(1) {
        Some(arg) => {
            output_markov(&markov, &message, length, Some(arg));
        }

        None => {
            output_markov(&markov, &message, length, None);
        }
    }
});

fn get_uid(name: &str) -> Option<u64> {
    if name.starts_with("<") && name.ends_with(">") {
        name.parse::<UserId>().ok().map_or(None, |x| Some(x.0))
    } else {
        None
    }
}

command!(generate_user(ctx, message, args) {
    let mut data =  ctx.data.lock().unwrap();
    let mut usermap = data.get_mut::<UserMap>().unwrap();

    let length = args.get(1)
        .map_or(DEFAULT_GENERATION_LENGTH,
                |x| x.parse::<u32>()
                    .ok()
                    .unwrap_or(DEFAULT_GENERATION_LENGTH));

    if let Some(arg) = args.get(0) {
        if let Some(markov) = get_uid(arg)
                .map_or(None,
                        |x| usermap.get(&x)) {
            output_markov(&markov, &message, length, None);
        }
    }
});
