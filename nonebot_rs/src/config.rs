use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// nbrs 配置文件名
pub static CONFIG_PATH: &str = "Nonebotrs.toml";

/// nbrs 配置项结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NbConfig {
    /// 全局配置
    pub global: GlobalConfig,
    /// bot 配置
    pub bots: Option<HashMap<String, BotConfig>>,
    /// Matcher 配置
    pub matchers: Option<HashMap<String, serde_json::Value>>,
    /// Schedule Job 配置
    pub jobs: Option<HashMap<String, serde_json::Value>>,
}

/// nbrs 全局配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    /// Host
    pub host: std::net::Ipv4Addr,
    /// Port
    pub port: u16,
    /// Debug 模式
    pub debug: bool,
    /// Trace 模式
    pub trace: Option<bool>,
    /// 全局管理员账号设置
    pub superusers: Vec<String>,
    /// 全局昵称设置
    pub nicknames: Vec<String>,
    /// 全局命令起始符设置
    pub command_starts: Vec<String>,
}

/// nbrs bot 配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotConfig {
    /// bot id
    #[serde(skip)]
    pub bot_id: String,
    /// 管理员账号设置
    pub superusers: Vec<String>,
    /// 昵称设置
    pub nicknames: Vec<String>,
    /// 命令起始符设置
    pub command_starts: Vec<String>,
}

impl Default for BotConfig {
    fn default() -> Self {
        BotConfig {
            bot_id: String::new(),
            superusers: vec![],
            nicknames: vec![],
            command_starts: vec![],
        }
    }
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
                nicknames: vec![],
                command_starts: vec!["/".to_string()],
            },
            bots: None,
            matchers: None,
            jobs: None,
        }
    }
}

impl NbConfig {
    /// 从配置文件读取配置
    pub fn load() -> Self {
        use colored::*;
        let config: NbConfig;
        let config_pathbuf = std::path::PathBuf::from(&CONFIG_PATH);
        if !config_pathbuf.exists() {
            config = NbConfig::default();
            let config_string = toml::to_string(&config).unwrap();
            std::fs::write(&config_pathbuf, &config_string).unwrap();
            println!("{}", "未发现配置文件，已新建配置文件。".green())
        } else {
            let config_string = std::fs::read_to_string(&config_pathbuf).unwrap();
            match toml::from_str(&config_string) {
                Ok(config_data) => {
                    config = config_data;
                    println!("{}", "载入配置成功！".bright_green());
                }
                Err(e) => {
                    println!("{} -> {}", "载入配置失败！".bright_red(), e);
                    std::process::exit(101);
                }
            };
        }
        config
    }

    pub fn get_matcher_config(&self, matcher_name: &str) -> Option<&serde_json::Value> {
        if let Some(matchers_config) = &self.matchers {
            matchers_config.get(matcher_name)
        } else {
            None
        }
    }

    pub fn get_job_config(&self, job_name: &str) -> Option<&serde_json::Value> {
        if let Some(job_config) = &self.jobs {
            job_config.get(job_name)
        } else {
            None
        }
    }

    /// 生成 BotConfig
    pub fn gen_bot_config(&self, bot_id: &str) -> BotConfig {
        let mut rbotconfig = BotConfig {
            bot_id: bot_id.to_string(),
            superusers: self.global.superusers.clone(),
            nicknames: self.global.nicknames.clone(),
            command_starts: self.global.command_starts.clone(),
        };
        if let Some(bots_config) = &self.bots {
            if let Some(bot_config) = bots_config.get(bot_id) {
                if !bot_config.superusers.is_empty() {
                    rbotconfig.superusers = bot_config.superusers.clone();
                }
                if !bot_config.nicknames.is_empty() {
                    rbotconfig.nicknames = bot_config.nicknames.clone();
                }
                if !bot_config.command_starts.is_empty() {
                    rbotconfig.command_starts = bot_config.command_starts.clone();
                }
            }
        }
        rbotconfig
    }
}
