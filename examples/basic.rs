//! Basic example with a few stateless listeners
//! listening to messages and reactions and that
//! responds by sending messages and creating invites.

#[macro_use]
extern crate automate;

use automate::{Error, Context, Configuration, Automate};
use automate::gateway::{MessageReactionAddDispatch, MessageCreateDispatch};
use automate::http::{CreateMessage, NewInvite, CreateAttachment};
use std::fs::File;
use std::io::Read;

#[listener]
async fn say_hello(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != ctx.bot.id && message.content.to_lowercase().contains("hello") {
        let content = Some(format!("Hello {}!", message.author.username));

        ctx.create_message(message.channel_id, CreateMessage {
            content,
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn send_rust_logo(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != ctx.bot.id && message.content.to_lowercase().starts_with("!rust") {
        let content = Some(String::from("Sure, here's the Rust logo!"));

        let mut image = File::open("assets/rust.png").unwrap();
        let mut image_content = Vec::new();
        image.read_to_end(&mut image_content)?;

        let rust = CreateAttachment {
            name: String::from("rust.png"),
            mime: String::from("image/png"),
            content: image_content
        };

        ctx.create_message(message.channel_id, CreateMessage {
            content,
            attachment: Some(rust),
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn invite(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != ctx.bot.id && message.content.to_lowercase().contains("invite") {
        let invite = ctx.create_invite(message.channel_id, NewInvite {
            max_age: 3600 * 24,
            max_uses: 1,
            temporary: true,
            unique: false,
        }).await?.code;

        let content = format!("Here's your invite {}: https://discord.gg/{}", message.author.username, invite);

        ctx.create_message(message.channel_id, CreateMessage {
            content: Some(content),
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn tell_reaction(ctx: &Context, reac: &MessageReactionAddDispatch) -> Result<(), Error> {
    if reac.user_id != ctx.bot.id {
        let content = format!("{}?!", reac.emoji.name);

        let sent_msg = ctx.create_message(reac.channel_id, CreateMessage {
            content: Some(content),
            ..Default::default()
        }).await?;

        ctx.create_reaction(sent_msg.channel_id, sent_msg.id, &reac.emoji).await?;
    }

    Ok(())
}

fn main() {
    let config = Configuration::from_env("DISCORD_API_TOKEN")
        .register(stateless!(say_hello, send_rust_logo, invite, tell_reaction));

    Automate::launch(config)
}
