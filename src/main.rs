use clap::{Parser, Subcommand};
use std::{fs, env};

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
                Ok(()) => { init_repo(); }
                Err(_) => { println!("Tig repository already exists!"); }
            };
        },
        Commands::Status {} => {
            println!("Checking status!");
        },
        Commands::Add { files } => {
            match process_files(&files) {
                Ok(()) => { files.iter().for_each(|f| println!("adding file: {f}")) }
                Err(e) => { println!("Error adding files: {e:?}") }
            }
        }
    }
}

fn init_repo() {
    let current_path; 
    match env::current_exe() {
        Ok(exe_path) => current_path = exe_path.clone(),
        Err(_) => { return }
    };
    println!("initializing tig repo in: {}", current_path.display());
}

fn process_files(files: &Vec<String>) -> Result<(), &str> {
    
    for file in files {
        
    }
    Ok(())
}
