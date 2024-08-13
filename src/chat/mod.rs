use crate::driver::{ops::Driver, ops::SupportedDriver, Channel, CHAT_END_SIGNAL};
use crate::env::Variables;
use anyhow::{anyhow, Result};
use history::ChatsHistory;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tonic::Status;

pub mod history;

// Constant
const STORED_UID_PATH: &str = "chats";

#[derive(Default)]
pub struct Handler<T>
where
    T: AsRef<str>,
{
    chats: Arc<Mutex<HashMap<String, Channel<T>>>>,
    stored_chats: Arc<Mutex<ChatsHistory>>,
    stored_chats_path: PathBuf,
    driver: Driver,
}

impl Handler<String> {
    pub async fn new(supported_driver: SupportedDriver, env: Option<Variables>) -> Result<Self> {
        let mut driver = Driver::default();
        driver.load_driver(supported_driver, env).await?;

        Ok(Self {
            driver,
            stored_chats_path: PathBuf::from(format!("./{}.json", STORED_UID_PATH)),
            ..Default::default()
        })
    }

    pub async fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading the existing chat");
        let driver = self.driver.get_driver();
        let chats = driver.load_history().await?;

        self.chats = Arc::new(Mutex::new(chats));

        Ok(())
    }

    pub async fn new_chat(&self) -> Result<String> {
        let driver = self.driver.get_driver();
        let (channel, uid) = driver.new_chat().await?;

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

    pub async fn send_message<T>(
        &self,
        uuid: &str,
        msg: &str,
        tx: Sender<Result<T, Status>>,
    ) -> Result<()>
    where
        T: std::convert::From<String> + 'static,
    {
        // Load the chat given the uuid
        let handle = self.chats.clone();
        let mut lock = handle.lock().await;

        let Some(channel) = lock.get_mut(uuid) else {
            return Err(anyhow!("Unable to get the lock"));
        };

        if let Err(err) = channel.tx.send(msg.to_string()).await {
            tx.send(Err(Status::internal(err.to_string())))
                .await
                .map_err(|err| anyhow!(err.to_string()))?;
            tx.closed().await;
        };

        while let Some(tokens) = channel.rx.recv().await {
            match tokens.as_str() {
                // Close the channel sender when the end word identifier has been sent
                CHAT_END_SIGNAL => {
                    return Ok(());
                }
                _ => tx
                    .send(Ok(T::from(tokens)))
                    .await
                    .map_err(|err| anyhow!(err.to_string()))?,
            };
        }

        Ok(())
    }
}
