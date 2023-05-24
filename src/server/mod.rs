use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{routing::post, Router, Server};
use tokio::sync::RwLock;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::{
    cmd::ServerConfig,
    devices::TapoDevice,
    server::{actions::make_router, state::StateInit},
};

use self::state::State;

mod actions;
mod auth;
mod errors;
mod sessions;
mod state;

pub use errors::{ApiError, ApiResult};

pub type SharedStateInner = State;
pub type SharedState = Arc<RwLock<SharedStateInner>>;

pub async fn serve(
    config: ServerConfig,
    devices: Vec<TapoDevice>,
    sessions_file: PathBuf,
) -> Result<()> {
    let ServerConfig {
        port,
        auth_password,
    } = config;

    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .allow_origin(
            // TODO: make this configurable
            AllowOrigin::any(),
        );

    let app = Router::new()
        .route("/login", post(auth::login))
        .nest("/actions", make_router())
        .layer(cors)
        .with_state(Arc::new(RwLock::new(
            State::init(StateInit {
                // TODO: hash?
                auth_password,
                devices,
                sessions_file,
            })
            .await?,
        )));

    let addr = format!("0.0.0.0:{port}").parse().unwrap();

    println!("Launching server on {addr}...");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
