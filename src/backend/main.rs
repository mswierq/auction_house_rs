use crate::backend_service::DefaultBackendService;
use backend_service::backend_proto::backend_server::BackendServer;
use tonic::transport::Server;

mod backend;
mod backend_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(BackendServer::new(DefaultBackendService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
