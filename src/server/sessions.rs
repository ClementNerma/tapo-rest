use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result, bail};
use axum::extract::{Query, State};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::RwLock};

use super::{ApiResult, SharedState};

pub struct Sessions {
    path: PathBuf,
    // TODO: use a concurrent map type instead of a big RwLock
    map: RwLock<HashMap<String, Session>>,
}

impl Sessions {
    pub async fn create(path: PathBuf) -> Result<Self> {
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
        })
    }

    pub async fn get(&self, id: &str) -> Option<Session> {
        self.map.read().await.get(id).cloned()
    }

    pub async fn insert(&self) -> Result<String> {
        let session = Session {};

        let id = Self::_gen_session_id();

        let mut map_lock = self.map.write().await;

        if map_lock.contains_key(&id) {
            bail!("A session already exists with the provided ID!");
        }

        map_lock.insert(id.clone(), session);

        self._flush(&map_lock).await?;

        Ok(id)
    }

    async fn _flush(&self, map: &HashMap<String, Session>) -> Result<()> {
        let str = serde_json::to_string(&map).unwrap();

        fs::write(&self.path, &str)
            .await
            .context("Failed to flush sessions to disk")?;

        Ok(())
    }

    fn _gen_session_id() -> String {
        let mut rng = rand::rng();

        (1..32)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect::<String>()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    // TODO: permissions? only access to specific bulbs, etc.?
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

    let device = loaded_config
        .devices
        .get(&device)
        .with_context(|| format!("Unkown device: {device}"))?;

    device
        .refresh_session()
        .await
        .context("Failed to refresh device's session")?;

    Ok(())
}
