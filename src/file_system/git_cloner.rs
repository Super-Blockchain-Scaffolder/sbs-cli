use std::error::Error;

use git2::{Repository, RepositoryInitMode, RepositoryInitOptions};

use crate::{
    data_readers::args_reader::Cli, pure_functions::repo_name_extracter::get_repo_name_from_url,
};

pub fn clone_repo(url: &str, cli: &Cli) -> Result<(), Box<dyn Error>> {
    let repo_name = get_repo_name_from_url(url);

    println!("cloning...");

    // let mut clone_opts = CloneOptions::new();
    // clone_opts.flags(GitCloneFlags::FORCE);

    let mut initOptions: RepositoryInitOptions = RepositoryInitOptions::new();
    // .mode(RepositoryInitMode::all());

    // initOptions.mode(RepositoryInitMode::all());

    // initOptions.

    let location_to_clone_into: String = if cli.current_directory {
        ".".to_string()

       
    } else {
        match &cli.named_directory {
            Some(name) => format!("./{}/", name),
            None => panic!("Cloning into named directory but no name was provided!"),
        }
    };


    if cli.current_directory {

        // TODO - figure out how to clone into current directory? 

        //  let mut init_opts = RepositoryInitOptions::new();
        // // init_opts.shared(false);

        // let repo = match Repository::init(".") {
        //     Ok(repo) => repo,
        //     Err(e) => panic!("failed to initialize repository: {}", e),
        // };

        // return match repo.remote("origin", url) {
        //     Ok(mut remote) => match remote.fetch(&["master"], None, None) {
        //         Ok(repo_) => Ok(()),
        //         Err(e) => panic!("failed to fetch: {}", e),
        //     },
        //     Err(e) => panic!("failed to create remote: {}", e),
        // };

    }

    Repository::clone(url, location_to_clone_into)?;
    Ok(())

    // match Repository::clone(url, location_to_clone_into) {
    // Ok(repo) => Ok(repo),
    // Err(err) => {
    // if err.code() == -4 {
    // println!("Something!!")
    // }
    // }
    // }

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
