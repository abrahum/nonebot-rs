// use std::collections::HashMap;
use crate::log::{colored::*, event, Level};
use serde::Deserialize;
use std::collections::HashMap;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Scheduler Plugin struct
pub struct Scheduler {
    scheduler: JobScheduler,
    config: SchedulerConfig,
}

impl std::fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schduler")
            .field("config", &self.config)
            .finish()
    }
}

/// Scheduler Plugin Config struct
#[derive(Debug, Deserialize)]
pub struct SchedulerConfig {
    #[serde(default)]
    disable: bool,
    #[serde(flatten)]
    jobs: HashMap<String, JobConfig>,
}

#[derive(Debug, Deserialize)]
pub struct JobConfig {
    #[serde(default)]
    disable: bool,
    #[serde(flatten)]
    custom: HashMap<String, String>,
}

impl crate::Plugin for Scheduler {
    fn run(&self, _: crate::EventReceiver, _: crate::BotGettter) {
        tokio::spawn(self.scheduler.start());
    }

    fn plugin_name(&self) -> &'static str {
        "Scheduler"
    }

    fn load_config(&mut self, config: toml::Value) {
        self.config = config.try_into().expect("Scheduler load config fail");
        event!(
            Level::INFO,
            "[{}] Loaded config {:?}",
            self.plugin_name().red(),
            self.config
        );
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            scheduler: JobScheduler::new(),
            config: SchedulerConfig {
                disable: false,
                jobs: HashMap::new(),
            },
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.scheduler.add(job).unwrap();
    }
}
