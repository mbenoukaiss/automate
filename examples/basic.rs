#[macro_use]
extern crate automate;

use automate::{Error, Discord, Context};
use automate::gateway::{MessageReactionAddDispatch, MessageCreateDispatch};
use automate::http::{CreateMessage, NewInvite};
use std::env;

#[listener]
async fn say_hello(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != ctx.bot().id && message.content.to_lowercase().contains("hello") {
        let content = Some(format!("Hello {}!", message.author.username));

        ctx.create_message(message.channel_id, CreateMessage {
            content,
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn invite(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != ctx.bot().id && message.content.to_lowercase().contains("invite") {
        let invite = ctx.create_invite(message.channel_id, NewInvite {
            max_age: 3600 * 24,
            max_uses: 1,
            temporary: true,
            unique: false,
        }).await?.code;

        let content = format!("Here's your invite {} : https://discord.gg/{}", message.author.username, invite);

        ctx.create_message(message.channel_id, CreateMessage {
            content: Some(content),
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn tell_reaction(ctx: &mut Context, reac: &MessageReactionAddDispatch) -> Result<(), Error> {
    if reac.user_id != ctx.bot().id {
        let content = format!("{}?!", reac.emoji.name);

        let sent_msg = ctx.create_message(reac.channel_id, CreateMessage {
            content: Some(content),
            ..Default::default()
        }).await?;

        ctx.create_reaction(sent_msg.channel_id, sent_msg.id, &reac.emoji.name).await?;
    }

    Ok(())
}

fn main() {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .register(functions!(say_hello, invite, tell_reaction))
        .connect_blocking()
}
