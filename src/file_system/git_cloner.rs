use std::error::Error;

use git2::Repository;

use crate::{
    data_readers::args_reader::Cli, pure_functions::repo_name_extracter::get_repo_name_from_url,
};

pub fn clone_repo(url: &str, cli: &Cli) -> Result<Repository, Box<dyn Error>> {
    let repo_name = get_repo_name_from_url(url);

    println!("cloning...");

    let location_to_clone_into: String = if cli.current_directory {
        "./".to_string()
    } else {
        match &cli.named_directory {
            Some(name) => format!("./{}/", name),
            None => panic!("Cloning into named directory but no name was provided!"),
        }
    };

    Ok(Repository::clone(url, location_to_clone_into)?)

    // match create_dir {
    //     true => Ok(Repository::clone(url, &format!("{}{}", location, repo_name))?),
    //     false => match Repository::clone(url, &format!("{}", location)) {
    //         Ok(repo) => {
    //             println!("Clone worked!");
    //             Ok(repo)
    //         }
    //         Err(e) => {
    //             println!("Clone didn't work {:?}!", e);
    //             panic!("Couldn't clone")
    //         },
    //     },
    // }
}
