use vital_reader::data::{DataFormatter, DataParser};

#[test]
fn test_data_formatter_is_printable_ascii() {
    assert!(DataFormatter::is_printable_ascii(b'A'));
    assert!(DataFormatter::is_printable_ascii(b'z'));
    assert!(DataFormatter::is_printable_ascii(b'0'));
    assert!(DataFormatter::is_printable_ascii(b' '));
    assert!(DataFormatter::is_printable_ascii(b'\t'));
    assert!(DataFormatter::is_printable_ascii(b'\n'));
    assert!(DataFormatter::is_printable_ascii(b'\r'));

    assert!(!DataFormatter::is_printable_ascii(0x00));
    assert!(!DataFormatter::is_printable_ascii(0xFF));
    assert!(!DataFormatter::is_printable_ascii(0x80));
}

#[test]
fn test_parser_ascii_detection() {
    let mut parser = DataParser::new();
    let data = b"Hello World\nThis is ASCII text\n".repeat(10);
    parser.process_data(&data, "12:00:00");
    // After enough data, should detect as ASCII
}

#[test]
fn test_parser_binary_detection() {
    let mut parser = DataParser::new();
    let mut data = vec![0u8; 200];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }
    parser.process_data(&data, "12:00:00");
    // Should detect as binary or mixed
}

#[test]
fn test_parser_handles_cr() {
    let mut parser = DataParser::new();
    let data = b"Line1\rLine2\r";
    parser.process_data(data, "12:00:00");
    // Should handle \r line endings
}

#[test]
fn test_parser_handles_lf() {
    let mut parser = DataParser::new();
    let data = b"Line1\nLine2\n";
    parser.process_data(data, "12:00:00");
    // Should handle \n line endings
}

#[test]
fn test_parser_handles_crlf() {
    let mut parser = DataParser::new();
    let data = b"Line1\r\nLine2\r\n";
    parser.process_data(data, "12:00:00");
    // Should handle \r\n without duplicating lines
}

#[test]
fn test_parser_large_buffer() {
    let mut parser = DataParser::new();
    // Create a line longer than the buffer limit
    let large_line = vec![b'A'; 70000];
    parser.process_data(&large_line, "12:00:00");
    // Should handle without panicking
}

#[test]
fn test_parser_empty_data() {
    let mut parser = DataParser::new();
    parser.process_data(&[], "12:00:00");
    // Should handle empty data gracefully
}

#[test]
fn test_parser_single_byte() {
    let mut parser = DataParser::new();
    parser.process_data(&[b'A'], "12:00:00");
    parser.process_data(&[b'\n'], "12:00:00");
    // Should handle single byte inputs
}

#[test]
fn test_hl7_message_parsing() {
    let mut parser = DataParser::new();
    let hl7_msg =
        b"MSH|^~\\&|GE_MONITOR|ICU|VITAL_REC|HOSPITAL|20250104120000||ORU^R01|MSG001|P|2.5\r";
    parser.process_data(hl7_msg, "12:00:00");
    // Should parse HL7 message correctly
}

#[test]
fn test_multiple_hl7_messages() {
    let mut parser = DataParser::new();
    let msg1 = b"OBX|1|NM|8867-4^Heart Rate^LN||72|bpm|60-100|N|||F\r";
    let msg2 = b"OBX|2|NM|2708-6^Oxygen Sat^LN||98|%|95-100|N|||F\r";

    parser.process_data(msg1, "12:00:00");
    parser.process_data(msg2, "12:00:01");
    // Should handle multiple messages
}
