use crate::client_session_service::create_client_session_service;
use tonic::transport::Server;
mod client_session_service;
mod token_engine;
mod users_storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(create_client_session_service())
        .serve(addr)
        .await?;

    Ok(())
}
