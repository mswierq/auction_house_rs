use crate::client_session_service::create_client_session_service;
use crate::token_verifier_service::create_token_verifier_service;
use std::sync::Arc;
use token_engine::TokenBroker;
use tonic::transport::Server;
mod client_session_service;
mod token_engine;
mod token_verifier_service;
mod user_credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let tokens = Arc::new(TokenBroker::new());

    Server::builder()
        .add_service(create_client_session_service(tokens.clone()))
        .add_service(create_token_verifier_service(tokens.clone()))
        .serve(addr)
        .await?;

    Ok(())
}
