// use std::collections::HashMap;
use tokio_cron_scheduler::{Job, JobScheduler};

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

#[derive(Debug)]
pub struct SchedulerConfig {}

impl crate::Plugin for Scheduler {
    fn run(&self, _: crate::EventReceiver, _: crate::BotGettter) {
        tokio::spawn(self.scheduler.start());
    }
    fn plugin_name(&self) -> &'static str {
        "Scheduler"
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            scheduler: JobScheduler::new(),
            config: SchedulerConfig {},
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.scheduler.add(job).unwrap();
    }
}
