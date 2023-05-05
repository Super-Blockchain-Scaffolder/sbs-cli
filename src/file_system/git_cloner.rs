use std::error::Error;

use git2::Repository;

use crate::pure_functions::repo_name_extracter::get_repo_name_from_url;

pub fn clone_repo(
    url: &str,
    location: &str,
    // repo_name: &str,
) -> Result<Repository, Box<dyn Error>> {

    let repo_name = get_repo_name_from_url(url);

    println!("cloning...");

    Ok(Repository::clone(
        url,
        &format!("{}", location),
        // &format!("{}{}", location, "foo"),
    )?)
}