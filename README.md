# Vital Reader

[![codecov](https://codecov.io/gh/UTBM-Alison/vital-reader/graph/badge.svg?token=GSUrkVcR1R)](https://codecov.io/gh/UTBM-Alison/vital-reader)
[![Build Status](https://github.com/UTBM-Alison/vital-reader/workflows/CI/badge.svg)](https://github.com/UTBM-Alison/vital-reader/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cross-platform serial port data reader for medical devices, designed to capture and process HL7 data streams from GE monitors and Dräger ventilators/humidifiers.

## Features

- **Multi-device support**: GE multiparametric monitors, Dräger ventilators and humidifiers
- **Smart data detection**: Automatically identifies ASCII, binary, or mixed data formats
- **HL7 protocol**: Full support for HL7 v2.5 medical data interchange
- **Real-time monitoring**: Timestamped output with millisecond precision
- **Interactive CLI**: Easy-to-use interface for testing and configuration
- **Command builder**: Generate ready-to-use listener commands
- **Cross-platform**: Windows, Linux, and macOS support

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/UTBM-Alison/vital-reader.git
cd vital-reader

# Build release version
cargo build --release

# Run
./target/release/vital-reader --cli
```

### Basic Usage

```bash
# Interactive mode
vital-reader --cli

# Direct connection
vital-reader --port COM3 --config "115200,0,8,1"

# With statistics
vital-reader --port /dev/ttyUSB0 --baud 115200 --stats
```

## Supported Devices

### GE Multiparametric Monitor
- ECG (Heart Rate, Rhythm)
- RESP (Respiratory Rate)
- PNI (Non-invasive Blood Pressure)
- SPO2 (Oxygen Saturation)
- Temperature
- PI (Invasive Arterial Pressure)
- DC (Cardiac Output)
- CO2 (End-tidal CO2)
- BIS (Bispectral Index)
- EEG (Alpha/Beta Power)
- SvO2 (Mixed Venous Oxygen Saturation)

### Dräger Equipment
- **Ventilator**: Tidal volume, minute volume, pressures, PEEP, FiO2, I:E ratio
- **Humidifier**: Temperature, humidity, water level

## Configuration

### Serial Port Settings

The config string format: `baud,parity,data_bits,stop_bits`

**Parity values:**
- `0` = None
- `1` = Odd
- `2` = Even

**Examples:**
```bash
vital-reader --port COM3 --config "115200,0,8,1"  # 115200 8N1
vital-reader --port COM3 --config "57600,2,7,1"   # 57600 7E1
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage

# Run specific test module
cargo test config::tests
```

### Project Structure

```
vital-reader/
├── src/
│   ├── config/          # Serial configuration
│   ├── port/            # Port detection and connection
│   ├── data/            # Data parsing and formatting
│   ├── fake/            # Test data generators
│   ├── cli/             # Interactive CLI
│   └── reader/          # Session management
├── tests/               # Integration tests
└── benches/             # Performance benchmarks
```

## Roadmap

### V1 (Current)
- ✅ RS232 data reception from GE and Dräger devices
- ✅ HL7 message parsing and display
- ✅ Interactive testing tools
- ✅ Cross-platform support

### V2 (Planned)
- 🔄 Continuous background process
- 🔄 Data retransmission via BLE or REST API
- 🔄 Multi-device simultaneous monitoring
- 🔄 Data logging and archival

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust for performance and reliability
- Inspired by the VitalConnect project for serial-to-WiFi conversion
- Designed for healthcare environments requiring robust data capture

## Support

For issues, questions, or feature requests, please [open an issue](https://github.com/UTBM-Alison/vital-reader/issues) on GitHub.