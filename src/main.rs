fn main() {

    let args_passed_in = read_args();

    validate_args(&args_passed_in);

    let full_args = prompt_for_needed_data(args_passed_in);

    let starter_data = get_starter_data_from_name(full_args.starter_template);

    if full_args.current_directory {
        git_cloner.clone_repo_in_current_dir(starter_data);
    } else {
        git_cloner.clone_repo_in_named_dir(starter_data, dir_name);
    }

    git_deleter.delete_git_folder()

    println!("Successfully scaffolded!");
}
