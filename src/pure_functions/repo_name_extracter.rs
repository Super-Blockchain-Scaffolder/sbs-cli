pub fn get_repo_name_from_url(url: &str) -> String {
    url.split("/")
        .last()
        .expect(&format!("url \"{}\" contains no repo name...", url))
        .to_string()
}

#[cfg(test)]
mod get_repo_name_tests {
    use std::error::Error;

    #[test]
    fn extracts_repo_name_from_full_url() -> Result<(), Box<dyn Error>> {
        use crate::pure_functions::repo_name_extracter::get_repo_name_from_url;

        let example_url = "https://github.com/vivainio/rraf";
        let expected = "rraf";

        assert_eq!(expected, &get_repo_name_from_url(example_url));

        Ok(())
    }
}
