use serenity::utils::MessageBuilder;

command!(help(_ctx, msg, _args) {
    let author = &msg.author;

    let message = MessageBuilder::new()
    .mention(author.clone())
    .push("\nTo use markov-bot, say: ")
    .push("\"-gen\" ")
    .push("or, for user specific markov chains, say: ")
    .push("\"-genuser (mention)\" ")
    .push("followed by the length you want the sentence generated to be, for more information on markov chains: ")
    .push("<https://en.wikipedia.org/wiki/Markov_chain>")
    .build();

    let _ = msg.channel_id.say(&message);
});
