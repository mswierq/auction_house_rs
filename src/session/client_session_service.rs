use crate::token_engine::Engine;
use crate::users_storage::UsersStorage;
use client_session_proto::client_session_server::{ClientSession, ClientSessionServer};
use client_session_proto::{ChangePasswordRequest, LoginRequest, RegisterRequest, TokenResponse};
use tonic::{Request, Response, Status};
pub mod client_session_proto {
    tonic::include_proto!("auction_house_rs.session.client");
}

pub fn create_client_session_service() -> ClientSessionServer<ClientSessionImpl> {
    ClientSessionServer::new(ClientSessionImpl::new())
}

pub struct ClientSessionImpl {
    tokens: Engine,
    users: UsersStorage,
}

impl ClientSessionImpl {
    fn new() -> Self {
        Self {
            tokens: Engine::new(),
            users: UsersStorage::new(),
        }
    }
}

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
