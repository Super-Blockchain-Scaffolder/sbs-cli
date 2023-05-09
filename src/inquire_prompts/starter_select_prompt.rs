use std::error::Error;

use inquire::Select;

pub fn ask_which_starter_to_scaffold(options: &Vec<String>) -> Result<String, Box<dyn Error>> {
    print!("\n");
    let ans = Select::new(
        "Which starter template would you like to scaffold?",
        options.to_vec(),
    )
    .prompt();

    match ans {
        Ok(choice) => Ok(choice.to_string()),
        Err(_) => panic!("Error choosing starter to scaffold"),
    }
}
