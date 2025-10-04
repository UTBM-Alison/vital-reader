use serialport::SerialPortInfo;

pub struct PortDetector;

impl PortDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn get_available_ports() -> Vec<SerialPortInfo> {
        serialport::available_ports().unwrap_or_default()
    }

    pub fn list_ports(&self) {
        let ports = Self::get_available_ports();
        
        if ports.is_empty() {
            println!("  No serial ports detected.");
            println!("  Common ports to check:");
            self.suggest_common_ports();
        } else {
            for (idx, port) in ports.iter().enumerate() {
                println!("  [{}] {}", idx + 1, port.port_name);
                
                match &port.port_type {
                    serialport::SerialPortType::UsbPort(info) => {
                        println!("      Type: USB");
                        if let Some(manufacturer) = &info.manufacturer {
                            println!("      Manufacturer: {}", manufacturer);
                        }
                        if let Some(product) = &info.product {
                            println!("      Product: {}", product);
                        }
                        println!("      VID:PID: {:04x}:{:04x}", info.vid, info.pid);
                    }
                    serialport::SerialPortType::PciPort => {
                        println!("      Type: PCI");
                    }
                    serialport::SerialPortType::BluetoothPort => {
                        println!("      Type: Bluetooth");
                    }
                    serialport::SerialPortType::Unknown => {
                        println!("      Type: Unknown");
                    }
                }
                println!();
            }
        }
    }

    fn suggest_common_ports(&self) {
        #[cfg(target_os = "windows")]
        {
            println!("    Windows: COM1, COM3, COM4");
        }
        
        #[cfg(target_os = "linux")]
        {
            println!("    Linux: /dev/ttyUSB0, /dev/ttyUSB1, /dev/ttyACM0");
        }
        
        #[cfg(target_os = "macos")]
        {
            println!("    macOS: /dev/cu.usbserial, /dev/cu.usbmodem");
        }
    }

    pub fn suggest_port(&self) -> Option<String> {
        let ports = Self::get_available_ports();
        
        for port in &ports {
            if let serialport::SerialPortType::UsbPort(_) = port.port_type {
                return Some(port.port_name.clone());
            }
        }
        
        ports.first().map(|p| p.port_name.clone())
    }

    pub fn test_port(port_name: &str, baud: u32) -> Result<String, String> {
        match serialport::new(port_name, baud)
            .timeout(std::time::Duration::from_millis(100))
            .open()
        {
            Ok(_) => Ok(format!("✓ Port {} is accessible", port_name)),
            Err(e) => Err(format!("✗ Cannot open port {}: {}", port_name, e)),
        }
    }
}

impl Default for PortDetector {
    fn default() -> Self {
        Self::new()
    }
}