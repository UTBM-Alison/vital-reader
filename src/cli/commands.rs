use anyhow::Result;
use serialport::{DataBits, Parity, StopBits};
use std::time::Duration;

use super::UI;
use crate::data::DataParser;
use crate::fake::{CustomGenerator, Hl7Generator, VitalSignsGenerator, WaveformGenerator};
use crate::port::{PortDetector, PortSelector};

pub struct Commands;

impl Commands {
    pub fn list_ports() -> Result<()> {
        UI::print_section_header("Available Serial Ports");

        let detector = PortDetector::new();
        detector.list_ports();

        UI::wait_for_enter();
        Ok(())
    }

    pub fn test_port() -> Result<()> {
        UI::print_section_header("Test Port Connectivity");

        let port_name = PortSelector::select_interactive()?;
        let baud: u32 = UI::prompt_with_default("Enter baud rate [115200]: ", "115200")
            .parse()
            .unwrap_or(115200);

        println!("\nTesting {}...", port_name);
        match PortDetector::test_port(&port_name, baud) {
            Ok(msg) => println!("{}", msg),
            Err(msg) => println!("{}", msg),
        }

        UI::wait_for_enter();
        Ok(())
    }

    pub fn monitor_ports() -> Result<()> {
        UI::print_section_header("Monitor Port for Data Activity");

        let ports = PortDetector::get_available_ports();

        if ports.is_empty() {
            println!("No serial ports detected!");
            UI::wait_for_enter();
            return Ok(());
        }

        println!("\nAvailable ports:");
        for (idx, port) in ports.iter().enumerate() {
            println!("  [{}] {}", idx + 1, port.port_name);
        }

        let duration: u64 =
            UI::prompt_with_default("\nMonitoring duration in seconds [10]: ", "10")
                .parse()
                .unwrap_or(10);
        let baud: u32 = UI::prompt_with_default("Baud rate [115200]: ", "115200")
            .parse()
            .unwrap_or(115200);

        println!("\nMonitoring all ports for {} seconds...", duration);
        println!("────────────────────────────────────────────────────");

        let mut activity_detected = false;
        let start = std::time::Instant::now();

        while start.elapsed().as_secs() < duration {
            for port_info in &ports {
                if let Ok(mut port) = serialport::new(&port_info.port_name, baud)
                    .timeout(Duration::from_millis(100))
                    .open()
                {
                    let mut buffer = vec![0u8; 256];
                    if let Ok(n) = port.read(&mut buffer) {
                        if n > 0 {
                            activity_detected = true;
                            println!("✓ Data detected on {}: {} bytes", port_info.port_name, n);
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        println!("────────────────────────────────────────────────────");
        if !activity_detected {
            println!("No data activity detected on any port.");
        } else {
            println!("Monitoring complete.");
        }

        UI::wait_for_enter();
        Ok(())
    }

    pub fn connect_and_read() -> Result<()> {
        UI::print_section_header("Connect and Read from Port");

        let port_name = PortSelector::select_interactive()?;

        println!("\nConfiguration options:");
        println!("  [1] Use config string (e.g., 57600,0,8,1)");
        println!("  [2] Configure individually");
        let choice = UI::prompt("Select [1/2]: ");

        let (baud, data_bits, parity, stop_bits) = if choice == "1" {
            Self::get_config_from_string()?
        } else {
            Self::get_config_individually()?
        };

        println!("\n════════════════════════════════════════════════════");
        println!("Configuration:");
        println!("  Port:      {}", port_name);
        println!("  Baud:      {}", baud);
        println!("  Data bits: {:?}", data_bits);
        println!("  Parity:    {:?}", parity);
        println!("  Stop bits: {:?}", stop_bits);
        println!("════════════════════════════════════════════════════");
        println!("\nConnecting... (Press Ctrl+C to stop)");

        let mut port = serialport::new(&port_name, baud)
            .timeout(Duration::from_millis(100))
            .data_bits(data_bits)
            .parity(parity)
            .stop_bits(stop_bits)
            .open()?;

        let mut parser = DataParser::new();
        let mut buffer = vec![0u8; 1024];
        let mut total_bytes = 0u64;
        let start_time = std::time::Instant::now();

        println!("Connected! Reading data...");
        println!("────────────────────────────────────────────────────\n");

        let read_duration = Duration::from_secs(30);
        let mut last_data_time = std::time::Instant::now();
        let no_data_timeout = Duration::from_secs(5);

        loop {
            match port.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    total_bytes += n as u64;
                    last_data_time = std::time::Instant::now();
                    let timestamp = chrono::Local::now().format("%H:%M:%S%.3f").to_string();
                    parser.process_data(&buffer[..n], &timestamp);
                }
                Ok(_) => {
                    if last_data_time.elapsed() > no_data_timeout && total_bytes == 0 {
                        println!("\nNo data received after 5 seconds.");
                        println!("Check if:");
                        println!("  - Device is powered on");
                        println!("  - Correct port is selected");
                        println!("  - Baud rate matches device settings");
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    // Timeout is normal
                }
                Err(e) => {
                    println!("\nRead error: {}", e);
                    break;
                }
            }

            if start_time.elapsed() > read_duration {
                println!("\n\nReading time limit reached (30 seconds).");
                break;
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        println!("\n════════════════════════════════════════════════════");
        println!("Session Statistics:");
        println!("  Total bytes:      {}", total_bytes);
        println!("  Duration:         {:?}", start_time.elapsed());
        if total_bytes > 0 {
            println!(
                "  Average rate:     {:.2} bytes/sec",
                total_bytes as f64 / start_time.elapsed().as_secs_f64()
            );
            parser.print_stats();
        }
        println!("════════════════════════════════════════════════════");

        UI::wait_for_enter();
        Ok(())
    }

    pub fn send_fake_data() -> Result<()> {
        UI::print_section_header("Send Fake Data to Port (Testing)");

        let port_name = PortSelector::select_interactive()?;
        let baud: u32 = UI::prompt_with_default("Baud rate [115200]: ", "115200")
            .parse()
            .unwrap_or(115200);

        println!("\nFake Data Presets:");
        println!("  [1] Vital Signs (ASCII) - Simulates patient monitor");
        println!("  [2] Waveform Data (Binary) - Simulates ECG/waveform");
        println!("  [3] HL7 Complete (GE Monitor + Dräger) - Full medical data");
        println!("  [4] Custom text message");
        println!("  [5] Custom hex data");
        println!("  [6] Continuous random data stream");

        let preset = UI::prompt("\nSelect preset [1-6]: ");

        let mut port = serialport::new(&port_name, baud)
            .timeout(Duration::from_millis(1000))
            .open()?;

        println!("\n════════════════════════════════════════════════════");
        println!("Sending fake data...");
        println!("════════════════════════════════════════════════════\n");

        match preset.as_str() {
            "1" => VitalSignsGenerator::send(&mut port, 10)?,
            "2" => WaveformGenerator::send(&mut port)?,
            "3" => Hl7Generator::send(&mut port)?,
            "4" => CustomGenerator::send_text(&mut port)?,
            "5" => CustomGenerator::send_hex(&mut port)?,
            "6" => CustomGenerator::send_continuous(&mut port)?,
            _ => println!("Invalid preset selected."),
        }

        println!("\n════════════════════════════════════════════════════");
        println!("Data sending complete.");

        UI::wait_for_enter();
        Ok(())
    }

    fn get_config_from_string() -> Result<(u32, DataBits, Parity, StopBits)> {
        println!("\nExamples:");
        println!("  57600,0,8,1  (57600 baud, no parity, 8 data bits, 1 stop bit)");
        println!("  9600,2,7,1   (9600 baud, even parity, 7 data bits, 1 stop bit)");
        println!("  115200,1,8,2 (115200 baud, odd parity, 8 data bits, 2 stop bits)");

        let config_str = UI::prompt("\nConfig string: ");
        let parts: Vec<&str> = config_str.split(',').collect();

        if parts.len() != 4 {
            println!("Error: Invalid config format!");
            return Err(anyhow::anyhow!("Invalid config format"));
        }

        let baud: u32 = parts[0].parse().unwrap_or(115200);
        let parity_num: u8 = parts[1].parse().unwrap_or(0);
        let parity = match parity_num {
            1 => Parity::Odd,
            2 => Parity::Even,
            _ => Parity::None,
        };
        let data_bits = match parts[2].parse().unwrap_or(8) {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            _ => DataBits::Eight,
        };
        let stop_bits = match parts[3].parse().unwrap_or(1) {
            2 => StopBits::Two,
            _ => StopBits::One,
        };

        Ok((baud, data_bits, parity, stop_bits))
    }

    fn get_config_individually() -> Result<(u32, DataBits, Parity, StopBits)> {
        let baud: u32 = UI::prompt_with_default("\nBaud rate [115200]: ", "115200")
            .parse()
            .unwrap_or(115200);

        let data_bits_input = UI::prompt_with_default("Data bits (5-8) [8]: ", "8");
        let data_bits = match data_bits_input.parse().unwrap_or(8) {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            _ => DataBits::Eight,
        };

        let parity_input = UI::prompt_with_default("Parity (none/odd/even) [none]: ", "none");
        let parity = match parity_input.to_lowercase().as_str() {
            "odd" | "o" => Parity::Odd,
            "even" | "e" => Parity::Even,
            _ => Parity::None,
        };

        let stop_bits_input = UI::prompt_with_default("Stop bits (1/2) [1]: ", "1");
        let stop_bits = match stop_bits_input.parse().unwrap_or(1) {
            2 => StopBits::Two,
            _ => StopBits::One,
        };

        Ok((baud, data_bits, parity, stop_bits))
    }

    pub fn generate_command() -> Result<()> {
        UI::print_section_header("Generate Command for Listener");

        // Check if vital-reader is in PATH
        let in_path = Self::check_if_in_path();

        if !in_path {
            println!("\n⚠ 'vital-reader' is not in your system PATH");
            println!("You'll need to use the full path to the executable.");
            println!("\nCommon locations:");
            #[cfg(target_os = "windows")]
            {
                println!("  - .\\target\\release\\vital-reader.exe");
                println!("  - C:\\path\\to\\vital-reader.exe");
            }
            #[cfg(not(target_os = "windows"))]
            {
                println!("  - ./target/release/vital-reader");
                println!("  - /usr/local/bin/vital-reader");
            }
        } else {
            println!("\n✓ 'vital-reader' found in PATH");
        }

        println!("\n────────────────────────────────────────────────────");

        // Get configuration
        let port_name = PortSelector::select_interactive()?;

        println!("\nConfiguration method:");
        println!("  [1] Use config string (e.g., 57600,0,8,1)");
        println!("  [2] Configure individually");
        let choice = UI::prompt("Select [1/2]: ");

        let (config_str, show_stats) = if choice == "1" {
            println!("\nExamples:");
            println!("  57600,0,8,1  (57600 baud, no parity, 8 data bits, 1 stop bit)");
            println!("  9600,2,7,1   (9600 baud, even parity, 7 data bits, 1 stop bit)");
            let cfg = UI::prompt("\nConfig string: ");
            let stats = UI::prompt("Show statistics? (y/n) [n]: ");
            (Some(cfg), stats.to_lowercase() == "y")
        } else {
            let baud = UI::prompt_with_default("\nBaud rate [115200]: ", "115200");
            let data_bits = UI::prompt_with_default("Data bits (5-8) [8]: ", "8");
            let parity = UI::prompt_with_default("Parity (none/odd/even) [none]: ", "none");
            let stop_bits = UI::prompt_with_default("Stop bits (1/2) [1]: ", "1");
            let stats = UI::prompt("Show statistics? (y/n) [n]: ");

            // Convert to config string format
            let parity_num = match parity.to_lowercase().as_str() {
                "odd" | "o" => "1",
                "even" | "e" => "2",
                _ => "0",
            };
            let cfg = format!("{},{},{},{}", baud, parity_num, data_bits, stop_bits);
            (Some(cfg), stats.to_lowercase() == "y")
        };

        // Build the command
        let exe_name = if in_path {
            "vital-reader".to_string()
        } else {
            #[cfg(target_os = "windows")]
            {
                ".\\target\\release\\vital-reader.exe".to_string()
            }
            #[cfg(not(target_os = "windows"))]
            {
                "./target/release/vital-reader".to_string()
            }
        };

        let mut command = format!("{} --port {}", exe_name, port_name);

        if let Some(cfg) = config_str {
            command.push_str(&format!(" --config \"{}\"", cfg));
        }

        if show_stats {
            command.push_str(" --stats");
        }

        // Display the command
        println!("\n════════════════════════════════════════════════════");
        println!("Generated Command:");
        println!("════════════════════════════════════════════════════");
        println!("\n{}\n", command);
        println!("════════════════════════════════════════════════════");

        println!("\n📋 Copy the command above and run it in your terminal.");

        #[cfg(target_os = "windows")]
        {
            println!("\nWindows PowerShell example:");
            println!("  {}", command);
            println!("\nWindows CMD example:");
            println!("  {}", command);
        }

        #[cfg(not(target_os = "windows"))]
        {
            println!("\nLinux/macOS example:");
            println!("  {}", command);
        }

        if !in_path {
            println!("\n💡 Tip: Add vital-reader to your PATH to use it from anywhere:");
            #[cfg(target_os = "windows")]
            {
                println!("  1. Copy vital-reader.exe to C:\\Users\\YourName\\.local\\bin\\");
                println!("  2. Add that directory to your PATH environment variable");
            }
            #[cfg(not(target_os = "windows"))]
            {
                println!("  sudo cp ./target/release/vital-reader /usr/local/bin/");
            }
        }

        UI::wait_for_enter();
        Ok(())
    }

    fn check_if_in_path() -> bool {
        #[cfg(target_os = "windows")]
        let exe_name = "vital-reader.exe";
        #[cfg(not(target_os = "windows"))]
        let exe_name = "vital-reader";

        if let Ok(path_var) = std::env::var("PATH") {
            let paths = std::env::split_paths(&path_var);

            for path in paths {
                let exe_path = path.join(exe_name);
                if exe_path.exists() {
                    return true;
                }
            }
        }

        false
    }
}
