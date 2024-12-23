use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for User {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub album_id: i32,
    pub photographer_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Album {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            album_id: row.try_get("album_id")?,
            photographer_id: row.try_get("photographer_id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Photo {
    pub photo_id: i32,
    pub album_id: i32,
    pub s3_path: String,
    pub uploaded_at: NaiveDateTime,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Photo {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            photo_id: row.try_get("photo_id")?,
            album_id: row.try_get("album_id")?,
            s3_path: row.try_get("s3_path")?,
            uploaded_at: row.try_get("uploaded_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invitation {
    pub invitation_id: i32,
    pub album_id: i32,
    pub client_id: i32,
    pub photographer_id: i32,
    pub invite_token: String,
    pub created_at: NaiveDateTime,
}
impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Invitation {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            invitation_id: row.try_get("invitation_id")?,
            album_id: row.try_get("album_id")?,
            client_id: row.try_get("client_id")?,
            photographer_id: row.try_get("photographer_id")?,
            invite_token: row.try_get("invite_token")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoSelection {
    pub selection_id: i32,
    pub album_id: i32,
    pub client_id: i32,
    pub confirmed_at: NaiveDateTime,
}
impl<'r> FromRow<'r, sqlx::postgres::PgRow> for PhotoSelection {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            selection_id: row.try_get("selection_id")?,
            album_id: row.try_get("album_id")?,
            client_id: row.try_get("client_id")?,
            confirmed_at: row.try_get("confirmed_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub notification_id: i32,
    pub recipient_id: i32,
    pub message: String,
    pub is_read: bool,
    pub created_at: NaiveDateTime,
}
