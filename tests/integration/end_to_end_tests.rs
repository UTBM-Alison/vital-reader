use vital_reader::config::SerialConfig;
use vital_reader::data::DataParser;

#[test]
fn test_config_to_parser_flow() {
    // Test complete flow: config creation -> parser creation -> data processing
    let config = SerialConfig::from_string("115200,0,8,1").unwrap();
    assert_eq!(config.baud, 115200);

    let mut parser = DataParser::new();
    let test_data = b"Test data flow\r";
    parser.process_data(test_data, "12:00:00");
}

#[test]
fn test_hl7_parsing_workflow() {
    let mut parser = DataParser::new();

    // Simulate receiving multiple HL7 messages - use slices instead of arrays
    let messages: Vec<&[u8]> = vec![
        b"MSH|^~\\&|GE_MONITOR|ICU|VITAL_REC|HOSPITAL|20250104120000||ORU^R01|MSG001|P|2.5\r",
        b"PID|1||123456^^^HOSPITAL^MR||DOE^JOHN^A||19800515|M\r",
        b"OBX|1|NM|8867-4^Heart Rate^LN||72|bpm|60-100|N|||F\r",
    ];

    for msg in messages {
        parser.process_data(msg, "12:00:00");
    }
}

#[test]
fn test_multiple_config_formats() {
    let configs = vec![
        ("115200,0,8,1", 115200),
        ("57600,0,8,1", 57600),
        ("9600,2,7,1", 9600),
    ];

    for (config_str, expected_baud) in configs {
        let config = SerialConfig::from_string(config_str).unwrap();
        assert_eq!(config.baud, expected_baud);
    }
}

#[test]
fn test_data_processing_pipeline() {
    let mut parser = DataParser::new();

    // ASCII data
    parser.process_data(b"ASCII Line 1\r", "12:00:00");
    parser.process_data(b"ASCII Line 2\n", "12:00:01");

    // Binary data
    parser.process_data(&[0x00, 0xFF, 0x80], "12:00:02");

    // Mixed data
    parser.process_data(b"Mixed\x00\xFF\r", "12:00:03");
}
