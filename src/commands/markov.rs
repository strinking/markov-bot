use markov::Markov;

command!(generate(ctx, message, args) {
    let mut data = ctx.data.lock().unwrap();
    let markov = data.get_mut::<Markov>().unwrap();

    match args.get(0) {
        Some(arg) => {
            let length: i32 = arg.parse().unwrap();
            println!("{}", length);
            let _ = message.channel_id.say(markov.generate(length).as_str());
            println!("{}", markov.generate(1));
        }

        None => {
            let _ = message.channel_id.say(markov.generate(20).as_str());
        }
    }
});
