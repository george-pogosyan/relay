use actix_web::error::Error;
use actix_web::middleware::{Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse};
use http::header;
use sentry::integrations::failure::capture_fail;

use constants::SERVER;

/// forces the mimetype to json for some cases.
pub struct ForceJson;

impl<S> Middleware<S> for ForceJson {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started, Error> {
        req.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        Ok(Started::Done)
    }
}

pub struct CaptureSentryError;

impl<S> Middleware<S> for CaptureSentryError {
    fn response(&self, _req: &mut HttpRequest<S>, resp: HttpResponse) -> Result<Response, Error> {
        // TODO: newer versions of actix will support the backtrace on the actix
        // error.  In that case we want to emit a custom error event to sentry
        // that includes that backtrace (maybe also have a sentry-actix package).
        if resp.status().is_server_error() {
            if let Some(error) = resp.error() {
                capture_fail(error.cause());
            }
        }
        Ok(Response::Done(resp))
    }
}

/// Adds the common relay headers.
pub struct AddCommonHeaders;

impl<S> Middleware<S> for AddCommonHeaders {
    fn response(
        &self,
        _req: &mut HttpRequest<S>,
        mut resp: HttpResponse,
    ) -> Result<Response, Error> {
        resp.headers_mut()
            .insert(header::SERVER, header::HeaderValue::from_static(SERVER));
        Ok(Response::Done(resp))
    }
}
