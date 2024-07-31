use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{error::Error, path::PathBuf};
use tokio::sync::mpsc::{Receiver, Sender};

pub mod driver;
pub mod kalosm;

pub struct Channel<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Chats {
    pub uids: Vec<String>,
}

#[derive(Debug)]
pub enum DriverError {
    ModelLoad(String),
    LoadHistory(String),
    Deserialize(String),
    Session(String),
}

impl Error for DriverError {}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelLoad(msg) => write!(f, "Error while loading the model {msg}"),
            Self::LoadHistory(msg) => write!(f, "Unable to load the history {msg}"),
            Self::Deserialize(msg) => write!(f, "Unable to deserialize message {msg}"),
            Self::Session(msg) => write!(f, "Error occurred with the session {msg}"),
        }
    }
}

#[async_trait]
pub trait DriverOperator: Send {
    /// Create a new driver
    async fn set_model(&mut self) -> Result<(), DriverError>;
    // Create a new chat and return an uid representing the id of the chat
    async fn new_chat(&self) -> Result<(Channel<String>, String), DriverError>;
    /// Load the history of the conversation
    async fn load_history(
        &self,
        path: &PathBuf,
    ) -> Result<HashMap<String, Channel<String>>, DriverError>;
}
