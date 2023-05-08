use crate::data_readers::args_reader::Cli;
use std::error::Error;

/// Does NOT allow both named_directory value and current_directory true
pub fn validate_cli_args(cli: &Cli) -> Result<(), Box<dyn Error>> {
    if cli.current_directory && cli.named_directory.is_some() {
        panic!("Can't use both named directory and current directory flags!")
    }

    Ok(())
}

#[cfg(test)]
mod validate_cli_args_tests {
    use crate::data_readers::args_reader::Cli;
    use std::error::Error;

    use crate::pure_functions::validate_cli_args::validate_cli_args;

    #[test]
    fn doesnt_error_when_shouldnt_error() -> Result<(), Box<dyn Error>> {
        // current_dir false, no named dir
        validate_cli_args(&Cli {
            current_directory: false,
            named_directory: None,
            starter_template: None,
            art_skipped: false,
        })?;

        // current_dir true, no named dir
        validate_cli_args(&Cli {
            current_directory: true,
            named_directory: None,
            starter_template: None,
            art_skipped: false,
        })?;

        // current_dir false, named dir
        validate_cli_args(&Cli {
            current_directory: false,
            named_directory: Some("foo".to_string()),
            starter_template: None,
            art_skipped: false,
        })?;

        Ok(())
    }

    #[test]
    fn errors_with_both_current_dir_and_named_dir() -> Result<(), Box<dyn Error>> {
        // current_dir true AND named dir
        let result = std::panic::catch_unwind(|| {
            validate_cli_args(&Cli {
                current_directory: true,
                named_directory: Some("foo".to_string()),
                starter_template: None,
                art_skipped: false,
            })
        });

        assert!(result.is_err());

        Ok(())
    }
}
