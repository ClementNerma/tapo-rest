use std::{fs, path::PathBuf, sync::Arc};

use anyhow::{bail, Context, Result};
use axum::{
    routing::{get, post},
    Router,
};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::{
    cmd::{PasswordArgGroup, ServerConfig},
    devices::TapoDevice,
    server::{actions::make_router, state::StateInit},
};

use self::state::State;

mod actions;
mod auth;
mod discovery;
mod errors;
mod sessions;
mod state;

pub use actions::TapoDeviceType;
pub use errors::{ApiError, ApiResult};

pub type SharedStateInner = State;
pub type SharedState = Arc<RwLock<SharedStateInner>>;

pub async fn serve(
    config: ServerConfig,
    devices: Vec<TapoDevice>,
    sessions_file: PathBuf,
) -> Result<()> {
    let ServerConfig { port, password } = config;

    let PasswordArgGroup {
        auth_password,
        password_from_file,
    } = password;

    let auth_password = match (auth_password, password_from_file) {
        (Some(auth_password), None) => auth_password,

        (None, Some(file)) => {
            if !file.is_file() {
                bail!(
                    "Provided file password path does not exist: {}",
                    file.display()
                );
            }

            fs::read_to_string(&file).with_context(|| {
                format!("Failed to read file password at path: {}", file.display())
            })?
        }

        (Some(_), Some(_)) | (None, None) => unreachable!(),
    };

    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .allow_origin(
            // TODO: make this configurable
            AllowOrigin::any(),
        );

    let app = Router::new()
        .route("/login", post(auth::login))
        .route("/discover", get(discovery::discover_devices))
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

    let addr = format!("0.0.0.0:{port}");

    println!("Launching server on {addr}...");

    let tcp_listener = TcpListener::bind(addr).await?;

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .map_err(Into::into)
}
