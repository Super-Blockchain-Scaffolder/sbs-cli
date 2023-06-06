use git2::{FetchOptions, RemoteCallbacks};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use tempfile;

use super::dir_deleter::delete_git_folder;
use crate::data_readers::args_reader::Cli;

pub fn clone_repo(url: &str, cli: &Cli) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut location_to_clone_into: String = if cli.current_directory {
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
    let mut location_to_clone_into_path = std::path::Path::new(&location_to_clone_into);

    // Create a RemoteCallbacks struct with a progress callback
    let mut callbacks = RemoteCallbacks::new();
    callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        std::io::stdout().flush().unwrap();
        true
    });

    // Set up authentication and fetch options
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    fetch_options.download_tags(git2::AutotagOption::All);

    // Create a builder for the clone operation
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    // Clone the repository
    if cli.current_directory {
        // Make a new directory
        let temp_dir = tempfile::Builder::new()
            .prefix("sbs_tmp")
            .tempdir()
            .expect("Couldn't create temporary directory!");

        location_to_clone_into = temp_dir.into_path().to_string_lossy().to_string();
        location_to_clone_into_path = std::path::Path::new(&location_to_clone_into);
        println!("\nTemporary directory created: {}", location_to_clone_into);
        builder.clone(url, &location_to_clone_into_path)?;

        // Copy and replace the files from the sbs_tmp directory to the current directory
        copy_files(&location_to_clone_into, &current_dir)?;
    } else {
        builder.clone(url, &location_to_clone_into_path)?;
    }

    // Convert the location_to_clone_into String to a Path

    delete_git_folder(location_to_clone_into)?;

    Ok(())
}

fn copy_files(source_dir: &str, destination_dir: &std::path::Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination_dir.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&destination_path)?;
            copy_files(&source_path.to_string_lossy(), &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}
