use vital_reader::data::{DataFormatter, DataType};

#[test]
fn test_is_printable_ascii_letters() {
    assert!(DataFormatter::is_printable_ascii(b'A'));
    assert!(DataFormatter::is_printable_ascii(b'z'));
}

#[test]
fn test_is_printable_ascii_numbers() {
    assert!(DataFormatter::is_printable_ascii(b'0'));
    assert!(DataFormatter::is_printable_ascii(b'9'));
}

#[test]
fn test_is_printable_ascii_space() {
    assert!(DataFormatter::is_printable_ascii(b' '));
}

#[test]
fn test_is_printable_ascii_tab() {
    assert!(DataFormatter::is_printable_ascii(b'\t'));
}

#[test]
fn test_is_printable_ascii_newline() {
    assert!(DataFormatter::is_printable_ascii(b'\n'));
}

#[test]
fn test_is_printable_ascii_carriage_return() {
    assert!(DataFormatter::is_printable_ascii(b'\r'));
}

#[test]
fn test_is_not_printable_null() {
    assert!(!DataFormatter::is_printable_ascii(0x00));
}

#[test]
fn test_is_not_printable_high_bytes() {
    assert!(!DataFormatter::is_printable_ascii(0xFF));
    assert!(!DataFormatter::is_printable_ascii(0x80));
}

#[test]
fn test_is_not_printable_control_chars() {
    assert!(!DataFormatter::is_printable_ascii(0x01));
    assert!(!DataFormatter::is_printable_ascii(0x1F));
}

#[test]
fn test_printable_range() {
    // Test boundary of printable range (32-126)
    assert!(!DataFormatter::is_printable_ascii(31));
    assert!(DataFormatter::is_printable_ascii(32));
    assert!(DataFormatter::is_printable_ascii(126));
    assert!(!DataFormatter::is_printable_ascii(127));
}

#[test]
fn test_format_ascii_simple() {
    let data = b"Hello World";
    let result = DataFormatter::format_ascii(data, "12:00:00");
    assert!(result.is_some());
    let formatted = result.unwrap();
    assert!(formatted.contains("Hello World"));
    assert!(formatted.contains("12:00:00"));
}

#[test]
fn test_format_ascii_empty() {
    let data = b"";
    let result = DataFormatter::format_ascii(data, "12:00:00");
    assert!(result.is_none());
}

#[test]
fn test_format_binary() {
    let data = &[0x02, 0x1A, 0xFF, 0x3C];
    let result = DataFormatter::format_binary(data, "12:00:00");
    assert!(result.contains("02"));
    assert!(result.contains("1A"));
    assert!(result.contains("FF"));
    assert!(result.contains("3C"));
    assert!(result.contains("4 bytes"));
}

#[test]
fn test_format_data_ascii() {
    let data = b"Test ASCII data";
    let result = DataFormatter::format_data(data, &DataType::Ascii, "12:00:00");
    assert!(result.is_some());
    assert!(result.unwrap().contains("Test ASCII data"));
}

#[test]
fn test_format_data_binary() {
    let data = &[0x00, 0xFF, 0xAA];
    let result = DataFormatter::format_data(data, &DataType::Binary, "12:00:00");
    assert!(result.is_some());
    assert!(result.unwrap().contains("BINARY"));
}

#[test]
fn test_format_data_mixed() {
    let data = b"Text\x00\xFF";
    let result = DataFormatter::format_data(data, &DataType::Mixed, "12:00:00");
    assert!(result.is_some());
    assert!(result.unwrap().contains("MIXED"));
}

#[test]
fn test_format_mixed_with_newlines() {
    let data = b"Line1\nLine2\r\nData\x00";
    let result = DataFormatter::format_mixed(data, "12:00:00");
    assert!(result.contains("MIXED"));
}

#[test]
fn test_format_ascii_with_carriage_return() {
    let data = b"Test\rData\r";
    let result = DataFormatter::format_ascii(data, "12:00:00");
    assert!(result.is_some());
}

#[test]
fn test_format_ascii_only_whitespace() {
    let data = b"   \r\n";
    let result = DataFormatter::format_ascii(data, "12:00:00");
    assert!(result.is_none());
}

#[test]
fn test_format_mixed_all_printable() {
    let data = b"All text here";
    let result = DataFormatter::format_mixed(data, "12:00:00");
    assert!(result.contains("All text here"));
}

#[test]
fn test_format_ascii_with_only_control_chars() {
    // Test what actually happens with control chars
    let data = &[0x00, 0x01, 0x02, 0x03];
    
    // First verify from_utf8 behavior
    let utf8_result = String::from_utf8(data.to_vec());
    assert!(utf8_result.is_ok(), "from_utf8 should succeed for these bytes");
    
    let result = DataFormatter::format_ascii(data, "12:00:00");
    
    // If from_utf8 succeeds and creates a non-empty string after trim, 
    // format_ascii should return Some
    // If the string is empty after trimming, it returns None
    // Control chars \x00-\x03 are not whitespace, so they won't be trimmed
    // Therefore the result should be Some
    assert!(result.is_some(), "Should return Some for valid UTF-8 control chars that don't get trimmed");
}

#[test]
fn test_format_mixed_with_only_hex() {
    let data = &[0xFF, 0xFE, 0xFD];
    let result = DataFormatter::format_mixed(data, "12:00:00");
    assert!(result.contains("["));
    assert!(result.contains("]"));
}

#[test]
fn test_format_empty_mixed() {
    let data = b"";
    let result = DataFormatter::format_mixed(data, "12:00:00");
    assert!(result.contains("MIXED"));
}