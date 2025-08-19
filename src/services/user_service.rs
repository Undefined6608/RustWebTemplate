use uuid::Uuid;

use crate::{
    db::DbPool,
    error::{AppError, Result},
    models::{CreateUserRequest, LoginRequest, User},
    utils::{hash_password, verify_password},
};

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: &DbPool, request: CreateUserRequest) -> Result<User> {
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(pool)
        .await?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("User with this email already exists".to_string()));
        }

        // Hash password
        let password_hash = hash_password(&request.password)?;

        // Create user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, name)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.name)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn authenticate_user(pool: &DbPool, request: LoginRequest) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid email or password".to_string()))?;

        let is_valid = verify_password(&request.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Authentication("Invalid email or password".to_string()));
        }

        Ok(user)
    }

    pub async fn get_user_by_id(pool: &DbPool, user_id: Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn get_all_users(pool: &DbPool) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}
