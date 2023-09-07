use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::server::TapoDeviceType;

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
    pub ip_addr: IpAddr,
}
