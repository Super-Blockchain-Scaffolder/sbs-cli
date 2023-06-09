use git2::{FetchOptions, RemoteCallbacks};
use std::env;
use std::error::Error;
use std::fs;
use std::io::stdout;
use std::io::Write;
use tempfile;

use super::dir_deleter::delete_git_folder;
use crate::data_readers::args_reader::Cli;

pub fn clone_repo(url: &str, cli: &Cli) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut location_to_clone_into: String = if cli.current_directory {
        println!("\nScaffolding to: {}\n", &current_dir.display());
        ".".to_string()
    } else {
        match &cli.named_directory {
            Some(name) => {
                println!("\nScaffolding to: {}/{}\n", &current_dir.display(), name);
                format!("./{}/", name)
            }
            None => panic!("Cloning into named directory but no name was provided!"),
        }
    };
    
    let mut location_to_clone_into_path = std::path::Path::new(&location_to_clone_into);

    // Create a RemoteCallbacks struct with a progress callback
    let mut callbacks = RemoteCallbacks::new();
    let mut progress_complete = false;

    callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            // print!("Resolving deltas {}/{}\r", stats.indexed_deltas(), stats.total_deltas());
        } else if stats.total_objects() > 0 {
            let progress = (stats.received_objects() as f32 / stats.total_objects() as f32) * 100.0;
            print!("\r[");
            for i in 0..50 {
                if i as f32 <= progress / 2.0 {
                    print!("=");
                } else if i as f32 <= progress {
                    print!(">");
                } else {
                    print!(" ");
                }
            }
            print!("] {:.2}%\r", progress);
        }
        stdout().flush().unwrap();
    
        // Check if the received objects are equal to the total objects and print 100% progress
        if stats.received_objects() == stats.total_objects() && !progress_complete{
            print!("\r[");
            for _ in 0..50 {
                print!("=");
            }
            print!("] 100.00%\n");
            stdout().flush().unwrap();
            progress_complete = true;

        }
    
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
