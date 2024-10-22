use clap::{Parser, Subcommand};

use repo::Repo;

mod blob;
mod bytes_reader;
mod codec;
mod hash;
mod input_output;
mod object;
#[cfg(test)]
mod reference_impl;
mod repo;
#[cfg(test)]
mod test_utils;
mod tree_node;

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
    HashObject {
        #[arg(short, required = true)]
        write: bool,
        file: String,
    },
    LsTree {
        #[arg(long)]
        name_only: bool,
        tree_ish: String,
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
        Commands::HashObject { file, .. } => {
            repo.hash_object(&file);
        }
        Commands::LsTree {
            name_only,
            tree_ish,
        } => {
            repo.ls_tree(name_only, &tree_ish);
        }
    }
}
