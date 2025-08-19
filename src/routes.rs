use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    config::Config,
    db::DbPool,
    handlers::{get_all_users, get_profile, login, register},
    middleware::auth_middleware,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}

pub fn create_routes(pool: DbPool, config: Config) -> Router {
    let app_state = AppState { pool, config: config.clone() };

    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route("/profile", get(get_profile))
        .route("/users", get(get_all_users))
        .layer(middleware::from_fn_with_state(config, auth_middleware));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api", protected_routes)
        .route("/health", get(health_check))
        .with_state(app_state)
}

async fn health_check() -> &'static str {
    "OK"
}
