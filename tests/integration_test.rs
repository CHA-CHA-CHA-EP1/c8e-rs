use c8e_rs::C8e;
use std::time::Duration;

#[cfg(test)]
mod integration_tests {
    use super::*;

    // Helper function to get test credentials from environment
    fn get_test_credentials() -> Option<(String, String, String)> {
        let domain = std::env::var("CONFLUENCE_DOMAIN").ok()?;
        let email = std::env::var("CONFLUENCE_EMAIL").ok()?;
        let token = std::env::var("CONFLUENCE_TOKEN").ok()?;
        Some((domain, email, token))
    }

    #[tokio::test]
    #[ignore] // Use --ignored flag to run: cargo test -- --ignored
    async fn test_real_confluence_ping() {
        if let Some((domain, email, token)) = get_test_credentials() {
            let client = C8e::new(&domain)
                .auth(email, token)
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build client");

            let result = client.ping().await;
            assert!(result.is_ok(), "Ping should succeed with valid credentials");
            assert!(
                result.unwrap(),
                "Ping should return true for successful connection"
            );
        } else {
            println!("Skipping real API test - set CONFLUENCE_DOMAIN, CONFLUENCE_EMAIL, CONFLUENCE_TOKEN environment variables");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_domain_ping() {
        let client = C8e::new("invalid-domain-that-does-not-exist.com")
            .auth("test@example.com", "fake-token")
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to build client");

        let result = client.ping().await;
        assert!(
            result.is_ok(),
            "Request should complete even for invalid domain"
        );
        assert!(
            !result.unwrap(),
            "Ping should return false for invalid domain"
        );
    }

    #[test]
    fn test_builder_missing_domain() {
        let result = C8e::new("").auth("test@example.com", "token").build();

        assert!(result.is_err(), "Should fail when domain is empty");
    }

    #[test]
    fn test_builder_missing_auth() {
        let result = C8e::new("example.atlassian.net").build();
        assert!(
            result.is_err(),
            "Should fail when authentication is missing"
        );
    }

    #[test]
    fn test_url_construction() {
        // Test domain without protocol
        let client1 = C8e::new("example.atlassian.net")
            .auth("test@example.com", "token")
            .build()
            .unwrap();

        // Test domain with https protocol
        let client2 = C8e::new("https://example.atlassian.net")
            .auth("test@example.com", "token")
            .build()
            .unwrap();

        // Both should work (we can't directly test the base_url since it's private,
        // but the build() should succeed)
        assert!(format!("{:?}", client1).contains("example.atlassian.net"));
        assert!(format!("{:?}", client2).contains("example.atlassian.net"));
    }
}

