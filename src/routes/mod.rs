mod health;
mod user;

use crate::middleware::logging_middleware::LoggingMiddlewareLayer;
use crate::services::user::UserService;
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn create_routes(user_service: Arc<UserService>) -> Router {
    // Create stateless health routes
    let health_routes = health::routes();
    
    // Create user routes with specific state type and provide the state
    let user_routes = user::routes().with_state(user_service.clone());
    
    // Convert health routes to have the same state type
    // let health_routes_with_state = health_routes.with_state(user_service.clone());
    
    // Combine routes - now both have the same state type
    let api_routes = Router::new()
        .nest("/health", health_routes)
        .nest("/users", user_routes);

    Router::new()
        .route("/", axum::routing::get(crate::handlers::health::root))
        .nest("/api/v1", api_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(LoggingMiddlewareLayer::new())
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
}