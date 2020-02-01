# Automate &nbsp; [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mbenoukaiss/automate/Checks?style=for-the-badge)](https://github.com/mbenoukaiss/automate/actions) [![GitHub issues](https://img.shields.io/github/issues/mbenoukaiss/automate?style=for-the-badge)](https://github.com/mbenoukaiss/automate/issues) [![Crates.io](https://img.shields.io/crates/v/automate?style=for-the-badge)](https://crates.io/crates/automate) [![Crates.io](https://img.shields.io/crates/l/automate?style=for-the-badge)](https://github.com/mbenoukaiss/automate/blob/master/LICENSE)
Automate is a low level and asynchronous rust library for interacting with the Discord API

# Getting started
Automate currently only works with Rust nightly. The tested version and the one used in CI is
`nightly-2020-01-31`. Refer to [rust edition guide](https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html)
to learn how to switch to rust nightly.

In order to use Automate in your project, add the following line to your `Cargo.toml` under the `[dependencies]` section :
```
automate = "0.1.4"
```

You can then write the following in your `main.rs`. This simple example will respond Hello <name of the user>! to any
user posting a message in any channel while ignoring messages from bots.
In order for this example to work, you need to define the `DISCORD_API_TOKEN` environment variable. You can create a
bot and generate a token on [Discord's developers portal](https://discordapp.com/developers/applications/).

```rust
#![allow(where_clauses_object_safety)]

use automate::async_trait;
use automate::{Error, Discord, Listener, Session};
use automate::gateway::MessageCreateDispatch;
use automate::http::CreateMessage;
use std::env;

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_message_create(&mut self, session: &Session, message: &MessageCreateDispatch) -> Result<(), Error> {
        let message = &message.0;

        if message.author.id != session.bot().id {
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

# Contributing
Any kind of contribution is welcome, from issues to pull requests. For major changes, please open an issue first to discuss what you would like to change.
Please make sure to update tests as appropriate.

# License
Licensed under the [MIT license](LICENSE).