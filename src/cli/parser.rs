use clap::{Parser, Subcommand};
use crate::storage::storage_trait::Storage; 
use crate::storage::file_storage::FileStorage;
use crate::risk_analyzer::hibp_risk_analyzer::HIBPRiskAnalyzer;
use crate::risk_analyzer::risk_analyzer_trait::RiskAnalyzer;


#[derive(Parser, Debug)]
#[command(author = "Eliran Turgeman", version = "1.0.0", about = "Simple password-store CLI", long_about = None)]
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
    },
    Analyze {
        #[arg(short, long)]
        key: Option<String>
    }
}

#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::Analyze { key } => {
            match key {
                Some(value) => {
                    let password_result = storage.get(value.clone());
                    match password_result {
                        Ok(Some(password)) => {
                            let analyzer = HIBPRiskAnalyzer{};
                            let compromised = analyzer.check_password(&password).await?;
                            if compromised{
                                println!("Password for {} is compromised!", value);
                            } else {
                                println!("Password for {} is safe.", value);
                            }
                        },
                        Ok(None) => println!("Key not found."),
                        Err(e) => eprintln!("Failed to get key: {}", e),
                    }
                },
                None => {
                    let passwords_result = storage.get_all();
                    if let Ok(Some(passwords)) = passwords_result {
                        let password_refs: Vec<&str> = passwords.iter().map(AsRef::as_ref).collect();
                        let analyzer = HIBPRiskAnalyzer{};
                        analyzer.check_all_passwords(password_refs).await?;
                    } else {
                        println!("No passwords to scan.");
                    }
                },
            }
        }
    }
    Ok(())
}
