# Automate
Automate is an **asynchronous** rust library for interacting with the Discord API

# Getting started
Automate is currently unstable and only works with Rust nightly. In order to add it to your project, add the following
line to your `Cargo.toml` under the `[dependencies]` section :
```
automate = "0.1.0"
```

You can then write the following in your `main.rs`. This simple example will respond Hello <name of the user>! to any
user posting a message in any channel while ignoring messages from bots.

```
use automate::{tokio, async_trait};
use automate::{Error, Discord, Listener, Session};
use automate::models::{CreateMessage, MessageCreateDispatch};

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_message_create(&mut self, session: &Session, message: &MessageCreateDispatch) -> Result<(), Error> {
        if !message.author.bot.unwrap_or(false) {
            let content = Some(format!("Hello {}!", message.author.username));

            session.create_message(message.channel_id, CreateMessage {
                content,
                ..Default::default()
            }).await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    automate::setup_logging();

    Discord::new("discord api token goes here")
        .with_listener(Box::new(MessageListener))
        .connect().await?
}
```

# License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Serde by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.