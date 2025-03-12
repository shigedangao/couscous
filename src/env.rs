use std::env;

#[derive(Debug, Clone)]
pub struct Variables {
    pub ollama_host: String,
    pub ollama_port: String,
    pub grpc_address: String,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            ollama_host: "http://localhost".to_string(),
            ollama_port: "11434".to_string(),
            grpc_address: "127.0.0.1:50051".to_string(),
        }
    }
}

pub(crate) fn load_env_variables() -> Variables {
    if let Err(err) = dotenvy::dotenv() {
        println!("Unable to load the env due to {}", err)
    }

    let mut var = Variables::default();

    for (key, value) in env::vars() {
        let key = key.to_lowercase();

        match key.as_str() {
            "ollama_host" => var.ollama_host = value,
            "ollama_port" => var.ollama_port = value,
            "grpc_server_address" => var.grpc_address = value,
            _ => {}
        }
    }

    var
}
