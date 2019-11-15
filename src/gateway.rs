use ws::CloseCode;
use crate::{json, Error, Listener};
use crate::map;
use crate::models::*;
use crate::json::Nullable;
use crate::http::HttpAPI;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use futures::executor;
use std::ops::Deref;

macro_rules! call_dispatcher {
    ($data:ident as $payload:ty => $self:ident.$method:ident) => {{
        let payload: $payload = <$payload as ::automate::json::FromJson>::from_json(&$data)?;

        if let Nullable::Value(val) = payload.s {
            *$self.sequence_number.lock().unwrap() = Some(val);
        }

        $self.$method(payload.d).await?
    }};
}

macro_rules! dispatcher {
    ($name:ident: $type:ty) => {
        async fn $name(&self, payload: $type) -> Result<(), Error> {
            for listener in &mut *self.session.listeners.lock().unwrap() {
                (*listener).$name(&self.session, &payload).await?
            }

            Ok(())
        }
    }
}

pub struct GatewayAPI;

impl GatewayAPI {
    pub async fn connect(http: HttpAPI, listeners: Arc<Mutex<Vec<Box<dyn Listener + Send>>>>) -> ! {
        let mut delayer = Delayer::new();
        let session_id = Rc::new(RefCell::new(None));
        let sequence_number = Arc::new(Mutex::new(None));

        loop {
            let execution: Result<(), Error> = try {
                let gateway_bot = http.gateway_bot().await?;

                ws::connect(gateway_bot.url.as_ref(), |client| {
                    GatewayHandler {
                        session: Session {
                            sender: client,
                            http: http.clone(),
                            listeners: listeners.clone()
                        },
                        session_id: Rc::clone(&session_id),
                        sequence_number: Arc::clone(&sequence_number),
                        heartbeat_confirmed: Arc::new(AtomicBool::new(true)),
                        heartbeat: None,
                    }
                })?;

                delayer.reset();
            };

            if let Err(err) = execution {
                error!("Connection was interrupted with '{}'", err.msg);
            } else {
                error!("Connection interrupted");
            }

            delayer.delay();

            if let Some(sid) = &*session_id.borrow() {
                info!("Attempting to resume session {} with sequence_number: {}", sid, sequence_number.lock().unwrap().unwrap());
            } else {
                info!("Attempting to reconnect");
            }
        }
    }
}

struct Delayer {
    delay: usize
}

impl Delayer {
    const DELAYS: [u64; 10] = [5, 5, 5, 15, 30, 60, 120, 120, 300, 600];

    pub fn new() -> Delayer {
        Delayer {
            delay: 0
        }
    }

    pub fn delay(&mut self) {
        thread::sleep(Duration::from_secs(Delayer::DELAYS[self.delay]));

        if self.delay < 9 {
            self.delay += 1;
        }
    }

    pub fn reset(&mut self) {
        self.delay = 0
    }
}

pub struct Session {
    sender: ws::Sender,
    http: HttpAPI,
    listeners: Arc<Mutex<Vec<Box<dyn Listener + Send>>>>
}

impl Session {
    #[inline]
    pub fn send<M: Into<ws::Message>>(&self, msg: M) -> Result<(), Error> {
        Ok(self.sender.send(msg)?)
    }
}

impl Deref for Session {
    type Target = HttpAPI;

    fn deref(&self) -> &Self::Target {
        &self.http
    }
}

struct GatewayHandler {
    session: Session,
    session_id: Rc<RefCell<Option<String>>>,
    sequence_number: Arc<Mutex<Option<i32>>>,
    heartbeat_confirmed: Arc<AtomicBool>,
    heartbeat: Option<mpsc::Sender<bool>>,
}

impl GatewayHandler {
    async fn dispatch_payload(&mut self, data: &str) -> Result<(), Error> {
        match json::json_root_search::<u8>("op", data)? {
            0 => self.dispatch_event(data).await?,
            9 => call_dispatcher!(data as Payload<InvalidSession> => self.on_invalid_session),
            10 => call_dispatcher!(data as Payload<Hello> => self.on_hello),
            11 => self.on_heartbeat_ack().await?,
            unknown_op => warn!("Received unknown opcode '{}': \n{}", unknown_op, data)
        }

        Ok(())
    }

