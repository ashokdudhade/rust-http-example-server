use crate::domain::{
    errors::AppResult,
    requests::{CreateUserRequest, ListUsersQuery, UpdateUserRequest},
    responses::{ApiResponse, UserProfileResponse, UserResponse, UsersListResponse},
};
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

pub async fn list_users(
    Query(query): Query<ListUsersQuery>,
    State(service): State<AppState>,
) -> AppResult<Json<ApiResponse<UsersListResponse>>> {
    let response = service.list_users(query)?;
    Ok(Json(ApiResponse::new(response)))
}

pub async fn create_user(
    State(service): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UserResponse>>)> {
    let response = service.create_user(payload)?;
    Ok((StatusCode::CREATED, Json(ApiResponse::new(response))))
}

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(service): State<AppState>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let response = service.get_user(id)?;
    Ok(Json(ApiResponse::new(response)))
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(service): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let response = service.update_user(id, payload)?;
    Ok(Json(ApiResponse::new(response)))
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(service): State<AppState>,
) -> AppResult<StatusCode> {
    service.delete_user(id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_user_profile(
    Path(id): Path<Uuid>,
    State(service): State<AppState>,
) -> AppResult<Json<ApiResponse<UserProfileResponse>>> {
    let response = service.get_user_profile(id)?;
    Ok(Json(ApiResponse::new(response)))
}