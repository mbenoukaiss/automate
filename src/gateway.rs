use ws::{Sender, Settings, WebSocket};
use crate::{json, AutomateaError};
use crate::{api, get, post, map};
use crate::models::{Payload, DispatchReady, DispatchGuildCreate, Hello, Identify, Gateway, Heartbeat, DispatchPresencesReplace, DispatchPresenceUpdate, DispatchMessageCreate, DispatchMessageUpdate, DispatchMessageDeleteBulk, DispatchMessageDelete};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::json::Nullable;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

macro_rules! handle_payload {
    ($data:ident as $payload:ty => $self:ident.$method:ident) => {{
        let payload: $payload = <$payload as ::automatea::json::FromJson>::from_json(&$data)?;

        *$self.last_sequence_number.lock().unwrap() = Nullable::from(payload.s);
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
            last_sequence_number: Arc::new(Mutex::new(Nullable::Null)),
            last_heartbeat_confirmed: Arc::new(AtomicBool::new(true)),
            heartbeat: None,
        }
    }
}

struct ClientHandler {
    ws: ws::Sender,
    last_sequence_number: Arc<Mutex<Nullable<u32>>>,
    last_heartbeat_confirmed: Arc<AtomicBool>,
    heartbeat: Option<JoinHandle<()>>,
}

impl ClientHandler {
    fn dispatch_payload(&mut self, data: &String) -> Result<(), AutomateaError> {
        match json::json_root_search::<u8>("op", data)? {
            0 => self.dispatch_event(data)?,
            10 => handle_payload!(data as Payload<Hello> => self.on_hello),
            11 => self.on_heartbeat_ack()?,
            unknown_op => warn!("Received unknown opcode '{}': \n{}", unknown_op, data)
        }

        Ok(())
    }

    fn dispatch_event(&mut self, data: &String) -> Result<(), AutomateaError>{
        match json::json_root_search::<String>("t", data)?.as_str() {
            DispatchReady::EVENT_NAME => handle_payload!(data as Payload<DispatchReady> => self.on_ready),
            DispatchGuildCreate::EVENT_NAME => handle_payload!(data as Payload<DispatchGuildCreate> => self.on_guild_create),
            DispatchPresencesReplace::EVENT_NAME => info!("Ignoring presence replace event"),
            DispatchPresenceUpdate::EVENT_NAME => handle_payload!(data as Payload<DispatchPresenceUpdate> => self.on_presence_update),
            DispatchMessageCreate::EVENT_NAME => handle_payload!(data as Payload<DispatchMessageCreate> => self.on_message_create),
            DispatchMessageUpdate::EVENT_NAME => handle_payload!(data as Payload<DispatchMessageUpdate> => self.on_message_update),
            DispatchMessageDelete::EVENT_NAME => handle_payload!(data as Payload<DispatchMessageDelete> => self.on_message_delete),
            DispatchMessageDeleteBulk::EVENT_NAME => handle_payload!(data as Payload<DispatchMessageDeleteBulk> => self.on_message_delete_bulk),
            unknown_event => warn!("Received unknown event: '{}': \n{}", unknown_event, data)
        }

        Ok(())
    }

    fn on_ready(&self, payload: DispatchReady) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_guild_create(&self, payload: DispatchGuildCreate) -> Result<(), AutomateaError> {
        //TODO: keep a list of guilds and users

        println!("{:?}", payload);
        Ok(())
    }

    fn on_presence_update(&self, payload: DispatchPresenceUpdate) -> Result<(), AutomateaError> {
        //TODO: keep track of user presences

        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_create(&self, payload: DispatchMessageCreate) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        if payload.0.author.username != "Rust" { //dirty "if it's not the bot"
            post!(api!("/channels/", payload.0.channel_id, "/messages"), map! {
                "content" => "Hello"
            });
        }

        Ok(())
    }

    fn on_message_update(&self, payload: DispatchMessageUpdate) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_delete(&self, payload: DispatchMessageDelete) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_delete_bulk(&self, payload: DispatchMessageDeleteBulk) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_hello(&mut self, payload: Hello) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        self.heartbeat = {
            let last_sequence_number = self.last_sequence_number.clone();
            let last_heartbeat_confirmed = self.last_heartbeat_confirmed.clone();
            let heartbeat_snd = self.ws.clone();

            Some(thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(payload.heartbeat_interval as u64));

                    if !last_heartbeat_confirmed.load(Ordering::Relaxed) {
                        warn!("Zombied connection detected");
                    }

                    if let Err(e) = heartbeat_snd.send(Heartbeat(*last_sequence_number.lock().unwrap())) {
                        error!("Failed to send heartbeat: {}", e);
                    } else {
                        last_heartbeat_confirmed.store(false, Ordering::Relaxed);
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
            large_threshold: None,
            shard: None,
            presence: None,
            guild_subscriptions: Some(true)
        };

        self.ws.send(identify)?;

        Ok(())
    }

    fn on_heartbeat_ack(&mut self) -> Result<(), AutomateaError> {
        self.last_heartbeat_confirmed.store(true, Ordering::Relaxed);

        info!("Received heartbeat acknowledgement");
        Ok(())
    }
}

impl ws::Handler for ClientHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if let ws::Message::Text(data) = msg {
            if let Err(err) = self.dispatch_payload(&data) {
                error!("An error occurred while reading message: {}\n{}", err.msg, data);
            }
        } else {
            error!("Unknown message type received");
        }

        Ok(())
    }
}