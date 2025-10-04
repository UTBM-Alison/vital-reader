#[test]
fn test_all_data_type_combinations() {
    use vital_reader::data::DataParser;
    
    let mut parser = DataParser::new();
    
    // Test with exactly 30% ASCII (Binary detection threshold)
    let mut data = vec![b'A'; 30];
    data.extend(vec![0xFF; 70]);
    parser.process_data(&data, "12:00:00");
    
    // Test with exactly 95% ASCII (ASCII detection threshold)
    let mut data = vec![b'B'; 95];
    data.extend(vec![0xFE; 5]);
    parser.process_data(&data, "12:00:00");
}

#[test]
fn test_serial_config_error_messages() {
    use vital_reader::SerialConfig;
    
    // Test specific error messages
    let result = SerialConfig::from_string("abc,0,8,1");
    assert!(result.unwrap_err().to_string().contains("Invalid baud"));
    
    let result = SerialConfig::from_string("9600,5,8,1");
    assert!(result.unwrap_err().to_string().contains("Parity must be"));
}

#[test]
fn test_port_detector_all_port_types() {
    use vital_reader::port::PortDetector;
    
    let _detector = PortDetector::new();  // Add underscore prefix
    let ports = PortDetector::get_available_ports();
    
    // Process all port types (even if empty)
    for port in ports {
        match &port.port_type {
            serialport::SerialPortType::UsbPort(_) => {},
            serialport::SerialPortType::PciPort => {},
            serialport::SerialPortType::BluetoothPort => {},
            serialport::SerialPortType::Unknown => {},
        }
    }
}