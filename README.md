# Automate &nbsp; [![Actions Status](https://github.com/mbenoukaiss/automate/workflows/Checks/badge.svg)](https://github.com/mbenoukaiss/automate/actions) ![GitHub issues](https://img.shields.io/github/issues/mbenoukaiss/automate) ![Crates.io](https://img.shields.io/crates/v/automate) ![Crates.io](https://img.shields.io/crates/l/automate)
Automate is an asynchronous rust library for interacting with the Discord API

**DISCLAIMER: This crate is at its very early stage and does not have most functionnalities a discord bot would need (no
voice, and only channel manipulation operations). Please don't use this crate except for experimenting.
The crate also only works in rust nightly.**

# Getting started
Automate is currently unstable and only works with Rust nightly. In order to add it to your project, add the following
line to your `Cargo.toml` under the `[dependencies]` section :
```
automate = "0.1.2"
```

You can then write the following in your `main.rs`. This simple example will respond Hello <name of the user>! to any
user posting a message in any channel while ignoring messages from bots.
In order for this example to work, you need to define the `DISCORD_API_TOKEN` environment variable. You can create a
bot and generate a token on [Discord's developers portal](https://discordapp.com/developers/applications/).

```rust
use automate::{tokio, async_trait};
use automate::{Error, Discord, Listener, Session};
use automate::models::{CreateMessage, MessageCreateDispatch};
use std::env;

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

fn main() -> Result<(), Error> {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with_listener(Box::new(MessageListener))
        .connect_blocking()?;
}
```

# License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Automate by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.