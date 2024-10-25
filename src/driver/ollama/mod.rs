use super::{Channel, DriverError, DriverOperator};
use crate::chat::history::ChatsHistory;
use crate::env::Variables;
use ::async_trait::async_trait;
use ::ollama_rs::Ollama as OllamaHandler;
use ::uuid::Uuid;
use std::collections::HashMap;

mod session;

// Constant
const LLAMA_MODEL: &str = "llama3.1:latest";

#[derive(Clone, Default)]
pub struct Ollama {
    model: OllamaHandler,
}

#[async_trait]
impl DriverOperator for Ollama {
    async fn set_model(&mut self, env: Option<Variables>) -> Result<(), DriverError> {
        let model = match env {
            Some(var) => OllamaHandler::new(
                var.ollama_host,
                var.ollama_port
                    .parse::<u16>()
                    .expect("Expect port to be a u16"),
            ),
            None => OllamaHandler::new("127.0.0.1".to_string(), 11434),
        };

        self.model = model;

        Ok(())
    }

    async fn new_chat(&self) -> Result<(Channel<String>, String), DriverError> {
        let uid = Uuid::new_v4().to_string();
        let (tx, rx) =
            session::handle_session(uid.clone(), self.model.clone(), LLAMA_MODEL.to_string());

        Ok((Channel { tx, rx }, uid))
    }

    async fn load_history(&self) -> Result<HashMap<String, Channel<String>>, DriverError> {
        let chats = ChatsHistory::read_contents()
            .await
            .map_err(|err| DriverError::Deserialize(err.to_string()))?;

        let mut cons = HashMap::new();

        for uid in chats.uids {
            let (tx, rx) =
                session::handle_session(uid.clone(), self.model.clone(), LLAMA_MODEL.to_string());

            cons.insert(uid, Channel { tx, rx });
        }

        Ok(cons)
    }
}