    /// Takes a full payload, deserializes it and sends
    /// it to the right method.
    /// Returns an error when receiving an unknown event.
    #[allow(clippy::cognitive_complexity)]
    // Currently disabling cognitive complexity since clippy
    // expands the macros (bug) before calculating CoC.
    async fn dispatch_event(&mut self, data: &str) -> Result<(), Error> {
        match json::json_root_search::<String>("t", data)?.as_str() {
            ReadyDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ReadyDispatch> => self.on_ready),
            ResumedDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ResumedDispatch> => self.on_resumed),
            GuildCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildCreateDispatch> => self.on_guild_create),
            GuildUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildUpdateDispatch> => self.on_guild_update),
            PresencesReplaceDispatch::EVENT_NAME => info!("Ignoring presence replace event"),
            MessageCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageCreateDispatch> => self.on_message_create),
            MessageUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageUpdateDispatch> => self.on_message_update),
            MessageDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageDeleteDispatch> => self.on_message_delete),
            MessageDeleteBulkDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageDeleteBulkDispatch> => self.on_message_delete_bulk),
            MessageReactionAddDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionAddDispatch> => self.on_reaction_add),
            MessageReactionRemoveDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveDispatch> => self.on_reaction_remove),
            MessageReactionRemoveAllDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveAllDispatch> => self.on_reaction_remove_all),
            PresenceUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<PresenceUpdateDispatch> => self.on_presence_update),
            TypingStartDispatch::EVENT_NAME => call_dispatcher!(data as Payload<TypingStartDispatch> => self.on_typing_start),
            unknown_event => return Error::err(format!("Unknown event {}", unknown_event))
        }

        Ok(())
    }

    dispatcher!(on_guild_create: GuildCreateDispatch);
    dispatcher!(on_guild_update: GuildUpdateDispatch);
    dispatcher!(on_message_create: MessageCreateDispatch);
    dispatcher!(on_message_update: MessageUpdateDispatch);
    dispatcher!(on_message_delete: MessageDeleteDispatch);
    dispatcher!(on_message_delete_bulk: MessageDeleteBulkDispatch);
    dispatcher!(on_reaction_add: MessageReactionAddDispatch);
    dispatcher!(on_reaction_remove: MessageReactionRemoveDispatch);
    dispatcher!(on_reaction_remove_all: MessageReactionRemoveAllDispatch);
    dispatcher!(on_presence_update: PresenceUpdateDispatch);
    dispatcher!(on_typing_start: TypingStartDispatch);

    async fn on_hello(&mut self, payload: Hello) -> Result<(), Error> {
        println!("{:?}", payload);

        if let Some(sid) = &*self.session_id.borrow() {
            let resume = Resume {
                token: self.session.http.token().clone(),
                session_id: sid.to_owned(),
                seq: self.sequence_number.lock().unwrap().unwrap(),
            };

            self.session.send(resume)?;
            info!("Requested to resume session");
        } else {
            let identify = Identify {
                token: self.session.http.token().clone(),
                properties: map! {
                    "$os" => "linux",
                    "$browser" => "automate",
                    "$device" => "automate"
                },
                compress: None,
                large_threshold: None,
                shard: None,
                presence: None,
                guild_subscriptions: Some(true),
            };

            self.session.send(identify)?;
        }

        self.heartbeat = {
            let sequence_number = self.sequence_number.clone();
            let heartbeat_confirmed = self.heartbeat_confirmed.clone();
            let sender = self.session.sender.clone();

            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let rs: Result<(), Error> = try {
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

    async fn on_ready(&mut self, payload: ReadyDispatch) -> Result<(), Error> {
        println!("{:?}", payload);

        *self.session_id.borrow_mut() = Some(payload.session_id);
        Ok(())
    }

    async fn on_resumed(&mut self, _payload: ResumedDispatch) -> Result<(), Error> {
        info!("Successfully resumed session");
        Ok(())
    }

    async fn on_invalid_session(&mut self, payload: InvalidSession) -> Result<(), Error> {
        if !payload.0 {
            *self.session_id.borrow_mut() = None;

            warn!("Invalid session, shutting down connection");
            self.session.sender.shutdown()?;
        }

        Ok(())
    }

    async fn on_heartbeat_ack(&mut self) -> Result<(), Error> {
        self.heartbeat_confirmed.store(true, Ordering::Relaxed);

        info!("Received heartbeat acknowledgement");
        Ok(())
    }
}

impl ws::Handler for GatewayHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if let ws::Message::Text(data) = msg {
            if let Err(err) = executor::block_on(self.dispatch_payload(&data)) {
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