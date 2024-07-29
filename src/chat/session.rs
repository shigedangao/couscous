use kalosm::language::*;
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub(crate) fn create_session(
    uid: String,
    existing_session: Option<PathBuf>,
) -> (Sender<String>, Receiver<String>, String) {
    let (tx_client, mut rx_client) = mpsc::channel::<String>(1000);
    let (tx_chat, rx_chat) = mpsc::channel(1000);
    let path = PathBuf::from(format!("./{}.llama", uid));

    tokio::spawn(async move {
        let model = Llama::new_chat().await.unwrap();
        let owned_chat_tx = tx_chat.clone();
        let mut chat =
            Chat::builder(model).with_system_prompt("The assistant will act like a pirate");

        if let Some(session_path) = existing_session {
            chat = chat.with_try_session_path(session_path);
        }

        let mut chat_handler = chat.build();

        while let Some(msg) = rx_client.recv().await {
            let mut stream = chat_handler.add_message(&msg);

            while let Some(tokens) = stream.next().await {
                if let Err(err) = owned_chat_tx.send(tokens).await {
                    println!("Error while sending data {}", err.to_string());
                }
            }

            if let Err(err) = chat_handler.save_session(&path).await {
                println!("Unable to save the chat locally {}", err.to_string());
            };

            if let Err(err) = owned_chat_tx.send("<<end>>".to_string()).await {
                println!("Error while sending closed message {}", err.to_string());
            };
        }
    });

    (tx_client, rx_chat, uid)
}
