use crate::data_readers::args_reader::Cli;
use crate::inquire_prompts::clone_overwrite_confirm::ask_to_overwrite_in_current_dir;
use serde_yaml::{to_string, Value};
use std::error::Error;
use std::fs;

use crate::inquire_prompts::current_dir_confirm::ask_to_scaffold_in_current_dir;
use crate::inquire_prompts::folder_name_prompt::ask_what_to_call_named_dir;
use crate::inquire_prompts::starter_select_prompt::ask_which_starter_to_scaffold;

pub fn prompt_current_directory_if_needed(cli: Cli) -> Result<Cli, Box<dyn Error>> {
    let mut new_cli: Cli = cli.clone();

    if cli.current_directory {
        // trying to scaffold in current directory
        // just pass along to next step
        prompt_for_current_dir_files_overwrite_if_needed();
    } else {
        match &new_cli.named_directory {
            Some(_name) => (), // just pass along to next step
            None => {
                let new_current_dir_bool = ask_to_scaffold_in_current_dir()?;
                new_cli.current_directory = new_current_dir_bool;

                if new_current_dir_bool {
                    prompt_for_current_dir_files_overwrite_if_needed();
                }
            }
        }
    }

    Ok(new_cli)
}

pub fn prompt_for_current_dir_files_overwrite_if_needed() {
    let current_dir = ".";
    let has_files = fs::read_dir(current_dir).unwrap().next().is_some();

    if has_files {
        println!("There are files in the current directory");
        match ask_to_overwrite_in_current_dir() {
            Ok(true) => panic!("Sorry, not yet supported... 😅"),
            Ok(false) => panic!("Cloning in the current directory without overwriting files is currently unsupported. [insert link to open github issue here]"),
            Err(_err) => panic!("Error confirming file overwrite for current directory scaffold")
        };
    } else {
        println!("There are no files in the current directory");
    }
}

pub fn prompt_directory_name_if_needed(cli: Cli) -> Result<Cli, Box<dyn Error>> {
    let mut new_cli: Cli = cli.clone();

    if !cli.current_directory && cli.named_directory.is_none() {
        new_cli.named_directory = Some(ask_what_to_call_named_dir()?);
    }

    Ok(new_cli)
}

pub fn prompt_starter_name_if_needed(
    cli: &Cli,
    starters_list: &Value,
) -> Result<Cli, Box<dyn Error>> {
    let mut new_cli: Cli = cli.clone();

    let starters_seq = starters_list.as_sequence().unwrap();

    let starter_names = starters_seq
        .iter()
        .map(|starter: &Value| {
            let name = to_string(starter.get("name").unwrap()).unwrap();
            name.trim_end_matches("\n").to_string()
        })
        .collect();

    match &cli.starter_template {
        Some(_starter_name) => (), // just pass along to next step
        None => {
            let starter_name = ask_which_starter_to_scaffold(&starter_names)?;
            new_cli.starter_template = Some(starter_name);
        }
    }

    Ok(new_cli)
}
