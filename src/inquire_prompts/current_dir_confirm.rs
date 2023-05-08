use std::error::Error;

use inquire::{Confirm, InquireError};

pub fn ask_to_scaffold_in_current_dir() -> Result<bool, Box<dyn Error>> {
    Ok(Confirm::new("Would you like to scaffold in the current directory?")
        .with_default(false)
        // .with_help_message("I'm an example of some help text!")
        .prompt()?)
}