use crate::password_generator::generator::generate_strong_password;
use crate::risk_analyzer::hibp_risk_analyzer::HIBPRiskAnalyzer;
use crate::risk_analyzer::risk_analyzer_trait::RiskAnalyzer;
use crate::storage::file_storage::FileStorage;
use crate::storage::storage_trait::Storage;
use clap::{Parser, Subcommand};
use dirs;

/// Simple Password-Store CLI
///
/// This CLI tool allows you to securely manage your passwords. It provides functionalities
/// to set new passwords, retrieve existing passwords, analyze passwords for compromises,
/// and generate strong passwords automatically.
#[derive(Parser, Debug)]
#[command(author = "Eliran Turgeman", version = "1.0.0", about = "A simple CLI for managing passwords.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Sets a new password for a given key.
    ///
    /// This command allows you to securely store a new password under a specified key.
    /// If the key already exists, its password will be overwritten.
    Set {
        #[arg(
            short,
            long,
            help = "The unique key under which to store the password."
        )]
        key: String,

        #[arg(short, long, help = "The password to store.")]
        value: String,
    },

    /// Retrieves a password by key.
    ///
    /// This command retrieves and displays the password stored under the specified key.
    /// If the key does not exist, an error message will be shown.
    Get {
        #[arg(short, long, help = "The key for which to retrieve the password.")]
        key: String,
    },

    /// Analyzes passwords for potential compromises.
    ///
    /// This command checks if the password(s) stored under the specified key, or all passwords
    /// if no key is provided, have been compromised in known data breaches. It uses the
    /// "Have I Been Pwned" API to perform the analysis.
    Analyze {
        #[arg(
            short,
            long,
            help = "The key of the password to analyze. If omitted, all passwords are analyzed."
        )]
        key: Option<String>,
    },

    /// Generates a strong, random password.
    ///
    /// This command generates a strong password of a specified length. The generated password
    /// is displayed but not stored. Use the 'set' command to store it if desired.
    Generate {
        #[arg(
            short,
            long,
            help = "Optionally specify a key to directly store the generated password."
        )]
        key: Option<String>,

        #[arg(
            short,
            long,
            default_value_t = 12,
            help = "The length of the password to generate. Defaults to 12 characters."
        )]
        length: usize,
    },
}

fn init_storage() -> Result<FileStorage, Box<dyn std::error::Error>> {
    let storage_path = dirs::home_dir();
    match storage_path {
        Some(value) => {
            let path = format!("{}/.pw/store.json", value.display());
            Ok(FileStorage::new(&path))
        },
        None => {
            eprintln!("Could not find the home directory");
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not find the home directory")))
        }
    }
}


#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let storage = init_storage()?;

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
        Commands::Analyze { key } => match key {
            Some(value) => {
                let password_result = storage.get(value.clone());
                match password_result {
                    Ok(Some(password)) => {
                        let analyzer = HIBPRiskAnalyzer {};
                        let compromised = analyzer.check_password(&password).await?;
                        if compromised {
                            println!("Password for {} is compromised!", value);
                        } else {
                            println!("Password for {} is safe.", value);
                        }
                    }
                    Ok(None) => println!("Key not found."),
                    Err(e) => eprintln!("Failed to get key: {}", e),
                }
            }
            None => {
                let passwords_result = storage.get_all();
                if let Ok(Some(passwords)) = passwords_result {
                    let password_refs: Vec<&str> = passwords.iter().map(AsRef::as_ref).collect();
                    let analyzer = HIBPRiskAnalyzer {};
                    analyzer.check_all_passwords(password_refs).await?;
                } else {
                    println!("No passwords to scan.");
                }
            }
        },
        Commands::Generate { key, length } => match key {
            Some(value) => {
                let password = generate_strong_password(length);
                match storage.set(value.clone(), password.clone()) {
                    Ok(()) => println!(
                        "Password generated: {}, and saved under key '{}'",
                        password, value
                    ),
                    Err(e) => eprintln!("Failed to set key: {}", e),
                }
            }
            None => {
                let password = generate_strong_password(length);
                println!("Password generated: {}", password);
            }
        },
    }
    Ok(())
}
