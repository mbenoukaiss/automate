#[macro_use]
extern crate automate;

use automate::{Error, Discord, Session};
use automate::gateway::{MessageReactionAddDispatch, MessageCreateDispatch};
use automate::http::{CreateMessage, NewInvite};
use std::env;

#[listener]
async fn say_hello(session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != session.bot().id && message.content.to_lowercase().contains("hello") {
        let content = Some(format!("Hello {}!", message.author.username));

        session.create_message(message.channel_id, CreateMessage {
            content,
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn invite(session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != session.bot().id && message.content.to_lowercase().contains("invite") {
        let invite = session.create_invite(message.channel_id, NewInvite {
            max_age: 3600 * 24,
            max_uses: 1,
            temporary: true,
            unique: false,
        }).await?.code;

        let content = format!("Here's your invite {} : https://discord.gg/{}", message.author.username, invite);

        session.create_message(message.channel_id, CreateMessage {
            content: Some(content),
            ..Default::default()
        }).await?;
    }

    Ok(())
}

#[listener]
async fn tell_reaction(session: &Session, reac: &MessageReactionAddDispatch) -> Result<(), Error> {
    if reac.user_id != session.bot().id {
        let content = format!("{}?!", reac.emoji.name);

        let sent_msg = session.create_message(reac.channel_id, CreateMessage {
            content: Some(content),
            ..Default::default()
        }).await?;

        session.create_reaction(sent_msg.channel_id, sent_msg.id, &reac.emoji.name).await?;
    }

    Ok(())
}

fn main() {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with(functions!(say_hello, invite, tell_reaction))
        .connect_blocking()
}
