use crate::db;
use crate::error::http_error::{HttpError, HttpErrorType};
use crate::info_request_log;
use crate::repository::request_limits::{ImplRequestLimitsRepository, RequestLimitsRepository};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
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
        let user_id_result = req
            .match_info()
            .get("user_id")
            .ok_or_else(|| {
                Error::from(HttpError {
                    error_type: HttpErrorType::AuthError,
                    cause: None,
                    message: None,
                })
            })
            .and_then(|uid_str| Ok(uid_str.parse::<u64>().map_err(|e| HttpError::from(e))));
        let fut = self.service.call(req);
        Box::pin(async move {
            match user_id_result {
                Ok(user_id) => {
                    let res = fut.await?;
                    Ok(res)
                }
                Err(e) => {
                    let error_response = HttpResponse::InternalServerError().finish();
                    Ok(req.error_response(error_response))
                }
            }
        })
    }
}
