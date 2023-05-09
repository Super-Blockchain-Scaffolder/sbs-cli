use std::error::Error;

use inquire::Confirm;

pub fn ask_to_scaffold_in_current_dir() -> Result<bool, Box<dyn Error>> {
    print!("\n");
    Ok(
        Confirm::new("Would you like to scaffold in the current directory?")
            .with_default(false)
            .prompt()?,
    )
}
