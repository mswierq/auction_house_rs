use tonic::{transport::Server, Request, Response, Status};

use client_session_proto::client_session_server::{ClientSession, ClientSessionServer};
use client_session_proto::{ChangePasswordRequest, LoginRequest, RegisterRequest, TokenResponse};
pub mod client_session_proto {
    tonic::include_proto!("auction_house_rs.session.client");
}

#[derive(Default)]
pub struct ClientSessionImpl;

#[tonic::async_trait]
impl ClientSession for ClientSessionImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let reply = TokenResponse {
            token: "dummy token".into(),
        };

        Ok(Response::new(reply))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let reply = TokenResponse {
            token: "dummy token".into(),
        };

        Ok(Response::new(reply))
    }

    async fn logout(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn delete_account(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let reply = TokenResponse {
            token: "dummy token".into(),
        };

        Ok(Response::new(reply))
    }

    async fn refresh_token(&self, request: Request<()>) -> Result<Response<TokenResponse>, Status> {
        let reply = TokenResponse {
            token: "dummy token".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let session = ClientSessionImpl::default();

    Server::builder()
        .add_service(ClientSessionServer::new(session))
        .serve(addr)
        .await?;

    Ok(())
}
