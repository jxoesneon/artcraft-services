//! Integration test for Services monitor.

use artcraft_services_ui_martensite::ServiceMonitorApp;

#[test]
fn test_services_workflow() {
    let mut app = ServiceMonitorApp::new();
    assert!(app.monitor.connected);
    app.monitor.update_metrics(8, 0);
    assert_eq!(app.monitor.active_workers, 8);
}
