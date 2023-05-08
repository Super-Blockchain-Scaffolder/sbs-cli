use std::error::Error;

use git2::Repository;

use crate::pure_functions::repo_name_extracter::get_repo_name_from_url;

pub fn clone_repo(
    url: &str,
    location: &str,
    create_dir: bool,
) -> Result<Repository, Box<dyn Error>> {
    let repo_name = get_repo_name_from_url(url);

    println!("cloning...");

    match create_dir {
        true => Ok(Repository::clone(url, &format!("{}{}", location, repo_name))?),
        false => match Repository::clone(url, &format!("{}", location)) {
            Ok(repo) => {
                println!("Clone worked!");
                Ok(repo)
            }
            Err(e) => { 
                println!("Clone didn't work {:?}!", e);
                panic!("Couldn't clone")
            },
        },
    }
}
