use std::env;

#[derive(Debug, Default, Clone)]
pub struct Variables {
    pub ollama_host: String,
    pub ollama_port: String,
    pub grpc_address: String,
}

pub(crate) fn load_env_variables() -> Option<Variables> {
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

    Some(var)
}
