use serenity::utils::MessageBuilder;

command!(help(ctx, msg, args) {
    let ref author = msg.author;

    let message = MessageBuilder::new()
    .mention(author.clone())
    .push(", to use markov-bot, say \"-generate\" followed by the length (in words) you want the sentence generated to be, for more information on markov chains: ")
    .push("<http://setosa.io/ev/markov-chains/>")
    .push(", <https://en.wikipedia.org/wiki/Markov_chain>")
    .build();

    let _ = msg.channel_id.say(&message);
});
