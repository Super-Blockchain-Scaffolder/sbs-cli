use clap::Parser;

#[derive(Parser)]
pub struct Cli {

    #[arg(short, long)]
    pub current_directory: Option<bool>,

    #[arg(short, long)]
    pub named_directory: Option<String>,

    #[arg(short, long)]
    pub starter_template: Option<String>,
}

pub fn read_args() -> Cli {
    Cli::parse()
}