use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone)]
pub enum TokenType {
    Refresh,
    Access,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
    pub token_type: TokenType,
    pub exp_date: usize,
}

impl TokenClaims {
    pub fn generate_access(user_id: i32) -> String {
        TokenClaims::generate_token(user_id, TokenType::Access)
    }

    pub fn generate_refresh(user_id: i32) -> String {
        TokenClaims::generate_token(user_id, TokenType::Refresh)
    }

    fn generate_token(user_id: i32, token_type: TokenType) -> String {
        let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set!")
                .as_bytes(),
        )
        .unwrap();
        let claims = TokenClaims {
            id: user_id,
            token_type: token_type.clone(),
            exp_date: match token_type {
                TokenType::Access => (Utc::now() + Duration::minutes(5)).timestamp() as usize,
                TokenType::Refresh => (Utc::now() + Duration::days(30)).timestamp() as usize,
            },
        };
        claims.sign_with_key(&jwt_secret).unwrap()
    }
}
