use reqwest::Client;
use serde::{Serialize, Deserialize};
use ws::{Message, CloseCode};

macro_rules! api {
    ($dest:expr) => {
        concat!("https://discordapp.com/api/v6", $dest)
    }
}

macro_rules! get {
    ($client:expr, $dest:expr) => {
        $client.get(api!($dest))
            .header("Authorization", "Bot XwFiHgZJd0V3Sv8Nvj65YNvxSF7ARES0")
            .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/dirscod, 0.1.0)")
            .send()?
            .json()?
    }
}

macro_rules! deserialize {
    ($data:expr) => {
        serde_json::from_str(&$data).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Opcode<D> {
    pub op: u8,
    pub d: D,
    pub s: Option<u32>,
    pub t: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub heartbeat_interval: u32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let client = Client::new();

    let gateway: Gateway = get!(client, "/gateway");

    ws::connect(gateway.url, |out| {
        move |msg: Message| {

            if let Message::Text(data) = msg {
                let hello: Opcode<Hello> = deserialize!(data);

                println!("Received : {:?}", hello);
            } else {
                out.close_with_reason(CloseCode::Error, "Unknown message type")?;
            }

            Ok(())
        }
    })?;

    Ok(())
}

fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} in {target} [{level}]: {message}",
                date = chrono::Local::now().format("%H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("dirscod", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}