use clap::{Parser, Subcommand};
use crate::storage::storage_trait::Storage;
use crate::storage::file_storage::FileStorage;

#[derive(Parser, Debug)]
#[command(author = "Eliran Turgeman", version = "1.0.0", about = "Simple KV-store CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Set {
        #[arg(short, long)]
        key: String,

        #[arg(short, long)]
        value: String
    },
    Get {
        #[arg(short, long)]
        key: String
    }
}

pub(crate) fn main() {
    let storage = FileStorage::new("C:\\Users\\elira\\store.json");

    let cli = Cli::parse();
    match cli.command {
        Commands::Set { key, value } => match storage.set(key, value) {
            Ok(()) => println!("Key set successfully"),
            Err(e) => eprintln!("Failed to set key: {}", e),
        },
        Commands::Get { key } => match storage.get(key) {
            Ok(Some(value)) => println!("Value: {}", value),
            Ok(None) => println!("Key not found"),
            Err(e) => eprintln!("Failed to get key: {}", e),
        },
    }
}
