use std::error::Error;

pub fn get_repo_name_from_url(url: &str) -> String {
    url.split("/")
        .last()
        .expect(&format!("url \"{}\" contains no repo name...", url))
        .to_string()
}

#[test]
fn gets_repo_name() -> Result<(), Box<dyn Error>> {
    // use crate::pure_functions::repo_name_extracter::get_repo_name;

    use crate::pure_functions::repo_name_extracter::get_repo_name_from_url;

    let example_url = "https://github.com/vivainio/rraf";
    let expected = "rraf";

    assert_eq!(expected, &get_repo_name_from_url(example_url));

    Ok(())
}