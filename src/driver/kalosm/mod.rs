use super::DriverOperator;
use super::*;
use crate::chat::history::ChatsHistory;
use crate::env::Variables;
use ::kalosm::language::Llama;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

mod session;

#[derive(Clone, Default)]
pub struct Kalosm {
    model: Option<Llama>,
}

#[async_trait]
impl DriverOperator for Kalosm {
    async fn set_model(&mut self, _: Option<Variables>) -> Result<(), DriverError> {
        let model = Llama::new_chat()
            .await
            .map_err(|err| DriverError::ModelLoad(err.to_string()))?;

        self.model = Some(model);

        Ok(())
    }

    async fn new_chat(&self) -> Result<(Channel<String>, String), DriverError> {
        let uid = Uuid::new_v4().to_string();
        let Some(model) = self.model.as_ref() else {
            return Err(DriverError::ModelLoad(
                "Unable to load the llama model".to_string(),
            ));
        };

        let path = PathBuf::from(format!("./{}.llama", uid));
        let (tx_client, rx_chat, _) = session::create_session(uid.clone(), path, model.clone());

        Ok((
            Channel {
                tx: tx_client,
                rx: rx_chat,
            },
            uid,
        ))
    }

    async fn load_history(&self) -> Result<HashMap<String, Channel<String>>, DriverError> {
        let chats = ChatsHistory::read_contents()
            .await
            .map_err(|err| DriverError::Deserialize(err.to_string()))?;

        let model = self.model.as_ref().unwrap();
        let mut cons = HashMap::new();
        for uid in chats.uids {
            let model = model.clone();
            let path = PathBuf::from(format!("./{}.llama", uid));

            let (tx, rx, uid) = session::create_session(uid, path, model);

            cons.insert(uid, Channel { tx, rx });
        }

        Ok(cons)
    }
}
