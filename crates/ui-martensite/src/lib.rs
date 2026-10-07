//! Sovereign retained-mode daemon monitor for ArtCraft Services built on Martensite.

pub mod sync_state;
pub mod theme;

pub struct ServiceMonitorApp {
    pub monitor: sync_state::SyncMonitor,
}

impl ServiceMonitorApp {
    pub fn new() -> Self {
        Self {
            monitor: sync_state::SyncMonitor::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_init() {
        let app = ServiceMonitorApp::new();
        assert_eq!(app.monitor.active_workers, 0);
    }
}
