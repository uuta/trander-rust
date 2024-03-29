use crate::db;
use crate::error::http_error::{HttpError, HttpErrorType};
use crate::info_request_log;
use crate::repository::request_limits::{ImplRequestLimitsRepository, RequestLimitsRepository};
use crate::repository::users::{ImplUsersRepository, UsersRepository};
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};
use tracing::info;

pub struct PostProcessing;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for PostProcessing
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PostProcessingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PostProcessingMiddleware { service }))
    }
}

pub struct PostProcessingMiddleware<S> {
    /// The next service to call
    service: S,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for PostProcessingMiddleware<S>
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
        let email_result = req
            .extensions()
            .get::<String>()
            .ok_or_else(|| {
                Error::from(HttpError {
                    error_type: HttpErrorType::AuthError,
                    cause: None,
                    message: Some(format!("email is not found: {:?}", req.uri())),
                })
            })
            .and_then(|email| Ok(email.clone()));
        let fut = self.service.call(req);
        Box::pin(async move {
            match email_result {
                Ok(email) => {
                    let res = fut.await?;
                    if let Some(db) = res.request().app_data::<Data<db::DbPool>>() {
                        if let Ok(mut conn) = db.get().map_err(|e| HttpError::from(e)) {
                            let users_repo = ImplUsersRepository;
                            let user_res = users_repo
                                .get_by_email(&mut conn, &email)
                                .map_err(|e| HttpError::from(e))?;

                            let request_limits_repo = ImplRequestLimitsRepository;
                            request_limits_repo
                                .decrement(user_res.id, &mut conn)
                                .map_err(|e| HttpError::from(e))?;
                        }
                    }
                    Ok(res)
                }
                // TODO:
                Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
                    "Internal Server Error {:}",
                    e
                ))),
            }
        })
    }
}
