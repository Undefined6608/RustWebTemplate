use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    config::Config,
    error::{AppError, Result},
    utils::verify_jwt,
};

pub async fn auth_middleware(
    State(config): State<Config>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Authentication("Invalid authorization header format".to_string()))?;

    let claims = verify_jwt(token, &config.jwt_secret)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    // Add user_id to request extensions
    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}
