use std::collections::HashMap;
use super::DataFormatter;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Ascii,
    Binary,
    Mixed,
}

pub struct DataParser {
    ascii_count: u64,
    binary_count: u64,
    total_count: u64,
    line_buffer: Vec<u8>,
    detected_type: DataType,
    char_frequency: HashMap<u8, u64>,
    last_was_cr: bool,
}

impl DataParser {
    pub fn new() -> Self {
        Self {
            ascii_count: 0,
            binary_count: 0,
            total_count: 0,
            line_buffer: Vec::new(),
            detected_type: DataType::Mixed,
            char_frequency: HashMap::new(),
            last_was_cr: false,
        }
    }

    fn detect_data_type(&mut self) {
        if self.total_count < 100 {
            return;
        }

        let ascii_ratio = self.ascii_count as f64 / self.total_count as f64;
        
        self.detected_type = if ascii_ratio > 0.95 {
            DataType::Ascii
        } else if ascii_ratio < 0.3 {
            DataType::Binary
        } else {
            DataType::Mixed
        };
    }

    pub fn process_data(&mut self, data: &[u8], timestamp: &str) {
        for &byte in data {
            self.total_count += 1;
            *self.char_frequency.entry(byte).or_insert(0) += 1;

            if DataFormatter::is_printable_ascii(byte) {
                self.ascii_count += 1;
            } else {
                self.binary_count += 1;
            }

            if byte == b'\r' {
                self.line_buffer.push(byte);
                self.last_was_cr = true;
                self.flush_line(timestamp);
            } else if byte == b'\n' {
                if !self.last_was_cr {
                    self.line_buffer.push(byte);
                    self.flush_line(timestamp);
                }
                self.last_was_cr = false;
            } else {
                self.last_was_cr = false;
                self.line_buffer.push(byte);
                
                if self.line_buffer.len() > 65536 {
                    self.flush_line(timestamp);
                }
            }
        }

        self.detect_data_type();
    }

    fn flush_line(&mut self, timestamp: &str) {
        if self.line_buffer.is_empty() {
            return;
        }

        if let Some(formatted) = DataFormatter::format_data(&self.line_buffer, &self.detected_type, timestamp) {
            println!("{}", formatted);
        }

        self.line_buffer.clear();
    }

    pub fn print_stats(&self) {
        println!("\nData Analysis:");
        println!("  Total bytes:      {}", self.total_count);
        println!("  ASCII bytes:      {} ({:.1}%)", 
                 self.ascii_count, 
                 self.ascii_count as f64 / self.total_count as f64 * 100.0);
        println!("  Binary bytes:     {} ({:.1}%)", 
                 self.binary_count,
                 self.binary_count as f64 / self.total_count as f64 * 100.0);
        println!("  Detected type:    {:?}", self.detected_type);

        let mut freq_vec: Vec<_> = self.char_frequency.iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("\nMost common bytes:");
        for (byte, count) in freq_vec.iter().take(5) {
            let char_repr = if DataFormatter::is_printable_ascii(**byte) {
                format!("'{}' ", **byte as char)
            } else {
                String::new()
            };
            println!("    0x{:02X} {}: {} times ({:.1}%)",
                     byte, char_repr, count,
                     **count as f64 / self.total_count as f64 * 100.0);
        }
    }
}

impl Default for DataParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_detection() {
        let mut parser = DataParser::new();
        let data = b"Hello World\n".repeat(20);
        parser.process_data(&data, "12:00:00");
        assert_eq!(parser.detected_type, DataType::Ascii);
    }

    #[test]
    fn test_cr_line_ending() {
        let mut parser = DataParser::new();
        let data = b"Line1\rLine2\r";
        parser.process_data(data, "12:00:00");
    }

    #[test]
    fn test_crlf_line_ending() {
        let mut parser = DataParser::new();
        let data = b"Line1\r\nLine2\r\n";
        parser.process_data(data, "12:00:00");
    }
}