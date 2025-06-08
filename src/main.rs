use clap::{Parser, Subcommand};
use std::{fs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {},
    Status {},
    Add {
        #[arg()]
        files: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Init {} => {
            match fs::create_dir(".tig") {
                Ok(()) => { println!("Initializing an empty repository!"); }
                Err(_) => { println!("Tig repository already exists!"); }
            };
        },
        Commands::Status {} => {
            println!("Checking status!");
        },
        Commands::Add { files } => {
            files
                .iter()
                .for_each(|filename| println!("Adding {filename}"));
        }
    }
}
