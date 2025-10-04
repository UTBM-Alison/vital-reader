// Library entry point - exports all public modules

pub mod cli;
pub mod config;
pub mod data;
pub mod fake;
pub mod port;
pub mod reader;

// Re-export commonly used types
pub use config::SerialConfig;
pub use data::{DataFormatter, DataParser};
pub use port::{PortDetector, PortSelector};
pub use reader::ReaderSession;
