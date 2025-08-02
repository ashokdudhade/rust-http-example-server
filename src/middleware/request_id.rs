use axum::{extract::Request, response::Response};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::Span;
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

        // Create a tracing span that will be active for the entire request
        let span = tracing::info_span!(
            "http_request",
            request_id = %request_id,
            method = %req.method(),
            uri = %req.uri(),
            version = ?req.version(),
        );

        let mut inner = self.inner.clone();
        Box::pin(async move {
            // Enter the span - all logs within this request will include these fields
            let _guard = span.enter();
            
            tracing::info!("Request started");
            
            let start_time = std::time::Instant::now();
            let result = inner.call(req).await;
            let duration = start_time.elapsed();
            
            match &result {
                Ok(response) => {
                    tracing::info!(
                        status = %response.status(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    );
                }
                Err(_) => {
                    tracing::error!(
                        duration_ms = %duration.as_millis(),
                        "Request failed"
                    );
                }
            }
            
            // Add request ID to response headers
            if let Ok(mut response) = result {
                response
                    .headers_mut()
                    .insert("x-request-id", request_id.parse().unwrap());
                Ok(response)
            } else {
                result
            }
        })
    }
}