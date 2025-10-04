use anyhow::Result;
use std::io::{self, Write};
use std::time::Duration;

pub struct CustomGenerator;

impl CustomGenerator {
    pub fn send_text<W: Write>(port: &mut W) -> Result<()> {
        print!("\nEnter text to send (will add \\n): ");
        io::stdout().flush()?;
        let mut text = String::new();
        io::stdin().read_line(&mut text)?;

        let data = format!("{}\n", text.trim());
        port.write_all(data.as_bytes())?;
        port.flush()?;
        println!("Sent: {}", data.trim());

        Ok(())
    }

    pub fn send_hex<W: Write>(port: &mut W) -> Result<()> {
        print!("\nEnter hex bytes (space separated, e.g., 02 1A FF 3C): ");
        io::stdout().flush()?;
        let mut hex_input = String::new();
        io::stdin().read_line(&mut hex_input)?;

        let bytes: Result<Vec<u8>, _> = hex_input
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16))
            .collect();

        match bytes {
            Ok(data) => {
                port.write_all(&data)?;
                port.flush()?;
                println!("Sent {} bytes: {:02X?}", data.len(), data);
            }
            Err(e) => println!("Error parsing hex: {}", e),
        }

        Ok(())
    }

    pub fn send_continuous<W: Write>(port: &mut W) -> Result<()> {
        println!("Sending continuous random data...");
        println!("Press Ctrl+C to stop\n");

        let mut rng = 0u32;
        for i in 0..1000 {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let value = (rng % 256) as u8;

            port.write_all(&[value])?;

            if i % 100 == 0 {
                port.flush()?;
                println!("Sent {} bytes...", i);
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        println!("\nSent 1000 bytes total.");
        Ok(())
    }
}
