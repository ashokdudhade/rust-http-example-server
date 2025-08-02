use crate::handlers::user;
use crate::services::user::UserService;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<UserService>> {
    Router::new()
        .route("/", get(user::list_users).post(user::create_user))
        .route(
            "/:id", 
            get(user::get_user)
                .put(user::update_user)
                .delete(user::delete_user)
        )
        .route("/:id/profile", get(user::get_user_profile))
}