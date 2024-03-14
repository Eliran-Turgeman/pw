use async_trait::async_trait;
use anyhow::{Result, Context};
use futures::future::try_join_all;


#[async_trait]
pub trait RiskAnalyzer {
    async fn check_password(&self, password: &str) -> Result<bool, anyhow::Error>;
    
    async fn check_all_passwords(&self, passwords: Vec<&str>) -> Result<(), anyhow::Error> {
        let check_futures = passwords.into_iter()
            .map(|password| self.check_password(password))
            .collect::<Vec<_>>();
        try_join_all(check_futures).await.context("Failed to check all passwords")?;

        Ok(())
    }
}
