use backend_service::backend_proto::backend_server::BackendServer;
use backend_service::BackendService;
use tonic::transport::Server;
mod backend;
mod backend_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(BackendServer::new(BackendService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
