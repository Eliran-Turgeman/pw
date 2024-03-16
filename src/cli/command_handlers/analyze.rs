use crate::storage::storage_trait::Storage;
use crate::risk_analyzer::hibp_risk_analyzer::HIBPRiskAnalyzer;
use crate::risk_analyzer::risk_analyzer_trait::RiskAnalyzer;

pub async fn analyze_handler(
    key: Option<String>,
    storage: &dyn Storage,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(value) = key {
        let password_result = storage.get(value.clone())?;
        if let Some(password) = password_result {
            let analyzer = HIBPRiskAnalyzer {};
            let compromised = analyzer.check_password(&password).await?;
            if compromised {
                println!("Password for {} is compromised!", value);
            } else {
                println!("Password for {} is safe.", value);
            }
        } else {
            println!("Key not found.");
        }
    } else {
        let passwords_result = storage.get_all()?;
        if let Some(passwords) = passwords_result {
            let password_refs: Vec<&str> = passwords.iter().map(AsRef::as_ref).collect();
            let analyzer = HIBPRiskAnalyzer {};
            analyzer.check_all_passwords(password_refs).await?;
        } else {
            println!("No passwords to scan.");
        }
    }
    Ok(())
}
