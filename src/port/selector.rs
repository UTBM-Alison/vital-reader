use super::PortDetector;
use anyhow::Result;
use std::io::{self, Write};

/// Interactive port selection
pub struct PortSelector;

impl PortSelector {
    /// Select a port interactively with a numbered menu
    pub fn select_interactive() -> Result<String> {
        let detector = PortDetector::new();
        let ports = PortDetector::get_available_ports();

        if ports.is_empty() {
            println!("\n⚠ No serial ports detected!");
            detector.list_ports();
            return Self::prompt_manual_entry();
        }

        println!("\nAvailable ports:");
        for (idx, port) in ports.iter().enumerate() {
            println!("  [{}] {}", idx + 1, port.port_name);
            match &port.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    if let Some(product) = &info.product {
                        println!("      → {}", product);
                    }
                }
                serialport::SerialPortType::BluetoothPort => {
                    println!("      → Bluetooth Serial Port");
                }
                _ => {}
            }
        }
        println!("  [0] Enter custom port name");

        print!("\nSelect port [1-{}]: ", ports.len());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice: usize = input.trim().parse().unwrap_or(0);

        if choice == 0 {
            Self::prompt_manual_entry()
        } else if choice > 0 && choice <= ports.len() {
            Ok(ports[choice - 1].port_name.clone())
        } else {
            println!("Invalid selection, using first port");
            Ok(ports[0].port_name.clone())
        }
    }

    /// Prompt user to manually enter a port name
    fn prompt_manual_entry() -> Result<String> {
        print!("Enter port name: ");
        io::stdout().flush()?;
        let mut port_name = String::new();
        io::stdin().read_line(&mut port_name)?;
        Ok(port_name.trim().to_string())
    }

    /// Get a port name, either from CLI arg or interactive selection
    pub fn get_port(cli_port: Option<&str>) -> Result<String> {
        if let Some(port) = cli_port {
            Ok(port.to_string())
        } else {
            Self::select_interactive()
        }
    }
}
