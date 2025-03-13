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

    let mut chat_handler = model.chat();
    if let Some(old_session) = ::std::fs::read(&existing_session)
        .ok()
        .and_then(|b| LlamaChatSession::from_bytes(&b).ok())
    {
        chat_handler = chat_handler.with_session(old_session);
    }

    tokio::spawn(async move {
        let owned_chat_tx = tx_chat.clone();

        while let Some(msg) = rx_client.recv().await {
            let mut stream = chat_handler.add_message(&msg);

            while let Some(tokens) = stream.next().await {
                if let Err(err) = owned_chat_tx.send(tokens).await {
                    println!("Error while sending data {}", err);
                }
            }

            if let Ok(session) = chat_handler.session() {
                if let Err(err) = session
                    .to_bytes()
                    .map(|bytes| std::fs::write(&path, bytes))
                    .map_err(|err| dbg!(err))
                {
                    dbg!(err);
                }
            }

            if let Err(err) = owned_chat_tx.send(CHAT_END_SIGNAL.to_string()).await {
                println!("Error while sending closed message {}", err);
            };
        }
    });

    (tx_client, rx_chat, uid)
}
