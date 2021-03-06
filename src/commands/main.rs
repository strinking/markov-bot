use serenity::utils::MessageBuilder;

command!(help(ctx, msg, args) {
    let _ = ctx;
    let _ = args;

    let message = MessageBuilder::new()
        .mention(msg.author.clone())
        .push("\n")
        .push("Commands:\n")
        .push("`-gen [length]` - ")
        .push("Generates a message, with an optional length\n")
        .push("`-genuser \\@mention [length]` - ")
        .push("Generates based on a particular user\n")
        .push("For more information on Markov chains:")
        .push("<https://en.wikipedia.org/wiki/Markov_chain>")
        .build();
    let _ = msg.channel_id.say(&message);
});
