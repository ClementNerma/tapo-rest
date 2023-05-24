use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub account: TapoCredentials,
    pub devices: Vec<TapoConnectionInfos>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TapoCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TapoConnectionInfos {
    pub name: String,
    pub device_type: TapoDeviceType,
    pub ip_addr: Ipv4Addr,
}

#[derive(Serialize, Deserialize)]
pub enum TapoDeviceType {
    L510,
    L530,
}
