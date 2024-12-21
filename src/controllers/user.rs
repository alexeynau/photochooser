use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{models::User, AppState};

/// Create a new user
/// POST /sign_up
#[axum::debug_handler]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(sign_up): Json<SignUp>,
) -> Result<(), (StatusCode, String)> {
    let password_hash = bcrypt::hash(sign_up.password, bcrypt::DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    sqlx::query(
        r#"
    INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
    "#,
    )
    .bind(sign_up.username)
    .bind(sign_up.email)
    .bind(password_hash)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Login a user
/// POST /login
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(login): Json<Login>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        r#"
    SELECT * FROM users
    WHERE email = $1
    "#,
    )
    .bind(login.email)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if bcrypt::verify(login.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        Ok(Json(user))
    } else {
        Err((StatusCode::UNAUTHORIZED, "invalid password".to_string()))
    }
}

#[derive(serde::Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

/// Get the current user by email
/// GET /user?email={email}
#[axum::debug_handler]
pub async fn get_user_by_email(
    State(state): State<AppState>,
    Query(request): Query<GetUserByEmailRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        r#"
    SELECT * FROM users
    WHERE email = $1
    "#,
    )
    .bind(request.email)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

#[derive(serde::Deserialize)]
pub struct GetUserByEmailRequest {
    pub email: String,
}