use std::sync::Arc;
use tokio::sync::RwLock;

mod config;
mod collectors;
mod metrics;
mod api;
mod lifecycle;

use metrics::store::MetricsStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let metrics = Arc::new(RwLock::new(MetricsStore::new()));

    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        collectors::system::run(metrics_clone).await;
    });

    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        collectors::apache::run(metrics_clone).await;
    });

    api::server::run(metrics).await?;

    Ok(())
}
