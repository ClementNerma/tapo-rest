use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::server::TapoDeviceType;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub devices: Vec<TapoConnectionInfos>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TapoConnectionInfos {
    pub name: String,
    pub device_type: TapoDeviceType,
    pub ip_addr: IpAddr,
}
