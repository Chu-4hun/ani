use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

use crate::token::{TokenClaims, TokenType};

pub async fn validator_refresh(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    validator(req, credentials, TokenType::Refresh).await
}
pub async fn validator_acces(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    validator(req, credentials, TokenType::Access).await
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
    token_type: TokenType,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            if value.token_type != token_type {
                return Err((
                    AuthenticationError::from(bearer::Config::default()).into(),
                    req,
                ));
            }
            if value.exp_date < Utc::now().timestamp() as usize {
                return Err((
                    AuthenticationError::from(bearer::Config::default()).into(),
                    req,
                ));
            }
            req.extensions_mut().insert(value);

            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
