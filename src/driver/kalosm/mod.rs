use super::*;
use ::kalosm::language::Llama;
use async_trait::async_trait;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::task::JoinSet;
use uuid::Uuid;

mod session;

#[derive(Clone, Default)]
pub struct Kalosm {
    model: Option<Llama>,
}

#[async_trait]
impl Driver for Kalosm {
    async fn set_model(&mut self) -> Result<(), DriverError> {
        let model = Llama::new_chat()
            .await
            .map_err(|err| DriverError::ModelLoad(err.to_string()))?;

        self.model = Some(model);

        Ok(())
    }

    async fn new_chat(&self) -> Result<(Channel<String>, String), DriverError> {
        let uid = Uuid::new_v4().to_string();
        let model = self.model.as_ref().unwrap();
        let (tx_client, rx_chat, _) = session::create_session(uid.clone(), None, model.clone());

        Ok((
            Channel {
                tx: tx_client,
                rx: rx_chat,
            },
            uid,
        ))
    }

    async fn load_history(
        &self,
        path: &PathBuf,
    ) -> Result<HashMap<String, Channel<String>>, DriverError> {
        let content = match tokio::fs::read(&path).await {
            Ok(content) => content,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(HashMap::new());
                }

                return Err(DriverError::LoadHistory(err.to_string()));
            }
        };

        let chats: Chats = serde_json::from_slice(&content)
            .map_err(|err| DriverError::Deserialize(err.to_string()))?;

        let model = self.model.as_ref().unwrap();
        let mut set = JoinSet::new();
        for uid in chats.uids {
            let model = model.clone();
            let path = PathBuf::from(format!("./{}.llama", uid));

            set.spawn(async move {
                let (tx, rx, uid) = session::create_session(uid, Some(path), model);

                (tx, rx, uid)
            });
        }

        let mut chats = HashMap::new();
        while let Some(res) = set.join_next().await {
            let (tx, rx, uid) = res.map_err(|err| DriverError::Session(err.to_string()))?;

            chats.insert(uid, Channel { tx, rx });
        }

        Ok(chats)
    }
}
