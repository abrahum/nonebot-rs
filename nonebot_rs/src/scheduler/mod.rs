// use std::collections::HashMap;
use crate::log::{colored::*, event, Level};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Prelude for Scheduler Plugin
pub mod prelude {
    pub use crate::message::Message;
    pub use tokio_cron_scheduler::Job;
}

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

/// Config for each Job
#[derive(Debug, Deserialize)]
pub struct JobConfig {
    #[serde(flatten)]
    custom: HashMap<String, toml::Value>,
}

#[async_trait]
impl crate::Plugin for Scheduler {
    fn run(&self, _: crate::EventReceiver, _: crate::BotGetter) {
        if !self.config.disable {
            tokio::spawn(self.scheduler.start());
        }
    }

    fn plugin_name(&self) -> &'static str {
        "Scheduler"
    }

    async fn load_config(&mut self, config: toml::Value) {
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
