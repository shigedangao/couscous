use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::{path::PathBuf, sync::LazyLock};

static STORED_UID_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("./chats.json"));

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ChatsHistory {
    pub uids: Vec<String>,
}

impl ChatsHistory {
    pub async fn read_contents() -> Result<Self> {
        let content = match tokio::fs::read(&*STORED_UID_PATH).await {
            Ok(content) => content,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(Self::default());
                }

                return Err(anyhow!("Unable to load the history"));
            }
        };

        let chats: ChatsHistory = serde_json::from_slice(&content)?;

        Ok(chats)
    }
}
