use axum::{extract::{Query, State}, http::StatusCode, Json};

use crate::{models::Album, AppState};

/// Create a new album
/// POST /album
pub async fn create_album(
    State(state): State<AppState>,
    Json(album): Json<CreateAlbumRequest>,
) -> Result<Json<Album>, (StatusCode, String)> {
    let album = sqlx::query_as::<_, Album>(
        r#"
    INSERT INTO albums (photographer_id, name)
        VALUES ($1, $2)
        RETURNING *
    "#,
    )
    .bind(album.photographer_id)
    .bind(album.name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(album))
}

#[derive(serde::Deserialize)]
pub struct CreateAlbumRequest {
    pub photographer_id: i32,
    pub name: String,
}


/// Get created albums by photographer_id
/// GET /albums/created?photographer_id={photographer_id}
pub async fn get_albums_created_by_photographer_id(
    State(state): State<AppState>,
    Query(query): Query<GetAlbumsCreatedQuery>,
) -> Result<Json<Vec<Album>>, (StatusCode, String)> {
    let albums = sqlx::query_as::<_, Album>(
        r#"
    SELECT * FROM albums
    WHERE photographer_id = $1
    "#,
    )
    .bind(query.photographer_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(albums))
}

#[derive(serde::Deserialize)]
pub struct GetAlbumsCreatedQuery {
    pub photographer_id: i32,
}