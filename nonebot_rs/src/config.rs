use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static CONFIG_PATH: &str = "Nonebotrs.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct NbConfig {
    pub global: GlobalConfig,
    pub bots: Option<HashMap<String, BotConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub host: std::net::Ipv4Addr,
    pub port: u16,
    pub debug: bool,
    pub trace: Option<bool>,
    pub superusers: Vec<String>,
    pub nickname: Vec<String>,
    pub command_start: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotConfig {
    pub superusers: Vec<String>,
    pub nickname: Vec<String>,
    pub command_start: Vec<String>,
}

impl Default for NbConfig {
    fn default() -> Self {
        NbConfig {
            global: GlobalConfig {
                host: std::net::Ipv4Addr::new(127, 0, 0, 1),
                port: 8088,
                debug: true,
                trace: None,
                superusers: vec![],
                nickname: vec![],
                command_start: vec!["/".to_string()],
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

    pub fn get_bot_config(&self, bot_id: &str) -> Option<&BotConfig> {
        if let Some(bots_config) = &self.bots {
            if let Some(bot_config) = bots_config.get(bot_id) {
                Some(bot_config)
            } else {
                None
            }
        } else {
            None
        }
    }
}
