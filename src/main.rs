use anyhow::Result;
use clap::Parser;

use vital_reader::cli::run_cli_mode;
use vital_reader::{PortDetector, ReaderSession, SerialConfig};

#[derive(Parser, Debug)]
#[command(name = "vital-reader")]
#[command(about = "Serial port data reader for medical devices (GE/Dräger scopes)", long_about = None)]
struct Args {
    /// Serial port path (e.g., COM3, /dev/ttyUSB0)
    #[arg(short, long)]
    port: Option<String>,

    /// Baud rate
    #[arg(short, long, default_value = "115200")]
    baud: u32,

    /// Data bits (5, 6, 7, 8)
    #[arg(long, default_value = "8")]
    data_bits: u8,

    /// Parity (none, odd, even)
    #[arg(long, default_value = "none")]
    parity: String,

    /// Stop bits (1, 2)
    #[arg(long, default_value = "1")]
    stop_bits: u8,

    /// Configuration string format: baud,parity,data_bits,stop_bits (e.g., "57600,0,8,1")
    /// Overrides individual settings if provided
    #[arg(short, long)]
    config: Option<String>,

    /// Enable interactive CLI mode for testing
    #[arg(long)]
    cli: bool,

    /// Show statistics (bytes received, connection time)
    #[arg(long)]
    stats: bool,

    /// Read timeout in milliseconds
    #[arg(long, default_value = "100")]
    timeout: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.cli {
        run_cli_mode()?;
    } else {
        run_reader_mode(&args)?;
    }

    Ok(())
}

fn run_reader_mode(args: &Args) -> Result<()> {
    // Get port name
    let port_name = if let Some(ref p) = args.port {
        p.clone()
    } else {
        let detector = PortDetector::new();
        println!("No port specified. Available ports:");
        detector.list_ports();

        if let Some(suggested) = detector.suggest_port() {
            println!("\nUsing suggested port: {}", suggested);
            suggested
        } else {
            return Err(anyhow::anyhow!(
                "No serial ports found. Please specify --port"
            ));
        }
    };

    // Parse configuration
    let serial_config = if let Some(ref config_str) = args.config {
        println!("Using config string: {}", config_str);
        SerialConfig::from_string(config_str)?
    } else {
        SerialConfig::new(args.baud, args.data_bits, &args.parity, args.stop_bits)?
    };

    // Print configuration
    print_configuration(&port_name, &serial_config);

    // Create and run session
    let mut session = ReaderSession::new(&port_name, &serial_config, args.timeout, args.stats)?;
    session.run()?;

    Ok(())
}

fn print_configuration(port_name: &str, config: &SerialConfig) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║      VITAL SERIAL READER v0.1.0       ║");
    println!("╚════════════════════════════════════════╝");
    println!("\nConfiguration:");
    println!("  Port:         {}", port_name);
    println!("  Baud rate:    {}", config.baud);
    println!("  Data bits:    {:?}", config.data_bits);
    println!("  Parity:       {:?}", config.parity);
    println!("  Stop bits:    {:?}", config.stop_bits);
    println!("\nPress [h] for help, [q] to quit\n");
}
