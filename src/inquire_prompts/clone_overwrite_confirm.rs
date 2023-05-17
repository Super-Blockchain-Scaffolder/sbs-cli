use std::error::Error;

use inquire::Confirm;

pub fn ask_to_overwrite_in_current_dir() -> Result<bool, Box<dyn Error>> {
    print!("\n");
    Ok(Confirm::new("Your current directory is not empty!\n\nBss will overwrite conflicting files in your existing current directory with those in the starter template.\n\nDo you wish to continue?")
        .with_default(true)
        // .with_help_message("I'm an example of some help text!")
        .prompt()?)
}