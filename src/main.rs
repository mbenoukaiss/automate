extern crate self as automate;
#[macro_use] extern crate log;
#[macro_use] extern crate automate_proc;

pub mod models;

pub use as_json::AsJson;

mod as_json;

use reqwest::Client;
use ws::{Message, CloseCode};
use std::io::{Error, ErrorKind};
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
            .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/automate, 0.1.0)")
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let client = Client::new();

    let gateway: Gateway = get!(client, "/gateway");

    ws::connect(gateway.url, |out| {
        move |msg: Message| {
            if let Message::Text(data) = msg {
                match find_opcode(&data) {
                    Ok(0) => {
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
                                "$browser" => "automate",
                                "$device" => "automate"
                            },
                            compress: None
                        };

                        out.send(identify);
                    },
                    Ok(op) => {
                        error!("Unknown opcode received: {}\n{}", op, data);
                    },
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

fn find_opcode(candidate: &str) -> Result<u8, Error> {
    //get candidate slice starting at the first digit of the opcode
    let opcode_begin = {
        let op_key_end = match candidate.find("\"op\"") {
            Some(i) => i + 4,
            None => return Err(Error::new(ErrorKind::InvalidData, "Could not find op key"))
        };

        match candidate[op_key_end..].find(char::is_numeric) {
            Some(i) => &candidate[op_key_end + i..],
            None => return Err(Error::new(ErrorKind::InvalidData, "Could not find op value"))
        }
    };

    let mut iter = opcode_begin.chars();

    let mut prev_index = 0;

    let mut opcode = None;

    while opcode.is_none() {
        if let Some(next) = iter.next() {
            if !next.is_numeric() {
                opcode = Some(&opcode_begin[..prev_index]);
            }

            prev_index += 1;
        } else {
            opcode = Some(&opcode_begin[..prev_index]);
        }
    }

    if let Some(opcode) = opcode {
        return match opcode.parse::<u8>() {
            Ok(opcode) => Ok(opcode),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
        };
    }

    Err(Error::new(ErrorKind::InvalidData, "Failed to find opcode"))
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
        .level_for("automate", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
