use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{models::User, AppState};

/// Create a new user
/// 
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
/// 
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
/// 
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::{body::Body, http::Request, Router};
//     // use hyper::StatusCode;
//     use serde_json::json;
//     use sqlx::{Pool, Sqlite};
//     use tokio::sync::Mutex;
//     use std::sync::Arc;
//     use tower::ServiceExt; // for `app.oneshot()`

//     async fn setup_test_db() -> Pool<Sqlite> {
//         let pool = Pool::connect("sqlite::memory:").await.unwrap();
//         sqlx::query(
//             r#"
//             CREATE TABLE users (
//                 id INTEGER PRIMARY KEY AUTOINCREMENT,
//                 username TEXT NOT NULL,
//                 email TEXT NOT NULL UNIQUE,
//                 password_hash TEXT NOT NULL
//             );
//             "#,
//         )
//         .execute(&pool)
//         .await
//         .unwrap();
//         pool
//     }

//     #[tokio::test]
//     async fn test_sign_up() {
//         let pool = setup_test_db().await;
//         let state = AppState {
//             pool:pool.clone(), 
//             minio: Arc::new(Mutex::new(minio::s3::ClientBuilder::new("").build().unwrap())), 
//         };

//         let app = Router::new().route("/sign_up", axum::routing::post(sign_up)).layer(axum::AddExtensionLayer::new(state));

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/sign_up")
//                     .method("POST")
//                     .header("content-type", "application/json")
//                     .body(Body::from(
//                         json!({
//                             "username": "testuser",
//                             "email": "test@example.com",
//                             "password": "password123"
//                         })
//                         .to_string(),
//                     ))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);
//     }

//     #[tokio::test]
//     async fn test_login() {
//         let pool = setup_test_db().await;
//         let state = AppState { pool: pool.clone() };
//         let app = Router::new().route("/login", axum::routing::post(login)).layer(axum::AddExtensionLayer::new(state));

//         // First, create a user
//         sqlx::query(
//             r#"
//             INSERT INTO users (username, email, password_hash)
//             VALUES (?, ?, ?)
//             "#,
//         )
//         .bind("testuser")
//         .bind("test@example.com")
//         .bind(bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap())
//         .execute(&pool)
//         .await
//         .unwrap();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/login")
//                     .method("POST")
//                     .header("content-type", "application/json")
//                     .body(Body::from(
//                         json!({
//                             "email": "test@example.com",
//                             "password": "password123"
//                         })
//                         .to_string(),
//                     ))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);
//     }

//     #[tokio::test]
//     async fn test_get_user_by_email() {
//         let pool = setup_test_db().await;
//         let state = AppState { pool: pool.clone() };
//         let app = Router::new().route("/user", axum::routing::get(get_user_by_email)).layer(axum::AddExtensionLayer::new(state));

//         // First, create a user
//         sqlx::query(
//             r#"
//             INSERT INTO users (username, email, password_hash)
//             VALUES (?, ?, ?)
//             "#,
//         )
//         .bind("testuser")
//         .bind("test@example.com")
//         .bind(bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap())
//         .execute(&pool)
//         .await
//         .unwrap();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/user?email=test@example.com")
//                     .method("GET")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);
//     }
// }
