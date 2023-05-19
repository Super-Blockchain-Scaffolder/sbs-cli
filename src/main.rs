mod console_output {
    pub(super) mod ascii_art;
}

mod data_readers {
    pub(super) mod args_reader;
    pub(super) mod master_list_reader;
}

mod pure_functions {
    pub(super) mod starter_finder;
    pub(super) mod validate_cli_args;
}

mod file_system {
    pub(super) mod git_cloner;
    pub(super) mod dir_deleter;
}

mod inquire_prompts {
    pub(super) mod clone_overwrite_confirm;
    pub(super) mod current_dir_confirm;
    pub(super) mod folder_name_prompt;
    pub(super) mod starter_select_prompt;
}

use serde_yaml::to_string;
use std::error::Error;

use crate::console_output::ascii_art::print_sbs_ascii_art;

use crate::data_readers::args_reader::read_args;
use crate::data_readers::master_list_reader::get_master_list_data;

use crate::pure_functions::starter_finder::get_starter_data_from_list_by_name;
use crate::pure_functions::validate_cli_args::validate_cli_args;

use crate::file_system::git_cloner::clone_repo;

mod prompt_manager;
use prompt_manager::{
    prompt_current_directory_if_needed, prompt_directory_name_if_needed,
    prompt_starter_name_if_needed,
};

fn main() -> Result<(), Box<dyn Error>> {
    
    let args_passed_in = read_args();
    
    if !args_passed_in.art_skipped {
        print_sbs_ascii_art()
    };

    println!("\nLet's scaffold a new blockchain project!");

    validate_cli_args(&args_passed_in)?;

    let master_list_data = get_master_list_data()?;

    // let starters_list = master_list_data.get("starter-templates").unwrap();

    let args_unknown_dir_or_name = prompt_current_directory_if_needed(args_passed_in)?;
    let args_unknown_name = prompt_directory_name_if_needed(args_unknown_dir_or_name)?;
    let full_args = prompt_starter_name_if_needed(&args_unknown_name, &master_list_data)?;

    let name_to_find = match &full_args.starter_template {
        Some(template_name) => template_name,
        None => panic!("Couldn't find any starter template name!"),
    };

    let starter_data = get_starter_data_from_list_by_name(&master_list_data, &name_to_find)?;

    let repo_url_with_newline = to_string(starter_data.get("repo-url").unwrap()).unwrap();
    let repo_url = repo_url_with_newline.trim_end_matches("\n");

    clone_repo(&repo_url, &full_args)?;

    println!("\nSuccessfully scaffolded!\n");

    Ok(())
}
