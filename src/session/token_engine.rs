use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct TokenBroker {
    key: Hmac<Sha256>,
}

type TokenType<S> = Token<Header, BTreeMap<String, String>, S>;

impl TokenBroker {
    pub fn new() -> Self {
        Self {
            key: Hmac::new_from_slice(b"secret").unwrap(), // TODO: read a key from file
        }
    }

    pub fn create_new_token(&self, user: &str) -> Result<String, Box<dyn std::error::Error>> {
        let header = Header {
            algorithm: AlgorithmType::Hs256,
            ..Default::default()
        };
        let mut claims = BTreeMap::new();
        claims.insert("user", user);
        let token = Token::new(header, claims).sign_with_key(&self.key)?;
        Ok(token.as_str().to_owned())
    }

    pub fn verify_token(&self, token_str: &str) -> Result<String, Box<dyn std::error::Error>> {
        let result: Result<TokenType<_>, _> = token_str.verify_with_key(&self.key);
        if let Ok(token) = result {
            if let Some(user) = token.claims().get("user") {
                Ok(user.to_owned())
            } else {
                Err("Invalid token - user claim not found".into())
            }
        } else {
            Err("Invalid token - verification has failed".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_new_token() {
        let engine = TokenBroker::new();
        let token_str = engine.create_new_token("user").unwrap();
        let token: TokenType<_> = token_str.verify_with_key(&engine.key).unwrap();
        assert_eq!(token.claims().get("user").unwrap(), "user");
    }

    #[test]
    fn test_verify_token() {
        let engine = TokenBroker::new();
        let token_str = engine.create_new_token("user").unwrap();
        assert_eq!(engine.verify_token(&token_str).unwrap(), "user");
    }

    #[test]
    fn test_verify_invalid_token() {
        let engine = TokenBroker::new();
        let token = "invalid token".to_owned();
        assert!(engine.verify_token(&token).is_err());
    }
}
