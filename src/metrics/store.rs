use std::collections::HashMap;

#[derive(Default)]
pub struct MetricsStore {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,

    pub request_count: u64,
    pub status_counts: HashMap<u16, u64>,
}

impl MetricsStore {
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_used: 0,
            memory_total: 0,
            request_count: 0,
            status_counts: HashMap::new(),
        }
    }

    pub fn record_status(&mut self, status: u16) {
        self.request_count += 1;

        let entry = self.status_counts.entry(status).or_insert(0);
        *entry += 1;
    }
}
