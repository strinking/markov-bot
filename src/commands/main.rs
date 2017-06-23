use serenity::utils::MessageBuilder;

command!(help(_ctx, msg, _args) {
    let author = &msg.author;

    let message = MessageBuilder::new()
    .mention(author.clone())
    .push("\nTo use markov-bot, say: ")
    .push("\"-gen\" ")
    .push("or, for user specific markov chains, say: ")
    .push("\"-genuser (mention)\" ")
    .push("followed by the length you want the generated")
    .push("sentence to be")
    .push("")
    .push("For more information on Markov chains:")
    .push("<https://en.wikipedia.org/wiki/Markov_chain>")
    .build();

    let _ = msg.channel_id.say(&message);
});
