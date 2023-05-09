use std::error::Error;

use inquire::{InquireError, Text, required, length};

pub fn ask_what_to_call_named_dir() -> Result<String, Box<dyn Error>> {
    print!("\nOk, we'll create a new directory here for the project.\n\n");
    Ok(Text::new("What would you like to name the directory?")
        .with_validator(required!())
        .prompt()?)
}
