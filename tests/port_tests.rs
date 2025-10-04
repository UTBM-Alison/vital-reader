use vital_reader::port::PortDetector;

#[test]
fn test_port_detector_creation() {
    let detector = PortDetector::new();
    // Should create without error
    drop(detector);
}

#[test]
fn test_get_available_ports() {
    let _ports = PortDetector::get_available_ports();
    // Should return a vector (empty or not depending on system)
    // Just verify it doesn't panic
}

#[test]
fn test_suggest_port() {
    let detector = PortDetector::new();
    let suggested = detector.suggest_port();
    // May or may not find a port depending on system
    if let Some(port) = suggested {
        assert!(!port.is_empty());
    }
}

#[test]
fn test_port_detector_list_ports() {
    let detector = PortDetector::new();
    // Should not panic
    detector.list_ports();
}
