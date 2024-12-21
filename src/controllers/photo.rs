use std::collections::HashMap;

use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    Json,
};
use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs, PutObjectApiArgs},
    response::PutObjectApiResponse,
};

use crate::{models::Photo, AppState};

/// Upload a photo to an album
/// POST /upload
#[axum::debug_handler]
pub async fn upload_photo(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Photo>, (StatusCode, String)> {
    // Temporary storage for form fields
    let mut fields: HashMap<String, String> = HashMap::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to read multipart field: {}", e),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            // Handle file upload
            let file_name = field.file_name().unwrap_or("uploaded_file").to_string();

            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Failed to read file bytes: {}", e),
                )
            })?;

            // upload to minio
            let response = upload_photo_to_minio("photos", &file_name, &data, &state)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to upload file to minio: {}", e),
                    )
                })?;
            fields.insert("file_path".to_string(), response.object_name);
        } else {
            // Handle text fields
            let value = field.text().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Failed to read field text: {}", e),
                )
            })?;
            fields.insert(name, value);
        }
    }

    // Validate and construct response
    let album_id = fields
        .get("album_id")
        .ok_or((
            StatusCode::BAD_REQUEST,
            "Missing field: album_id".to_string(),
        ))?
        .clone()
        .parse::<i32>()
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to parse album_id: {}", e),
            )
        })?;

    let file_path = fields
        .get("file_path")
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed while save photo".to_string(),
        ))?
        .clone();

    let photo = sqlx::query_as::<_, Photo>(
        r#"
    INSERT INTO photos (album_id, s3_path)
        VALUES ($1, $2)
        RETURNING *
    "#,
    )
    .bind(album_id)
    .bind(file_path)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(photo))
}

pub async fn upload_photo_to_minio(
    bucket_name: &str,
    object_name: &str,
    data: &[u8],
    state: &AppState,
) -> Result<PutObjectApiResponse, minio::s3::error::Error> {
    if !state
        .minio
        .lock()
        .await
        .bucket_exists(&BucketExistsArgs::new(bucket_name)?)
        .await?
    {
        state
            .minio
            .lock()
            .await
            .make_bucket(&MakeBucketArgs::new(bucket_name)?)
            .await?;
    }

    let response = state
        .minio
        .lock()
        .await
        .put_object_api(&mut PutObjectApiArgs::new(bucket_name, object_name, data)?)
        .await?;

    Ok(response)
}

/// Get all photos in an album
/// GET /photos?album_id={album_id}
#[axum::debug_handler]
pub async fn get_photos_by_album_id(
    State(state): State<AppState>,
    Query(query): Query<GetPhotosByAlbumIdRequest>,
) -> Result<Json<Vec<Photo>>, (StatusCode, String)> {
    let photos = sqlx::query_as::<_, Photo>(
        r#"
    SELECT * FROM photos
    WHERE album_id = $1
    "#,
    )
    .bind(query.album_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(photos))
}

#[derive(serde::Deserialize)]
pub struct GetPhotosByAlbumIdRequest {
    pub album_id: i32,
}