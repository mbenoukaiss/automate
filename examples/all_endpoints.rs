#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed

use automate::{async_trait, Listener, Session};
use automate::{Error, Discord};
use std::env;
use automate::gateway::{MessageReactionAddDispatch, VerificationLevel, MessageNotificationLevel, ExplicitContentFilterLevel};
use automate::http::NewGuild;

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_reaction_add(&mut self, session: &Session, data: &MessageReactionAddDispatch) -> Result<(), Error> {
        if data.user_id != session.bot().id {
            session.create_reaction(data.channel_id, data.message_id, &data.emoji).await?;

            session.gateway().await?;
            session.curent_user().await?;
            session.gateway_bot().await?;

            let mut regions = session.voice_regions().await?;
            let guild = session.bot_guilds().await?.remove(0);
            session.guild(guild).await?;
            session.create_guild(NewGuild {
                name: String::from("Test"),
                region: regions.remove(0).id,
                icon: String::new(),
                verification_level: VerificationLevel::None,
                default_message_notifications: MessageNotificationLevel::AllMessages,
                explicit_content_filter: ExplicitContentFilterLevel::Disabled,
                roles: vec![],
                channels: vec![]
            }).await?;
        }

        Ok(())
    }
}

fn main() {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with_listener(MessageListener)
        .connect_blocking()
}
