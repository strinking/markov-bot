use usermap::UserMap;
use database::connect;
use std::collections::HashMap;
use serenity::model::{Message, UserId};
use markov::{Markov, parse_user_messages};

const DEFAULT_GENERATION_LENGTH: u32 = 20;
const ERROR_MESSAGE: &str = "(Haven't collected enough data yet)";

fn get_uid(name: &str) -> Option<u64> {
    if name.starts_with("<") && name.ends_with(">") {
        name.parse::<UserId>().ok().map_or(None, |x| Some(x.0))
    } else {
        None
    }
}

fn output_markov(markov: &Markov, mes: &Message, length: u32, start: Option<&str>) {
    let result = match start {
        Some(word) => markov.generate_from_word(length, word),
        None => markov.generate(length),
    };

    let msg = match result {
        Some(ref text) => text.as_str(),
        None => ERROR_MESSAGE,
    };

    let _ = mes.channel_id.say(msg);
}

fn gen_user(usermap: &mut HashMap<u64, Markov>, length: Option<&String>, starting_word: Option<&String>, message: &Message, uid: Option<&String>) {
    let word = starting_word
        .map_or(None,
                |x| Some(x.as_str()));
    
    let length = length
        .map_or(DEFAULT_GENERATION_LENGTH,
                |x| x.parse::<u32>()
                .ok()
                .unwrap_or(DEFAULT_GENERATION_LENGTH));
    
    let uid = match uid {
        Some(u) => { get_uid(u) }
        None => None
    };
    
    match usermap.contains_key(&uid.unwrap()) {
        true => {
            if let Some(markov) = uid
                .map_or(None,
                        |x| usermap.get(&x)) {
                    output_markov(&markov, &message, length, word);
                }
        }
        
        false => {
            let connection = connect();
            if let Some(_) = uid {
                let mut markov = usermap.entry(uid.unwrap()).or_insert(Markov::new());
                parse_user_messages(&connection, markov, uid.unwrap());             
                output_markov(&markov, &message, length, word);
            }
        } 
    }
}


command!(generate(ctx, mes, args) {
    let mut data = ctx.data.lock().unwrap();
    let mut markov = data.get_mut::<Markov>().unwrap();

    let length = args.get(0)
        .map_or(DEFAULT_GENERATION_LENGTH,
                |x| x.parse::<u32>()
                .ok()
                .unwrap_or(DEFAULT_GENERATION_LENGTH));
    output_markov(&markov, &mes, length, None);
});

command!(generate_user(ctx, mes, args) {
    let mut data =  ctx.data.lock().unwrap();
    let mut usermap = data.get_mut::<UserMap>().unwrap();
    gen_user(usermap, args.get(1), None, &mes, args.get(0));
});


command!(generate_from_word(ctx, mes, args) {
    let mut data = ctx.data.lock().unwrap();
    let mut markov = data.get_mut::<Markov>().unwrap();

    let length: u32 = args.get(1)
        .map_or(DEFAULT_GENERATION_LENGTH,
                |x| x.parse::<u32>()
                .ok()
                .unwrap_or(DEFAULT_GENERATION_LENGTH));
    
    if let Some(word) = args.get(0)
        .map_or(None, |x| Some(x.as_str())) {
            output_markov(&markov, &mes, length, Some(word));
    }
});


command!(generate_word_from_user(ctx, mes, args) {
    let mut data = ctx.data.lock().unwrap();
    let mut usermap = data.get_mut::<UserMap>().unwrap();
    gen_user(usermap, args.get(2), args.get(1), mes, args.get(0));
});
