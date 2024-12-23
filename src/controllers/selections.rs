use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{models::{Photo, PhotoSelection}, AppState};

/// Select photos from an album
/// 
/// POST /selections
#[axum::debug_handler]
pub async fn select_photos(
    State(state): State<AppState>,
    Json(selections): Json<SelectionsRequest>,
) -> Result<Json<SelectionsRequest>, (StatusCode, String)> {
    // Check if the album exists
    let album = sqlx::query(
        r#"
    SELECT * FROM albums
    WHERE album_id = $1
    "#,
    )
    .bind(selections.album_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if album.is_none() {
        return Err((StatusCode::BAD_REQUEST, "Album does not exist".to_string()));
    }

    // Create the selections
    for photo_id in selections.photo_ids.iter() {
        sqlx::query(
            r#"
        INSERT INTO photo_selections (photo_id, client_id)
        VALUES ($1, $2)
        "#,
        )
        .bind(photo_id)
        .bind(selections.client_id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(Json(selections))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SelectionsRequest {
    pub client_id: i32,
    pub album_id: i32,
    pub photo_ids: Vec<i32>,
}

/// Get all selections for a client and album
/// 
/// GET /selections?client_id={client_id}&album_id={album_id}

#[axum::debug_handler]
pub async fn get_selections_by_client_and_album(
    State(state): State<AppState>,
    Query(query): Query<SelectionsQuery>,
) -> Result<Json<Vec<PhotoSelection>>, (StatusCode, String)> {
    // photo_selections (photo_id, client_id)
    let selections = sqlx::query_as::<_, PhotoSelection>(
        r#"
    SELECT * FROM photo_selections
    JOIN photos ON photo_selections.photo_id = photos.photo_id
    WHERE photo_selections.client_id = $1 AND photos.album_id = $2
    "#,
    )
    .bind(query.client_id)
    .bind(query.album_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(selections))
}

#[derive(serde::Deserialize)]
pub struct SelectionsQuery {
    pub client_id: i32,
    pub album_id: i32,
}

/// Get all seleced photos for a client and album
/// 
/// GET /selected_photos?client_id={client_id}&album_id={album_id}

#[axum::debug_handler]
pub async fn get_selected_photos_by_client_and_album(
    State(state): State<AppState>,
    Query(query): Query<SelectionsQuery>,
) -> Result<Json<Vec<Photo>>, (StatusCode, String)> {
    let photos = sqlx::query_as::<_, Photo>(
        r#"
    SELECT * FROM photos
    WHERE photo_id IN (
        SELECT photo_id FROM photo_selections
        WHERE client_id = $1
    )
    AND album_id = $2
    "#,
    )
    .bind(query.client_id)
    .bind(query.album_id)
    .fetch_all(&state.pool) // FIXME fetch optional
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(photos))
}
