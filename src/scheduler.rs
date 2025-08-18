use std::sync::Arc;
use log::{error, info};
use reqwest::Client;
use sqlx::Acquire;
use tokio::task::JoinHandle;
use crate::config::config::Config;
use crate::infrastructure::db::postgres::Database;
use crate::infrastructure::queue::queue_service::QueueService;
use crate::scheduler::block_watcher::BlockWatcher;
use crate::scheduler::rabbit_watcher::MessageIngestionService;

pub mod block_watcher;
mod rabbit_watcher;

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

    pub async fn launch_all_tasks(&mut self, queue_service: Option<Arc<QueueService>>, db: Option<Arc<Database>>) {
        let client_for_block_watcher = Arc::clone(&self.client);
        let config_for_block_watcher = Arc::clone(&self.config);

        let config_for_rabbit_watcher = Arc::clone(&self.config);

        let queue_service_for_block = queue_service.as_ref().map(Arc::clone);
        let queue_service_for_rabbit = queue_service.as_ref().map(Arc::clone);

        let mut block_watcher = BlockWatcher::new(client_for_block_watcher, config_for_block_watcher, queue_service_for_block);
        let mut message_ingestion_service_opt = if db.is_some() {
            let db = db.unwrap();
            Some(MessageIngestionService::new(config_for_rabbit_watcher, queue_service_for_rabbit, db))
        } else {
            None
        };

        let block_watcher_task = tokio::spawn(async move {
            let block_watcher_result = block_watcher.start_monitoring_new_blocks().await;

            if let Err(err) = block_watcher_result {
                error!("Scheduler Manager Error: {}", err.to_string());
            }
        });

        if let Some(mut message_ingestion_service) = message_ingestion_service_opt {
            let rabbit_watcher_task = tokio::spawn(async move {
                let message_ingestion_service_result = message_ingestion_service.start_monitoring_rabbit_messages().await;

                if let Err(err) = message_ingestion_service_result {
                    error!("Scheduler Manager Error: {}", err.to_string());
                }
            });

            self.tasks.push(rabbit_watcher_task)
        }

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