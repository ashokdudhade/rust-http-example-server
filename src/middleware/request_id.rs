use axum::{extract::Request, response::Response};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestIdLayer;

impl RequestIdLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdService<S> {
    inner: S,
}

impl<S> Service<Request> for RequestIdService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        // Check if request already has an ID
        let request_id = match req.headers().get("x-request-id") {
            Some(header_value) => {
                // Try to convert existing header to string
                match header_value.to_str() {
                    Ok(id) => id.to_string(),
                    Err(_) => {
                        // Invalid header value, generate new ID
                        let new_id = Uuid::new_v4().to_string();
                        req.headers_mut().insert("x-request-id", new_id.parse().unwrap());
                        new_id
                    }
                }
            }
            None => {
                // No header exists, generate new ID
                let new_id = Uuid::new_v4().to_string();
                req.headers_mut().insert("x-request-id", new_id.parse().unwrap());
                new_id
            }
        };

        let mut inner = self.inner.clone();
        Box::pin(async move {
            let mut response = inner.call(req).await?;
            response
                .headers_mut()
                .insert("x-request-id", request_id.parse().unwrap());
            Ok(response)
        })
    }
}