use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tonic::Status;
use uuid::Uuid;

mod session;

// Constant
const STORED_UID_PATH: &str = "chats";

#[derive(Serialize, Deserialize, Default, Clone)]
struct Chats {
    uids: Vec<String>,
}

struct Channel<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

#[derive(Default, Clone)]
pub struct Handler<T> {
    chats: Arc<Mutex<HashMap<String, Channel<T>>>>,
    stored_chats: Arc<Mutex<Chats>>,
    stored_chats_path: PathBuf,
}

impl Handler<String> {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            stored_chats_path: PathBuf::from(format!("./{}.json", STORED_UID_PATH)),
            ..Default::default()
        })
    }

    pub async fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading the existing chat");
        // Read the stored chats.json to restore the existing chats
        let content = match tokio::fs::read(&self.stored_chats_path).await {
            Ok(content) => content,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(());
                }

                return Err(err.into());
            }
        };

        let chats: Chats = serde_json::from_slice(&content)?;
        self.stored_chats = Arc::new(Mutex::new(chats.clone()));

        let mut set = JoinSet::new();

        for uid in chats.uids {
            let path = PathBuf::from(format!("./{}.llama", uid));

            set.spawn(async move {
                let (tx, rx, uid) = session::create_session(uid, Some(path));

                (tx, rx, uid)
            });
        }

        let mut chats = HashMap::new();
        while let Some(res) = set.join_next().await {
            let (tx, rx, uid) = res?;

            chats.insert(uid, Channel { tx, rx });
        }

        self.chats = Arc::new(Mutex::new(chats));

        Ok(())
    }

    pub async fn new_chat(&self) -> Result<String, Box<dyn std::error::Error>> {
        let uid = Uuid::new_v4().to_string();

        let channel = self.start_chat_session(uid.clone(), None).await;

        // Store the chat in the handler for later usage
        let handle = self.chats.clone();
        let mut lock = handle.lock().await;
        lock.insert(uid.clone(), channel);

        let sc_handle = self.stored_chats.clone();
        let mut sc_lock = sc_handle.lock().await;
        sc_lock.uids.push(uid.clone());

        tokio::fs::write(&self.stored_chats_path, serde_json::to_string(&*sc_lock)?).await?;

        Ok(uid)
    }

    async fn start_chat_session(
        &self,
        uid: String,
        existing_session: Option<PathBuf>,
    ) -> Channel<String> {
        let (tx_client, rx_chat, _) = session::create_session(uid, existing_session);

        Channel {
            tx: tx_client,
            rx: rx_chat,
        }
    }

    pub async fn send_message<T>(
        &self,
        uuid: &str,
        msg: &str,
        tx: Sender<Result<T, Status>>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::convert::From<String> + 'static,
    {
        // Load the chat given the uuid
        let handle = self.chats.clone();
        let mut lock = handle.lock().await;

        let Some(channel) = lock.get_mut(uuid) else {
            return Err("Unable to get the lock".into());
        };

        if let Err(err) = channel.tx.send(msg.to_string()).await {
            tx.send(Err(Status::internal(err.to_string()))).await?;
            tx.closed().await;
        };

        while let Some(tokens) = channel.rx.recv().await {
            match tokens.as_str() {
                // Close the channel sender when the end word identifier has been sent
                "<<end>>" => {
                    return Ok(());
                }
                _ => tx.send(Ok(T::from(tokens))).await?,
            };
        }

        Ok(())
    }
}
