// TODO: Delete
// use crate::error::http_error::HttpError;
// use actix_web::{dev, http::StatusCode, Error};
//
// pub fn error_middleware<S, B>(
//     req: dev::ServiceRequest,
//     srv: &S,
// ) -> impl futures::Future<Output = Result<dev::ServiceResponse<B>, Error>>
// where
//     S: dev::Service<dev::ServiceRequest, Response = dev::ServiceResponse<B>, Error = Error>,
//     B: 'static,
// {
//     let fut = srv.call(req);
//     async {
//         let res = fut.await?;
//
//         if res.response().status().is_client_error() {
//             match res.response().status() {
//                 StatusCode::BAD_REQUEST => Err(Error::from(HttpError::new(
//                     "BadRequest",
//                     "Bad request occurred".to_string(),
//                 ))),
//                 StatusCode::UNAUTHORIZED => Err(Error::from(HttpError::new(
//                     "Unauthorized",
//                     "Unauthorized request".to_string(),
//                 ))),
//                 StatusCode::NOT_FOUND => Err(Error::from(HttpError::new(
//                     "NotFound",
//                     "Resource not found".to_string(),
//                 ))),
//                 _ => Err(Error::from(HttpError::new(
//                     "OtherClientError",
//                     "Other client error occurred".to_string(),
//                 ))),
//             }
//         } else if res.response().status().is_server_error() {
//             Err(Error::from(HttpError::new(
//                 "InternalServerError",
//                 "Internal server error occurred".to_string(),
//             )))
//         } else {
//             Ok(res)
//         }
//     }
// }
