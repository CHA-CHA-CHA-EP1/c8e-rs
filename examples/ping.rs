use c8e_rs::C8e;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment variables or use defaults
    let domain =
        std::env::var("CONFLUENCE_DOMAIN").unwrap_or_else(|_| "example.atlassian.net".to_string());
    let email =
        std::env::var("CONFLUENCE_EMAIL").unwrap_or_else(|_| "your-email@example.com".to_string());
    let token = std::env::var("CONFLUENCE_TOKEN").unwrap_or_else(|_| "your-api-token".to_string());

    println!("🔗 Connecting to Confluence: {}", domain);
    println!("📧 Using email: {}", email);

    // Create the Confluence client
    let client = C8e::new(&domain)
        .auth(email, token)
        .timeout(Duration::from_secs(30))
        .build()?;

    println!("🏓 Testing connection...");

    // Test the connection
    match client.ping().await {
        Ok(true) => {
            println!("✅ Connection successful! Confluence API is reachable.");
        }
        Ok(false) => {
            println!("❌ Connection failed! Check your domain and credentials.");
        }
        Err(e) => {
            println!("💥 Error occurred: {}", e);
        }
    }

    Ok(())
}

