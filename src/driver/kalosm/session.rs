use super::{CHAT_END_SIGNAL, DEFAULT_CHANNEL_BUFFER};
use kalosm::language::*;
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub(crate) fn create_session(
    uid: String,
    existing_session: PathBuf,
    model: Llama,
) -> (Sender<String>, Receiver<String>, String) {
    let (tx_client, mut rx_client) = mpsc::channel::<String>(DEFAULT_CHANNEL_BUFFER);
    let (tx_chat, rx_chat) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);
    let path = PathBuf::from(format!("./{}.llama", uid));

    tokio::spawn(async move {
        let owned_chat_tx = tx_chat.clone();
        let mut chat_handler = Chat::builder(model)
            .with_system_prompt("L'assistant va parler comme un bon Français")
            .with_try_session_path(existing_session)
            .build();

        while let Some(msg) = rx_client.recv().await {
            let mut stream = chat_handler.add_message(&msg);

            while let Some(tokens) = stream.next().await {
                if let Err(err) = owned_chat_tx.send(tokens).await {
                    println!("Error while sending data {}", err);
                }
            }

            if let Err(err) = chat_handler.save_session(&path).await {
                println!("Unable to save the chat locally {}", err);
            };

            if let Err(err) = owned_chat_tx.send(CHAT_END_SIGNAL.to_string()).await {
                println!("Error while sending closed message {}", err);
            };
        }
    });

    (tx_client, rx_chat, uid)
}
