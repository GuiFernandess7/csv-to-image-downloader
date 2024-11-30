use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct RCArgs {
    #[clap(subcommand)]
    pub mode: Modes
}

#[derive(Subcommand, Debug, Clone)]
pub enum Modes {
    Convert {
        file_path: String,
        output_folder: String,
    },
    Download {
        file_data_path: String,
        output_folder: String,
    },
}
