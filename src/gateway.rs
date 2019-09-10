use ws::{Sender, Settings, WebSocket};
use crate::{json, AutomateaError};
use crate::{get, map};
use crate::models::{Payload, Ready, GuildCreate, Hello, Identify, Gateway, Heartbeat};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::json::Nullable;

macro_rules! handle_payload {
    ($data:ident as $payload:ty => $self:ident.$method:ident) => {{
        let payload: $payload = <$payload as ::automatea::json::FromJson>::from_json(&$data)?;

        $self.last_sequence_number = Nullable::from(payload.s);
        $self.$method(payload.d)?
    }};
}

const CONNECTIONS: usize = 10_000;

pub struct GatewayClient {
    _ws: WebSocket<ClientFactory>
}

impl GatewayClient {
    pub fn connect() -> Result<GatewayClient, AutomateaError> {
        let gateway: Gateway = get!("/gateway");

        let mut websocket = ws::Builder::new()
            .with_settings(Settings {
                max_connections: CONNECTIONS,
                ..Settings::default()
            })
            .build(ClientFactory)?;

        websocket.connect(gateway.url)?;

        Ok(GatewayClient {
            _ws: websocket.run()?
        })
    }
}

struct ClientFactory;

impl ws::Factory for ClientFactory {
    type Handler = ClientHandler;

    fn connection_made(&mut self, ws: Sender) -> Self::Handler {
        ClientHandler {
            ws,
            last_sequence_number: Nullable::Null,
            heartbeat: None,
        }
    }
}

struct ClientHandler {
    ws: ws::Sender,
    last_sequence_number: Nullable<u32>,
    heartbeat: Option<JoinHandle<()>>,
}

impl ClientHandler {
    fn on_ready(&self, payload: Ready) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        Ok(())
    }

    fn on_guild_create(&self, payload: GuildCreate) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        Ok(())
    }

    fn on_hello(&mut self, payload: Hello) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        self.heartbeat = {
            let hearbeat_snd = self.ws.clone();

            Some(thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(payload.heartbeat_interval as u64));

                    if let Err(e) = hearbeat_snd.send(Heartbeat(Nullable::Null)) {
                        error!("Failed to send heartbeat: {}", e);
                    } else {
                        info!("Successfully sent heartbeat");
                    }
                }
            }))
        };

        let identify = Identify {
            token: "NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY".to_owned(),
            properties: map! {
                "$os" => "linux",
                "$browser" => "automatea",
                "$device" => "automatea"
            },
            compress: None,
        };

        self.ws.send(identify)?;

        Ok(())
    }
}

impl ws::Handler for ClientHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if let ws::Message::Text(data) = msg {
            let err: Result<(), AutomateaError> = try {
                match json::json_root_search::<u8>("op", &data)? {
                    0 => match json::json_root_search::<String>("t", &data)?.as_str() {
                        Ready::EVENT_NAME => handle_payload!(data as Payload<Ready> => self.on_ready),
                        GuildCreate::EVENT_NAME => handle_payload!(data as Payload<GuildCreate> => self.on_guild_create),
                        unknown_event => error!("Unknown event: '{}': \n{}", unknown_event, data)
                    },
                    10 => handle_payload!(data as Payload<Hello> => self.on_hello),
                    unknown_op => error!("Received unknown opcode '{}': \n{}", unknown_op, data)
                }
            };

            if let Err(err) = err {
                error!("An error occurred while reading message: {}\n{}", err.msg, data);
            }
        } else {
            error!("Unknown message type received");
        }

        Ok(())
    }
}