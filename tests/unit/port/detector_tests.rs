use vital_reader::port::PortDetector;

#[test]
fn test_port_detector_new() {
    let detector = PortDetector::new();
    assert!(std::mem::size_of_val(&detector) == 0);
}

#[test]
fn test_port_detector_default() {
    let _detector = PortDetector::default();
}

#[test]
fn test_get_available_ports() {
    let _ports = PortDetector::get_available_ports();
}

#[test]
fn test_list_ports_no_panic() {
    let detector = PortDetector::new();
    detector.list_ports();
}

#[test]
fn test_suggest_port() {
    let detector = PortDetector::new();
    let result = detector.suggest_port();
    if let Some(port) = result {
        assert!(!port.is_empty());
    }
}

#[test]
fn test_test_port_invalid() {
    let result = PortDetector::test_port("INVALID_PORT_12345", 115200);
    assert!(result.is_err());
}

#[test]
fn test_test_port_various_bauds() {
    for baud in [9600, 19200, 38400, 57600, 115200] {
        let result = PortDetector::test_port("NONEXISTENT", baud);
        assert!(result.is_err());
    }
}

#[test]
fn test_port_names_edge_cases() {
    for name in ["", " ", "COM", "/dev/", "/dev/null"] {
        let _result = PortDetector::test_port(name, 115200);
    }
}

#[test]
fn test_error_message_format() {
    let result = PortDetector::test_port("TEST_PORT", 9600);
    if let Err(msg) = result {
        assert!(msg.contains("TEST_PORT"));
        assert!(msg.starts_with("✗"));
    }
}

#[test]
fn test_thread_safety() {
    use std::thread;

    let handles: Vec<_> = (0..5)
        .map(|_| {
            thread::spawn(|| {
                let detector = PortDetector::new();
                let _ = detector.suggest_port();
                let _ = PortDetector::get_available_ports();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_ports() {
    for port in ["COM1", "COM10", "COM256"] {
        let _result = PortDetector::test_port(port, 115200);
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_linux_ports() {
    for port in ["/dev/ttyUSB0", "/dev/ttyACM0", "/dev/ttyS0"] {
        let _result = PortDetector::test_port(port, 115200);
    }
}

#[test]
fn test_extreme_baud_rates() {
    for baud in [1, 300, 921600, 3000000] {
        let result = PortDetector::test_port("FAKE", baud);
        assert!(result.is_err());
    }
}

#[test]
fn test_multiple_detector_instances() {
    let _d1 = PortDetector::new();
    let _d2 = PortDetector::new();
    let _d3 = PortDetector::default();
}

#[test]
fn test_suggest_common_ports_indirectly() {
    let detector = PortDetector::new();
    detector.list_ports();
}

#[test]
fn test_detector_suggest_common_ports() {
    use vital_reader::port::PortDetector;
    
    let detector = PortDetector::new();
    // This will print suggested common ports based on OS
    detector.list_ports();
}

#[test]
fn test_test_port_with_timeout() {
    use vital_reader::port::PortDetector;
    
    // Test various invalid ports
    let ports = vec!["", " ", "INVALID", "/dev/null", "COM999"];
    
    for port in ports {
        let result = PortDetector::test_port(port, 9600);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot open"));
    }
}
