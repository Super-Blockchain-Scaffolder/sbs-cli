mod data_readers {
    pub(super) mod args_reader;
    pub(super) mod master_list_reader;
}

mod pure_functions {
    pub(super) mod starter_finder;
    pub(super) mod repo_name_extracter;
}

mod file_system {
    pub(super) mod git_cloner;
}

use std::error::Error;
use serde_yaml::{to_string, Value};

use crate::data_readers::args_reader::read_args;
use crate::data_readers::master_list_reader::get_master_list_data;

use crate::pure_functions::starter_finder::get_starter_data_from_list_by_name;

use crate::file_system::git_cloner::clone_repo;

fn main() -> Result<(), Box<dyn Error>> {
    let args_passed_in = read_args();

    println!("{:?}", args_passed_in);

    // validate_args(&args_passed_in);

    // let full_args = prompt_for_needed_data(args_passed_in);

    let master_list_data = get_master_list_data()?;

    println!("{:?}", master_list_data);
    
    let starters_list = master_list_data.get("starter-templates").unwrap();
    println!("{:?}", starters_list);

    let name_to_find = match args_passed_in.starter_template {
        Some(template_name) => template_name,
        None => panic!("Couldn't find any starter template name!")
    };

    let starter_data = get_starter_data_from_list_by_name(starters_list, &name_to_find)?;

    println!("starter data: {:?}", starter_data);

    let repo_url_with_newline = to_string(starter_data.get("repo-url").unwrap()).unwrap();
    let repo_url = repo_url_with_newline.trim_end_matches("\n");


    println!("repo_url: {}", repo_url);

    clone_repo(&repo_url, "./")?;

    println!("cloned!");

    // if full_args.current_directory {
    //     git_cloner.clone_repo_in_current_dir(starter_data.repo_url);
    // } else {
    //     git_cloner.clone_repo_in_named_dir(starter_data, dir_name);
    // }

    // git_deleter.delete_git_folder()

    // println!("Successfully scaffolded!");

    Ok(())
}
