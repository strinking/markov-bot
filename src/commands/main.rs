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
        match args.get(0) {
            Some(arg) => {
                match arg.to_lowercase().as_str() {
                    "online" => {
                        ctx.set_presence(None, OnlineStatus::Online, false);
                    }
                    
                    "invisible" => {
                        ctx.set_presence(None, OnlineStatus::Invisible, false);
                    }
                    
                    "invis" => {
                        ctx.set_presence(None, OnlineStatus::Invisible, false);
                    }
                    
                    "dnd" => {
                        ctx.set_presence(None, OnlineStatus::DoNotDisturb, false);
                    }
                    
                    "idle" => {
                        ctx.set_presence(None, OnlineStatus::Idle, false);
                    }
                    
                    "reset" => {
                        ctx.set_presence(None, OnlineStatus::Online, false);
                    }
                    
                    _ => {
                        ctx.set_presence(None, OnlineStatus::Online, false);
                    }
                }
            }
            None => {
                ctx.set_presence(None, OnlineStatus::Online, false);
            }
        }
    }
});

