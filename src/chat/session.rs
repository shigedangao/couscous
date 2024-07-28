use kalosm::language::*;
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub(crate) fn create_session(
    mut model: Llama,
    uid: String,
    existing_session: Option<PathBuf>,
) -> (Sender<String>, Receiver<String>, String) {
    let (tx_client, mut rx_client) = mpsc::channel(1000);
    let (tx_chat, rx_chat) = mpsc::channel(1000);
    let path = PathBuf::from(format!("./{}.llama", uid));

    tokio::spawn(async move {
        let owned_chat_tx = tx_chat.clone();
        let mut chat = Chat::builder(&mut model).with_system_prompt("What can i do for you ?");

        if let Some(session_path) = existing_session {
            chat = chat.with_session_path(session_path).unwrap();
        }

        let mut chat_handler = chat.build();

        while let Some(msg) = rx_client.recv().await {
            let Ok(payload) = chat_handler.add_message(msg).await else {
                if let Err(err) = owned_chat_tx
                    .send("Unable to process message".to_string())
                    .await
                {
                    println!("Unable to send message due to {}", err.to_string())
                }

                return;
            };

            let text = payload.all_text().await;
            if let Err(err) = owned_chat_tx.send(text).await {
                println!("Unable to send message due to {}", err.to_string())
            };

            if let Err(err) = chat_handler.save_session(&path).await {
                println!("Unable to save the chat locally {}", err.to_string());
            };
        }
    });

    (tx_client, rx_chat, uid)
}
