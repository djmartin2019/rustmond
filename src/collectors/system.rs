use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use sysinfo::System;

use crate::metrics::store::MetricsStore;

pub async fn run(metrics: Arc<RwLock<MetricsStore>>) {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        let cpu = system.global_cpu_info().cpu_usage();
        let used_mem = system.used_memory();
        let total_mem = system.total_memory();

        {
            let mut store = metrics.write().await;
            store.cpu_usage = cpu;
            store.memory_used = used_mem;
            store.memory_total = total_mem;
        }

        sleep(Duration::from_secs(5)).await;
    }
}


