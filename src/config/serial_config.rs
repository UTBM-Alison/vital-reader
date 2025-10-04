use anyhow::{Context, Result};
use serialport::{DataBits, Parity, StopBits};

/// Serial port configuration
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub baud: u32,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
}

impl SerialConfig {
    /// Create a new serial configuration
    pub fn new(baud: u32, data_bits: u8, parity: &str, stop_bits: u8) -> Result<Self> {
        Ok(Self {
            baud,
            data_bits: Self::parse_data_bits(data_bits)?,
            parity: Self::parse_parity(parity)?,
            stop_bits: Self::parse_stop_bits(stop_bits)?,
        })
    }

    /// Parse config from string format: "baud,parity,data_bits,stop_bits"
    /// Example: "115200,0,8,1"
    /// Parity: 0=none, 1=odd, 2=even
    pub fn from_string(config: &str) -> Result<Self> {
        let parts: Vec<&str> = config.split(',').collect();
        
        if parts.len() != 4 {
            return Err(anyhow::anyhow!(
                "Invalid config format. Expected: baud,parity,data_bits,stop_bits (e.g., 57600,0,8,1)"
            ));
        }

        let baud: u32 = parts[0].parse()
            .context("Invalid baud rate")?;
        
        let parity_num: u8 = parts[1].parse()
            .context("Invalid parity (0=none, 1=odd, 2=even)")?;
        let parity = match parity_num {
            0 => Parity::None,
            1 => Parity::Odd,
            2 => Parity::Even,
            _ => return Err(anyhow::anyhow!("Parity must be 0 (none), 1 (odd), or 2 (even)")),
        };

        let data_bits_num: u8 = parts[2].parse()
            .context("Invalid data bits")?;
        let data_bits = Self::parse_data_bits(data_bits_num)?;

        let stop_bits_num: u8 = parts[3].parse()
            .context("Invalid stop bits")?;
        let stop_bits = Self::parse_stop_bits(stop_bits_num)?;

        Ok(Self {
            baud,
            data_bits,
            parity,
            stop_bits,
        })
    }

    fn parse_data_bits(bits: u8) -> Result<DataBits> {
        match bits {
            5 => Ok(DataBits::Five),
            6 => Ok(DataBits::Six),
            7 => Ok(DataBits::Seven),
            8 => Ok(DataBits::Eight),
            _ => Err(anyhow::anyhow!("Invalid data bits: {}", bits)),
        }
    }

    fn parse_parity(parity: &str) -> Result<Parity> {
        match parity.to_lowercase().as_str() {
            "none" | "n" => Ok(Parity::None),
            "odd" | "o" => Ok(Parity::Odd),
            "even" | "e" => Ok(Parity::Even),
            _ => Err(anyhow::anyhow!("Invalid parity: {}", parity)),
        }
    }

    fn parse_stop_bits(bits: u8) -> Result<StopBits> {
        match bits {
            1 => Ok(StopBits::One),
            2 => Ok(StopBits::Two),
            _ => Err(anyhow::anyhow!("Invalid stop bits: {}", bits)),
        }
    }
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baud: 115200,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_string() {
        let config = SerialConfig::from_string("115200,0,8,1").unwrap();
        assert_eq!(config.baud, 115200);
        assert_eq!(config.data_bits, DataBits::Eight);
    }

    #[test]
    fn test_invalid_config_string() {
        assert!(SerialConfig::from_string("115200,0,8").is_err());
    }
}