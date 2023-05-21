use clap::Parser;

///  Command line utility for scaffolding blockchain-related projects!
#[derive(Parser, Debug, Clone)]
#[command(author, about, long_about = None)]
pub struct Cli {
    ///  Whether or not you want to scaffold in the current directory.
    #[arg(short, long)]
    pub current_directory: bool,

    ///  The name of a folder to create for scaffolding projects into.  
    #[arg(short, long)]
    pub named_directory: Option<String>,

    ///  Which template you want to chose.
    #[arg(short, long)]
    pub starter_template: Option<String>,

    ///  Whether or not you want the artowrk omitted.
    #[arg(short, long)]
    pub art_skipped: bool,

    ///  Currently installed version of the sbs-cli.
    #[arg(short, long)]
    pub version: bool, 
    
    // TODO
    // #[arg(short, long)]
    // pub list_sync_skipped: bool
}

pub fn read_args() -> Cli {
    Cli::parse()
}
