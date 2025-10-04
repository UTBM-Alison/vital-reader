use crate::config::SerialConfig;
use anyhow::{Context, Result};
use serialport::{FlowControl, SerialPort};
use std::time::Duration;

/// Manages serial port connection
pub struct PortConnection {
    port: Box<dyn SerialPort>,
}

impl PortConnection {
    /// Open a serial port with the given configuration
    pub fn open(port_name: &str, config: &SerialConfig, timeout_ms: u64) -> Result<Self> {
        let port = serialport::new(port_name, config.baud)
            .timeout(Duration::from_millis(timeout_ms))
            .data_bits(config.data_bits)
            .parity(config.parity)
            .stop_bits(config.stop_bits)
            .flow_control(FlowControl::None)
            .open()
            .context(format!("Failed to open port {}", port_name))?;

        Ok(Self { port })
    }

    /// Read data from the port
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        match self.port.read(buffer) {
            Ok(n) => Ok(n),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(0),
            Err(e) => Err(anyhow::anyhow!("Read error: {}", e)),
        }
    }

    /// Write data to the port
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.port.write(data).context("Failed to write to port")
    }

    /// Flush the port
    pub fn flush(&mut self) -> Result<()> {
        self.port.flush().context("Failed to flush port")
    }

    /// Check if port is still valid
    pub fn is_connected(&self) -> bool {
        // Simple check - in real impl, could ping the port
        true
    }

    /// Get the port name
    pub fn name(&self) -> Option<String> {
        self.port.name()
    }
}
