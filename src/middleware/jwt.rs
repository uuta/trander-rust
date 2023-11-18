use crate::error::http_error::{HttpError, HttpErrorType};
use crate::info_request_log;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
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
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JWTClaims {
    sub: String,
    company: String,
    exp: usize,
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
        let fut = self.service.call(req);
        Box::pin(async move {
            info_request_log!();
            let token = match token_result {
                Ok(t) => t,
                Err(e) => return Err(Error::from(HttpError::from(e))),
            };

            let validation = Validation::new(Algorithm::HS256); // 使用するアルゴリズムに応じて変更

            // JWTのデコードと検証
            match decode::<JWTClaims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &validation,
            ) {
                Ok(c) => c,
                Err(_) => {
                    return Err(Error::from(HttpError {
                        cause: None,
                        message: Some("No valid token found".to_string()),
                        error_type: HttpErrorType::AuthError,
                    }))
                }
            };
            fut.await
        })
    }
}

fn extract_bearer_token(req: &ServiceRequest) -> Result<String, HttpError> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str["Bearer ".len()..];
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
