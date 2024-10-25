use chat::Handler;
use server::include::couscous::couscous_server::CouscousServer;
use server::include::couscous::FILE_DESCRIPTOR_SET;
use server::Rpc;
use std::sync::Arc;
use std::sync::LazyLock;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::Any;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

mod chat;
mod driver;
mod env;
mod server;

static GRPC_WEB_CORS: LazyLock<CorsLayer> = LazyLock::new(|| {
    CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any())
        .allow_methods(AllowMethods::any())
        .expose_headers(Any)
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "ollama")]
    let driver = driver::ops::SupportedDriver::Ollama;

    #[cfg(not(feature = "ollama"))]
    let driver = driver::ops::SupportedDriver::Kalosm;

    // Read environment variables
    let env_var = env::load_env_variables();

    let mut chat_handler: Handler<String> = Handler::new(driver, env_var.clone()).await?;
    chat_handler.load().await?;
    println!("Loading existing chat is finished");

    // Initialize the server
    let default_addr = "127.0.0.1:10001".parse()?;
    let addr = match env_var {
        Some(env) => env.grpc_address.parse().unwrap_or(default_addr),
        None => default_addr,
    };

    let srv = Rpc {
        chat: Arc::new(chat_handler),
    };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

    println!("Running server on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(GRPC_WEB_CORS.clone())
        .layer(GrpcWebLayer::new())
        .add_service(reflection_service)
        .add_service(CouscousServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}
