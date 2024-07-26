use kalosm::language::{Chat, TextStream};
use kalosm_llama::{Llama, LlamaSource};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Sender, Receiver};
use uuid::Uuid;

struct Channel<T> {
    tx: Sender<T>,
    rx: Receiver<T>
}

#[derive(Default)]
pub struct Handler<T> {
    chats: Arc<Mutex<HashMap<String, Channel<T>>>>,
}

impl Handler<String> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::default())
    }

    #[allow(dead_code)]
    async fn load() {
        unimplemented!()
    }

    pub async fn new_chat(&self) -> Result<String, Box<dyn std::error::Error>> {
        let uid = Uuid::new_v4().to_string();
        let model = Llama::builder()
            .with_source(LlamaSource::llama_7b_chat())
            .build()?;

        let channel = self.start_chat_session(model).await;

        // Store the chat in the handler for later usage
        let handle = self.chats.clone();
        let mut lock = handle.lock().await;
        lock.insert(uid.clone(), channel);

        Ok(uid)
    }

    async fn start_chat_session(&self, mut model: Llama) -> Channel<String> {
        let (tx_client, mut rx_client) = mpsc::channel(1000);
        let (tx_chat, rx_chat) = mpsc::channel(1000);

        tokio::spawn(async move {
            let owned_chat_tx = tx_chat.clone();
            let mut chat = Chat::builder(&mut model)
                .with_system_prompt("What can i do for you ?")
                .build();

            while let Some(msg) = rx_client.recv().await {
                println!("message received {msg}");
                let Ok(payload) = chat.add_message(msg).await else {
                    owned_chat_tx.send("Unable to process message".to_string()).await.unwrap();

                    return;
                };

                println!("message processed");
                let text = payload.all_text().await;
                println!("message sent");
                owned_chat_tx.send(text).await.unwrap();
            }
        });

        Channel { tx: tx_client, rx: rx_chat }
    }

    pub async fn send_message(
        &self,
        uuid: &str,
        msg: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Load the chat given the uuid
        let handle = self.chats.clone();
        let mut lock = handle.lock().await;

        let Some(channel) = lock.get_mut(uuid) else {
            return Err("Unable to get the lock".into())
        };

        channel.tx.send(msg.to_owned()).await?;

        let Some(msg) = channel.rx.recv().await else {
            return Err("Unable to get message from channel".into())
        };

        println!("received !");

        Ok(msg)
    }
}
