use crate::error::http_error::HttpError;
use actix_web::{dev, http::StatusCode, Error};

fn error_middleware<S, B>(
    req: dev::ServiceRequest,
    srv: &S,
) -> impl futures::Future<Output = Result<dev::ServiceResponse<B>, Error>>
where
    S: dev::Service<dev::ServiceRequest, Response = dev::ServiceResponse<B>, Error = Error>,
    B: 'static,
{
    let fut = srv.call(req);
    async {
        let res = fut.await?;

        if res.response().status().is_client_error() {
            match res.response().status() {
                StatusCode::BAD_REQUEST => Err(Error::from(HttpError { name: "BadRequest" })),
                StatusCode::UNAUTHORIZED => Err(Error::from(HttpError {
                    name: "Unauthorized",
                })),
                StatusCode::NOT_FOUND => Err(Error::from(HttpError { name: "NotFound" })),
                _ => Err(Error::from(HttpError {
                    name: "OtherClientError",
                })),
            }
        } else if res.response().status().is_server_error() {
            Err(Error::from(HttpError {
                name: "InternalServerError",
            }))
        } else {
            Ok(res)
        }
    }
}
