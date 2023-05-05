use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub current_directory: bool,

    #[arg(short, long)]
    pub named_directory: Option<String>,

    #[arg(short, long)]
    pub starter_template: Option<String>,

    #[arg(short, long)]
    pub art_skipped: bool
    
    // TODO
    // #[arg(short, long)]
    // pub list_sync_skipped: bool
}

pub fn read_args() -> Cli {
    Cli::parse()
}
