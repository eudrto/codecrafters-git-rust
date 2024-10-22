use clap::{Parser, Subcommand};

use repo::Repo;

mod blob;
mod bytes_reader;
mod codec;
mod input_output;
mod object;
#[cfg(test)]
mod reference_impl;
mod repo;
#[cfg(test)]
mod test_utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    CatFile {
        #[arg(short, required = true)]
        pretty: bool,
        object: String,
    },
}

pub fn run() {
    let cli = Cli::parse();
    let repo = Repo::new_current_dir();

    match cli.command {
        Commands::Init => {
            repo.init();
        }
        Commands::CatFile { object, .. } => {
            repo.cat_file(&object);
        }
    }
}
