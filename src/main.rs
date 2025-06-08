use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg()]
        files: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Add { files } => {
            files
                .iter()
                .for_each(|filename| println!("Adding {filename}"));
        }
    }
}
