use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    middleware,
    routing::{get, post},
};
use colored::Colorize;
use log::info;
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::{config::TapoConnectionInfos, server::actions::make_actions_router};

use self::{auth::auth_middleware, sessions::refresh_session, state::StateData};

mod actions;
mod auth;
mod errors;
mod loader;
mod sessions;
mod state;

pub use actions::TapoDeviceType;
pub use errors::{ApiError, ApiResult};

pub type SharedState = Arc<StateData>;

pub async fn serve(port: u16, config_path: PathBuf, sessions_file: PathBuf) -> Result<()> {
    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .allow_origin(
            // TODO: make this configurable
            AllowOrigin::any(),
        );

    let (actions_router, actions_route_uris) = make_actions_router();

    let state = Arc::new(StateData::init(config_path, sessions_file).await?);

    let app = Router::new()
        // Reload the configuration file
        .route("/reload-config", post(reload_config))
        // Refresh a device's session
        .route("/refresh-session", get(refresh_session))
        // List all available devices
        .route("/devices", get(list_devices))
        // Nested action routes
        .nest("/actions", actions_router)
        // Add authentication layer for all routes above
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            auth_middleware,
        ))
        // Login route
        .route("/login", post(auth::login))
        // List all available actions
        .route(
            "/actions",
            get(|| async move { actions_route_uris.join("\n") }),
        )
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");

    info!("Launching server on {}...", addr.bright_green());

    info!(
        "To see the list of all available actions, check {}",
        format!("{addr}/actions").bright_green()
    );

    let tcp_listener = TcpListener::bind(addr).await?;

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .map_err(Into::into)
}

async fn list_devices(state: State<Arc<StateData>>) -> Json<Vec<TapoConnectionInfos>> {
    Json(
        state
            .loaded_config
            .read()
            .await
            .devices
            .values()
            .map(|dev| dev.conn_infos().clone())
            .collect(),
    )
}

async fn reload_config(state: State<Arc<StateData>>) -> String {
    match state.reload_config().await {
        Ok(()) => "OK".to_owned(),
        Err(err) => format!("Error: {err}"),
    }
}
