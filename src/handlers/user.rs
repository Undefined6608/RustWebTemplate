use axum::{extract::State, Extension, Json};
use uuid::Uuid;

use crate::{
    error::Result,
    models::UserResponse,
    routes::AppState,
    services::UserService,
};

pub async fn get_profile(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = UserService::get_user_by_id(&app_state.pool, user_id).await?;
    Ok(Json(user.into()))
}

pub async fn get_all_users(
    State(app_state): State<AppState>,
    Extension(_user_id): Extension<Uuid>, // Require authentication
) -> Result<Json<Vec<UserResponse>>> {
    let users = UserService::get_all_users(&app_state.pool).await?;
    let user_responses: Vec<UserResponse> = users.into_iter().map(|user| user.into()).collect();
    Ok(Json(user_responses))
}
