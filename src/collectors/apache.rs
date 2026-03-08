use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::metrics::store::MetricsStore;

pub async fn run(_metrics: Arc<RwLock<MetricsStore>>) {
    loop {
        // Placeholder until log parsing is implemented
        tracing::info!("Apache collector heartbeat");

        sleep(Duration::from_secs(10)).await;
    }
}
