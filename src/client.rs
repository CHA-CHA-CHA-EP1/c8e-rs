use reqwest;
use std::time::Duration;

/// Main Confluence API client (C8e = ConfluencE)
#[derive(Debug, Clone)]
pub struct C8e {
    base_url: String,
    http_client: reqwest::Client,
    email: String,
    token: String,
}

/// Builder for creating C8e client
#[derive(Debug, Default)]
pub struct ConfluenceBuilder {
    domain: Option<String>,
    email: Option<String>,
    token: Option<String>,
    timeout: Option<Duration>,
}

impl C8e {
    /// Create a new client with domain (chainable)
    pub fn new(domain: &str) -> ConfluenceBuilder {
        ConfluenceBuilder::default().domain(domain)
    }

    /// Test connection to Confluence API v2 (requires authentication)
    pub async fn ping(&self) -> Result<bool, reqwest::Error> {
        // Using v2 API path: /wiki/api/v2/pages (requires auth)
        let url = format!("{}/wiki/api/v2/pages?limit=1", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .basic_auth(&self.email, Some(&self.token))
            .send()
            .await?;
        Ok(response.status().is_success())
    }
}

impl ConfluenceBuilder {
    /// Set the Confluence domain
    pub fn domain<S: Into<String>>(mut self, domain: S) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Set authentication credentials
    pub fn auth<S: Into<String>>(mut self, email: S, token: S) -> Self {
        self.email = Some(email.into());
        self.token = Some(token.into());
        self
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the C8e client
    pub fn build(self) -> Result<C8e, Box<dyn std::error::Error>> {
        let domain = self.domain.ok_or("Domain is required")?;
        if domain.is_empty() {
            return Err("Domain cannot be empty".into());
        }
        let email = self.email.ok_or("Email is required")?;
        let token = self.token.ok_or("API token is required")?;

        // Create HTTP client with basic auth
        let mut client_builder = reqwest::Client::builder().user_agent("c8e-rs/0.1.0");

        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        let http_client = client_builder.build()?;

        // Construct base URL from domain
        let base_url = if domain.starts_with("http://") || domain.starts_with("https://") {
            domain
        } else {
            format!("https://{}", domain)
        };

        Ok(C8e {
            base_url,
            http_client,
            email,
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_domain() {
        let client = C8e::new("suthiphong-thaisuriya.atlassian.net")
            .auth("test@example.com", "token123")
            .build();

        assert!(client.is_ok());
    }

    #[test]
    fn test_full_url() {
        let client = C8e::new("https://suthiphong-thaisuriya.atlassian.net")
            .auth("test@example.com", "token123")
            .build();

        assert!(client.is_ok());
    }

    #[test]
    fn test_with_timeout() {
        let client = C8e::new("suthiphong-thaisuriya.atlassian.net")
            .auth("test@example.com", "token123")
            .timeout(Duration::from_secs(30))
            .build();

        assert!(client.is_ok());
    }
}
