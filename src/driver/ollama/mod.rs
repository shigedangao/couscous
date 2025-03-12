use super::{Channel, DriverError, DriverOperator};
use crate::env::Variables;
use ::async_trait::async_trait;
use ::ollama_rs::Ollama as OllamaHandler;
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
    async fn set_model(&mut self, env: Variables) -> Result<(), DriverError> {
        let model = OllamaHandler::new(
            env.ollama_host,
            env.ollama_port.parse::<u16>().unwrap_or(11434),
        );

        self.model = model;

        Ok(())
    }

    async fn new_chat(&self) -> Result<(Channel<String>, String), DriverError> {
        let (tx, rx) = session::handle_session(self.model.clone(), LLAMA_MODEL.to_string());

        // Dummy value for the uid set here.
        Ok((Channel { tx, rx }, "1".to_string()))
    }

    async fn load_history(&self) -> Result<HashMap<String, Channel<String>>, DriverError> {
        unimplemented!();
    }
}
