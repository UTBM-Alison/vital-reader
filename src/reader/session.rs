use anyhow::Result;
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::time::Duration;

use super::SessionStats;
use crate::config::SerialConfig;
use crate::data::DataParser;
use crate::port::PortConnection;

pub struct ReaderSession {
    port: PortConnection,
    parser: DataParser,
    stats: SessionStats,
    show_stats: bool,
}

impl ReaderSession {
    pub fn new(
        port_name: &str,
        config: &SerialConfig,
        timeout_ms: u64,
        show_stats: bool,
    ) -> Result<Self> {
        let port = PortConnection::open(port_name, config, timeout_ms)?;

        Ok(Self {
            port,
            parser: DataParser::new(),
            stats: SessionStats::new(),
            show_stats,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        println!(
            "[{}] Connected to {}",
            Self::format_timestamp(),
            self.port.name().unwrap_or_default()
        );
        println!("────────────────────────────────────────────────────────────────");

        let mut buffer = vec![0u8; 1024];

        enable_raw_mode()?;

        let result = self.read_loop(&mut buffer);

        disable_raw_mode()?;

        if self.show_stats {
            self.print_session_stats();
        }

        result
    }

    fn read_loop(&mut self, buffer: &mut [u8]) -> Result<()> {
        loop {
            // Check for keyboard input
            if let Some(cmd) = self.check_for_input()? {
                match cmd.as_str() {
                    "QUIT" => {
                        println!("\n\n[{}] Disconnecting...", Self::format_timestamp());
                        break;
                    }
                    "HELP" => self.print_help(),
                    "SEND" => self.handle_send_command()?,
                    _ => {}
                }
            }

            // Read from serial port
            match self.port.read(buffer) {
                Ok(n) if n > 0 => {
                    self.stats.add_bytes(n);
                    self.parser
                        .process_data(&buffer[..n], &Self::format_timestamp());
                }
                Ok(_) => {
                    // No data available
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Read error: {}", e));
                }
            }

            std::thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }

    fn check_for_input(&self) -> Result<Option<String>> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => return Ok(Some("QUIT".to_string())),
                    KeyCode::Char('s') => return Ok(Some("SEND".to_string())),
                    KeyCode::Char('h') => return Ok(Some("HELP".to_string())),
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    fn print_help(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║        VITAL READER COMMANDS          ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ [q] - Quit application                ║");
        println!("║ [s] - Send command to device          ║");
        println!("║ [h] - Show this help                  ║");
        println!("╚════════════════════════════════════════╝\n");
    }

    fn handle_send_command(&mut self) -> Result<()> {
        disable_raw_mode()?;
        print!("\nEnter command to send: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if !input.is_empty() {
            self.send_command(input)?;
        }
        enable_raw_mode()?;
        Ok(())
    }

    fn send_command(&mut self, command: &str) -> Result<()> {
        let data = format!("{}\r\n", command);
        self.port.write(data.as_bytes())?;
        self.port.flush()?;
        println!("[{}] SENT: {}", Self::format_timestamp(), command);
        Ok(())
    }

    fn print_session_stats(&self) {
        println!("\n────────────────────────────────────────────────────────────────");
        println!("Statistics:");
        println!("  Total bytes received: {}", self.stats.total_bytes());
        println!("  Connection time:      {:?}", self.stats.elapsed());
        println!(
            "  Average rate:         {:.2} bytes/sec",
            self.stats.average_rate()
        );
        self.parser.print_stats();
    }

    fn format_timestamp() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }
}
