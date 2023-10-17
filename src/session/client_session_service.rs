use crate::token_engine::Engine;
use crate::users_storage::UsersStorage;
use client_session_proto::client_session_server::{ClientSession, ClientSessionServer};
use client_session_proto::{ChangePasswordRequest, LoginRequest, RegisterRequest, TokenResponse};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
pub mod client_session_proto {
    tonic::include_proto!("auction_house_rs.session.client");
}

pub fn create_client_session_service() -> ClientSessionServer<ClientSessionImpl> {
    ClientSessionServer::new(ClientSessionImpl::new())
}

pub struct ClientSessionImpl {
    tokens: Engine,
    users: Arc<Mutex<UsersStorage>>,
}

const AUTH_HEADER: &str = "authorization";

impl ClientSessionImpl {
    fn new() -> Self {
        Self {
            tokens: Engine::new(),
            users: Arc::new(Mutex::new(UsersStorage::new())),
        }
    }

    fn get_token_response(&self, user: &str) -> Result<Response<TokenResponse>, Status> {
        if let Ok(token) = self.tokens.create_new_token(user) {
            Ok(Response::new(TokenResponse { token }))
        } else {
            Err(Status::new(
                tonic::Code::Internal,
                "Failed to create token".to_string(),
            ))
        }
    }

    // TODO turn this into a macro
    fn exec_authorized<TReq, TRsp, F>(
        &self,
        request: Request<TReq>,
        callback: F,
    ) -> Result<Response<TRsp>, Status>
    where
        F: Fn(Request<TReq>, &str, &str) -> Result<Response<TRsp>, Status>,
    {
        let auth_metadata = request.metadata().get(AUTH_HEADER);
        if let None = auth_metadata {
            return Err(Status::new(
                tonic::Code::Internal,
                "Failed to get token".to_string(),
            ));
        }
        let token = auth_metadata.unwrap().to_owned();
        let result = self.tokens.verify_token(&token.to_str().unwrap()[7..]); // skip "Bearer "
        if let Ok(user) = result {
            callback(request, &user, &token.to_str().unwrap())
        } else {
            Err(Status::new(
                tonic::Code::PermissionDenied,
                "Invalid token".to_string(),
            ))
        }
    }
}

