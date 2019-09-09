use ws::{Sender, Settings, WebSocket};
use crate::{json, AutomateaError};
use crate::{get, map, deserialize};
use crate::models::{Payload, Ready, GuildCreate, Hello, Identify, Gateway};

const CONNECTIONS: usize = 10_000;

pub struct GatewayClient {
    ws: WebSocket<ClientFactory>
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
            ws: websocket.run()?
        })
    }

}

struct ClientFactory;

impl ws::Factory for ClientFactory {
    type Handler = ClientHandler;

    fn connection_made(&mut self, ws: Sender) -> Self::Handler {
        ClientHandler {
            ws
        }
    }

}

struct ClientHandler {
    ws: ws::Sender
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

    fn on_hello(&self, payload: Hello) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

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
                        Ready::EVENT_NAME => self.on_ready(deserialize!(data as Payload<Ready>)?.d)?,
                        GuildCreate::EVENT_NAME => self.on_guild_create(deserialize!(data as Payload<GuildCreate>)?.d)?,
                        unknown_event => error!("Unknown event: '{}': \n{}", unknown_event, data)
                    },
                    10 => self.on_hello(deserialize!(data as Payload<Hello>)?.d)?,
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