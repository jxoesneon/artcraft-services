//! Worker pool and sync status models.

pub struct SyncMonitor {
    pub active_workers: usize,
    pub queued_renders: usize,
    pub connected: bool,
}

impl SyncMonitor {
    pub fn new() -> Self {
        Self {
            active_workers: 0,
            queued_renders: 0,
            connected: true,
        }
    }

    pub fn update_metrics(&mut self, workers: usize, queue: usize) {
        self.active_workers = workers;
        self.queued_renders = queue;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let mut m = SyncMonitor::new();
        m.update_metrics(4, 12);
        assert_eq!(m.active_workers, 4);
        assert_eq!(m.queued_renders, 12);
    }
}
