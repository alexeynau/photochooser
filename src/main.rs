use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

pub mod controllers;
pub mod models;
use controllers::{
    album::{create_album, get_albums_created_by_photographer_id},
    invitation::{create_invitation, get_albums_invited_to, get_invitations_by_user_id},
    photo::{get_photo_by_id, get_photos_by_album_id, upload_photo},
    selections::{get_selected_photos_by_client_and_album, get_selections_by_client_and_album, select_photos},
    user::{get_user_by_email, login, sign_up},
};
use minio::s3::{
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub minio: Arc<Mutex<Client>>,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenvy::var("DATABASE_URL")?)
        .await?;

    let base_url = dotenvy::var("MINIO_SERVER_URL")?.parse::<BaseUrl>()?;

    let provider = StaticProvider::new(
        &dotenvy::var("MINIO_ACCESS_KEY")?,
        &dotenvy::var("MINIO_SECRET_KEY")?,
        None,
    );
    let minioclient = ClientBuilder::new(base_url)
        .provider(Some(Box::new(provider)))
        .build()?;

    let state = AppState {
        pool,
        minio: Arc::new(Mutex::new(minioclient)),
    };

    let router = Router::new()
        .route("/sign_up", post(sign_up))
        .route("/login", post(login))
        .route("/album", post(create_album))
        .route("/upload", post(upload_photo))
        .route("/invitation", post(create_invitation))
        .route("/albums/created", get(get_albums_created_by_photographer_id))
        .route("/user", get(get_user_by_email))
        .route("/invintations", get(get_invitations_by_user_id))
        .route("/albums/invited", get(get_albums_invited_to))
        .route("/photos", get(get_photos_by_album_id))
        .route("/selections", post(select_photos))
        .route("/selections", get(get_selections_by_client_and_album))
        .route("/selected_photo", get(get_selected_photos_by_client_and_album))
        .route("/photo", get(get_photo_by_id))
        .with_state(state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
