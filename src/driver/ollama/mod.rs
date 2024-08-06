use super::{Channel, DriverError, DriverOperator};
use crate::chat::history::ChatsHistory;
use ::async_trait::async_trait;
use ::ollama_rs::Ollama as OllamaHandler;
use ::uuid::Uuid;
use std::collections::HashMap;

mod session;

// Constant
const LLAMA_MODEL: &str = "llama3.1:latest";
const HISTORY: u16 = 100;

#[derive(Clone, Default)]
pub struct Ollama {
    model: OllamaHandler,
}

#[async_trait]
impl DriverOperator for Ollama {
    async fn set_model(&mut self) -> Result<(), DriverError> {
        let model = OllamaHandler::new_default_with_history(HISTORY);

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
