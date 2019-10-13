use ws::CloseCode;
use crate::{json, AutomateaError};
use crate::{api, get, post, map};
use crate::models::*;
use crate::json::Nullable;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;

macro_rules! handle_payload {
    ($data:ident as $payload:ty => $self:ident.$method:ident) => {{
        let payload: $payload = <$payload as ::automatea::json::FromJson>::from_json(&$data)?;

        if let Nullable::Value(val) = payload.s {
            *$self.sequence_number.lock().unwrap() = Some(val);
        }

        $self.$method(payload.d)?
    }};
}

pub struct GatewayClient;

impl GatewayClient {
    pub fn connect() -> ! {
        let session_id = Rc::new(RefCell::new(None));
        let sequence_number = Arc::new(Mutex::new(None));

        loop {
            let execution: Result<(), AutomateaError> = try {
                let gateway: Result<Gateway, AutomateaError> = get!("/gateway");

                ws::connect(gateway?.url.as_str(), |client| {
                    ClientHandler {
                        ws: client,
                        session_id: Rc::clone(&session_id),
                        sequence_number: Arc::clone(&sequence_number),
                        heartbeat_confirmed: Arc::new(AtomicBool::new(true)),
                        heartbeat: None,
                    }
                })?;
            };

            if let Err(err) = execution {
                error!("Connection was interrupted with '{}'", err.msg);
            } else {
                error!("Connection interrupted");
            }

            thread::sleep(Duration::from_secs(5));

            if let Some(sid) = &*session_id.borrow() {
                info!("Attempting to resume session {} with sequence_number: {}", sid, sequence_number.lock().unwrap().unwrap());
            } else {
                info!("Attempting to reconnect");
            }
        }
    }
}

struct ClientHandler {
    ws: ws::Sender,
    session_id: Rc<RefCell<Option<String>>>,
    sequence_number: Arc<Mutex<Option<i32>>>,
    heartbeat_confirmed: Arc<AtomicBool>,
    heartbeat: Option<mpsc::Sender<bool>>,
}

impl ClientHandler {
    fn dispatch_payload(&mut self, data: &str) -> Result<(), AutomateaError> {
        match json::json_root_search::<u8>("op", data)? {
            0 => self.dispatch_event(data)?,
            9 => handle_payload!(data as Payload<InvalidSession> => self.on_invalid_session),
            10 => handle_payload!(data as Payload<Hello> => self.on_hello),
            11 => self.on_heartbeat_ack()?,
            unknown_op => warn!("Received unknown opcode '{}': \n{}", unknown_op, data)
        }

        Ok(())
    }

