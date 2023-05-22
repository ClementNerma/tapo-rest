use std::collections::HashMap;

use crate::devices::TapoDevice;

pub struct State {
    pub auth_password: String,
    pub devices: HashMap<String, TapoDevice>,
    pub sessions: HashMap<String, Session>,
}

impl State {
    pub fn new(auth_password: String, devices: Vec<TapoDevice>) -> Self {
        Self {
            auth_password,
            devices: devices
                .into_iter()
                .map(|device| (device.name.clone(), device))
                .collect(),
            sessions: HashMap::new(),
        }
    }
}

pub struct Session {
    // TODO: permissions? only access to specific bulbs, etc.?
}
