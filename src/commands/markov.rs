use markov::Markov;
use serenity::model::{Message, UserId};
use usermap::UserMap;

const DEFAULT_GENERATION_LENGTH: u32 = 20;
const ERROR_MESSAGE: &str = "(Haven't collected enough data yet)";

fn output_markov(markov: &Markov, message: &Message, length: u32) {
    match markov.generate(length) {
        Some(x) => {
            let _ = message.channel_id.say(x.as_str());
        }
        None => {
            let _ = message.channel_id.say(ERROR_MESSAGE);
        }
    }
}

command!(generate(ctx, message, args) {
    let mut data = ctx.data.lock().unwrap();
    let markov = data.get_mut::<Markov>().unwrap();

    match args.get(0) {
        Some(arg) => {
            if let Ok(len) = arg.parse::<u32>() {
                output_markov(&markov, &message, len);
            }
        },
        None => {
            output_markov(&markov, &message, DEFAULT_GENERATION_LENGTH);
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
            output_markov(&markov, &message, length);
        }
    }
});
