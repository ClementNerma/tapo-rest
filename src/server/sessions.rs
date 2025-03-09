use std::{collections::HashMap, path::PathBuf};

use anyhow::{bail, Context, Result};
use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use tokio::fs;

pub struct Sessions {
    path: PathBuf,
    map: HashMap<String, Session>,
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

        Ok(Self { path, map })
    }

    pub fn get(&self, id: &str) -> Option<&Session> {
        self.map.get(id)
    }

    pub async fn insert(&mut self) -> Result<String> {
        let session = Session {};

        let id = Self::_gen_session_id();

        if self.map.contains_key(&id) {
            bail!("A session already exists with the provided ID!");
        }

        self.map.insert(id.clone(), session);

        self._flush().await?;

        Ok(id)
    }

    async fn _flush(&self) -> Result<()> {
        let str = serde_json::to_string(&self.map).unwrap();

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
