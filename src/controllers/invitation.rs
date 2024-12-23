use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{Album, Invitation},
    AppState,
};


/// Create a new invitation for a user to join an album.
/// connect user_id and album_id in the invitations table.
/// 
/// POST /invitation
#[axum::debug_handler]
pub async fn create_invitation(
    State(state): State<AppState>,
    Json(invitation): Json<InvitationRequest>,
) -> Result<Json<InvitationRequest>, (StatusCode, String)> {

    if invitation.client_id == invitation.photographer_id {
        return Err((StatusCode::BAD_REQUEST, "Photographer cannot invite themselves".to_string()));
    }

    // Check if the user and album exist
    let user = sqlx::query(
        r#"
    SELECT * FROM users
    WHERE user_id = $1
    "#,
    )
    .bind(invitation.client_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if user.is_none() {
        return Err((StatusCode::BAD_REQUEST, "User does not exist".to_string()));
    }

    let album = sqlx::query_as::<_, Album>(
        r#"
    SELECT * FROM albums
    WHERE album_id = $1
    "#,
    )
    .bind(invitation.album_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match album {
        Some(album) => {
            if album.photographer_id != invitation.photographer_id {
                return Err((StatusCode::BAD_REQUEST, "Photographer cannot invite themselves".to_string()));
            }

        }
        None => {
            return Err((StatusCode::BAD_REQUEST, "Album does not exist".to_string()));
        }
        
    }


    // Create the invitation
    sqlx::query(
        r#"
    INSERT INTO invitations (client_id, album_id)
    VALUES ($1, $2, $3)
    "#,
    )
    .bind(invitation.client_id)
    .bind(invitation.album_id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(invitation))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct InvitationRequest {
    pub client_id: i32,
    pub album_id: i32,
    pub photographer_id: i32,
}

/// Get all invitations for a user
/// 
/// GET /invitations?client_id={client_id}
#[axum::debug_handler]
pub async fn get_invitations_by_user_id(
    State(state): State<AppState>,
    Query(query): Query<GetInvitationsByUserIdRequest>,
) -> Result<Json<Vec<Invitation>>, (StatusCode, String)> {
    let invitations = sqlx::query_as::<_, Invitation>(
        r#"
    SELECT * FROM invitations
    WHERE client_id = $1
    "#,
    )
    .bind(query.client_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(invitations))
}

#[derive(serde::Deserialize)]
pub struct GetInvitationsByUserIdRequest {
    pub client_id: i32,
}

/// Get all albums a user has been invited to
/// 
/// GET /albums/invited?client_id={client_id}
#[axum::debug_handler]
pub async fn get_albums_invited_to(
    State(state): State<AppState>,
    Query(query): Query<GetAlbumsInvitedToRequest>,
) -> Result<Json<Vec<Album>>, (StatusCode, String)> {
    let albums = sqlx::query_as::<_, Album>(
        r#"
    SELECT * FROM albums
    WHERE album_id IN (
        SELECT album_id FROM invitations
        WHERE client_id = $1
    )
    "#,
    )
    .bind(query.client_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(albums))
}

#[derive(serde::Deserialize)]
pub struct GetAlbumsInvitedToRequest {
    pub client_id: i32,
}
