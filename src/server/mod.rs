use std::sync::Arc;

use anyhow::Result;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router, Server,
};
use tokio::sync::RwLock;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::{cmd::ServerConfig, devices::TapoDevice};

use self::state::State;

mod actions;
mod auth;
mod state;

pub type SharedStateInner = State;
pub type SharedState = Arc<RwLock<SharedStateInner>>;

pub type ApiResult<T> = Result<T, (StatusCode, String)>;

pub async fn serve(config: ServerConfig, devices: Vec<TapoDevice>) -> Result<()> {
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
        .route("/actions/on", get(actions::on))
        .route("/actions/off", get(actions::off))
        .route("/actions/set-brightness", get(actions::set_brightness))
        .layer(cors)
        .with_state(Arc::new(RwLock::new(State::new(
            // TODO: hash?
            auth_password,
            devices,
        ))));

    let addr = format!("0.0.0.0:{port}").parse().unwrap();

    println!("Launching server on {addr}...");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
