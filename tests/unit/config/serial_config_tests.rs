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

#[test]
fn test_config_all_common_baud_rates() {
    let baud_rates = vec![
        300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400,
    ];

    for baud in baud_rates {
        let config = SerialConfig::new(baud, 8, "none", 1).unwrap();
        assert_eq!(config.baud, baud);
    }
}

#[test]
fn test_config_invalid_stop_bits() {
    assert!(SerialConfig::new(9600, 8, "none", 0).is_err());
    assert!(SerialConfig::new(9600, 8, "none", 3).is_err());
}

#[test]
fn test_config_case_insensitive_parity() {
    assert!(SerialConfig::new(9600, 8, "NONE", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "ODD", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "EVEN", 1).is_ok());
    assert!(SerialConfig::new(9600, 8, "None", 1).is_ok());
}

#[test]
fn test_config_from_string_whitespace() {
    // Should fail with whitespace
    let result = SerialConfig::from_string("115200, 0, 8, 1");
    assert!(result.is_err());
}

#[test]
fn test_config_from_string_empty() {
    let result = SerialConfig::from_string("");
    assert!(result.is_err());
}

#[test]
fn test_config_from_string_too_many_fields() {
    let result = SerialConfig::from_string("115200,0,8,1,extra");
    assert!(result.is_err());
}

#[test]
fn test_config_clone() {
    let config1 = SerialConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.baud, config2.baud);
}

#[test]
fn test_parse_data_bits_all_values() {
    for bits in [5, 6, 7, 8] {
        let config = SerialConfig::from_string(&format!("9600,0,{},1", bits));
        assert!(config.is_ok());
    }
}

#[test]
fn test_invalid_data_bits_in_string() {
    let result = SerialConfig::from_string("9600,0,4,1");
    assert!(result.is_err());
}

#[test]
fn test_invalid_stop_bits_in_string() {
    let result = SerialConfig::from_string("9600,0,8,3");
    assert!(result.is_err());
}

#[test]
fn test_parse_stop_bits_both_values() {
    let config1 = SerialConfig::from_string("9600,0,8,1");
    assert!(config1.is_ok());
    
    let config2 = SerialConfig::from_string("9600,0,8,2");
    assert!(config2.is_ok());
}

#[test]
fn test_parse_parity_all_cases() {
    use vital_reader::SerialConfig;
    
    // Test all parity parsing paths
    let configs = vec![
        ("9600,0,8,1", serialport::Parity::None),
        ("9600,1,8,1", serialport::Parity::Odd),
        ("9600,2,8,1", serialport::Parity::Even),
    ];
    
    for (config_str, expected_parity) in configs {
        let config = SerialConfig::from_string(config_str).unwrap();
        assert_eq!(config.parity, expected_parity);
    }
}

#[test]
fn test_parse_invalid_parity_values() {
    use vital_reader::SerialConfig;
    
    let invalid_parities = vec!["9600,3,8,1", "9600,99,8,1", "9600,-1,8,1"];
    
    for config_str in invalid_parities {
        assert!(SerialConfig::from_string(config_str).is_err());
    }
}