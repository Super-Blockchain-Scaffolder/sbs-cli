use std::error::Error;

use inquire::{InquireError, Text, required, length};

pub fn ask_what_to_call_named_dir() -> Result<String, Box<dyn Error>> {
    Ok(Text::new("Ok, we'll create a new directory here for the project.\nWhat would you like to name the directory?")
        .with_validator(required!())
        .prompt()?)
}
