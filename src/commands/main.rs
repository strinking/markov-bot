use serenity::utils::MessageBuilder;

command!(help(ctx, message, args) {
    let _ = ctx;
    let _ = args;

    let msg = MessageBuilder::new()
        .mention(message.author.clone())
        .push("\n")
        .push("Commands:\n")
        .push("`%gen [length] [word]` - ")
        .push("Generates a message, with an optional length and/or by word\n")
        .push("`%genuser \@mention [length]` - ")
        .push("Generates based on a particular user\n")
        .push("For more information on Markov chains: ")
        .push("<https://en.wikipedia.org/wiki/Markov_chain>\n")
        .push("For more information about the bot: https://github.com/strinking/markov-bot")
        .build();
    let _ = message.channel_id.say(&msg);
});
