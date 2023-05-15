use std::error::Error;

use git2::Repository;

use std::env;

use crate::data_readers::args_reader::Cli;

use super::dir_deleter::delete_git_folder;

pub fn clone_repo(url: &str, cli: &Cli) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir().unwrap();

    let location_to_clone_into: String = if cli.current_directory {
        println!("\nScaffolding to: {}", &current_dir.display());
        ".".to_string()
    } else {
        match &cli.named_directory {
            Some(name) => {
                println!("\nScaffolding to: {}/{}", &current_dir.display(), name);
                format!("./{}/", name)
            }
            None => panic!("Cloning into named directory but no name was provided!"),
        }
    };

    Repository::clone(url, &location_to_clone_into)?;

    delete_git_folder(location_to_clone_into)?;

    Ok(())
}
