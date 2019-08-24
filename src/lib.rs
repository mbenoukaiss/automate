#![feature(test)]

extern crate self as automatea;
extern crate test;
#[macro_use] extern crate log;
#[macro_use] extern crate automatea_proc;

pub mod models;

pub use json::{AsJson, FromJson};

pub mod json;

use reqwest::Client;
use ws::{Message, CloseCode};
use crate::models::{Payload, Ready, Hello, Identify, Gateway};

macro_rules! api {
    ($dest:expr) => {
        concat!("https://discordapp.com/api/v6", $dest)
    }
}

macro_rules! get {
    ($client:expr, $dest:expr) => {
        $client.get(api!($dest))
            .header("Authorization", "Bot NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY")
            .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/automatea, 0.1.0)")
            .send()?
            .json()?
    }
}

macro_rules! deserialize {
    ($data:expr) => {
        serde_json::from_str(&$data).unwrap();
    }
}

macro_rules! map {
    {$($key:expr => $val:expr),*} => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key.to_owned(), $val.to_owned());)*

        map
    }}
}

pub fn launch() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let client = Client::new();

    let gateway: Gateway = get!(client, "/gateway");

    ws::connect(gateway.url, |out| {
        move |msg: Message| {
            if let Message::Text(data) = msg {
                match json::json_root_search::<u8>("op", &data) {
                    Ok(0) => {
                        if let Ok(event_type) = json::json_root_search::<String>("t", &data) {
                            println!("Received ready of type : {}", event_type);
                        } else {
                            println!("wtf");
                        }

                        let ready: Payload<Ready> = deserialize!(data);
                        println!("Received ready : {:?}", ready);
                    }
                    Ok(10) => {
                        let hello: Payload<Hello> = deserialize!(data);
                        println!("Received hello : {:?}", hello);

                        let identify = Identify {
                            token: "NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY".to_owned(),
                            properties: map! {
                                "$os" => "linux",
                                "$browser" => "automatea",
                                "$device" => "automatea"
                            },
                            compress: None,
                        };

                        out.send(identify);
                    }
                    Ok(op) => {
                        error!("Unknown opcode received: {}\n{}", op, data);
                    }
                    Err(e) => error!("{}", e)
                }
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
        .level_for("automatea", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
