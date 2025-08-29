use crate::{
    auth::{create_jwt, hash_password, verify_password},
    errors::AppError,
    models::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // Validate the request
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("User already exists".to_string()));
    }

    // Hash the password
    let password_hash = hash_password(&payload.password, state.config.bcrypt_cost)?;

    // Create the user
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, first_name, last_name, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await?;

    // Create JWT token
    let token = create_jwt(&user.id.to_string(), &user.email, &state.config)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate the request
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

    // Find the user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    // Verify the password
    let is_valid = verify_password(&payload.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    // Create JWT token
    let token = create_jwt(&user.id.to_string(), &user.email, &state.config)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(Json(response))
}
