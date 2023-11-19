use crate::error::http_error::{HttpError, HttpErrorType};
use crate::info_request_log;
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};
use tracing::info;

pub struct JWTProcessing;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JWTProcessing
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTMiddleware { service }))
    }
}

pub struct JWTMiddleware<S> {
    /// The next service to call
    service: S,
}

/// https://github.com/Keats/jsonwebtoken#claims
/// TODO:
/// Add or remove fields according to your needs
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct JWTClaims {
    exp: usize,
    email: String,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for JWTMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info_request_log!();
        let token_result = extract_bearer_token(&req);
        let secret = match env::var("SUPABASE_JWT_SECRET") {
            Ok(s) => s,
            Err(_) => {
                return Box::pin(async {
                    Err(Error::from(HttpError {
                        cause: None,
                        message: Some("JWT secret not found".to_string()),
                        error_type: HttpErrorType::AuthError,
                    }))
                })
            }
        };
        let token = match token_result {
            Ok(t) => t,
            Err(e) => return Box::pin(async { Err(Error::from(HttpError::from(e))) }),
        };
        // JWTのデコードと検証
        match decode::<JWTClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS512), // 使用するアルゴリズムに応じて変更
        ) {
            Ok(c) => {
                info!("email: {}", c.claims.email);
                // TODO: リクエストに追加しているがmiddlewreで取得できない
                req.extensions_mut().insert(c.claims.email);
            }
            Err(err) => match *err.kind() {
                    ErrorKind::InvalidToken => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid token".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    ErrorKind::InvalidIssuer => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid issuer".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    // INFO: When I requested with a token which is encoded Base64, this error occurred.
                    ErrorKind::InvalidSignature => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid signature".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    ErrorKind::InvalidEcdsaKey => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid ecdsa key".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    ErrorKind::InvalidAudience => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid audience".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    ErrorKind::InvalidSubject => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid subject".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    _ => {
                        return Box::pin(async {Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Error occurred".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))})
                    }
                    // ErrorKind::InvalidRsaKey(String),
                    // ErrorKind::RsaFailedSigning,
                    // ErrorKind::InvalidAlgorithmName,
                    // ErrorKind::InvalidKeyFormat,
                    // ErrorKind::MissingRequiredClaim(String),
                    // ErrorKind::ExpiredSignature,
                    // ErrorKind::ImmatureSignature,
                    // ErrorKind::InvalidAlgorithm,
                    // ErrorKind::MissingAlgorithm,
                    // ErrorKind::Base64(base64::DecodeError),
                },
        };

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}

fn extract_bearer_token(req: &ServiceRequest) -> Result<String, HttpError> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.to_lowercase().starts_with("bearer ") {
                let token = auth_str["bearer ".len()..].trim(); // この行を修正
                return Ok(token.to_string());
            }
        }
    }
    Err(HttpError {
        cause: None,
        message: Some("No valid token found".to_string()),
        error_type: HttpErrorType::AuthError,
    })
}
