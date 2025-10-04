use super::{Commands, UI};
use anyhow::Result;

pub fn run_cli_mode() -> Result<()> {
    UI::print_header();

    loop {
        UI::print_main_menu();

        let input = UI::prompt("\nSelect option: ");

        match input.as_str() {
            "1" => Commands::list_ports()?,
            "2" => Commands::test_port()?,
            "3" => Commands::connect_and_read()?,
            "4" => Commands::send_fake_data()?,
            "5" => Commands::generate_command()?,
            "q" | "Q" => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }

    Ok(())
}
