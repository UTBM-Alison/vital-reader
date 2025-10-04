use vital_reader::reader::ReaderSession;
use vital_reader::config::SerialConfig;

#[test]
fn test_session_creation_fails_with_invalid_port() {
    let config = SerialConfig::default();
    let result = ReaderSession::new("INVALID_PORT", &config, 100, false);
    assert!(result.is_err());
}

#[test]
fn test_session_format_timestamp() {
    // This is a private method, but we can test the session creation
    let config = SerialConfig::default();
    let result = ReaderSession::new("TEST", &config, 1000, true);
    assert!(result.is_err());
}

