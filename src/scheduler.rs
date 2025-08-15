use std::sync::Arc;
use log::error;
use reqwest::Client;
use tokio::task::JoinHandle;
use crate::config::config::Config;
use crate::infrastructure::queue::queue_service::QueueService;
use crate::scheduler::block_watcher::BlockWatcher;

pub mod block_watcher;

pub struct SchedulerManager {
    tasks: Vec<JoinHandle<()>>,
    config: Arc<Config>,
    client: Arc<Client>
}

impl SchedulerManager {
    pub fn new(config: Arc<Config>, client: Arc<Client>) -> Self {
        SchedulerManager {
            tasks: Vec::new(),
            config,
            client
        }
    }

    pub async fn launch_all_tasks(&mut self, queue_service: Option<Arc<QueueService>>) {
        let client_for_block_watcher = Arc::clone(&self.client);
        let config_for_block_watcher = Arc::clone(&self.config);
        let mut block_watcher = BlockWatcher::new(client_for_block_watcher, config_for_block_watcher, queue_service);

        let block_watcher_task = tokio::spawn(async move {
            let block_watcher_result = block_watcher.start_monitoring_new_blocks().await;

            if let Err(err) = block_watcher_result {
                error!("Scheduler Manager Error: {}", err.to_string());
            }
        });

        self.tasks.push(block_watcher_task);
    }

    pub async fn wait_for_all_tasks(&mut self) {
        for task in self.tasks.drain(..) {
            match task.await {
                Ok(()) => {}
                Err(e) if e.is_cancelled() => error!("Task was cancelled"),
                Err(e) if e.is_panic() => error!("Task panicked: {e:?}"),
                Err(e) => error!("Join error: {e:?}"),
            }
        }
    }
}