use chat::Handler;
use server::include::couscous::couscous_server::CouscousServer;
use server::include::couscous::FILE_DESCRIPTOR_SET;
use server::Rpc;
use std::sync::Arc;
use tonic::transport::Server;

mod chat;
mod driver;
mod env;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "ollama")]
    let driver = driver::ops::SupportedDriver::Ollama;

    #[cfg(not(feature = "ollama"))]
    let driver = driver::ops::SupportedDriver::Kalosm;

    // Read environment variables
    let env_var = env::load_env_variables();

    let mut chat_handler: Handler<String> = Handler::new(driver, env_var).await?;
    chat_handler.load().await?;
    println!("Loading existing chat is finished");

    // Initialize the server
    let addr = "127.0.0.1:50051".parse()?;
    let srv = Rpc {
        chat: Arc::new(chat_handler),
    };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(CouscousServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}
