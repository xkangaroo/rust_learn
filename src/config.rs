use serde::Deserialize;
use std::fs;
use toml;

/// 配置文件
#[derive(Debug, Deserialize)]
pub struct Configs {
    /// 程序配置
    pub server: Server
}

/// server 配置文件
#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub name: String
}

pub fn load_config() -> Configs {
    let config_str = fs::read_to_string("config.toml").expect("Unable to read config");
    toml::de::from_str(&config_str).expect("Failed to parse config")
}




