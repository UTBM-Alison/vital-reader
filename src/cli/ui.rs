use std::io::{self, Write};

pub struct UI;

impl UI {
    pub fn print_header() {
        println!("\n╔════════════════════════════════════════════════════╗");
        println!("║     VITAL READER - INTERACTIVE CLI MODE           ║");
        println!("╚════════════════════════════════════════════════════╝\n");
    }

    pub fn print_main_menu() {
        println!("\n╔════════════════════════════════════════════════════╗");
        println!("║  Main Menu                                         ║");
        println!("╠════════════════════════════════════════════════════╣");
        println!("║  [1] List available serial ports                   ║");
        println!("║  [2] Test port connectivity                        ║");
        println!("║  [3] Connect and read from port                    ║");
        println!("║  [4] Send fake data to port (testing)              ║");
        println!("║  [5] Generate command for listener                 ║");
        println!("║  [q] Quit                                          ║");
        println!("╚════════════════════════════════════════════════════╝");
    }

    pub fn print_section_header(title: &str) {
        println!("\n════════════════════════════════════════════════════");
        println!("{}", title);
        println!("════════════════════════════════════════════════════");
    }

    pub fn prompt(message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn prompt_with_default(message: &str, default: &str) -> String {
        let input = Self::prompt(message);
        if input.is_empty() {
            default.to_string()
        } else {
            input
        }
    }

    pub fn wait_for_enter() {
        println!("\nPress Enter to continue...");
        let mut _input = String::new();
        let _ = io::stdin().read_line(&mut _input);
    }
}
