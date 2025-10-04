use serialport::{DataBits, Parity, StopBits};
use vital_reader::SerialConfig;

#[test]
fn test_config_from_string_valid() {
    let config = SerialConfig::from_string("115200,0,8,1").unwrap();
    assert_eq!(config.baud, 115200);
    assert_eq!(config.data_bits, DataBits::Eight);
    assert_eq!(config.parity, Parity::None);
    assert_eq!(config.stop_bits, StopBits::One);
}

#[test]
fn test_config_from_string_with_odd_parity() {
    let config = SerialConfig::from_string("57600,1,8,1").unwrap();
    assert_eq!(config.baud, 57600);
    assert_eq!(config.parity, Parity::Odd);
}

#[test]
fn test_config_from_string_with_even_parity() {
    let config = SerialConfig::from_string("9600,2,7,1").unwrap();
    assert_eq!(config.baud, 9600);
    assert_eq!(config.parity, Parity::Even);
    assert_eq!(config.data_bits, DataBits::Seven);
}

#[test]
fn test_config_from_string_two_stop_bits() {
    let config = SerialConfig::from_string("19200,0,8,2").unwrap();
    assert_eq!(config.stop_bits, StopBits::Two);
}

#[test]
fn test_config_from_string_invalid_format() {
    let result = SerialConfig::from_string("115200,0,8");
    assert!(result.is_err());
}

#[test]
fn test_config_from_string_invalid_baud() {
    let result = SerialConfig::from_string("invalid,0,8,1");
    assert!(result.is_err());
}

#[test]
fn test_config_from_string_invalid_parity() {
    let result = SerialConfig::from_string("115200,5,8,1");
    assert!(result.is_err());
}

#[test]
fn test_config_new() {
    let config = SerialConfig::new(115200, 8, "none", 1).unwrap();
    assert_eq!(config.baud, 115200);
    assert_eq!(config.data_bits, DataBits::Eight);
    assert_eq!(config.parity, Parity::None);
}

#[test]
fn test_config_default() {
    let config = SerialConfig::default();
    assert_eq!(config.baud, 115200);
    assert_eq!(config.data_bits, DataBits::Eight);
    assert_eq!(config.parity, Parity::None);
    assert_eq!(config.stop_bits, StopBits::One);
}

#[test]
fn test_config_various_data_bits() {
    assert!(SerialConfig::new(9600, 5, "none", 1).is_ok());
    assert!(SerialConfig::new(9600, 6, "none", 1).is_ok());
    assert!(SerialConfig::new(9600, 7, "none", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "none", 1).is_ok());
    assert!(SerialConfig::new(9600, 9, "none", 1).is_err());
}

#[test]
fn test_config_parity_variations() {
    assert!(SerialConfig::new(9600, 8, "none", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "n", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "odd", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "o", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "even", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "e", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "invalid", 1).is_err());
}
