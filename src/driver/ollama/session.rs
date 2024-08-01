use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama as OllamaHandler,
};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_stream::StreamExt;

pub(crate) fn handle_session(
    uid: String,
    model: OllamaHandler,
    model_name: String,
) -> (Sender<String>, Receiver<String>) {
    let (tx_client, mut rx_client) = mpsc::channel::<String>(1000);
    let (tx_chat, rx_chat) = mpsc::channel(1000);

    let mut model = model.clone();
    tokio::spawn(async move {
        let owned_tx_chat = tx_chat.clone();
        while let Some(msg) = rx_client.recv().await {
            let stream_res = model
                .send_chat_messages_with_history_stream(
                    ChatMessageRequest::new(
                        model_name.clone(),
                        vec![ChatMessage::user(msg.clone())],
                    ),
                    uid.to_owned(),
                )
                .await;

            let mut stream: ChatMessageResponseStream = match stream_res {
                Ok(res) => res,
                Err(err) => {
                    if let Err(err) = owned_tx_chat.send(err.to_string()).await {
                        println!("Error while sending error message {}", err.to_string());
                    };

                    return;
                }
            };

            while let Some(Ok(msg)) = stream.next().await {
                if let Some(chat) = msg.message {
                    if let Err(err) = owned_tx_chat.send(chat.content).await {
                        println!("Unable to send message due to error {}", err.to_string());
                    };
                }

                if msg.done {
                    if let Err(err) = owned_tx_chat.send("<<end>>".to_string()).await {
                        println!(
                            "Unable to send closing message due to error {}",
                            err.to_string()
                        );
                    };
                }
            }
        }
    });

    (tx_client, rx_chat)
}
