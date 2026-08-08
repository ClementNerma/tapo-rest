use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::server::TapoDeviceType;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tapo_credentials: TapoCredentials,
    pub devices: Vec<TapoConnectionInfos>,
    pub server: ServerConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TapoCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TapoConnectionInfos {
    pub name: String,
    pub device_type: TapoDeviceType,
    pub ip_addr: IpAddr,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub password: String,
    pub api_keys: Vec<ServerApiKey>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerApiKey {
    pub name: String,
    pub key: String,
}
