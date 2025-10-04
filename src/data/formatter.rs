use super::DataType;

/// Formats data for display
pub struct DataFormatter;

impl DataFormatter {
    /// Check if a byte is printable ASCII
    pub fn is_printable_ascii(byte: u8) -> bool {
        matches!(byte, 9 | 10 | 13 | 32..=126)
    }

    /// Format ASCII line for display
    pub fn format_ascii(data: &[u8], timestamp: &str) -> Option<String> {
        if let Ok(text) = String::from_utf8(data.to_vec()) {
            // Remove carriage returns and other control chars, keep only text
            let text = text
                .trim_end_matches('\r')
                .trim_end_matches('\n')
                .trim_end();
            if !text.is_empty() {
                return Some(format!("[{}] ASCII: {}", timestamp, text));
            }
        }
        None
    }

    /// Format binary line for display
    pub fn format_binary(data: &[u8], timestamp: &str) -> String {
        let hex_string: String = data
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "[{}] BINARY: [{}] ({} bytes)",
            timestamp,
            hex_string,
            data.len()
        )
    }

    /// Format mixed ASCII/binary line for display
    pub fn format_mixed(data: &[u8], timestamp: &str) -> String {
        let mut output = String::new();
        let mut hex_buffer = Vec::new();

        for &byte in data {
            // Skip displaying \r and \n in mixed mode
            if byte == b'\r' || byte == b'\n' {
                continue;
            }

            if Self::is_printable_ascii(byte) {
                // Flush any pending hex
                if !hex_buffer.is_empty() {
                    output.push_str(&format!("[{:02X?}]", hex_buffer));
                    hex_buffer.clear();
                }
                output.push(byte as char);
            } else {
                hex_buffer.push(byte);
            }
        }

        // Flush remaining hex
        if !hex_buffer.is_empty() {
            output.push_str(&format!("[{:02X?}]", hex_buffer));
        }

        format!("[{}] MIXED: {}", timestamp, output.trim_end())
    }

    /// Format data based on detected type
    pub fn format_data(data: &[u8], data_type: &DataType, timestamp: &str) -> Option<String> {
        match data_type {
            DataType::Ascii => Self::format_ascii(data, timestamp),
            DataType::Binary => Some(Self::format_binary(data, timestamp)),
            DataType::Mixed => Some(Self::format_mixed(data, timestamp)),
        }
    }
}
