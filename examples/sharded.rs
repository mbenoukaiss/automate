#[macro_use]
extern crate automate;

use automate::{Error, Context, Configuration, Automate, ShardManager};
use automate::gateway::MessageCreateDispatch;
use automate::http::CreateMessage;

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

fn main() {
    let config = Configuration::from_env("DISCORD_API_TOKEN")
        .register(functions!(say_hello));

    Automate::block_on(async move {
        ShardManager::with_config(config).await
            .set_total_shards(2)
            .setup(0)
            .setup(1)
            .launch().await;
    });
}
