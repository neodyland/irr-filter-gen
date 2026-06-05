pub mod irr;

#[cfg(test)]
mod tests {
    use crate::irr::IRRClient;

    #[tokio::test]
    async fn test_irr_client() -> anyhow::Result<()> {
        let mut client = IRRClient::connect("whois.radb.net:43").await?;
        let routes = client.route(15169).await?;

        assert!(!routes.is_empty());
        Ok(())
    }
}