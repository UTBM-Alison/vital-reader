use vital_reader::data::DataParser;

#[test]
fn test_parser_new() {
    let parser = DataParser::new();
    drop(parser);
}

#[test]
fn test_parser_default() {
    let parser = DataParser::default();
    drop(parser);
}

#[test]
fn test_parser_empty_data() {
    let mut parser = DataParser::new();
    parser.process_data(&[], "12:00:00");
}

#[test]
fn test_parser_single_byte() {
    let mut parser = DataParser::new();
    parser.process_data(&[b'A'], "12:00:00");
    parser.process_data(&[b'\n'], "12:00:00");
}

#[test]
fn test_parser_ascii_detection() {
    let mut parser = DataParser::new();
    let data = b"Hello World\nThis is ASCII text\n".repeat(10);
    parser.process_data(&data, "12:00:00");
}

#[test]
fn test_parser_binary_detection() {
    let mut parser = DataParser::new();
    let mut data = vec![0u8; 200];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }
    parser.process_data(&data, "12:00:00");
}

#[test]
fn test_parser_handles_cr() {
    let mut parser = DataParser::new();
    let data = b"Line1\rLine2\r";
    parser.process_data(data, "12:00:00");
}

#[test]
fn test_parser_handles_lf() {
    let mut parser = DataParser::new();
    let data = b"Line1\nLine2\n";
    parser.process_data(data, "12:00:00");
}

#[test]
fn test_parser_handles_crlf() {
    let mut parser = DataParser::new();
    let data = b"Line1\r\nLine2\r\n";
    parser.process_data(data, "12:00:00");
}

#[test]
fn test_parser_large_buffer() {
    let mut parser = DataParser::new();
    let large_line = vec![b'A'; 70000];
    parser.process_data(&large_line, "12:00:00");
}

#[test]
fn test_hl7_message() {
    let mut parser = DataParser::new();
    let msg = b"MSH|^~\\&|GE_MONITOR|ICU|VITAL_REC|HOSPITAL|20250104120000||ORU^R01|MSG001|P|2.5\r";
    parser.process_data(msg, "12:00:00");
}

#[test]
fn test_multiple_hl7_messages() {
    let mut parser = DataParser::new();
    let msg1 = b"OBX|1|NM|8867-4^Heart Rate^LN||72|bpm|60-100|N|||F\r";
    let msg2 = b"OBX|2|NM|2708-6^Oxygen Sat^LN||98|%|95-100|N|||F\r";

    parser.process_data(msg1, "12:00:00");
    parser.process_data(msg2, "12:00:01");
}

#[test]
fn test_parser_print_stats() {
    let mut parser = DataParser::new();
    parser.process_data(b"test data\n", "12:00:00");
    parser.print_stats();
}

#[test]
fn test_mixed_line_endings() {
    let mut parser = DataParser::new();
    let data = b"Line1\rLine2\nLine3\r\n";
    parser.process_data(data, "12:00:00");
}

#[test]
fn test_no_line_ending() {
    let mut parser = DataParser::new();
    parser.process_data(b"No line ending here", "12:00:00");
}

#[test]
fn test_data_type_detection_pure_ascii() {
    let mut parser = DataParser::new();
    let ascii_line = b"Pure ASCII text data\n";
    let mut ascii_data = Vec::new();
    for _ in 0..15 {
        ascii_data.extend_from_slice(ascii_line);
    }
    parser.process_data(&ascii_data, "12:00:00");
}

#[test]
fn test_data_type_detection_pure_binary() {
    let mut parser = DataParser::new();
    let binary_data: Vec<u8> = (0..200).map(|i| (i % 256) as u8).collect();
    parser.process_data(&binary_data, "12:00:00");
}

#[test]
fn test_data_type_detection_mixed() {
    let mut parser = DataParser::new();
    let mut mixed_data = Vec::new();
    for _ in 0..50 {
        mixed_data.extend_from_slice(b"Text");
        mixed_data.push(0x00);
        mixed_data.push(0xFF);
    }
    parser.process_data(&mixed_data, "12:00:00");
}

#[test]
fn test_line_buffer_exactly_at_limit() {
    let mut parser = DataParser::new();
    let large_data = vec![b'A'; 65537];
    parser.process_data(&large_data, "12:00:00");
}

#[test]
fn test_consecutive_cr_characters() {
    let mut parser = DataParser::new();
    let data = b"Data\r\r\rMore\r";
    parser.process_data(data, "12:00:00");
}

#[test]
fn test_lone_lf_after_non_cr() {
    let mut parser = DataParser::new();
    let data = b"Text\nMore\n";
    parser.process_data(data, "12:00:00");
}

#[test]
fn test_parser_less_than_100_bytes() {
    let mut parser = DataParser::new();
    parser.process_data(b"Short", "12:00:00");
}

#[test]
fn test_empty_line_flush() {
    let mut parser = DataParser::new();
    parser.process_data(b"\r", "12:00:00");
}

#[test]
fn test_parser_with_all_control_chars() {
    let mut parser = DataParser::new();
    let data: Vec<u8> = (0..32).collect();
    parser.process_data(&data, "12:00:00");
}

#[test]
fn test_parser_exact_100_bytes() {
    use vital_reader::data::DataParser;
    
    let mut parser = DataParser::new();
    let data = vec![b'A'; 100];
    parser.process_data(&data, "12:00:00");
    // Should trigger type detection at exactly 100 bytes
}

#[test]
fn test_parser_mixed_ratio_boundaries() {
    use vital_reader::data::DataParser;
    
    let mut parser = DataParser::new();
    // Create data with exactly 95% ASCII (boundary case)
    let mut data = vec![b'A'; 95];
    data.extend(vec![0xFF; 5]);
    parser.process_data(&data, "12:00:00");
}

#[test]
fn test_parser_flush_with_cr_lf_combinations() {
    use vital_reader::data::DataParser;
    
    let mut parser = DataParser::new();
    
    // Test CR followed by non-LF
    parser.process_data(b"\rX", "12:00:00");
    
    // Test LF after CR flag is set
    let mut parser2 = DataParser::new();
    parser2.process_data(b"\r", "12:00:00");
    parser2.process_data(b"\n", "12:00:01");
}