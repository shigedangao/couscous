use crate::driver::{CHAT_END_SIGNAL, DEFAULT_CHANNEL_BUFFER};
use ollama_rs::{
    Ollama as OllamaHandler,
    generation::chat::{ChatMessage, ChatMessageResponseStream, request::ChatMessageRequest},
};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_stream::StreamExt;

pub(crate) fn handle_session(
    model: OllamaHandler,
    model_name: String,
) -> (Sender<String>, Receiver<String>) {
    let (tx_client, mut rx_client) = mpsc::channel::<String>(DEFAULT_CHANNEL_BUFFER);
    let (tx_chat, rx_chat) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

    let model = model.clone();
    tokio::spawn(async move {
        let owned_tx_chat = tx_chat.clone();
        while let Some(msg) = rx_client.recv().await {
            let stream_res = model
                .send_chat_messages_stream(ChatMessageRequest::new(
                    model_name.clone(),
                    vec![ChatMessage::user(msg.clone())],
                ))
                .await;

            let mut stream: ChatMessageResponseStream = match stream_res {
                Ok(res) => res,
                Err(err) => {
                    if let Err(err) = owned_tx_chat.send(CHAT_END_SIGNAL.to_string()).await {
                        println!("Error while sending error message {}", err);
                    };

                    println!("Unable to continue to parse message due to error {err}");

                    return;
                }
            };

            while let Some(msg) = stream.next().await {
                let msg = match msg {
                    Ok(msg) => msg,
                    Err(_) => {
                        if let Err(err) = owned_tx_chat.send(CHAT_END_SIGNAL.to_owned()).await {
                            println!("Unable to send message due to error {}", err);
                        }

                        continue;
                    }
                };

                if let Err(err) = owned_tx_chat.send(msg.message.content).await {
                    println!("Unable to send message due to error {}", err);
                };

                if msg.done {
                    if let Err(err) = owned_tx_chat.send(CHAT_END_SIGNAL.to_string()).await {
                        println!("Unable to send closing message due to error {}", err);
                    };
                }
            }
        }
    });

    (tx_client, rx_chat)
}
