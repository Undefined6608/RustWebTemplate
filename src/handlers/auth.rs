use axum::{extract::State, Json};

use crate::{
    error::Result,
    models::{AuthResponse, CreateUserRequest, LoginRequest},
    routes::AppState,
    services::UserService,
    utils::generate_jwt,
};

pub async fn register(
    State(app_state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>> {
    let user = UserService::create_user(&app_state.pool, request).await?;
    let token = generate_jwt(user.id, &app_state.config.jwt_secret)?;
    
    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(response))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let user = UserService::authenticate_user(&app_state.pool, request).await?;
    let token = generate_jwt(user.id, &app_state.config.jwt_secret)?;
    
    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(response))
}
