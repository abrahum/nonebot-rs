use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static CONFIG_PATH: &str = "Nonebotrs.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct NbConfig {
    global: GlobalConfig,
    bots: Option<HashMap<String, BotConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    host: std::net::Ipv4Addr,
    port: u16,
    debug: bool,
    superusers: Option<Vec<String>>,
    command_start: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    superusers: Option<Vec<String>>,
    nickname: Option<Vec<String>>,
    command_start: Option<Vec<String>>,
}

impl Default for NbConfig {
    fn default() -> Self {
        NbConfig {
            global: GlobalConfig {
                host: std::net::Ipv4Addr::new(127, 0, 0, 1),
                port: 8080,
                debug: true,
                superusers: None,
                command_start: None,
            },
            bots: None,
        }
    }
}

impl NbConfig {
    pub fn load() -> Self {
        let config: NbConfig;
        let config_pathbuf = std::path::PathBuf::from(&CONFIG_PATH);
        if !config_pathbuf.exists() {
            config = NbConfig::default();
            let config_string = toml::to_string(&config).unwrap();
            std::fs::write(&config_pathbuf, &config_string).unwrap();
        } else {
            let config_string = std::fs::read_to_string(&config_pathbuf).unwrap();
            config = toml::from_str(&config_string).unwrap();
        }
        config
    }
}
