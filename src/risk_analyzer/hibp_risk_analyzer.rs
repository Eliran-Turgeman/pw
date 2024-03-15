use super::risk_analyzer_trait::RiskAnalyzer;
use anyhow::Result;
use async_trait::async_trait;
use sha1::{Digest, Sha1};

pub struct HIBPRiskAnalyzer;

impl HIBPRiskAnalyzer {
    fn hash_password(&self, password: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        format!("{:X}", result)
    }
}

#[async_trait]
impl RiskAnalyzer for HIBPRiskAnalyzer {
    async fn check_password(&self, password: &str) -> Result<bool, anyhow::Error> {
        let hashed_password_hex = self.hash_password(password);
        let prefix = &hashed_password_hex[..5];
        let suffix = &hashed_password_hex[5..];

        let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?.text().await?;

        if resp.lines().any(|line| line.ends_with(suffix)) {
            println!("Password '{}' is compromised!", password);
            Ok(true)
        } else {
            println!("Password '{}' is safe.", password);
            Ok(false)
        }
    }
}
