# Automate &nbsp; [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mbenoukaiss/automate/Checks?style=for-the-badge)](https://github.com/mbenoukaiss/automate/actions) [![GitHub issues](https://img.shields.io/github/issues/mbenoukaiss/automate?style=for-the-badge)](https://github.com/mbenoukaiss/automate/issues) [![Crates.io](https://img.shields.io/crates/v/automate?style=for-the-badge)](https://crates.io/crates/automate) [![Crates.io](https://img.shields.io/crates/l/automate?style=for-the-badge)](https://github.com/mbenoukaiss/automate/blob/master/LICENSE)
Automate is a low level and asynchronous rust library for interacting with the Discord API

# Getting started
Automate currently only works with Rust nightly. The tested version and the one used in CI is
`nightly-2020-01-31`. Refer to [rust edition guide](https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html)
to learn how to switch to rust nightly.

In order to use Automate in your project, add the following line to your `Cargo.toml` :
```
[dependencies]
automate = "0.2.1"
```

You can then write the following in your `main.rs`. This simple example will respond Hello <name of the user>! to any
user posting a message in any channel while ignoring messages from bots.
In order for this example to work, you need to define the `DISCORD_API_TOKEN` environment variable. You can create a
bot and generate a token on [Discord's developers portal](https://discordapp.com/developers/applications/).

```rust
#[macro_use]
extern crate automate;

use automate::{Error, Discord, Session};
use automate::gateway::MessageCreateDispatch;
use automate::http::CreateMessage;
use std::env;

#[listener]
async fn say_hello(session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.author.id != session.bot().id {
        let content = Some(format!("Hello {}!", message.author.username));

        session.create_message(message.channel_id, CreateMessage {
            content,
            ..Default::default()
        }).await?;
    }

    Ok(())
}

fn main() {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with(functions!(say_hello))
        .connect_blocking()
}
```

For examples with more details, see in the `examples` folder.

# Contributing
Any kind of contribution is welcome, from issues to pull requests. For major changes, please open an issue first to discuss what you would like to change.
Please make sure to update tests as appropriate.

# License
Licensed under the [MIT license](LICENSE).