#[tonic::async_trait]
impl ClientSession for ClientSessionImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let data = request.into_inner();
        {
            let mut users = self.users.lock().unwrap();
            if let Err(status) = users.add_user(&data.username, &data.password) {
                return Err(Status::new(tonic::Code::AlreadyExists, status.to_string()));
            }
        }
        self.get_token_response(&data.username)
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let data = request.into_inner();
        {
            let users = self.users.lock().unwrap();
            if let Err(status) = users.verify_user(&data.username, &data.password) {
                return Err(Status::new(
                    tonic::Code::PermissionDenied,
                    status.to_string(),
                ));
            }
        }
        self.get_token_response(&data.username)
    }

    async fn logout(&self, request: Request<()>) -> Result<Response<()>, Status> {
        // TODO: put a token into a blacklist
        Ok(Response::new(()))
    }

    async fn delete_account(&self, request: Request<()>) -> Result<Response<()>, Status> {
        self.exec_authorized(request, |_, user, _| {
            let mut users = self.users.lock().unwrap();
            users.remove_user(&user);
            Ok(Response::new(()))
        })
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        self.exec_authorized(request, |request, user, _| {
            let data = request.into_inner();
            let mut users = self.users.lock().unwrap();
            if let Err(status) = users.verify_user(&user, &data.old_password) {
                return Err(Status::new(
                    tonic::Code::PermissionDenied,
                    status.to_string(),
                ));
            }
            if let Err(status) = users.update_user(&user, &data.new_password) {
                return Err(Status::new(tonic::Code::Internal, status.to_string()));
            }
            // TODO: invalidate the old token!
            self.get_token_response(&user) // generate new token
        })
    }

    async fn refresh_token(&self, request: Request<()>) -> Result<Response<TokenResponse>, Status> {
        self.exec_authorized(request, |request, user, _token| {
            // TODO: invalidate the old token!
            self.get_token_response(&user)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use client_session_proto::client_session_server::ClientSession;
    #[tokio::test]
    async fn test_register() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let response = service.register(request).await.unwrap();
        assert!(service
            .tokens
            .verify_token(&response.into_inner().token)
            .is_ok());
    }

    #[tokio::test]
    async fn test_try_register_twice() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        assert!(service.register(request).await.is_err());
    }

    #[tokio::test]
    async fn test_login() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let response = service.login(request).await.unwrap();
        assert!(service
            .tokens
            .verify_token(&response.into_inner().token)
            .is_ok());
    }

    #[tokio::test]
    async fn test_remove_user() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let token_resp = service.login(request).await.unwrap();
        let mut request = tonic::Request::new(());
        request.metadata_mut().append(
            AUTH_HEADER,
            format!("Bearer {}", token_resp.into_inner().token)
                .parse()
                .unwrap(),
        );
        let _ = service.delete_account(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        });
        assert!(service.login(request).await.is_err());
    }

    #[tokio::test]
    async fn test_try_remove_user_without_login() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let mut request = tonic::Request::new(());
        request
            .metadata_mut()
            .append(AUTH_HEADER, "Bearer invalid token".parse().unwrap());
        assert!(service.delete_account(request).await.is_err());
    }

    #[tokio::test]
    async fn test_change_password() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let token_resp = service.login(request).await.unwrap();
        let mut request = tonic::Request::new(ChangePasswordRequest {
            old_password: "password".into(),
            new_password: "new password".into(),
        });
        request.metadata_mut().append(
            AUTH_HEADER,
            format!("Bearer {}", token_resp.into_inner().token)
                .parse()
                .unwrap(),
        );
        let _ = service.change_password(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "new password".into(),
        });
        let response = service.login(request).await.unwrap();
        assert!(service
            .tokens
            .verify_token(&response.into_inner().token)
            .is_ok());
    }

    #[tokio::test]
    async fn test_try_change_password_without_login() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let mut request = tonic::Request::new(ChangePasswordRequest {
            old_password: "password".into(),
            new_password: "new password".into(),
        });
        request
            .metadata_mut()
            .append(AUTH_HEADER, "Bearer invalid token".parse().unwrap());
        assert!(service.change_password(request).await.is_err());
    }

    #[tokio::test]
    async fn test_login_after_changing_password() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let token_resp = service.login(request).await.unwrap();
        let mut request = tonic::Request::new(ChangePasswordRequest {
            old_password: "password".into(),
            new_password: "new password".into(),
        });
        request.metadata_mut().append(
            AUTH_HEADER,
            format!("Bearer {}", token_resp.into_inner().token)
                .parse()
                .unwrap(),
        );
        let _ = service.change_password(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "new password".into(),
        });
        assert!(service.login(request).await.is_ok());
    }

    #[tokio::test]
    async fn test_refresh_token_after_login() {
        let service = ClientSessionImpl::new();
        let request = tonic::Request::new(RegisterRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let _ = service.register(request).await.unwrap();
        let request = tonic::Request::new(LoginRequest {
            username: "user".into(),
            password: "password".into(),
        });
        let token_resp = service.login(request).await.unwrap();
        let mut request = tonic::Request::new(());
        request.metadata_mut().append(
            AUTH_HEADER,
            format!("Bearer {}", token_resp.into_inner().token)
                .parse()
                .unwrap(),
        );
        let new_token_resp = service.refresh_token(request).await.unwrap();
        assert!(service
            .tokens
            .verify_token(&new_token_resp.into_inner().token)
            .is_ok());
    }

    #[tokio::test]
    async fn test_try_refresh_token_without_login() {
        let service = ClientSessionImpl::new();
        let mut request = tonic::Request::new(());
        request
            .metadata_mut()
            .append(AUTH_HEADER, "Bearer invalid token".parse().unwrap());
        assert!(service.refresh_token(request).await.is_err());
    }
}