    fn dispatch_event(&mut self, data: &str) -> Result<(), AutomateaError> {
        match json::json_root_search::<String>("t", data)?.as_str() {
            ReadyDispatch::EVENT_NAME => handle_payload!(data as Payload<ReadyDispatch> => self.on_ready),
            ResumedDispatch::EVENT_NAME => handle_payload!(data as Payload<ResumedDispatch> => self.on_resumed),
            GuildCreateDispatch::EVENT_NAME => handle_payload!(data as Payload<GuildCreateDispatch> => self.on_guild_create),
            PresencesReplaceDispatch::EVENT_NAME => info!("Ignoring presence replace event"),
            PresenceUpdateDispatch::EVENT_NAME => handle_payload!(data as Payload<PresenceUpdateDispatch> => self.on_presence_update),
            MessageCreateDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageCreateDispatch> => self.on_message_create),
            MessageUpdateDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageUpdateDispatch> => self.on_message_update),
            MessageDeleteDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageDeleteDispatch> => self.on_message_delete),
            MessageDeleteBulkDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageDeleteBulkDispatch> => self.on_message_delete_bulk),
            MessageReactionAddDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageReactionAddDispatch> => self.on_message_reaction_add),
            MessageReactionRemoveDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageReactionRemoveDispatch> => self.on_message_reaction_remove),
            MessageReactionRemoveAllDispatch::EVENT_NAME => handle_payload!(data as Payload<MessageReactionRemoveAllDispatch> => self.on_message_reaction_remove_all),
            TypingStartDispatch::EVENT_NAME => handle_payload!(data as Payload<TypingStartDispatch> => self.on_typing_start),
            unknown_event => warn!("Received unknown event: '{}': \n{}", unknown_event, data)
        }

        Ok(())
    }

    fn on_guild_create(&self, payload: GuildCreateDispatch) -> Result<(), AutomateaError> {
        //TODO: keep a list of guilds and users

        println!("{:?}", payload);
        Ok(())
    }

    fn on_presence_update(&self, payload: PresenceUpdateDispatch) -> Result<(), AutomateaError> {
        //TODO: keep track of user presences

        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_create(&self, payload: MessageCreateDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        if payload.0.author.username != "Rust" { //dirty "if it's not the bot"
            post!(api!("/channels/", payload.0.channel_id, "/messages"), map! {
                "content" => "Hello"
            })?;
        }

        Ok(())
    }

    fn on_message_update(&self, payload: MessageUpdateDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_delete(&self, payload: MessageDeleteDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_delete_bulk(&self, payload: MessageDeleteBulkDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_reaction_add(&self, payload: MessageReactionAddDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_reaction_remove(&self, payload: MessageReactionRemoveDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_message_reaction_remove_all(&self, payload: MessageReactionRemoveAllDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_typing_start(&self, payload: TypingStartDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);
        Ok(())
    }

    fn on_hello(&mut self, payload: Hello) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        if let Some(sid) = &*self.session_id.borrow() {
            let resume = Resume {
                token: "NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY".to_owned(),
                session_id: sid.to_owned(),
                seq: self.sequence_number.lock().unwrap().unwrap(),
            };

            self.ws.send(resume)?;
            info!("Requested to resume session");
        } else {
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
                guild_subscriptions: Some(true),
            };

            self.ws.send(identify)?;
        }

        self.heartbeat = {
            let sequence_number = self.sequence_number.clone();
            let heartbeat_confirmed = self.heartbeat_confirmed.clone();
            let sender = self.ws.clone();

            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let rs: Result<(), AutomateaError> = try {
                    while match rx.recv_timeout(Duration::from_millis(u64::from(payload.heartbeat_interval))) {
                        Ok(_) => false,
                        Err(RecvTimeoutError::Timeout) => true,
                        Err(RecvTimeoutError::Disconnected) => panic!("The other end was disconnected")
                    } {
                        if !heartbeat_confirmed.load(Ordering::Relaxed) {
                            warn!("Zombied connection detected, shutting down connection");
                            sender.shutdown()?;
                            break;
                        }

                        sender.send(Heartbeat(Nullable::from(*sequence_number.lock().unwrap())))?;
                        heartbeat_confirmed.store(false, Ordering::Relaxed);

                        info!("Successfully sent heartbeat");
                    }
                };

                if let Err(err) = rs {
                    error!("Heartbeat thread failed ({}), shutting down connection", err.msg);
                    sender.shutdown().expect("Failed to shutdown");
                }
            });

            Some(tx)
        };

        Ok(())
    }

    fn on_ready(&mut self, payload: ReadyDispatch) -> Result<(), AutomateaError> {
        println!("{:?}", payload);

        *self.session_id.borrow_mut() = Some(payload.session_id);
        Ok(())
    }

    fn on_resumed(&mut self, _payload: ResumedDispatch) -> Result<(), AutomateaError> {
        info!("Successfully resumed session");
        Ok(())
    }

    fn on_invalid_session(&mut self, payload: InvalidSession) -> Result<(), AutomateaError> {
        if !payload.0 {
            *self.session_id.borrow_mut() = None;
        }

        Ok(())
    }

    fn on_heartbeat_ack(&mut self) -> Result<(), AutomateaError> {
        self.heartbeat_confirmed.store(true, Ordering::Relaxed);

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

    fn on_close(&mut self, _code: CloseCode, _reason: &str) {
        self.heartbeat.as_ref()
            .expect("No heartbeat sender")
            .send(true) //might legitimately fail if the heartbeat thread failed, maybe expect is not appropriate
            .expect("Failed to reach the heartbeat thread");
    }
}