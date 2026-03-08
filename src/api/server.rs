use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{Router, routing::get, extract::State, Json};

use crate::metrics::store::MetricsStore;

pub async fn run(metrics: Arc<RwLock<MetricsStore>>) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/metrics/system", get(system_metrics))
        .with_state(metrics);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn system_metrics(
    State(metrics): State<Arc<RwLock<MetricsStore>>>,
) -> Json<serde_json::Value> {

    let store = metrics.read().await;

    Json(serde_json::json!({
        "cpu": store.cpu_usage,
        "memory_used": store.memory_used,
        "memory_total": store.memory_total
    }))
}
