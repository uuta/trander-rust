use crate::db;
use crate::error::http_error::{HttpError, HttpErrorType};
use crate::info_request_log;
use crate::repository::users::{ImplUsersRepository, UsersRepository};
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
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
    aud: String,
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

        // Change algorithm as you like
        let mut validation = Validation::new(Algorithm::HS256);
        let mut aud_set = HashSet::new();
        let aud = match env::var("SUPABASE_JWT_AUD") {
            Ok(s) => s,
            Err(e) => return Box::pin(async { Err(Error::from(HttpError::from(e))) }),
        };
        aud_set.insert(aud);
        validation.aud = Some(aud_set);

        // Decode JWT token and verify it
        match decode::<JWTClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => {
                if let Some(db) = req.request().app_data::<Data<db::DbPool>>() {
                    if let Ok(mut conn) = db.get().map_err(|e| HttpError::from(e)) {
                        let email = c.claims.email.clone();
                        let users_repo = ImplUsersRepository;
                        let user_res = users_repo.get_by_email(&mut conn, &email);
                        let user = match user_res {
                            Ok(s) => s,
                            Err(e) => match e {
                                // Register a new user if the user is not found
                                diesel::NotFound => {
                                    let user_add_res = users_repo
                                        .add(&mut conn, &email)
                                        .map_err(|e| HttpError::from(e));
                                    match user_add_res {
                                        Ok(_s) => {
                                            let user_res = users_repo
                                                .get_by_email(&mut conn, &email)
                                                .map_err(|e| HttpError::from(e));
                                            match user_res {
                                                Ok(s) => s,
                                                Err(e) => {
                                                    return Box::pin(async {
                                                        Err(Error::from(HttpError::from(e)))
                                                    });
                                                }
                                            }
                                        }
                                        Err(_e) => {
                                            return Box::pin(async {
                                                Err(Error::from(HttpError::from(e)))
                                            });
                                        }
                                    }
                                }
                                _ => {
                                    return Box::pin(async {
                                        Err(Error::from(HttpError::from(e)))
                                    });
                                }
                            },
                        };

                        info!("user: {:?}", user);
                        // TODO: Replace with a user
                        req.extensions_mut()
                            .insert(user.email.map_or("".to_string(), |s| s));
                    }
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::InvalidToken => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid token".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidIssuer => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid issuer".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                // INFO: When I requested with a token which is encoded Base64, this error occurred.
                ErrorKind::InvalidSignature => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid signature".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidEcdsaKey => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid ecdsa key".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidAudience => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid audience".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidSubject => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Requested with invalid subject".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidRsaKey(e) => {
                    let error_message = format!("Requested with rsa key: {}", e);
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some(error_message),
                            error_type: HttpErrorType::AuthError,
                        }))
                    });
                }
                ErrorKind::RsaFailedSigning => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Rsa failed signing".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidAlgorithmName => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Invalid algorithm name".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidKeyFormat => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Invalid key format".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::MissingRequiredClaim(e) => {
                    let error_message = format!("Missing required claim: {}", e);
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some(error_message),
                            error_type: HttpErrorType::AuthError,
                        }))
                    });
                }
                ErrorKind::ExpiredSignature => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Expired signature".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::ImmatureSignature => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Immature signature".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::InvalidAlgorithm => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Invalid algorithm".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                ErrorKind::MissingAlgorithm => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Missing algorithm".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                }
                _ => {
                    return Box::pin(async {
                        Err(Error::from(HttpError {
                            cause: None,
                            message: Some("Error occurred".to_string()),
                            error_type: HttpErrorType::AuthError,
                        }))
                    })
                } // ErrorKind::Base64(base64::DecodeError),
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
                let token = auth_str["bearer ".len()..].trim();
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
