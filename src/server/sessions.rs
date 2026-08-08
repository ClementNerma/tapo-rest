use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::{Context, Result, bail};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use rand::{RngExt, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::RwLock};

use super::{ApiResult, SharedState};

pub struct Sessions {
    path: PathBuf,
    map: RwLock<HashMap<String, Session>>,
    session_lifespan: Option<Duration>,
}

impl Sessions {
    pub async fn create(path: PathBuf, session_lifespan: Option<Duration>) -> Result<Self> {
        let map = if path.exists() {
            let sessions_str = fs::read_to_string(&path)
                .await
                .context("Failed to read sessions file")?;

            serde_json::from_str(&sessions_str).context("Failed to parse sessions file")?
        } else {
            HashMap::new()
        };

        Ok(Self {
            path,
            map: RwLock::new(map),
            session_lifespan,
        })
    }

    pub async fn get(&self, id: &str) -> Option<Session> {
        self.map.read().await.get(id).cloned()
    }

    pub async fn insert(&self) -> Result<String> {
        let mut map_lock = self.map.write().await;

        let session = Session {
            created_at: SystemTime::now(),
            expires_at: self
                .session_lifespan
                .map(|lifespan| SystemTime::now() + lifespan),
        };

        let id = Self::gen_session_id();

        if map_lock.contains_key(&id) {
            bail!("A session already exists with the provided ID!");
        }

        map_lock.insert(id.clone(), session);

        self.flush(&map_lock).await?;

        Ok(id)
    }

    async fn flush(&self, map: &HashMap<String, Session>) -> Result<()> {
        let str = serde_json::to_string(&map).unwrap();

        fs::write(&self.path, &str)
            .await
            .context("Failed to flush sessions to disk")?;

        Ok(())
    }

    fn gen_session_id() -> String {
        let mut rng = rand::rng();

        (1..32)
            .map(|_| char::from(rng.sample(Alphanumeric)))
            .collect::<String>()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

#[derive(Deserialize)]
pub struct RefreshDeviceSessionParams {
    device: String,
}

pub async fn refresh_session(
    State(state): State<SharedState>,
    Query(params): Query<RefreshDeviceSessionParams>,
) -> ApiResult<()> {
    let RefreshDeviceSessionParams { device } = params;

    let loaded_config = state.loaded_config.read().await;

    let device = loaded_config.devices.get(&device).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Unknown device: {device}"),
        )
    })?;

    device
        .refresh_session()
        .await
        .context("Failed to refresh device's session")?;

    Ok(())
}
