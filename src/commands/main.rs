use serenity::utils::MessageBuilder;
use serenity::model::OnlineStatus;
use status::Status;

lazy_static! {
    static ref ALLOWED_USER_IDS: Vec<u64> = vec![76043245804589056, 98633956773093376];
}

command!(help(ctx, message, args) {
    let _ = ctx;
    let _ = args;

    let msg = MessageBuilder::new()
        .mention(message.author.clone())
        .push("\n")
        .push("Commands:\n")
        .push("`%gen [length] [word]` - ")
        .push("Generates a message, with an optional length and/or by word\n")
        .push("`%genuser @mention [length]` - ")
        .push("Generates based on a particular user\n")
        .push("For more information on Markov chains: ")
        .push("<https://en.wikipedia.org/wiki/Markov_chain>\n")
        .push("For more information about the bot: https://github.com/strinking/markov-bot")
        .build();
    let _ = message.channel_id.say(&msg);
});

command!(status(ctx, message, args) {
    if ALLOWED_USER_IDS.contains(&message.author.id.0) {
        ctx.set_presence(None, Status::from_str(args.get(0).unwrap()).unwrap_or(OnlineStatus::Online), false);
    }
});

command!(game(ctx, message, args) {
    if ALLOWED_USER_IDS.contains(&message.author.id.0) {
        let arg = args.join(" ");
        ctx.set_game_name(arg.as_str());
    }
});

command!(name(ctx, message, args) {
    if ALLOWED_USER_IDS.contains(&message.author.id.0) {
        let arg = args.join(" ");
        let _ = ctx.edit_profile(|p| p.username(arg.as_str())).expect("could not set name");
    }
});

command!(nick(ctx, message, args) {
    let _ = ctx;
    if ALLOWED_USER_IDS.contains(&message.author.id.0) {
        let arg = args.join(" ");
        let _ = message.guild_id().unwrap().edit_nickname(Some(arg.as_str()));
    }
});
