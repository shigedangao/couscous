use std::env;

#[derive(Debug, Default)]
pub struct Variables {
    pub ollama_host: String,
    pub ollama_port: String,
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
            _ => {}
        }
    }

    Some(var)
}
