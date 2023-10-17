use crate::token_engine::TokenBroker;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use verifier_proto::token_verifier_server::{TokenVerifier, TokenVerifierServer};
use verifier_proto::TokenRequest;

pub mod verifier_proto {
    tonic::include_proto!("auction_house_rs.session.token_verifier");
}

pub fn create_token_verifier_service(
    tokens: Arc<TokenBroker>,
) -> TokenVerifierServer<TokenVerifierService> {
    TokenVerifierServer::new(TokenVerifierService::new(tokens))
}

pub struct TokenVerifierService {
    tokens: Arc<TokenBroker>,
}

impl TokenVerifierService {
    pub fn new(tokens: Arc<TokenBroker>) -> Self {
        Self { tokens }
    }
}

#[tonic::async_trait]
impl TokenVerifier for TokenVerifierService {
    async fn verify_token(&self, request: Request<TokenRequest>) -> Result<Response<()>, Status> {
        let token = request.into_inner().token;
        if let Err(_) = self.tokens.verify_token(&token) {
            return Err(Status::new(
                tonic::Code::PermissionDenied,
                "Invalid token".to_string(),
            ));
        }
        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_verify_token() {
        let tokens = Arc::new(TokenBroker::new());
        let service = TokenVerifierService::new(tokens.clone());
        let token_str = tokens.create_new_token("user").unwrap();
        let request = Request::new(TokenRequest { token: token_str });
        assert!(service.verify_token(request).await.is_ok());
    }

    #[tokio::test]
    async fn test_verify_invalid_token() {
        let tokens = Arc::new(TokenBroker::new());
        let service = TokenVerifierService::new(tokens.clone());
        let request = Request::new(TokenRequest {
            token: "invalid token".to_string(),
        });
        assert!(service.verify_token(request).await.is_err());
    }
}
