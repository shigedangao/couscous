use crate::env::Variables;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::mpsc::{Receiver, Sender};

pub mod kalosm;
#[cfg(feature = "ollama")]
pub mod ollama;
pub mod ops;

pub struct Channel<T>
where
    T: AsRef<str>,
{
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

#[derive(Debug)]
pub enum DriverError {
    ModelLoad(String),
    Deserialize(String),
}

impl Error for DriverError {}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelLoad(msg) => write!(f, "Error while loading the model {msg}"),
            Self::Deserialize(msg) => write!(f, "Unable to deserialize message {msg}"),
        }
    }
}

#[async_trait]
pub trait DriverOperator: Send {
    // Create a new chat and return an uid representing the id of the chat
    async fn new_chat(&self) -> Result<(Channel<String>, String), DriverError>;
    /// Create a new driver
    async fn set_model(&mut self, env: Option<Variables>) -> Result<(), DriverError>;
    /// Load the history of the conversation
    async fn load_history(&self) -> Result<HashMap<String, Channel<String>>, DriverError>;
}
